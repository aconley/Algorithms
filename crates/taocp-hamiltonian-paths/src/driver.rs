//! The CEGAR loop itself.
//!
//! Internal to this module tree.  The public entry points in
//! [`super`](super) drive it and expose only the answer.  Everything here is
//! steppable because the *sequence of rounds* is the interesting object: it is
//! what per-round rendering and any experiment comparing refinement strategies
//! need to observe.
//!
//! The loop is the usual shape.  An abstraction of "Hamiltonian cycle" — a
//! *cycle cover* — is encoded into CNF and solved.  If it is unsatisfiable, no
//! Hamiltonian cycle exists and that answer is conclusive.  If it is
//! satisfiable, the model is decoded into a `CycleCover`; when that cover is a
//! single spanning cycle the search is done, and otherwise it is spurious and
//! cut clauses ruling it out are added before solving again.
//!
//! The loop works on the cover's `SUCC`/`PRED`/`CID` arrays throughout, as
//! Knuth's Algorithm C does.  [`Decomposition`] is the ordered-segment *view* of
//! that state, built only when [`CegarSearch::decomposition`] is called.
//!
//! Refinement here is **monotone** — every round only adds clauses and never
//! retracts one — so every clause CaDiCaL learned in an earlier round stays
//! sound, and the ordinary incremental interface gives the desired "resume from
//! accumulated state" behaviour with no extra machinery.  Do not build any
//! clause-retraction mechanism.
//!
//! The abstraction and the refinement rule are **not** specified here.  They
//! come from Knuth and live in sibling modules: `encoding` builds the cycle
//! cover formula (C1, C2), `cycles` decodes and merges covers (C4, C6), and
//! `refinement` derives the cut clauses (C8).

use super::cycles::CycleCover;
use super::encoding::{cycle_cover_cnf, ArcVars};
use super::precheck::{check_preconditions, Obstruction};
use super::refinement::cut_clauses;
use super::segment::{Decomposition, Segment};
use petgraph::graph::UnGraph;
use rustsat::solvers::{
    FreezeVar, GetInternalStats, LimitConflicts, Solve, SolverResult,
};
use rustsat_cadical::CaDiCaL;
use std::time::Duration;

/// Knobs on a single search.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Config {
    /// Give up after this many refinement rounds, if set.
    pub max_rounds: Option<usize>,
    /// Give up after this many solver conflicts in total, if set.
    pub max_conflicts: Option<usize>,
    /// Skip precondition checks and drive rejected graphs through the SAT solver.
    pub skip_preconditions: bool,
    /// Merge adjacent cycles of a spurious cover (step C6) before refining.
    ///
    /// On by default.  Turning it off is what makes the merge an A/B
    /// experiment rather than an assumption: it is an optimisation, so both
    /// settings must reach the same answer on every graph.
    pub merge_cycles: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_rounds: None,
            max_conflicts: None,
            skip_preconditions: false,
            merge_cycles: true,
        }
    }
}

/// What one refinement round did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Step {
    /// The abstraction admitted a model that is not a single spanning cycle.
    /// Clauses ruling it out have already been added; call `step` again.
    ///
    /// Carries only the number of cycles in the cover, which is Knuth's `t` and
    /// therefore free.  The loop itself never needs the ordered-segment view; a
    /// caller that does — a renderer, a test — asks for one with
    /// [`CegarSearch::decomposition`].
    ///
    /// This is the count *after* step C6's merging, which is the cover the cut
    /// clauses were built from and the one `decomposition` derives from.  The
    /// pre-merge count is in [`Stats::segments_before_merge_per_round`].
    Spurious { cycles: usize },
    /// A genuine Hamiltonian cycle.  The search is finished.
    Found(Segment),
    /// The abstraction is unsatisfiable, so no Hamiltonian cycle exists.  This
    /// is conclusive, not a failure to find one.
    NoCycle,
    /// A configured limit was hit before the question was settled.
    LimitReached,
}

/// Counters accumulated across a search.
///
/// These are not incidental instrumentation.  Round counts and clause growth are
/// what show whether one refinement rule beats another, which is the substance
/// of the exercise; treat them as part of the result.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Stats {
    /// Refinement rounds completed, not counting the round that settled it.
    pub rounds: usize,
    /// Clauses in the initial encoding of the abstraction.
    pub initial_clauses: usize,
    /// Clauses added by refinement, in total.
    pub refinement_clauses: usize,
    /// Clauses added by refinement, per round.
    pub clauses_per_round: Vec<usize>,
    /// Cycles in the cover, per round — how badly the abstraction was fragmented
    /// as refinement proceeded.  This is Knuth's `t`, read directly off the
    /// cycle cover; no `Decomposition` is built to obtain it.
    ///
    /// Counted *after* step C6, since that is the cover refinement acts on.
    pub segments_per_round: Vec<usize>,
    /// Cycles in the cover before step C6 merged any, per round.  The gap
    /// between this and `segments_per_round` is what merging bought; with
    /// `Config::merge_cycles` off the two are equal.
    pub segments_before_merge_per_round: Vec<usize>,
    /// Solver conflicts in total, read from the solver's own statistics.
    pub conflicts: usize,
    /// Time spent inside the solver.
    pub solve_time: Duration,
    /// Obstruction found by precondition check, if any.
    pub obstruction: Option<String>,
}

/// A Hamiltonian cycle search in progress.
pub(crate) struct CegarSearch<'g> {
    graph: &'g UnGraph<(), ()>,
    /// `None` if the solver isn't needed.
    solver: Option<CaDiCaL<'static, 'static>>,
    /// Next unused SAT variable index. Arc variables occupy `0..2*edge_count`;
    /// auxiliary variables introduced by cardinality encodings are handed out
    /// from here.
    #[allow(dead_code)]
    // read by fresh_var, itself unused until an am1 variant exists
    next_var: u32,
    config: Config,
    stats: Stats,
    /// The search's terminal answer, once it has one.  `step` returns this
    /// again on any call after the search has settled, instead of collapsing
    /// every terminal state into `NoCycle` — a search that already found a
    /// cycle must keep reporting `Found`, not silently flip to `NoCycle`.
    outcome: Option<Step>,
    /// The round's SUCC/PRED/CID state, retained so that `decomposition` can
    /// derive from it on request.
    cover: Option<CycleCover>,
}

/// A human-readable reason a precondition check rejected the graph.
fn describe_obstruction(obstruction: Obstruction) -> String {
    match obstruction {
        Obstruction::Empty => "graph is empty".to_string(),
        Obstruction::SelfLoop(e) => format!("self-loop at edge {}", e.index()),
        Obstruction::ParallelEdges(u, v) => format!(
            "parallel edges between vertices {} and {}",
            u.index(),
            v.index()
        ),
        Obstruction::Disconnected => "graph is disconnected".to_string(),
        Obstruction::LowDegree(v) => format!("vertex {} has degree < 2", v.index()),
        Obstruction::Bridge(e) => format!("bridge at edge {}", e.index()),
        Obstruction::ArticulationPoint(v) => {
            format!("articulation point at vertex {}", v.index())
        }
    }
}

impl<'g> CegarSearch<'g> {
    /// Sets up the solver and encodes the initial abstraction.
    ///
    /// Also freezes every arc variable. CaDiCaL eliminates variables during
    /// inprocessing, and adding a clause over an eliminated variable forces it
    /// to restore clauses — correct, but a performance cliff when it happens on
    /// every refinement round.  Since refinement clauses are built from arc
    /// variables (all 2*m of them, not m), freezing them up front is cheap
    /// insurance, and omitting it is the likeliest cause of unexplained
    /// slowness.
    pub(crate) fn new(
        graph: &'g UnGraph<(), ()>,
        config: Config,
    ) -> Result<Self, super::Error> {
        // Run precondition checks unless skipped.
        if !config.skip_preconditions {
            if let Some(obstruction) = check_preconditions(graph) {
                return Ok(CegarSearch {
                    graph,
                    solver: None,
                    next_var: (2 * graph.edge_count()) as u32,
                    config,
                    stats: Stats {
                        obstruction: Some(describe_obstruction(obstruction)),
                        ..Default::default()
                    },
                    outcome: Some(Step::NoCycle),
                    cover: None,
                });
            }
        }

        // No edges: every vertex would need an in-arc and an out-arc that
        // cannot exist, so no cycle cover — let alone a Hamiltonian one — is
        // possible, regardless of n.  Reachable only via `skip_preconditions`
        // (the checks above already catch every m == 0 graph otherwise: n ==
        // 0 is `Empty`, n == 1 is `LowDegree`, n >= 2 is `Disconnected`).
        // Answered directly rather than spinning up a solver for a foregone
        // conclusion — and n == 0 specifically must not reach the solver: its
        // cycle cover is vacuously t == 0, which is neither the t == 1 that
        // C5 wants nor the t > 1 that `cut_clauses` assumes (it panics via
        // `debug_assert!(t > 1, ...)` and an out-of-range slice otherwise).
        let m = graph.edge_count() as u32;
        if m == 0 {
            return Ok(CegarSearch {
                graph,
                solver: None,
                next_var: 0,
                config,
                stats: Stats::default(),
                outcome: Some(Step::NoCycle),
                cover: None,
            });
        }

        // Create the solver and reserve variables.
        let mut solver = CaDiCaL::default();
        solver
            .reserve(rustsat::types::Var::new(2 * m - 1))
            .map_err(|e| super::Error::Solver(e.to_string()))?;

        // Freeze all 2m arc variables to prevent inprocessing elimination.
        for var_idx in 0..(2 * m) {
            solver
                .freeze_var(rustsat::types::Var::new(var_idx))
                .map_err(|e| super::Error::Solver(e.to_string()))?;
        }

        // Encode the cycle cover CNF (Algorithm C steps C1-C2).
        let cnf = cycle_cover_cnf(graph);
        let initial_clauses = cnf.len();
        solver
            .add_cnf(cnf)
            .map_err(|e| super::Error::Solver(e.to_string()))?;

        // Apply max_conflicts limit if set.
        if let Some(conflicts) = config.max_conflicts {
            solver
                .limit_conflicts(Some(conflicts as u32))
                .map_err(|e| super::Error::Solver(e.to_string()))?;
        }

        Ok(CegarSearch {
            graph,
            solver: Some(solver),
            next_var: 2 * m,
            config,
            stats: Stats {
                initial_clauses,
                ..Default::default()
            },
            outcome: None,
            cover: None,
        })
    }

    /// Runs one round.
    ///
    /// Returns [`Step::Spurious`] having *already* added the refinement clauses,
    /// so a caller that ignores the round still makes progress.
    pub(crate) fn step(&mut self) -> Result<Step, super::Error> {
        // The search has already settled — report the same answer again
        // rather than re-solving (or, worse, collapsing a `Found` into a
        // `NoCycle` on a second call).
        if let Some(outcome) = &self.outcome {
            return Ok(outcome.clone());
        }
        let solver = self
            .solver
            .as_mut()
            .expect("solver exists once outcome is unresolved");

        let start_time = std::time::Instant::now();

        // C3: Solve.
        let result = solver
            .solve()
            .map_err(|e| super::Error::Solver(e.to_string()))?;
        self.stats.solve_time += start_time.elapsed();
        self.stats.conflicts = solver.conflicts();

        match result {
            SolverResult::Unsat => {
                // Unsatisfiable means conclusively no cycle exists.
                self.outcome = Some(Step::NoCycle);
                return Ok(Step::NoCycle);
            }
            SolverResult::Interrupted => {
                // Hit a limit (conflicts or decision limit); do not mark the
                // search finished, so a caller can raise the limit and retry.
                return Ok(Step::LimitReached);
            }
            SolverResult::Sat => {
                // Continue to C4.
            }
        }

        // Get the full solution.
        let model = solver
            .full_solution()
            .map_err(|e| super::Error::Solver(e.to_string()))?;

        // C4: Decode the model into a cycle cover.
        let vars = ArcVars::new(self.graph);
        let mut cover = CycleCover::from_model(self.graph, &vars, &model)?;

        // C5: Done?
        if cover.t() == 1 {
            return self.finish_found(cover);
        }

        // C6: Merge adjacent cycles, which only ever joins them along real
        // edges of the graph.  So the merged cover is one the solver could
        // have returned, and C7 below can accept it as an answer.
        let cycles_before_merge = cover.t();
        if self.config.merge_cycles {
            cover.merge(self.graph);
        }
        let t = cover.t();

        // C7: Done?
        if t == 1 {
            return self.finish_found(cover);
        }

        // C8: Generate cut clauses to eliminate this spurious cover.
        let cut = cut_clauses(self.graph, &vars, &cover);

        match cut {
            super::refinement::Cut::NoCycle => {
                // No way for the cycles to reconnect; no Hamiltonian cycle exists.
                self.outcome = Some(Step::NoCycle);
                return Ok(Step::NoCycle);
            }
            super::refinement::Cut::Clauses(clauses) => {
                // Add the clauses to the solver.
                let clause_count = clauses.len();
                for clause in clauses {
                    solver
                        .add_clause(clause)
                        .map_err(|e| super::Error::Solver(e.to_string()))?;
                }

                // Update statistics.  Solve time and conflicts were already
                // captured right after `solve()`, above.
                self.stats.refinement_clauses += clause_count;
                self.stats.clauses_per_round.push(clause_count);
                self.stats.segments_per_round.push(t);
                self.stats
                    .segments_before_merge_per_round
                    .push(cycles_before_merge);
                self.stats.rounds += 1;

                // Retain the cover for later decomposition requests.
                self.cover = Some(cover);

                // Check max_rounds limit.
                if let Some(max_rounds) = self.config.max_rounds {
                    if self.stats.rounds >= max_rounds {
                        return Ok(Step::LimitReached);
                    }
                }

                Ok(Step::Spurious { cycles: t })
            }
        }
    }

    /// Settles the search on the single spanning cycle `cover` describes.
    ///
    /// Shared by C5 and C7, which differ only in whether step C6 has run.
    ///
    /// Every one of the graph's `n` vertices gets a `CID` in C4, so `t == 1`
    /// always means that one cycle spans all of them and there is no valid way
    /// for `as_hamiltonian_cycle` to return `None` here.  If it ever did,
    /// falling through to C8 would treat a genuine Hamiltonian cycle as a
    /// spurious cover with no crossing edges and wrongly report `NoCycle`;
    /// `expect` instead, so a broken invariant panics rather than silently
    /// returning the wrong answer.
    fn finish_found(&mut self, cover: CycleCover) -> Result<Step, super::Error> {
        let decomposition = cover.to_decomposition()?;
        let cycle = decomposition
            .as_hamiltonian_cycle(self.graph.node_count())
            .expect("a single cycle-cover cycle spans every vertex");
        let mut answer = cycle.clone();
        answer.canonicalize();
        debug_assert!(
            answer.is_hamiltonian_cycle(self.graph),
            "step() is about to return an answer that is not actually a \
             Hamiltonian cycle of the searched graph: {answer}"
        );
        self.cover = Some(cover);
        self.outcome = Some(Step::Found(answer.clone()));
        Ok(Step::Found(answer))
    }

    /// The current round's cycle cover, as an ordered [`Decomposition`].
    ///
    /// Materialised on request rather than on every round.  The CEGAR loop runs
    /// entirely on Knuth's `SUCC`/`PRED`/`CID` arrays; this is the presentation
    /// view of them, wanted only by renderers, tests, and the final answer.
    ///
    /// `None` before the first [`step`](Self::step).  The inner `Result` fails
    /// only if the cover is malformed, which would mean the encoding and the
    /// decoder disagree.
    #[allow(dead_code)] // used by the renderers, phase 11
    pub(crate) fn decomposition(
        &self,
    ) -> Option<Result<Decomposition, super::SegmentError>> {
        self.cover.as_ref().map(|cover| cover.to_decomposition())
    }

    /// Steps until the search settles or hits a limit.
    pub(crate) fn run(&mut self) -> Result<Step, super::Error> {
        loop {
            let step = self.step()?;
            if !matches!(step, Step::Spurious { .. }) {
                return Ok(step);
            }
        }
    }

    /// Allocates a fresh auxiliary SAT variable.
    #[allow(dead_code)] // used by an am1 cardinality variant, if one is added
    pub(crate) fn fresh_var(&mut self) -> u32 {
        let var = self.next_var;
        self.next_var += 1;
        var
    }

    #[allow(dead_code)] // used by the renderers, phase 11
    pub(crate) fn graph(&self) -> &'g UnGraph<(), ()> {
        self.graph
    }

    #[allow(dead_code)] // used by the pub bench entry point, phase 12
    pub(crate) fn stats(&self) -> &Stats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::{knight_graph, petersen};
    use crate::testing::{graph_of, v};

    mod path_abc {
        use super::*;

        #[test]
        fn path_abc_unsatisfiable_at_round_zero() {
            // Path graph A-B-C has degree-1 vertices which force unit clauses.
            // Combined with asymmetry clauses, this makes the formula UNSAT.
            let graph = graph_of(3, &[(0, 1), (1, 2)]);

            let config = Config {
                skip_preconditions: true,
                ..Default::default()
            };

            let mut search =
                CegarSearch::new(&graph, config).expect("new should succeed");
            let result = search.step().expect("step should succeed");

            match result {
                Step::NoCycle => {
                    assert_eq!(
                        search.stats().rounds,
                        0,
                        "should be unsatisfiable at round 0"
                    );
                }
                other => panic!("expected NoCycle, got {:?}", other),
            }
        }
    }

    mod triangle {
        use super::*;

        #[test]
        fn triangle_found_in_zero_rounds() {
            // A triangle is a cycle cover by itself, so no refinement needed.
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);

            let config = Config::default();

            let mut search =
                CegarSearch::new(&graph, config).expect("new should succeed");
            let result = search.step().expect("step should succeed");

            match result {
                Step::Found(segment) => {
                    assert_eq!(
                        search.stats().rounds,
                        0,
                        "triangle should be found with no refinement"
                    );
                    assert!(
                        segment.is_hamiltonian_cycle(&graph),
                        "not a Hamiltonian cycle of the graph: {segment}"
                    );
                }
                other => panic!("expected Found, got {:?}", other),
            }
        }
    }

    mod k4 {
        use super::*;

        #[test]
        fn complete_graph_k4_found_in_zero_rounds() {
            // K4 is 4-regular. The only cycle cover is the Hamiltonian one.
            let graph = graph_of(4, &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);

            let config = Config::default();

            let mut search =
                CegarSearch::new(&graph, config).expect("new should succeed");
            let result = search.step().expect("step should succeed");

            match result {
                Step::Found(segment) => {
                    assert_eq!(
                        search.stats().rounds,
                        0,
                        "K4 should be found with no refinement"
                    );
                    assert!(
                        segment.is_hamiltonian_cycle(&graph),
                        "not a Hamiltonian cycle of the graph: {segment}"
                    );
                }
                other => panic!("expected Found, got {:?}", other),
            }
        }
    }

    mod petersen {
        use super::*;

        #[test]
        fn petersen_not_hamiltonian_requires_refinement() {
            // Petersen is 3-regular and 3-connected (clears preconditions),
            // but non-Hamiltonian. Its first model will be a spurious cover
            // (outer pentagon + inner pentagram), so at least one refinement
            // round is guaranteed.
            let graph = petersen();

            let config = Config::default();

            let mut search =
                CegarSearch::new(&graph, config).expect("new should succeed");

            // First step should produce a spurious cover with 2 cycles.
            let result = search.step().expect("step should succeed");
            match result {
                Step::Spurious { cycles } => {
                    assert!(cycles > 1, "first model should be spurious");
                }
                other => panic!("expected Spurious, got {:?}", other),
            }

            // Keep stepping until UNSAT.
            loop {
                match search.step().expect("step should succeed") {
                    Step::NoCycle => {
                        assert!(
                            search.stats().rounds >= 1,
                            "Petersen should need at least one refinement round"
                        );
                        break;
                    }
                    Step::Spurious { .. } => {
                        // Continue refining.
                    }
                    other => panic!("expected NoCycle eventually, got {:?}", other),
                }
            }
        }
    }

    mod triangular_prism {
        use super::*;

        /// Triangular prism: two triangles with a perfect matching between them.
        /// Vertices 0,1,2 form one triangle; 3,4,5 form the other;
        /// edges connect 0-3, 1-4, 2-5.
        fn triangular_prism() -> UnGraph<(), ()> {
            graph_of(
                6,
                &[
                    (0, 1),
                    (1, 2),
                    (2, 0), // first triangle
                    (3, 4),
                    (4, 5),
                    (5, 3), // second triangle
                    (0, 3),
                    (1, 4),
                    (2, 5), // matching edges
                ],
            )
        }

        #[test]
        fn triangular_prism_has_hamiltonian_cycle() {
            let graph = triangular_prism();

            let config = Config::default();

            let mut search =
                CegarSearch::new(&graph, config).expect("new should succeed");
            let result = search.run().expect("run should succeed");

            match result {
                Step::Found(segment) => {
                    // Assert that the answer is correct, not the round count
                    // (which depends on which spurious cover CaDiCaL finds first).
                    assert!(
                        segment.is_hamiltonian_cycle(&graph),
                        "not a Hamiltonian cycle of the graph: {segment}"
                    );
                }
                other => panic!("expected Found, got {:?}", other),
            }
        }
    }

    mod knight_6x6 {
        use super::*;

        #[test]
        fn knight_6x6_has_hamiltonian_cycle() {
            let graph = knight_graph(6, 6).0;

            let config = Config::default();

            let mut search =
                CegarSearch::new(&graph, config).expect("new should succeed");
            let result = search.run().expect("run should succeed");

            match result {
                Step::Found(segment) => {
                    assert!(
                        segment.is_hamiltonian_cycle(&graph),
                        "not a Hamiltonian cycle of the graph: {segment}"
                    );
                }
                other => panic!("expected Found, got {:?}", other),
            }
        }
    }

    mod merging {
        use super::*;

        /// Runs a graph to completion with merging off and on.
        fn both_ways(graph: &UnGraph<(), ()>) -> (Step, Step) {
            let mut answers = [Step::LimitReached, Step::LimitReached];
            for (answer, merge_cycles) in answers.iter_mut().zip([false, true]) {
                let config = Config {
                    merge_cycles,
                    ..Default::default()
                };
                let mut search =
                    CegarSearch::new(graph, config).expect("new should succeed");
                *answer = search.run().expect("run should succeed");
            }
            let [without, with] = answers;
            (without, with)
        }

        #[test]
        fn petersen_answers_the_same_either_way() {
            // Merging is an optimisation, so it may change how many rounds a
            // search takes but must never change what it decides.  Petersen
            // is the sharper of the two cases: its answer is a *proof* of
            // non-existence, which merging could only break by making the
            // solver see a formula it should not have.
            let (without, with) = both_ways(&petersen());
            assert_eq!(without, Step::NoCycle);
            assert_eq!(with, Step::NoCycle);
        }

        #[test]
        fn knight_6x6_answers_the_same_either_way() {
            let graph = knight_graph(6, 6).0;
            let (without, with) = both_ways(&graph);

            // Not the same *cycle* — merging returns one the solver never
            // proposed — so both answers are checked against the graph
            // instead of against each other.
            for answer in [without, with] {
                match answer {
                    Step::Found(segment) => assert!(
                        segment.is_hamiltonian_cycle(&graph),
                        "not a Hamiltonian cycle of the graph: {segment}"
                    ),
                    other => panic!("expected Found, got {other:?}"),
                }
            }
        }

        #[test]
        fn merging_is_recorded_against_the_pre_merge_count() {
            // Both per-round series are filled in on every refining round,
            // whether or not merging is on; with it off they are equal.
            let graph = petersen();
            let config = Config {
                merge_cycles: false,
                ..Default::default()
            };
            let mut search =
                CegarSearch::new(&graph, config).expect("new should succeed");
            search.run().expect("run should succeed");

            let stats = search.stats();
            assert!(stats.rounds >= 1, "Petersen must refine at least once");
            assert_eq!(
                stats.segments_before_merge_per_round,
                stats.segments_per_round
            );
            assert_eq!(stats.segments_per_round.len(), stats.rounds);
        }
    }

    mod config_and_stats {
        use super::*;

        #[test]
        fn default_config_has_no_limits_and_merges() {
            let config = Config::default();
            assert!(config.max_rounds.is_none());
            assert!(config.max_conflicts.is_none());
            assert!(!config.skip_preconditions);
            assert!(config.merge_cycles);
        }

        #[test]
        fn stats_tracks_initial_clauses() {
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);
            let config = Config::default();
            let search = CegarSearch::new(&graph, config).expect("new should succeed");

            // A triangle: 3 asymmetry + 6 at-least-one + 6 at-most-one = 15 clauses
            assert_eq!(search.stats().initial_clauses, 15);
        }
    }

    mod obstruction {
        use super::*;

        #[test]
        fn describe_obstruction_covers_every_variant() {
            assert_eq!(describe_obstruction(Obstruction::Empty), "graph is empty");
            assert_eq!(
                describe_obstruction(Obstruction::SelfLoop(
                    petgraph::graph::EdgeIndex::new(2)
                )),
                "self-loop at edge 2"
            );
            assert_eq!(
                describe_obstruction(Obstruction::ParallelEdges(v(0), v(1))),
                "parallel edges between vertices 0 and 1"
            );
            assert_eq!(
                describe_obstruction(Obstruction::Disconnected),
                "graph is disconnected"
            );
            assert_eq!(
                describe_obstruction(Obstruction::LowDegree(v(3))),
                "vertex 3 has degree < 2"
            );
            assert_eq!(
                describe_obstruction(Obstruction::Bridge(
                    petgraph::graph::EdgeIndex::new(5)
                )),
                "bridge at edge 5"
            );
            assert_eq!(
                describe_obstruction(Obstruction::ArticulationPoint(v(4))),
                "articulation point at vertex 4"
            );
        }

        #[test]
        fn rejected_graph_records_the_obstruction_message() {
            // A pendant vertex hanging off a triangle: vertex 3 has degree 1.
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 0), (2, 3)]);
            let search =
                CegarSearch::new(&graph, Config::default()).expect("new should succeed");
            assert_eq!(
                search.stats().obstruction.as_deref(),
                Some("vertex 3 has degree < 2")
            );
        }
    }

    mod terminal_state {
        use super::*;

        #[test]
        fn step_after_found_keeps_reporting_found() {
            // A search that has already found a Hamiltonian cycle must keep
            // reporting `Found` on further calls, not collapse into `NoCycle`
            // the way an obstruction or a proven-UNSAT search does.
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);
            let mut search =
                CegarSearch::new(&graph, Config::default()).expect("new should succeed");

            let first = search.step().expect("step should succeed");
            let Step::Found(segment) = first else {
                panic!("expected Found, got {:?}", first);
            };

            let second = search.step().expect("step should succeed");
            assert_eq!(
                second,
                Step::Found(segment),
                "a second call after Found must report the same answer, not NoCycle"
            );
        }

        #[test]
        fn step_after_no_cycle_keeps_reporting_no_cycle() {
            let graph = graph_of(3, &[(0, 1), (1, 2)]);
            let config = Config {
                skip_preconditions: true,
                ..Default::default()
            };
            let mut search =
                CegarSearch::new(&graph, config).expect("new should succeed");

            assert_eq!(search.step().expect("step should succeed"), Step::NoCycle);
            assert_eq!(search.step().expect("step should succeed"), Step::NoCycle);
        }

        #[test]
        fn skip_preconditions_on_edgeless_graph_short_circuits() {
            // No edges means no possible cycle cover, so `new` answers
            // directly without ever building a solver.
            let graph = graph_of(2, &[]);
            let config = Config {
                skip_preconditions: true,
                ..Default::default()
            };
            let mut search =
                CegarSearch::new(&graph, config).expect("new should succeed");
            assert_eq!(search.step().expect("step should succeed"), Step::NoCycle);
        }

        #[test]
        fn skip_preconditions_on_empty_graph_does_not_panic() {
            // n == 0 (and so m == 0) is the case that would otherwise reach
            // the solver: an empty cover has t == 0, which is neither C5's
            // t == 1 nor a t > 1 that `cut_clauses` can handle — it would
            // panic instead of resolving. The m == 0 short-circuit in `new`
            // must catch this before the solver is ever built.
            let graph = graph_of(0, &[]);
            let config = Config {
                skip_preconditions: true,
                ..Default::default()
            };
            let mut search =
                CegarSearch::new(&graph, config).expect("new should succeed");
            assert_eq!(search.step().expect("step should succeed"), Step::NoCycle);
        }
    }
}

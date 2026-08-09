//! Watching a search round by round, instead of asking for the answer.
//!
//! [`crate::find_hamiltonian_cycle`] runs the CEGAR loop to completion and
//! hands back the answer.  [`Search`] is the same loop with the lid off: it
//! advances one refinement round per [`step`](Search::step), and after each
//! one the cycle cover the solver proposed is available as a
//! [`Decomposition`] via [`cover`](Search::cover).
//!
//! That is what makes the round-by-round rendering in [`crate::render`]
//! reachable — emitting one image per round produces a flipbook of the
//! abstraction tightening, from a badly fragmented cover down to the single
//! spanning cycle that answers the question.  It is also how a benchmark gets
//! at [`Stats`] without the solver being run twice.
//!
//! The loop itself is unchanged; this is a public window onto it, not a
//! second implementation.

use petgraph::graph::UnGraph;

use crate::driver::{CegarSearch, Step};
use crate::segment::Decomposition;
use crate::solution::HamiltonianCycle;
use crate::{Config, Error, Stats};

/// What one refinement round did.
///
/// Mirrors the three outcomes the one-shot entry points distinguish, with the
/// intermediate case — the one those functions loop over internally — surfaced
/// as [`Progress::Refining`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Progress {
    /// The abstraction admitted a model that is not a single spanning cycle.
    /// Clauses ruling it out have already been added, so calling
    /// [`Search::step`] again makes progress.
    ///
    /// `cycles` is how many disjoint cycles the cover fell into — Knuth's `t`,
    /// counted after merging.  It is read straight off the cycle cover and is
    /// free; call [`Search::cover`] for the ordered view of those cycles.
    Refining { cycles: usize },
    /// A genuine Hamiltonian cycle.  The search is finished.
    Found(HamiltonianCycle),
    /// The abstraction went unsatisfiable, so no Hamiltonian cycle exists.
    /// This is a *proof*, not a failure to find one.
    NoCycle,
    /// A configured limit was hit before the question was settled — neither a
    /// witness nor a proof.
    LimitReached,
}

/// A Hamiltonian cycle search that can be watched round by round.
///
/// Searches for cycles only, like the engine itself; there is no path variant
/// here, because a path query is a cycle query on a graph with an added apex
/// vertex and the rounds would be reported against that larger graph rather
/// than the one the caller passed.
pub struct Search<'g> {
    inner: CegarSearch<'g>,
}

impl<'g> Search<'g> {
    /// Starts a search on `graph`, encoding the initial abstraction.
    ///
    /// Does not run any refinement round; the first [`step`](Self::step) does
    /// that.  Pass [`Config::default`] unless you are varying a knob — the
    /// merging on-versus-off comparison is exactly what `config` is for.
    pub fn new(graph: &'g UnGraph<(), ()>, config: Config) -> Result<Self, Error> {
        let inner = CegarSearch::new(graph, config)?;
        Ok(Search { inner })
    }

    /// Advances one refinement round.
    ///
    /// Once the search has settled, every later call reports the same terminal
    /// [`Progress`] again rather than advancing — a search that found a cycle
    /// keeps saying so, and does not quietly flip to [`Progress::NoCycle`].
    pub fn step(&mut self) -> Result<Progress, Error> {
        Ok(match self.inner.step()? {
            Step::Spurious { cycles } => Progress::Refining { cycles },
            Step::Found(segment) => Progress::Found(HamiltonianCycle::new(segment)),
            Step::NoCycle => Progress::NoCycle,
            Step::LimitReached => Progress::LimitReached,
        })
    }

    /// The cycle cover the most recent round decoded to, as an ordered
    /// [`Decomposition`] ready to hand to a renderer.
    ///
    /// `None` before the first [`step`](Self::step), and on a graph the
    /// precondition checks rejected without ever consulting the solver.  The
    /// inner `Err` means the encoding and the decoder disagree, which is a bug
    /// rather than an ordinary outcome.
    ///
    /// Derived on request rather than on every round: the loop runs on Knuth's
    /// `SUCC`/`PRED`/`CID` arrays, and this is the presentation view of them.
    pub fn cover(&self) -> Option<Result<Decomposition, Error>> {
        self.inner
            .decomposition()
            .map(|result| result.map_err(Error::from))
    }

    /// Counters accumulated so far: rounds, clause growth, solver conflicts,
    /// time in the solver.
    ///
    /// For this project these are as much the deliverable as the cycle is —
    /// they are what shows whether one refinement strategy beats another.
    pub fn stats(&self) -> &Stats {
        self.inner.stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::{knight_graph, petersen};
    use crate::testing::graph_of;
    use claim::assert_ok;

    mod stepping {
        use super::*;

        #[test]
        fn triangle_settles_on_the_first_step() {
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);
            let mut search = assert_ok!(Search::new(&graph, Config::default()));

            match assert_ok!(search.step()) {
                Progress::Found(cycle) => assert!(cycle.is_valid_for(&graph)),
                other => panic!("expected Found, got {other:?}"),
            }
        }

        #[test]
        fn terminal_progress_repeats_rather_than_advancing() {
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);
            let mut search = assert_ok!(Search::new(&graph, Config::default()));

            let first = assert_ok!(search.step());
            let second = assert_ok!(search.step());
            assert_eq!(first, second);
        }

        #[test]
        fn petersen_proves_no_cycle_exists() {
            let graph = petersen();
            let mut search = assert_ok!(Search::new(&graph, Config::default()));

            loop {
                match assert_ok!(search.step()) {
                    Progress::Refining { cycles } => assert!(cycles >= 1),
                    Progress::NoCycle => break,
                    other => panic!("expected refinement then NoCycle, got {other:?}"),
                }
            }
        }
    }

    mod observation {
        use super::*;

        #[test]
        fn no_cover_before_the_first_step() {
            let graph = petersen();
            let search = assert_ok!(Search::new(&graph, Config::default()));
            assert!(search.cover().is_none());
        }

        /// The whole point of the type: a spurious round must expose a cover
        /// with more than one segment, which is what the multi-segment
        /// renderers exist for.
        #[test]
        fn a_spurious_round_exposes_a_multi_segment_cover() {
            let graph = petersen();
            let mut search = assert_ok!(Search::new(&graph, Config::default()));

            let mut saw_multi_segment = false;
            loop {
                let progress = assert_ok!(search.step());
                if let Some(cover) = search.cover() {
                    let cover = assert_ok!(cover);
                    assert_eq!(cover.covered_vertices(), graph.node_count());
                    if cover.len() > 1 {
                        saw_multi_segment = true;
                    }
                }
                if !matches!(progress, Progress::Refining { .. }) {
                    break;
                }
            }
            assert!(
                saw_multi_segment,
                "Petersen should fragment into several cycles before settling"
            );
        }

        #[test]
        fn cover_matches_the_reported_cycle_count() {
            let graph = petersen();
            let mut search = assert_ok!(Search::new(&graph, Config::default()));

            while let Progress::Refining { cycles } = assert_ok!(search.step()) {
                let cover = assert_ok!(search.cover().expect("a round has been taken"));
                assert_eq!(cover.len(), cycles);
            }
        }
    }

    mod statistics {
        use super::*;

        #[test]
        fn rounds_and_clauses_accumulate() {
            let graph = knight_graph(6, 6).0;
            let mut search = assert_ok!(Search::new(&graph, Config::default()));

            while let Progress::Refining { .. } = assert_ok!(search.step()) {}

            let stats = search.stats();
            assert!(stats.initial_clauses > 0);
            assert_eq!(stats.clauses_per_round.len(), stats.rounds);
            assert_eq!(stats.segments_per_round.len(), stats.rounds);
        }
    }
}

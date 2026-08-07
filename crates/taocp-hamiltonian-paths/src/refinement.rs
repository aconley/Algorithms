//! Step C8 of Knuth's Algorithm C: cut clauses.
//!
//! Given a [`CycleCover`] with `t > 1` active cycles, this module builds the
//! clauses that rule the current model out.  For each active cycle `C`, it
//! finds every edge crossing the cut between `C` and its complement and emits
//! two clauses: one requiring some arc to leave `C`, one requiring some arc
//! to enter it.
//!
//! Both are violated by the current model, which is what guarantees progress:
//! every cycle of the model lies wholly inside `C` or wholly outside it, so
//! no model arc crosses the cut and every literal of both clauses is false.
//! Adding them therefore forces a genuinely different cover next round.  The
//! same argument still holds once phase 9 merges cycles, since a merged cycle
//! is a union of whole model cycles.
//!
//! See `.agents/algorithm.md`, "Step C8: Cut clauses", for Knuth's statement
//! of this step and the two implementation notes it flags.

use super::cycles::CycleCover;
use super::encoding::ArcVars;
use petgraph::graph::UnGraph;
use rustsat::types::{Clause, Lit};

/// The result of step C8.
#[derive(Debug)]
pub(crate) enum Cut {
    /// The cut clauses to add to the solver, two per active cycle (one per
    /// cycle when `t == 2`, since the second cycle's pair would duplicate
    /// the first's).
    Clauses(Vec<Clause>),
    /// Fewer than two edges cross some cut, so no Hamiltonian cycle exists.
    NoCycle,
}

/// Step C8: builds the cut clauses that exclude the current spurious cover.
///
/// For each active cycle `c`, walks `v` around the cycle from `HEAD[c]` via
/// `SUCC`, and for each neighbour `u` of `v` with `CID[u] != c` collects the
/// literal for arc `v -> u`.  Call that list `l_1 .. l_k`.  Two clauses are
/// emitted per cycle: `l_1 .. l_k` (some arc leaves the cycle) and the ⊕1
/// image of every literal, `l_1^1 .. l_k^1` (some arc enters it) — which is
/// exactly `Cut(complement C)`, since the reverse of every crossing arc also
/// crosses the cut.
///
/// Loop control follows C8: if `t > 2`, every active cycle is visited in
/// turn; otherwise (`t == 2`), only the first is.  See the comment on the
/// loop itself for why, and for where this departs from Knuth's phrasing.
///
/// If any cycle has fewer than two crossing edges, returns `Cut::NoCycle`
/// immediately: a cycle that cannot both leave and re-enter its complement
/// proves no Hamiltonian cycle exists, and that is not a case to be smoothed
/// into a unit clause.
pub(crate) fn cut_clauses(
    graph: &UnGraph<(), ()>,
    vars: &ArcVars,
    cover: &CycleCover,
) -> Cut {
    let active = cover.active();
    let t = active.len();
    // Callers reach C8 only from C5 (and, in phase 9, C7), both of which
    // return a Hamiltonian cycle when t == 1.  That matters more than it
    // looks: a single spanning cycle has no crossing edges at all, so the
    // k < 2 test below would report `NoCycle` — a *proof* of nonexistence
    // for a graph that had just produced a cycle.
    debug_assert!(t > 1, "C8 expects a spurious cover, but t == {t}");
    let mut clauses = Vec::with_capacity(2 * t);

    // C8's loop control: every active cycle when t > 2, and only the first
    // when t == 2, where v in CYC[0] <=> v not in CYC[1] makes the second
    // cycle's pair of clauses a duplicate of the first's.  Knuth writes that
    // second case as a jump to j = 2; a bounded slice says the same thing.
    for &c in &active[..if t > 2 { t } else { 1 }] {
        let head = cover.head[c];

        let mut crossing = Vec::new();
        let mut v = head;
        loop {
            for u in graph.neighbors(v) {
                if cover.cid[u.index()] != c {
                    let var = vars.var(v, u).expect("neighbor is adjacent to v");
                    crossing.push(var);
                }
            }
            v = cover.succ[v.index()];
            if v == head {
                break;
            }
        }

        if crossing.len() < 2 {
            return Cut::NoCycle;
        }

        clauses.push(Clause::from_iter(
            crossing.iter().map(|&var| Lit::positive(var)),
        ));
        clauses.push(Clause::from_iter(
            crossing.iter().map(|&var| Lit::positive(var ^ 1)),
        ));
    }

    Cut::Clauses(clauses)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{graph_of, knuth_cover, knuth_graph, model_of, v};
    use claim::assert_ok;

    /// Unwraps the clauses of a `Cut`, failing the test on `NoCycle`.
    fn expect_clauses(cut: Cut) -> Vec<Clause> {
        match cut {
            Cut::Clauses(clauses) => clauses,
            Cut::NoCycle => panic!("expected Cut::Clauses, got Cut::NoCycle"),
        }
    }

    /// Two triangles joined by two edges, `0-3` and `1-4`, so each triangle
    /// has exactly two edges crossing to the other.
    fn two_triangles() -> UnGraph<(), ()> {
        graph_of(
            6,
            &[
                (0, 1),
                (1, 2),
                (2, 0),
                (3, 4),
                (4, 5),
                (5, 3),
                (0, 3),
                (1, 4),
            ],
        )
    }

    fn two_triangles_cover(graph: &UnGraph<(), ()>) -> CycleCover {
        let vars = ArcVars::new(graph);
        let model = model_of(graph, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]);
        assert_ok!(CycleCover::from_model(graph, &vars, &model))
    }

    mod t_equals_two {
        use super::*;

        #[test]
        fn emits_exactly_two_clauses() {
            let graph = two_triangles();
            let cover = two_triangles_cover(&graph);
            assert_eq!(cover.t(), 2);

            let clauses =
                expect_clauses(cut_clauses(&graph, &ArcVars::new(&graph), &cover));
            assert_eq!(clauses.len(), 2);
        }

        #[test]
        fn asserts_the_exact_literal_sets() {
            let graph = two_triangles();
            let vars = ArcVars::new(&graph);
            let cover = two_triangles_cover(&graph);

            let clauses = expect_clauses(cut_clauses(&graph, &vars, &cover));

            // Cycle 1 walks 0, 1, 2 (HEAD[1] = 0). Crossing arcs, in walk
            // order: 0 -> 3, then 1 -> 4.
            let leave = Clause::from_iter([
                Lit::positive(vars.var(v(0), v(3)).unwrap()),
                Lit::positive(vars.var(v(1), v(4)).unwrap()),
            ]);
            let enter = Clause::from_iter([
                Lit::positive(vars.var(v(3), v(0)).unwrap()),
                Lit::positive(vars.var(v(4), v(1)).unwrap()),
            ]);

            assert_eq!(clauses, vec![leave, enter]);
        }

        #[test]
        fn the_two_clauses_of_a_pair_are_xor_one_images() {
            let graph = two_triangles();
            let cover = two_triangles_cover(&graph);
            let clauses =
                expect_clauses(cut_clauses(&graph, &ArcVars::new(&graph), &cover));

            let leave: Vec<u32> =
                clauses[0].iter().map(|lit| lit.var().idx32()).collect();
            let enter: Vec<u32> =
                clauses[1].iter().map(|lit| lit.var().idx32()).collect();
            assert_eq!(leave.len(), enter.len());
            for (l, e) in leave.iter().zip(&enter) {
                assert_eq!(l ^ 1, *e);
            }
        }
    }

    mod t_equals_three {
        use super::*;

        #[test]
        fn emits_exactly_six_clauses() {
            let cover = knuth_cover();
            assert_eq!(cover.t(), 3);
            let graph = knuth_graph();
            let vars = ArcVars::new(&graph);

            let clauses = expect_clauses(cut_clauses(&graph, &vars, &cover));
            assert_eq!(clauses.len(), 6);
        }

        /// Pins the exact literal sets for the book's three-cycle cover,
        /// worked out by hand from `knuth_graph`'s edge list, one clause per
        /// cycle in `active` order (which C8 visits sequentially).  Compared
        /// as sets, sorted by variable: which neighbour of a vertex is
        /// visited first is an artifact of petgraph's adjacency-list order,
        /// not something C8 specifies.
        #[test]
        fn asserts_the_exact_literal_sets() {
            let graph = knuth_graph();
            let vars = ArcVars::new(&graph);
            let cover = knuth_cover();

            let clauses = expect_clauses(cut_clauses(&graph, &vars, &cover));
            assert_eq!(clauses.len(), 6);

            let lit =
                |from: usize, to: usize| Lit::positive(vars.var(v(from), v(to)).unwrap());
            let xor1 = |lit: &Lit| Lit::positive(lit.var().idx32() ^ 1);
            let sorted_set = |clause: &Clause| -> Vec<u32> {
                let mut idxs: Vec<u32> = clause.iter().map(|l| l.var().idx32()).collect();
                idxs.sort_unstable();
                idxs
            };

            // Cycle 1 (A C G D = 0 2 6 3): crossing arcs are 0->1, 0->4,
            // 0->5, 2->8.
            let cycle1_leave = [lit(0, 1), lit(0, 4), lit(0, 5), lit(2, 8)];
            let cycle1_enter: Vec<Lit> = cycle1_leave.iter().map(xor1).collect();

            // Cycle 2 (B L J E I = 1 11 9 4 8): crossing arcs are 1->0,
            // 11->12, 4->0, 4->7, 8->2.
            let cycle2_leave = [lit(1, 0), lit(11, 12), lit(4, 0), lit(4, 7), lit(8, 2)];
            let cycle2_enter: Vec<Lit> = cycle2_leave.iter().map(xor1).collect();

            // Cycle 3 (F K M H = 5 10 12 7): crossing arcs are 5->0,
            // 12->11, 7->4.
            let cycle3_leave = [lit(5, 0), lit(12, 11), lit(7, 4)];
            let cycle3_enter: Vec<Lit> = cycle3_leave.iter().map(xor1).collect();

            let expected: Vec<Vec<u32>> = [
                Clause::from_iter(cycle1_leave),
                Clause::from_iter(cycle1_enter),
                Clause::from_iter(cycle2_leave),
                Clause::from_iter(cycle2_enter),
                Clause::from_iter(cycle3_leave),
                Clause::from_iter(cycle3_enter),
            ]
            .iter()
            .map(sorted_set)
            .collect();

            let actual: Vec<Vec<u32>> = clauses.iter().map(sorted_set).collect();
            assert_eq!(actual, expected);
        }
    }

    mod no_cycle {
        use super::*;

        /// A triangle and a 2-cycle joined by a single crossing edge,
        /// `0-3`: the triangle's only edge to the rest of the graph.  One
        /// crossing edge is not enough for the triangle to both leave and
        /// re-enter, so this is conclusive: no Hamiltonian cycle exists.
        #[test]
        fn a_single_crossing_edge_is_conclusive() {
            let graph = graph_of(5, &[(0, 1), (1, 2), (2, 0), (3, 4), (0, 3)]);
            let vars = ArcVars::new(&graph);
            let model = model_of(&graph, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 3)]);
            let cover = assert_ok!(CycleCover::from_model(&graph, &vars, &model));
            assert_eq!(cover.t(), 2);

            let cut = cut_clauses(&graph, &vars, &cover);
            assert!(matches!(cut, Cut::NoCycle), "expected NoCycle, got {cut:?}");
        }
    }
}

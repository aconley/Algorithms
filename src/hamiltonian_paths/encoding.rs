//! Steps C1 and C2 of Knuth's Algorithm C: encoding "does this graph have a
//! cycle cover" as CNF.
//!
//! This module builds the formula only.  It does not talk to a solver, which
//! is what makes it testable by asserting directly on clause sets rather than
//! on solver behaviour.
//!
//! See `.agents/algorithm.md`, "Clause conventions" and "Clauses encoding the
//! cycle cover", for the underlying math, and `.agents/plan.md`, "Variable
//! mapping", for the 0-based numbering used here — it supersedes
//! `algorithm.md`'s 2-based one, but keeps the ⊕1 pairing that makes the
//! asymmetry clauses and (later) the cut clauses work.

use petgraph::graph::{EdgeIndex, NodeIndex, UnGraph};
use rustsat::instances::Cnf;
use rustsat::types::{Clause, Lit};
use std::io::{self, Write};

/// The map between graph arcs and SAT variables.
///
/// For undirected edge index `e` with `graph.edge_endpoints(e) == Some((a,
/// b))`, `var(a -> b) = 2 * e.index()` and `var(b -> a) = 2 * e.index() + 1`,
/// so `var(u, v) ^ 1 == var(v, u)`.  Variables occupy `0 .. 2 * m`.
///
/// Deliberately does not build a dense `ADJ` matrix, unlike `algorithm.md`'s
/// Knuth transcription: `graph.find_edge` is O(deg) and already reports "not
/// adjacent" as `None`, which is everything a dense matrix would buy here.
pub(super) struct ArcVars<'g> {
    graph: &'g UnGraph<(), ()>,
}

impl<'g> ArcVars<'g> {
    pub(super) fn new(graph: &'g UnGraph<(), ()>) -> Self {
        ArcVars { graph }
    }

    /// The SAT variable for the arc `from -> to`, or `None` if `from` and
    /// `to` are not adjacent in the graph.
    pub(super) fn var(&self, from: NodeIndex, to: NodeIndex) -> Option<u32> {
        let edge = self.graph.find_edge(from, to)?;
        let (a, _b) = self
            .graph
            .edge_endpoints(edge)
            .expect("find_edge returned an edge with no endpoints");
        let base = 2 * edge.index() as u32;
        if a == from {
            Some(base)
        } else {
            Some(base + 1)
        }
    }

    /// The arc a variable denotes; the inverse of `var`.
    ///
    /// Panics if `var` does not name an arc of this graph — every arc
    /// variable this module hands out satisfies `var < n_vars()`, so a
    /// caller that only ever passes back values it received from `var` or
    /// from a solved model cannot trigger this.
    pub(super) fn arc(&self, var: u32) -> (NodeIndex, NodeIndex) {
        let edge = EdgeIndex::new((var / 2) as usize);
        let (a, b) = self
            .graph
            .edge_endpoints(edge)
            .expect("var does not name an edge of this graph");
        if var % 2 == 0 {
            (a, b)
        } else {
            (b, a)
        }
    }

    /// Number of arc variables, `2 * m`.
    pub(super) fn n_vars(&self) -> u32 {
        2 * self.graph.edge_count() as u32
    }
}

/// Builds the CNF encoding of "this graph has a cycle cover" (Algorithm C,
/// steps C1 and C2).
///
/// Emits, in order:
///
/// 1. **Asymmetry**, one clause per edge, forbidding 2-cycles.
/// 2. **At-least-one**, two clauses per vertex (one for outgoing arcs, one
///    for incoming), so every vertex has at least one arc each way.
/// 3. **At-most-one**, pairwise `d choose 2` clauses per vertex, for outgoing
///    and incoming arcs separately, so every vertex has at most one arc each
///    way.
///
/// Total clause count is `m + 2n + 2 * sum_v C(d_v, 2)`.
pub(super) fn cycle_cover_cnf(graph: &UnGraph<(), ()>) -> Cnf {
    let vars = ArcVars::new(graph);
    let mut cnf = Cnf::new();

    // 1. Asymmetry: an edge cannot contribute both of its arcs, which would
    // be a 2-cycle.
    for edge in graph.edge_indices() {
        let base = 2 * edge.index() as u32;
        cnf.add_clause(Clause::from([Lit::negative(base), Lit::negative(base + 1)]));
    }

    // 2 & 3. At-least-one and at-most-one, per vertex, for outgoing and
    // incoming arcs separately.
    for v in graph.node_indices() {
        let neighbors: Vec<NodeIndex> = graph.neighbors(v).collect();

        let out: Vec<u32> = neighbors
            .iter()
            .map(|&u| vars.var(v, u).expect("neighbor is adjacent to v"))
            .collect();
        // var(u, v) == var(v, u) ^ 1 by construction (the ⊕1 pairing), so
        // this reuses `out`'s lookups instead of re-running `find_edge` for
        // the reverse direction of every arc.
        let inn: Vec<u32> = out.iter().map(|&x| x ^ 1).collect();

        cnf.add_clause(Clause::from_iter(out.iter().map(|&x| Lit::positive(x))));
        cnf.add_clause(Clause::from_iter(inn.iter().map(|&x| Lit::positive(x))));

        for i in 0..out.len() {
            for j in (i + 1)..out.len() {
                cnf.add_clause(Clause::from([
                    Lit::negative(out[i]),
                    Lit::negative(out[j]),
                ]));
            }
        }
        for i in 0..inn.len() {
            for j in (i + 1)..inn.len() {
                cnf.add_clause(Clause::from([
                    Lit::negative(inn[i]),
                    Lit::negative(inn[j]),
                ]));
            }
        }
    }

    cnf
}

/// Dumps a CNF formula in DIMACS format.
///
/// A debugging aid only, per `.agents/overview.md`: running a standalone
/// solver on a dumped round separates "the encoding is wrong" from "the
/// refinement is wrong".  There is no DIMACS *input* path, and none should be
/// added.
pub(super) fn write_dimacs<W: Write>(cnf: &Cnf, w: &mut W) -> io::Result<()> {
    let n_vars = cnf
        .iter()
        .flat_map(|clause| clause.iter())
        .map(|lit| lit.var().idx32() + 1)
        .max()
        .unwrap_or(0);
    writeln!(w, "p cnf {} {}", n_vars, cnf.len())?;
    for clause in cnf.iter() {
        for lit in clause.iter() {
            let dimacs = lit.var().idx32() as i64 + 1;
            let dimacs = if lit.is_neg() { -dimacs } else { dimacs };
            write!(w, "{dimacs} ")?;
        }
        writeln!(w, "0")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn v(i: usize) -> NodeIndex {
        NodeIndex::new(i)
    }

    /// Builds an undirected graph with `order` vertices and the given edges.
    fn graph_of(order: usize, edges: &[(usize, usize)]) -> UnGraph<(), ()> {
        let mut graph = UnGraph::new_undirected();
        for _ in 0..order {
            graph.add_node(());
        }
        for &(a, b) in edges {
            graph.add_edge(v(a), v(b), ());
        }
        graph
    }

    /// A clause as a sorted vector of signed integers (positive for a
    /// positive literal, negative for a negated one, 1-based variable
    /// numbering), so clause sets can be compared order-independently.
    fn signed(clause: &Clause) -> Vec<i64> {
        let mut lits: Vec<i64> = clause
            .iter()
            .map(|lit| {
                let n = lit.var().idx32() as i64 + 1;
                if lit.is_neg() {
                    -n
                } else {
                    n
                }
            })
            .collect();
        lits.sort_unstable();
        lits
    }

    fn clause_set(cnf: &Cnf) -> HashSet<Vec<i64>> {
        cnf.iter().map(signed).collect()
    }

    /// C(d, 2), used to check the total clause count formula.
    fn choose2(d: usize) -> usize {
        d * d.saturating_sub(1) / 2
    }

    mod arc_vars {
        use super::*;

        #[test]
        fn round_trips_over_every_arc() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3), (3, 0), (0, 2)]);
            let vars = ArcVars::new(&graph);
            for edge in graph.edge_indices() {
                let (a, b) = graph.edge_endpoints(edge).unwrap();
                for &(from, to) in &[(a, b), (b, a)] {
                    let var = vars.var(from, to).expect("adjacent pair has a variable");
                    assert_eq!(vars.arc(var), (from, to));
                }
            }
        }

        #[test]
        fn pairs_forward_and_backward_arcs() {
            let graph = graph_of(3, &[(0, 1), (1, 2)]);
            let vars = ArcVars::new(&graph);
            let forward = vars.var(v(0), v(1)).unwrap();
            let backward = vars.var(v(1), v(0)).unwrap();
            assert_eq!(forward ^ 1, backward);
        }

        #[test]
        fn returns_none_for_non_adjacent_pair() {
            let graph = graph_of(3, &[(0, 1), (1, 2)]);
            let vars = ArcVars::new(&graph);
            assert_eq!(vars.var(v(0), v(2)), None);
        }

        #[test]
        fn n_vars_is_twice_the_edge_count() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3)]);
            let vars = ArcVars::new(&graph);
            assert_eq!(vars.n_vars(), 2 * graph.edge_count() as u32);
        }
    }

    mod cycle_cover_cnf {
        use super::*;

        /// The `A <-> B <-> C` path from `algorithm.md`, "Clauses encoding
        /// the cycle cover" — this is the doc's own illustrative example, not
        /// a worked example from the book itself; see
        /// `knuth_thirteen_vertex_example` below for that.  Edge 0 is A-B,
        /// edge 1 is B-C, so:
        ///
        /// - var(A->B) = 0, var(B->A) = 1
        /// - var(B->C) = 2, var(C->B) = 3
        ///
        /// which pins the numbering scheme against a small worked example.
        #[test]
        fn abc_path_matches_the_book() {
            let graph = graph_of(3, &[(0, 1), (1, 2)]);
            let cnf = cycle_cover_cnf(&graph);

            assert_eq!(cnf.len(), 10);

            let expected: HashSet<Vec<i64>> = [
                // Asymmetry: not(A->B) or not(B->A); not(B->C) or not(C->B).
                // Literals sorted ascending, matching `signed`'s normal form.
                vec![-2, -1],
                vec![-4, -3],
                // At-least-one, vertex A (degree 1): out = (A->B), in = (B->A).
                vec![1],
                vec![2],
                // At-least-one, vertex B (degree 2):
                // out = (B->A or B->C), in = (A->B or C->B).
                vec![2, 3],
                vec![1, 4],
                // At-least-one, vertex C (degree 1): out = (C->B), in = (B->C).
                vec![4],
                vec![3],
                // At-most-one, vertex B: out = not(B->A) or not(B->C);
                // in = not(A->B) or not(C->B).
                vec![-3, -2],
                vec![-4, -1],
            ]
            .into_iter()
            .collect();

            assert_eq!(clause_set(&cnf), expected);
        }

        #[test]
        fn triangle_clause_count() {
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);
            let cnf = cycle_cover_cnf(&graph);
            // 3 asymmetry + 6 at-least-one + 6 at-most-one.
            assert_eq!(cnf.len(), 15);
        }

        /// The general tripwire formula from `plan.md`: `m + 2n + 2 * sum_v
        /// C(d_v, 2)`, checked against a graph with mixed degrees.
        #[test]
        fn matches_the_general_clause_count_formula() {
            let graph =
                graph_of(5, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 0), (0, 2), (1, 3)]);
            let m = graph.edge_count();
            let n = graph.node_count();
            let degree_term: usize = graph
                .node_indices()
                .map(|v| choose2(graph.neighbors(v).count()))
                .sum();
            let expected = m + 2 * n + 2 * degree_term;

            let cnf = cycle_cover_cnf(&graph);
            assert_eq!(cnf.len(), expected);
        }
    }

    mod knuth_thirteen_vertex_example {
        use super::*;

        /// Knuth's own worked example, pictured in `.agents/algorithm.md`
        /// and reused as the merge fixture in `plan.md`'s phase 9: 13
        /// vertices named A through M, 21 edges.  Node indices follow the
        /// same A=0..M=12 mapping phase 9 uses, so this fixture is meant to
        /// be shared with it.
        ///
        /// The edge list was reconstructed from the book's stated
        /// at-least-one clauses for A, B and M, cross-checked against the
        /// three cycles `plan.md` gives for the merge example (a subset of
        /// this graph's edges), and confirmed vertex-by-vertex in
        /// conversation for the rest: C-D, D's third neighbour G, E's
        /// fourth neighbour H, and H-K.
        fn graph() -> UnGraph<(), ()> {
            graph_of(
                13,
                &[
                    (0, 1),
                    (0, 2),
                    (0, 3),
                    (0, 4),
                    (0, 5), // A: B C D E F
                    (1, 8),
                    (1, 11), // B: I L
                    (2, 3),
                    (2, 6),
                    (2, 8), // C: D G I
                    (3, 6), // D: G
                    (4, 7),
                    (4, 8),
                    (4, 9), // E: H I J
                    (5, 7),
                    (5, 10),  // F: H K
                    (7, 10),  // H: K
                    (7, 12),  // H: M
                    (9, 11),  // J: L
                    (10, 12), // K: M
                    (11, 12), // L: M
                ],
            )
        }

        #[test]
        fn has_the_stated_order_and_size() {
            let graph = graph();
            assert_eq!(graph.node_count(), 13);
            assert_eq!(graph.edge_count(), 21);
        }

        #[test]
        fn clause_count_matches_the_books_breakdown() {
            let cnf = cycle_cover_cnf(&graph());
            // 21 asymmetry (one per edge) + 26 at-least-one (2 per vertex)
            // + sum_v d_v(d_v - 1) at-most-one, exactly as the book states.
            // Degrees: A5 B3 C4 D3 E4 F3 G2 H4 I3 J2 K3 L3 M3, so
            // sum d(d-1) = 20+6+12+6+12+6+2+12+6+2+6+6+6 = 102.
            assert_eq!(cnf.len(), 21 + 26 + 102);
        }

        #[test]
        fn contains_the_at_least_one_clauses_quoted_in_the_book() {
            let graph = graph();
            let cnf = cycle_cover_cnf(&graph);
            let clauses = clause_set(&cnf);
            let vars = ArcVars::new(&graph);

            let lit_set = |pairs: &[(usize, usize)]| -> Vec<i64> {
                let mut lits: Vec<i64> = pairs
                    .iter()
                    .map(|&(from, to)| {
                        vars.var(v(from), v(to)).expect("edge exists") as i64 + 1
                    })
                    .collect();
                lits.sort_unstable();
                lits
            };

            // (AB ∨ AC ∨ AD ∨ AE ∨ AF) ∧ (BA ∨ CA ∨ DA ∨ EA ∨ FA)
            assert!(clauses.contains(&lit_set(&[
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (0, 5)
            ])));
            assert!(clauses.contains(&lit_set(&[
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0)
            ])));

            // (BA ∨ BI ∨ BL) ∧ (AB ∨ IB ∨ LB)
            assert!(clauses.contains(&lit_set(&[(1, 0), (1, 8), (1, 11)])));
            assert!(clauses.contains(&lit_set(&[(0, 1), (8, 1), (11, 1)])));

            // (MH ∨ MK ∨ ML) ∧ (HM ∨ KM ∨ LM)
            assert!(clauses.contains(&lit_set(&[(12, 7), (12, 10), (12, 11)])));
            assert!(clauses.contains(&lit_set(&[(7, 12), (10, 12), (11, 12)])));
        }
    }

    mod write_dimacs {
        use super::*;

        #[test]
        fn produces_a_well_formed_header_and_one_line_per_clause() {
            let graph = graph_of(3, &[(0, 1), (1, 2)]);
            let cnf = cycle_cover_cnf(&graph);

            let mut buf = Vec::new();
            write_dimacs(&cnf, &mut buf).unwrap();
            let text = String::from_utf8(buf).unwrap();
            let mut lines = text.lines();

            let header = lines.next().expect("header line");
            let parts: Vec<&str> = header.split_whitespace().collect();
            assert_eq!(parts[0], "p");
            assert_eq!(parts[1], "cnf");
            let n_vars: usize = parts[2].parse().unwrap();
            let n_clauses: usize = parts[3].parse().unwrap();
            assert_eq!(n_vars, ArcVars::new(&graph).n_vars() as usize);
            assert_eq!(n_clauses, cnf.len());

            let clause_lines: Vec<&str> = lines.collect();
            assert_eq!(clause_lines.len(), cnf.len());
            for line in clause_lines {
                assert!(line.trim_end().ends_with('0'));
            }
        }
    }
}

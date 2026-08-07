//! Test fixtures shared **between** the test modules of `hamiltonian_paths`.
//!
//! A helper used by only one module's tests lives in that module's own `mod
//! tests`, per `AGENTS.md`.  This file is only for the handful that are
//! genuinely duplicated across modules — `v`, `graph_of`, the 13-vertex Knuth
//! example — so that a change to one of them cannot silently drift out of
//! sync with copies elsewhere.
//!
//! It is `#[cfg(test)]`, so none of this costs anything in a release build.
//!
//! It lives here in the module tree rather than in `tests/common/` because
//! these fixtures traffic in `pub(super)` types — `ArcVars`, `CycleCover` —
//! that an integration-test crate, compiled outside the crate, cannot reach.

use super::cycles::CycleCover;
use super::encoding::ArcVars;
use claim::assert_ok;
use petgraph::graph::{NodeIndex, UnGraph};
use rustsat::types::{Assignment, Lit};

pub(super) fn v(i: usize) -> NodeIndex {
    NodeIndex::new(i)
}

/// Builds an undirected graph with `order` vertices and the given edges.
pub(super) fn graph_of(order: usize, edges: &[(usize, usize)]) -> UnGraph<(), ()> {
    let mut graph = UnGraph::new_undirected();
    for _ in 0..order {
        graph.add_node(());
    }
    for &(a, b) in edges {
        graph.add_edge(v(a), v(b), ());
    }
    graph
}

/// Marks each `(from, to)` pair as a true arc variable, leaving every other
/// variable at its default (`TernaryVal::DontCare`, which `from_model` treats
/// as false).
pub(super) fn model_of(graph: &UnGraph<(), ()>, arcs: &[(usize, usize)]) -> Assignment {
    let vars = ArcVars::new(graph);
    let mut model = Assignment::default();
    for &(from, to) in arcs {
        let var = vars.var(v(from), v(to)).expect("arc names a real edge");
        model.assign_lit(Lit::positive(var));
    }
    model
}

/// Knuth's own worked example, pictured in `.agents/algorithm.md`: 13
/// vertices named A through M, 21 edges.  Node indices follow the book's
/// A=0 .. M=12 mapping.  This helper and the two below it are shared by
/// `cycles.rs`'s and `encoding.rs`'s tests, and are reused as the merge
/// fixture in `plan.md`'s phase 9.
///
/// The edge list was reconstructed from the book's stated at-least-one
/// clauses for A, B and M, cross-checked against the three cycles
/// `plan.md` gives for the merge example (a subset of this graph's edges),
/// and confirmed vertex-by-vertex in conversation for the rest: C-D, D's
/// third neighbour G, E's fourth neighbour H, and H-K.
pub(super) fn knuth_graph() -> UnGraph<(), ()> {
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

/// The three cycles making up the book's cover: A-C-G-D (0-2-6-3),
/// B-L-J-E-I (1-11-9-4-8), F-K-M-H (5-10-12-7).  Directed according to the
/// book's SUCC array, so only these 13 arcs are true.
pub(super) fn knuth_cover_arcs() -> Vec<(usize, usize)> {
    vec![
        (0, 2),
        (2, 6),
        (6, 3),
        (3, 0),
        (1, 11),
        (11, 9),
        (9, 4),
        (4, 8),
        (8, 1),
        (5, 10),
        (10, 12),
        (12, 7),
        (7, 5),
    ]
}

pub(super) fn knuth_cover() -> CycleCover {
    let graph = knuth_graph();
    let vars = ArcVars::new(&graph);
    let model = model_of(&graph, &knuth_cover_arcs());
    assert_ok!(CycleCover::from_model(&graph, &vars, &model))
}

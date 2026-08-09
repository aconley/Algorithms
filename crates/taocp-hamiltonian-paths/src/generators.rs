//! Graph generators for benchmarking and demonstration.
//!
//! These are test/bench fixtures, not solver internals, which is why this
//! module is `pub`: a bench or a `[[bin]]` is a separate crate and cannot
//! reach `pub(crate)` items.  Per-problem data (board coordinates, grid
//! coordinates) lives *outside* the graph in a side table indexed by node
//! index — the graph itself stays `UnGraph<(), ()>`, because the solver has
//! no use for coordinates and putting them in node weights would push a
//! presentation concern into the solver's data structure.
//!
//! Every generator here follows the same two rules, both load-bearing for
//! the rest of the crate:
//!
//! - **No self-loops and no parallel edges, ever.**  `UnGraph` permits both,
//!   and `precheck.rs` rejects a graph containing either, so a generator
//!   that emits a duplicate edge would hand the solver a graph it refuses
//!   to run on.  Each adjacency is added exactly once, guarded by an
//!   index comparison rather than deduplicated after the fact.
//! - **Node index order matches side-table order.**  All nodes are added
//!   first, in side-table order, before any edge, so node index `i`
//!   corresponds to side-table entry `i`.  Later phases (and any renderer)
//!   depend on this.

use petgraph::graph::{NodeIndex, UnGraph};

/// A square on a knight's-graph board, indexed by node index in the graph
/// returned alongside it.
///
/// Node index `i` corresponds to `rank * files + file == i`, i.e. squares
/// are laid out rank-major: all files of rank 0, then all files of rank 1,
/// and so on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub rank: usize,
    pub file: usize,
}

/// A cell on a grid graph, indexed by node index in the graph returned
/// alongside it.
///
/// Node index `i` corresponds to `row * cols + col == i`, i.e. cells are
/// laid out row-major: all columns of row 0, then all columns of row 1, and
/// so on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

/// The eight relative offsets a knight can move.
const KNIGHT_MOVES: [(isize, isize); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

/// Builds a knight's-move graph on a `ranks` × `files` board.
///
/// Node index `i` is `Square { rank: i / files, file: i % files }`; the
/// returned `Vec<Square>` is indexed the same way, so `squares[i]` is the
/// square at node index `i`.  Each adjacency is added exactly once: for
/// every square and every knight move, the edge is added only when the
/// target's node index is strictly greater than the source's, which visits
/// every adjacent pair exactly once without a lookup to deduplicate.
///
/// `knight_graph(0, 0)` returns an empty graph rather than panicking.
pub fn knight_graph(ranks: usize, files: usize) -> (UnGraph<(), ()>, Vec<Square>) {
    let n = ranks * files;
    // Each interior square has at most 8 knight moves, each counted once
    // from the lower-indexed endpoint; this is a loose but cheap estimate.
    let mut graph = UnGraph::with_capacity(n, n * 4);
    let mut squares = Vec::with_capacity(n);

    for rank in 0..ranks {
        for file in 0..files {
            squares.push(Square { rank, file });
            graph.add_node(());
        }
    }

    let index = |rank: usize, file: usize| -> usize { rank * files + file };

    for rank in 0..ranks {
        for file in 0..files {
            let from = index(rank, file);
            for &(dr, df) in &KNIGHT_MOVES {
                let Some(nr) = offset(rank, dr) else {
                    continue;
                };
                let Some(nf) = offset(file, df) else {
                    continue;
                };
                if nr >= ranks || nf >= files {
                    continue;
                }
                let to = index(nr, nf);
                if from < to {
                    graph.add_edge(NodeIndex::new(from), NodeIndex::new(to), ());
                }
            }
        }
    }

    (graph, squares)
}

/// Applies a signed offset to an unsigned coordinate, returning `None` on
/// underflow (rather than panicking or wrapping) so callers can treat it as
/// "off the board" alongside an overflow-side bounds check.
fn offset(coord: usize, delta: isize) -> Option<usize> {
    if delta < 0 {
        coord.checked_sub(delta.unsigned_abs())
    } else {
        coord.checked_add(delta as usize)
    }
}

/// The four relative offsets of a rook's-move-by-one, i.e. the grid graph's
/// adjacency: up, down, left, right.
const GRID_MOVES: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

/// Builds a rectangular grid graph, `rows` × `cols`, with edges between
/// orthogonally adjacent cells.
///
/// Node index `i` is `Cell { row: i / cols, col: i % cols }`; the returned
/// `Vec<Cell>` is indexed the same way. As in [`knight_graph`], each
/// adjacency is added exactly once by only adding an edge when the
/// neighbour's node index is strictly greater than the source's.
///
/// `grid_graph(1, 1)` returns a single node and no edges rather than
/// panicking.
pub fn grid_graph(rows: usize, cols: usize) -> (UnGraph<(), ()>, Vec<Cell>) {
    let n = rows * cols;
    let mut graph = UnGraph::with_capacity(n, n * 2);
    let mut cells = Vec::with_capacity(n);

    for row in 0..rows {
        for col in 0..cols {
            cells.push(Cell { row, col });
            graph.add_node(());
        }
    }

    let index = |row: usize, col: usize| -> usize { row * cols + col };

    for row in 0..rows {
        for col in 0..cols {
            let from = index(row, col);
            for &(dr, dc) in &GRID_MOVES {
                let Some(nr) = offset(row, dr) else {
                    continue;
                };
                let Some(nc) = offset(col, dc) else {
                    continue;
                };
                if nr >= rows || nc >= cols {
                    continue;
                }
                let to = index(nr, nc);
                if from < to {
                    graph.add_edge(NodeIndex::new(from), NodeIndex::new(to), ());
                }
            }
        }
    }

    (graph, cells)
}

/// Builds the Petersen graph: 10 vertices, 15 edges, 3-regular.
///
/// The outer pentagon is 0-1-2-3-4-0, the inner pentagram connects each
/// `5 + i` to `5 + (i + 2) % 5`, and the five radii connect `i` to `5 + i`.
/// There is no natural per-vertex geometry for this graph (unlike the
/// board-based generators above), so there is no side table.
pub fn petersen() -> UnGraph<(), ()> {
    let mut graph = UnGraph::with_capacity(10, 15);
    for _ in 0..10 {
        graph.add_node(());
    }

    for i in 0..5 {
        graph.add_edge(NodeIndex::new(i), NodeIndex::new((i + 1) % 5), ());
    }
    for i in 0..5 {
        graph.add_edge(NodeIndex::new(5 + i), NodeIndex::new(5 + (i + 2) % 5), ());
    }
    for i in 0..5 {
        graph.add_edge(NodeIndex::new(i), NodeIndex::new(5 + i), ());
    }

    graph
}

/// A small xorshift64* PRNG, used instead of adding a `rand` dependency for
/// what is only a deterministic test/bench fixture generator.
struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    fn new(seed: u64) -> Self {
        // xorshift is undefined at a zero state (it is a fixed point), so
        // fall back to a fixed nonzero seed rather than propagating a
        // degenerate all-zero stream.
        Self {
            state: if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            },
        }
    }

    /// Returns the next value and advances the state.
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Returns a value uniformly distributed in `[0.0, 1.0)`.
    fn next_f64(&mut self) -> f64 {
        // Use the top 53 bits, the width of an f64's mantissa, so every
        // representable output is reachable with (close to) equal weight.
        const MANTISSA_BITS: u32 = 53;
        let bits = self.next_u64() >> (64 - MANTISSA_BITS);
        (bits as f64) / ((1u64 << MANTISSA_BITS) as f64)
    }
}

/// Builds a random graph on `n` vertices, independently including each of
/// the `n * (n - 1) / 2` possible edges with probability `edge_prob`.
///
/// Deterministic from `seed`: the same seed always produces the same edge
/// set.  Pairs are considered in `(i, j)` order with `i < j`, exactly once
/// each, so no self-loop or parallel edge can arise and no deduplication is
/// needed. `edge_prob <= 0.0` yields no edges, `edge_prob >= 1.0` yields the
/// complete graph on `n` vertices; `n < 2` (including `n == 0`) yields a
/// graph with no possible edges rather than panicking.
///
/// Both of those extremes bypass the generator entirely, so `seed` has no
/// effect at either one.  A `seed` of `0` is also remapped, since it is a
/// fixed point of the underlying xorshift, and so aliases one other seed.
pub fn random_graph(n: usize, edge_prob: f64, seed: u64) -> UnGraph<(), ()> {
    let mut rng = XorShift64::new(seed);
    let max_edges = n * n.saturating_sub(1) / 2;
    let mut graph = UnGraph::with_capacity(n, max_edges);
    for _ in 0..n {
        graph.add_node(());
    }

    // Decided once rather than per pair.  A NaN `edge_prob` fails every
    // comparison here, including the one below, and so yields no edges.
    if edge_prob <= 0.0 {
        return graph;
    }
    let complete = edge_prob >= 1.0;

    for i in 0..n {
        for j in (i + 1)..n {
            if complete || rng.next_f64() < edge_prob {
                graph.add_edge(NodeIndex::new(i), NodeIndex::new(j), ());
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Every generator's output must be free of self-loops and parallel
    /// edges: `precheck.rs` rejects both, so a generator that produced
    /// either would hand the solver a graph it refuses to run on.
    fn assert_simple(graph: &UnGraph<(), ()>) {
        let mut seen = HashSet::new();
        for edge in graph.raw_edges() {
            let (a, b) = (edge.source(), edge.target());
            assert_ne!(a, b, "self-loop at {a:?}");
            let key = if a < b { (a, b) } else { (b, a) };
            assert!(seen.insert(key), "parallel edge between {a:?} and {b:?}");
        }
    }

    mod knight_graph {
        use super::*;

        #[test]
        fn six_by_six_has_36_vertices_and_80_edges() {
            let (graph, squares) = knight_graph(6, 6);
            assert_eq!(graph.node_count(), 36);
            assert_eq!(graph.edge_count(), 80);
            assert_eq!(squares.len(), 36);
            assert_simple(&graph);
        }

        #[test]
        fn node_index_matches_side_table() {
            let (_, squares) = knight_graph(4, 5);
            for (i, square) in squares.iter().enumerate() {
                assert_eq!(
                    *square,
                    Square {
                        rank: i / 5,
                        file: i % 5
                    }
                );
            }
        }

        #[test]
        fn degenerate_zero_by_zero_does_not_panic() {
            let (graph, squares) = knight_graph(0, 0);
            assert_eq!(graph.node_count(), 0);
            assert_eq!(graph.edge_count(), 0);
            assert!(squares.is_empty());
        }

        #[test]
        fn one_by_one_has_no_edges() {
            let (graph, squares) = knight_graph(1, 1);
            assert_eq!(graph.node_count(), 1);
            assert_eq!(graph.edge_count(), 0);
            assert_eq!(squares.len(), 1);
        }

        /// Rebuilds `lib.rs`'s hand-written `knight_graph_5x5()` here — it is
        /// test-private there, so it cannot be imported — and asserts the
        /// generated edge set matches it exactly.
        ///
        /// That comparison alone is weaker than it looks: the reference uses
        /// the same move table and the same lower-index-wins guard as the
        /// generator, so a shared misconception would pass.  The independently
        /// known edge count of the 5x5 knight graph, 48, is asserted first for
        /// that reason, exactly as 80 is for the 6x6 case above.
        #[test]
        fn five_by_five_matches_hand_written_reference() {
            let moves: [(isize, isize); 8] = [
                (-2, -1),
                (-2, 1),
                (-1, -2),
                (-1, 2),
                (1, -2),
                (1, 2),
                (2, -1),
                (2, 1),
            ];
            let mut expected = HashSet::new();
            for row in 0..5isize {
                for col in 0..5isize {
                    let v1 = row * 5 + col;
                    for &(dr, dc) in &moves {
                        let (nr, nc) = (row + dr, col + dc);
                        if (0..5).contains(&nr) && (0..5).contains(&nc) {
                            let v2 = nr * 5 + nc;
                            if v1 < v2 {
                                expected.insert((v1 as usize, v2 as usize));
                            }
                        }
                    }
                }
            }

            let (graph, _) = knight_graph(5, 5);
            assert_eq!(graph.edge_count(), 48);
            assert_eq!(graph.edge_count(), expected.len());
            assert_simple(&graph);

            let actual: HashSet<(usize, usize)> = graph
                .raw_edges()
                .iter()
                .map(|e| (e.source().index(), e.target().index()))
                .collect();
            assert_eq!(actual, expected);
        }
    }

    mod grid_graph {
        use super::*;

        #[test]
        fn two_by_three_has_known_edges() {
            // 0 1 2
            // 3 4 5
            // Horizontal: (0,1) (1,2) (3,4) (4,5); vertical: (0,3) (1,4) (2,5).
            let (graph, cells) = grid_graph(2, 3);
            assert_eq!(graph.node_count(), 6);
            assert_eq!(cells.len(), 6);
            assert_simple(&graph);

            let expected: HashSet<(usize, usize)> =
                [(0, 1), (1, 2), (3, 4), (4, 5), (0, 3), (1, 4), (2, 5)]
                    .into_iter()
                    .collect();
            let actual: HashSet<(usize, usize)> = graph
                .raw_edges()
                .iter()
                .map(|e| (e.source().index(), e.target().index()))
                .collect();
            assert_eq!(actual, expected);
        }

        #[test]
        fn node_index_matches_side_table() {
            let (_, cells) = grid_graph(3, 4);
            for (i, cell) in cells.iter().enumerate() {
                assert_eq!(
                    *cell,
                    Cell {
                        row: i / 4,
                        col: i % 4
                    }
                );
            }
        }

        #[test]
        fn one_by_one_has_single_node_and_no_edges() {
            let (graph, cells) = grid_graph(1, 1);
            assert_eq!(graph.node_count(), 1);
            assert_eq!(graph.edge_count(), 0);
            assert_eq!(cells, vec![Cell { row: 0, col: 0 }]);
        }
    }

    mod petersen {
        use super::*;

        #[test]
        fn has_10_vertices_15_edges_and_is_3_regular() {
            let graph = petersen();
            assert_eq!(graph.node_count(), 10);
            assert_eq!(graph.edge_count(), 15);
            assert_simple(&graph);
            for node in graph.node_indices() {
                assert_eq!(
                    graph.neighbors(node).count(),
                    3,
                    "vertex {node:?} is not degree 3"
                );
            }
        }
    }

    mod random_graph {
        use super::*;

        #[test]
        fn zero_probability_has_no_edges() {
            let graph = random_graph(20, 0.0, 42);
            assert_eq!(graph.edge_count(), 0);
        }

        #[test]
        fn one_probability_is_complete() {
            let n = 12;
            let graph = random_graph(n, 1.0, 42);
            assert_eq!(graph.edge_count(), n * (n - 1) / 2);
            assert_simple(&graph);
        }

        #[test]
        fn same_seed_is_deterministic() {
            let a = random_graph(30, 0.5, 12345);
            let b = random_graph(30, 0.5, 12345);
            assert_eq!(edge_set(&a), edge_set(&b));
        }

        #[test]
        fn different_seeds_differ() {
            let a = random_graph(30, 0.5, 1);
            let b = random_graph(30, 0.5, 2);
            assert_ne!(edge_set(&a), edge_set(&b));
        }

        #[test]
        fn is_always_simple() {
            let graph = random_graph(25, 0.5, 7);
            assert_simple(&graph);
        }

        #[test]
        fn degenerate_zero_and_one_vertex_do_not_panic() {
            let empty = random_graph(0, 0.5, 1);
            assert_eq!(empty.node_count(), 0);
            assert_eq!(empty.edge_count(), 0);

            let single = random_graph(1, 0.5, 1);
            assert_eq!(single.node_count(), 1);
            assert_eq!(single.edge_count(), 0);
        }

        fn edge_set(graph: &UnGraph<(), ()>) -> HashSet<(usize, usize)> {
            graph
                .raw_edges()
                .iter()
                .map(|e| (e.source().index(), e.target().index()))
                .collect()
        }
    }
}

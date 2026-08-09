//! Turning a [`Decomposition`] into something a human can look at.
//!
//! Plain functions, called directly by whatever generated the graph — there
//! is no `Renderer` trait and no dispatch enum, since the caller always knows
//! which family it generated and there is no runtime dispatch problem to
//! solve.
//!
//! All three renderers accept a final answer — a [`crate::HamiltonianCycle`]
//! or [`crate::HamiltonianPath`] — or a mid-refinement [`Decomposition`], via
//! `impl Into<Decomposition>`.  Internally they all render a decomposition:
//! a genuine Hamiltonian cycle is the degenerate one-segment case, and a
//! spurious intermediate model from a refinement round is the multi-segment
//! case.  One renderer body serves both, which is what makes it possible to
//! emit one image per round and get a flipbook of the abstraction
//! tightening — see `.agents/overview.md`, "Rendering".

use crate::generators::Square;
use crate::segment::Decomposition;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{EdgeIndex, NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

/// A small fixed palette, cycled by segment index, so that a multi-segment
/// decomposition (a spurious cover mid-refinement) renders each disjoint
/// cycle in a visibly different color.
const PALETTE: [&str; 8] = [
    "red",
    "blue",
    "darkgreen",
    "purple",
    "orange",
    "brown",
    "magenta",
    "teal",
];

/// Maps each covered vertex to the label it should show on a board: its
/// 1-based position within its segment, prefixed by a letter identifying the
/// segment when there is more than one.
///
/// `decomposition.segments()` is already sorted into a canonical order (see
/// [`Decomposition::new`]), so the letter assignment — `'A'` for
/// `segments()[0]`, `'B'` for `segments()[1]`, and so on — is deterministic.
fn vertex_labels(decomposition: &Decomposition) -> HashMap<NodeIndex, String> {
    let multi = decomposition.len() > 1;
    let mut labels = HashMap::with_capacity(decomposition.covered_vertices());
    for (segment_index, segment) in decomposition.segments().iter().enumerate() {
        for (position, &vertex) in segment.vertices().iter().enumerate() {
            let visit = position + 1;
            let label = if multi {
                let letter = (b'A' + segment_index as u8) as char;
                format!("{letter}{visit}")
            } else {
                visit.to_string()
            };
            labels.insert(vertex, label);
        }
    }
    labels
}

/// Renders a knight's-board (or any rank/file board) decomposition as ASCII,
/// with each covered square showing its 1-based visit order within its
/// segment.
///
/// In the single-segment case (a genuine Hamiltonian cycle) squares show a
/// plain number.  In the multi-segment case (a spurious cover mid-refinement)
/// squares show a segment letter followed by the position within it, so a
/// reader can tell which disjoint cycle a square belongs to.  Every vertex is
/// covered by exactly one segment — cycle covers are total — so there are no
/// blank cells.
///
/// Rank 0 is printed first (top row), then increasing rank; within a row,
/// file 0 is printed first (left), then increasing file, matching
/// [`crate::generators::knight_graph`]'s own indexing.
pub fn board_ascii(
    decomposition: impl Into<Decomposition>,
    squares: &[Square],
    ranks: usize,
    files: usize,
) -> String {
    let decomposition = decomposition.into();
    let labels = vertex_labels(&decomposition);
    let width = labels.values().map(String::len).max().unwrap_or(1);

    let mut index_of = HashMap::with_capacity(squares.len());
    for (i, square) in squares.iter().enumerate() {
        index_of.insert((square.rank, square.file), NodeIndex::new(i));
    }

    let mut rows = Vec::with_capacity(ranks);
    for rank in 0..ranks {
        let mut cells = Vec::with_capacity(files);
        for file in 0..files {
            let cell = match index_of.get(&(rank, file)) {
                Some(vertex) => match labels.get(vertex) {
                    Some(label) => format!("{label:>width$}"),
                    None => format!("{:>width$}", "?"),
                },
                None => format!("{:>width$}", "?"),
            };
            cells.push(cell);
        }
        rows.push(cells.join(" "));
    }
    let mut out = rows.join("\n");
    out.push('\n');
    out
}

/// Renders `graph` as Graphviz DOT, highlighting the edges belonging to
/// `decomposition`'s segments.
///
/// Each segment is highlighted in a different color, cycling through a small
/// fixed palette by segment index — useful when `decomposition` is a
/// multi-cycle spurious cover, since it makes the disjoint cycles visually
/// distinguishable.  Render the output with Graphviz externally; petgraph has
/// no DOT reader and none is wanted.
pub fn dot(graph: &UnGraph<(), ()>, decomposition: impl Into<Decomposition>) -> String {
    let decomposition = decomposition.into();
    let mut highlighted: HashMap<EdgeIndex, usize> = HashMap::new();
    for (segment_index, segment) in decomposition.segments().iter().enumerate() {
        for (u, v) in segment.edges() {
            // `find_edge` is direction-agnostic on an `UnGraph`, so this
            // matches the edge regardless of which orientation the segment
            // traversed it in.
            if let Some(edge) = graph.find_edge(u, v) {
                highlighted.insert(edge, segment_index);
            }
        }
    }

    let get_edge_attributes =
        |_: &UnGraph<(), ()>, edge: petgraph::graph::EdgeReference<'_, ()>| {
            match highlighted.get(&edge.id()) {
                Some(&segment_index) => {
                    let color = PALETTE[segment_index % PALETTE.len()];
                    format!("color={color}, penwidth=2")
                }
                None => String::new(),
            }
        };
    let get_node_attributes = |_: &UnGraph<(), ()>, _: (NodeIndex, &())| String::new();

    // `()` implements `Debug` but not `Display`, and `Dot`'s `Display` impl
    // requires `NodeWeight: Display` / `EdgeWeight: Display`, which `()`
    // does not satisfy. Its `Debug` impl only requires `Debug`, so `{:?}`
    // is required here, not `{}`.
    format!(
        "{:?}",
        Dot::with_attr_getters(
            graph,
            &[Config::NodeIndexLabel, Config::EdgeNoLabel],
            &get_edge_attributes,
            &get_node_attributes,
        )
    )
}

/// A margin, in `coords` units, added around the computed bounding box so
/// vertices at the edge of the layout are not clipped against the `viewBox`.
const SVG_MARGIN: f64 = 20.0;

/// Renders `graph` as a hand-rolled SVG document, for geometric instances
/// where `coords` (indexed by node index, exactly like a generator's side
/// table) gives each vertex a natural position.
///
/// The full graph is drawn first as a faint background grid, then the edges
/// of `decomposition`'s segments are drawn on top in bold, distinct colors
/// (one per segment, cycling the same palette as [`dot`]), then a small
/// circle and node-index label at each vertex.  Drawing the whole graph
/// faintly underneath is deliberate; keeping the solution segments visually
/// dominant is what matters, not the literal exclusion of every other edge.
pub fn svg(
    graph: &UnGraph<(), ()>,
    decomposition: impl Into<Decomposition>,
    coords: &[(f64, f64)],
) -> String {
    if coords.is_empty() {
        return "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>\n".to_string();
    }
    let decomposition = decomposition.into();

    let (mut min_x, mut max_x) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut min_y, mut max_y) = (f64::INFINITY, f64::NEG_INFINITY);
    for &(x, y) in coords {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let view_min_x = min_x - SVG_MARGIN;
    let view_min_y = min_y - SVG_MARGIN;
    let view_width = (max_x - min_x) + 2.0 * SVG_MARGIN;
    let view_height = (max_y - min_y) + 2.0 * SVG_MARGIN;

    let mut out = String::new();
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{view_min_x} {view_min_y} {view_width} {view_height}\">\n"
    ));

    // Background grid: every edge of the graph, drawn thin and muted.
    for edge in graph.edge_references() {
        let (a, b) = (edge.source(), edge.target());
        if let (Some(&(x1, y1)), Some(&(x2, y2))) =
            (coords.get(a.index()), coords.get(b.index()))
        {
            out.push_str(&format!(
                "  <line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" \
                 stroke=\"lightgray\" stroke-width=\"1\" />\n"
            ));
        }
    }

    // The solution (or spurious cover): bold, one color per segment.
    for (segment_index, segment) in decomposition.segments().iter().enumerate() {
        let color = PALETTE[segment_index % PALETTE.len()];
        for (u, v) in segment.edges() {
            if let (Some(&(x1, y1)), Some(&(x2, y2))) =
                (coords.get(u.index()), coords.get(v.index()))
            {
                out.push_str(&format!(
                    "  <line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" \
                     stroke=\"{color}\" stroke-width=\"4\" />\n"
                ));
            }
        }
    }

    // A circle and label at every vertex.
    for (i, &(x, y)) in coords.iter().enumerate() {
        out.push_str(&format!(
            "  <circle cx=\"{x}\" cy=\"{y}\" r=\"4\" fill=\"black\" />\n"
        ));
        out.push_str(&format!(
            "  <text x=\"{x}\" y=\"{y}\" font-size=\"8\">{i}</text>\n"
        ));
    }

    out.push_str("</svg>\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::knight_graph;
    use crate::segment::Segment;
    use crate::solution::HamiltonianCycle;
    use claim::assert_ok;
    use petgraph::graph::NodeIndex;

    fn v(i: usize) -> NodeIndex {
        NodeIndex::new(i)
    }

    mod board_ascii {
        use super::*;

        /// A known closed knight's tour on a 6x6 board, found once by running
        /// `find_hamiltonian_cycle(&knight_graph(6, 6).0)` (see the phase 11
        /// implementation notes); hard-coded here rather than derived by
        /// calling the solver from the test, which would make the test slow
        /// and couple it to solver internals.
        fn six_by_six_tour_vertices() -> Vec<usize> {
            vec![
                0, 8, 4, 17, 28, 32, 24, 20, 7, 18, 31, 27, 35, 22, 9, 5, 16, 3, 11, 15,
                2, 10, 23, 34, 26, 30, 19, 6, 14, 1, 12, 25, 33, 29, 21, 13,
            ]
        }

        #[test]
        fn single_cycle_golden_string() {
            let (graph, squares) = knight_graph(6, 6);
            let vertices: Vec<NodeIndex> = six_by_six_tour_vertices()
                .into_iter()
                .map(NodeIndex::new)
                .collect();
            let segment = assert_ok!(Segment::new_closed(vertices));

            // A transcription typo in the hard-coded tour above should fail
            // loudly here, rather than silently golden-mastering a broken
            // tour.
            assert!(
                segment.is_hamiltonian_cycle(&graph),
                "hard-coded tour is not a genuine Hamiltonian cycle of the \
                 6x6 knight's graph"
            );

            let decomposition = assert_ok!(Decomposition::new(vec![segment]));
            let rendered = board_ascii(&decomposition, &squares, 6, 6);

            let expected = concat!(
                " 1 30 21 18  3 16\n",
                "28  9  2 15 22 19\n",
                "31 36 29 20 17  4\n",
                "10 27  8 35 14 23\n",
                " 7 32 25 12  5 34\n",
                "26 11  6 33 24 13\n",
            );
            assert_eq!(rendered, expected);
        }

        /// The same tour and golden string as above, fed through a
        /// `HamiltonianCycle` instead of a hand-built `Decomposition`, to pin
        /// the `impl Into<Decomposition>` plumbing: the output must be
        /// byte-identical.
        #[test]
        fn single_cycle_via_hamiltonian_cycle_matches_golden_string() {
            let (graph, squares) = knight_graph(6, 6);
            let vertices: Vec<NodeIndex> = six_by_six_tour_vertices()
                .into_iter()
                .map(NodeIndex::new)
                .collect();
            let segment = assert_ok!(Segment::new_closed(vertices));
            assert!(segment.is_hamiltonian_cycle(&graph));
            let cycle = HamiltonianCycle::new(segment);

            let rendered = board_ascii(&cycle, &squares, 6, 6);

            let expected = concat!(
                " 1 30 21 18  3 16\n",
                "28  9  2 15 22 19\n",
                "31 36 29 20 17  4\n",
                "10 27  8 35 14 23\n",
                " 7 32 25 12  5 34\n",
                "26 11  6 33 24 13\n",
            );
            assert_eq!(rendered, expected);
        }

        #[test]
        fn multi_segment_uses_letter_prefixes() {
            // A tiny 3x4 grid-like knight instance is unnecessary; use two
            // disjoint triangles on a hand-built graph laid out on a 2x3
            // "board" instead, addressed through a hand-built Square side
            // table so the test does not depend on any particular generator.
            let squares = vec![
                Square { rank: 0, file: 0 },
                Square { rank: 0, file: 1 },
                Square { rank: 0, file: 2 },
                Square { rank: 1, file: 0 },
                Square { rank: 1, file: 1 },
                Square { rank: 1, file: 2 },
            ];

            let a = assert_ok!(Segment::new_closed(vec![v(0), v(1), v(2)]));
            let b = assert_ok!(Segment::new_closed(vec![v(3), v(4), v(5)]));
            let decomposition = assert_ok!(Decomposition::new(vec![a, b]));

            let rendered = board_ascii(&decomposition, &squares, 2, 3);

            let expected = concat!("A1 A2 A3\n", "B1 B2 B3\n");
            assert_eq!(rendered, expected);
        }
    }

    mod dot {
        use super::*;
        use crate::testing::graph_of;

        #[test]
        fn contains_one_highlighted_entry_per_solution_edge() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
            let segment = assert_ok!(Segment::new_closed(vec![v(0), v(1), v(2), v(3)]));
            let decomposition = assert_ok!(Decomposition::new(vec![segment]));

            let rendered = dot(&graph, &decomposition);
            assert!(!rendered.is_empty());

            let solution_edges: usize = decomposition
                .segments()
                .iter()
                .map(|s| s.edges().len())
                .sum();
            let highlighted_count = rendered.matches("penwidth=2").count();
            assert_eq!(highlighted_count, solution_edges);
        }

        #[test]
        fn multi_segment_highlights_every_segments_edges() {
            let graph = graph_of(6, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]);
            let a = assert_ok!(Segment::new_closed(vec![v(0), v(1), v(2)]));
            let b = assert_ok!(Segment::new_closed(vec![v(3), v(4), v(5)]));
            let decomposition = assert_ok!(Decomposition::new(vec![a, b]));

            let rendered = dot(&graph, &decomposition);
            let solution_edges: usize = decomposition
                .segments()
                .iter()
                .map(|s| s.edges().len())
                .sum();
            let highlighted_count = rendered.matches("penwidth=2").count();
            assert_eq!(highlighted_count, solution_edges);
        }

        /// Pins the `impl Into<Decomposition>` plumbing: a `HamiltonianCycle`
        /// must highlight the same edges as the equivalent `Decomposition`.
        #[test]
        fn accepts_hamiltonian_cycle_via_into() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
            let segment = assert_ok!(Segment::new_closed(vec![v(0), v(1), v(2), v(3)]));
            let cycle = HamiltonianCycle::new(segment);

            let rendered = dot(&graph, &cycle);
            let highlighted_count = rendered.matches("penwidth=2").count();
            assert_eq!(highlighted_count, 4);
        }
    }

    mod svg {
        use super::*;
        use crate::testing::graph_of;

        #[test]
        fn well_formed_and_has_one_element_per_solution_edge() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
            let segment = assert_ok!(Segment::new_closed(vec![v(0), v(1), v(2), v(3)]));
            let decomposition = assert_ok!(Decomposition::new(vec![segment]));
            let coords = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];

            let rendered = svg(&graph, &decomposition, &coords);

            assert!(rendered.starts_with("<svg"));
            assert!(rendered.contains("</svg>"));

            let solution_edges: usize = decomposition
                .segments()
                .iter()
                .map(|s| s.edges().len())
                .sum();
            let highlighted_count = rendered.matches("stroke-width=\"4\"").count();
            assert_eq!(highlighted_count, solution_edges);
        }

        /// Pins the `impl Into<Decomposition>` plumbing: a `HamiltonianCycle`
        /// must highlight the same edges as the equivalent `Decomposition`.
        #[test]
        fn accepts_hamiltonian_cycle_via_into() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
            let segment = assert_ok!(Segment::new_closed(vec![v(0), v(1), v(2), v(3)]));
            let cycle = HamiltonianCycle::new(segment);
            let coords = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];

            let rendered = svg(&graph, &cycle, &coords);
            let highlighted_count = rendered.matches("stroke-width=\"4\"").count();
            assert_eq!(highlighted_count, 4);
        }

        #[test]
        fn empty_coords_does_not_panic() {
            let graph = graph_of(0, &[]);
            let decomposition = assert_ok!(Decomposition::new(vec![]));
            let rendered = svg(&graph, &decomposition, &[]);
            assert!(rendered.starts_with("<svg"));
            assert!(rendered.contains("</svg>"));
        }
    }
}

//! Preconditions that Algorithm C's abstraction cannot see, checked before the
//! CEGAR loop ever starts.
//!
//! See `.agents/algorithm.md`, just above "Algorithm C", for the three checks
//! Knuth lists: the graph is connected and nonempty, every vertex has degree
//! at least 2, and the graph has no bridge or cut vertex.  Each of these
//! genuinely proves no Hamiltonian cycle exists, so a hit here is reported by
//! the driver as `Ok(None)`, never as an error.
//!
//! Two further checks belong here that Knuth has no reason to mention, because
//! they are artifacts of `UnGraph` rather than of the mathematics: it permits
//! **self-loops** and **parallel edges**, and both break the arc-variable
//! mapping in `encoding.rs` — a self-loop gets two arc variables naming one
//! vertex, and parallel edges get independent variables for a single
//! adjacency.
//!
//! Connectivity, bridges and articulation points are petgraph's own
//! implementations (`.agents/overview.md` explicitly permits this); this
//! module only adds the self-loop and parallel-edge checks and the ordering
//! between all five.

use petgraph::algo::articulation_points::articulation_points;
use petgraph::algo::{bridges, connected_components};
use petgraph::graph::{EdgeIndex, NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use std::collections::HashSet;

/// A structural reason no Hamiltonian cycle can exist, found before the SAT
/// loop starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Obstruction {
    /// The graph has no vertices.
    Empty,
    /// An edge from a vertex to itself; breaks the arc-variable mapping.
    SelfLoop(EdgeIndex),
    /// Two edges between the same pair of vertices; breaks the arc-variable
    /// mapping, which assumes one adjacency has exactly two arc variables.
    ParallelEdges(NodeIndex, NodeIndex),
    /// More than one connected component.
    Disconnected,
    /// A vertex with fewer than two neighbours; no cycle can pass through it.
    LowDegree(NodeIndex),
    /// An edge whose removal would disconnect the graph.
    Bridge(EdgeIndex),
    /// A vertex whose removal would disconnect the graph.
    ArticulationPoint(NodeIndex),
}

/// Checks the preconditions Algorithm C relies on, in the order that avoids
/// running a later check on a graph the earlier ones already condemn:
///
/// 1. Empty.
/// 2. Self-loops and parallel edges (artifacts of `UnGraph`, checked first so
///    later checks never have to reason about a graph shaped like that).
/// 3. Connectivity.
/// 4. Minimum degree.
/// 5. Bridges, then articulation points.
///
/// Returns the first obstruction found, or `None` if the graph clears every
/// check.
pub(super) fn check_preconditions(graph: &UnGraph<(), ()>) -> Option<Obstruction> {
    if graph.node_count() == 0 {
        return Some(Obstruction::Empty);
    }

    for edge in graph.edge_indices() {
        let (a, b) = graph
            .edge_endpoints(edge)
            .expect("edge_indices() only yields edges with endpoints");
        if a == b {
            return Some(Obstruction::SelfLoop(edge));
        }
    }

    for v in graph.node_indices() {
        let mut seen: HashSet<NodeIndex> = HashSet::new();
        for u in graph.neighbors(v) {
            if !seen.insert(u) {
                let (lo, hi) = if v <= u { (v, u) } else { (u, v) };
                return Some(Obstruction::ParallelEdges(lo, hi));
            }
        }
    }

    if connected_components(graph) != 1 {
        return Some(Obstruction::Disconnected);
    }

    for v in graph.node_indices() {
        if graph.neighbors(v).count() < 2 {
            return Some(Obstruction::LowDegree(v));
        }
    }

    if let Some(edge) = bridges(graph).next() {
        return Some(Obstruction::Bridge(edge.id()));
    }

    // articulation_points() returns a HashSet with no stable order; take the
    // smallest index so the result is deterministic.
    if let Some(&v) = articulation_points(graph).iter().min() {
        return Some(Obstruction::ArticulationPoint(v));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hamiltonian_paths::testing::{graph_of, v};

    mod check_preconditions {
        use super::*;

        #[test]
        fn empty_graph_is_empty() {
            let graph = graph_of(0, &[]);
            assert_eq!(check_preconditions(&graph), Some(Obstruction::Empty));
        }

        #[test]
        fn self_loop_is_reported() {
            let graph = graph_of(3, &[(0, 0), (0, 1), (1, 2), (2, 0)]);
            let self_loop_edge = graph.find_edge(v(0), v(0)).unwrap();
            assert_eq!(
                check_preconditions(&graph),
                Some(Obstruction::SelfLoop(self_loop_edge))
            );
        }

        #[test]
        fn parallel_edges_are_reported() {
            let graph = graph_of(3, &[(0, 1), (0, 1), (1, 2), (2, 0)]);
            assert_eq!(
                check_preconditions(&graph),
                Some(Obstruction::ParallelEdges(v(0), v(1)))
            );
        }

        #[test]
        fn disconnected_graph_is_reported() {
            // Two triangles, no edge between them. Each triangle alone is
            // 2-connected, so this exercises connectivity specifically, not
            // degree or bridges.
            let graph = graph_of(6, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]);
            assert_eq!(check_preconditions(&graph), Some(Obstruction::Disconnected));
        }

        #[test]
        fn low_degree_vertex_is_reported() {
            // A triangle with a pendant vertex hanging off of it: vertex 3
            // has degree 1.
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 0), (2, 3)]);
            assert_eq!(
                check_preconditions(&graph),
                Some(Obstruction::LowDegree(v(3)))
            );
        }

        #[test]
        fn bridge_is_reported() {
            // Two triangles joined by a single edge: connected, every vertex
            // has degree >= 2, but the joining edge is a bridge (and its
            // endpoints are articulation points). Bridges are checked first,
            // so that is the variant produced.
            let graph =
                graph_of(6, &[(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 5), (5, 3)]);
            let bridge_edge = graph.find_edge(v(2), v(3)).unwrap();
            assert_eq!(
                check_preconditions(&graph),
                Some(Obstruction::Bridge(bridge_edge))
            );
        }

        #[test]
        fn articulation_point_is_reported() {
            // Two triangles sharing a single vertex: connected, every vertex
            // has degree >= 2, and there is no bridge (both triangles'
            // cycles remain after removing any one edge), but the shared
            // vertex is an articulation point.
            let graph = graph_of(5, &[(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 2)]);
            assert_eq!(
                check_preconditions(&graph),
                Some(Obstruction::ArticulationPoint(v(2)))
            );
        }

        #[test]
        fn two_connected_graph_passes() {
            // A 4-cycle: connected, minimum degree 2, no bridges, no
            // articulation points.
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
            assert_eq!(check_preconditions(&graph), None);
        }
    }
}

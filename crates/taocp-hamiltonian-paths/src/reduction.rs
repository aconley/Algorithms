//! Reducing Hamiltonian *path* queries to Hamiltonian *cycle* queries.
//!
//! The CEGAR engine only ever searches for Hamiltonian cycles.  Paths are
//! obtained by the standard construction: add one new **apex** vertex *u* to
//! *G*, joined to **every** vertex of *G*.  The resulting graph *G′* has a
//! Hamiltonian cycle exactly when *G* has a Hamiltonian path — the cycle enters
//! *u* from one end of the path and leaves towards the other, so deleting *u*
//! recovers the path.
//!
//! That is the whole reduction.  It takes no parameters, and the CEGAR code
//! never learns that paths exist at all.
//!
//! There is deliberately no way to ask for a path with particular endpoints.
//! Only "is there *any* Hamiltonian path" is wanted, which is the question CEGAR
//! answers; see the "Path-to-cycle reduction" section of `.agents/overview.md`
//! for the cost analysis behind that, and for what would have to change if
//! endpoint-constrained queries were ever needed.
//!
//! **The apex is added last**, so that `NodeIndex` 0..n−1 of *G′* are exactly
//! the vertices of *G* and the apex is index *n*.  Original edge indices are
//! likewise preserved, which means a SAT variable denotes the same edge in both
//! graphs.  Nothing here needs a remapping table, and nothing should introduce
//! one.

use super::segment::Segment;
use petgraph::graph::{NodeIndex, UnGraph};
use std::fmt;

/// Failure to reduce a path query to a cycle query, or to translate the answer back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReductionError {
    /// The graph has fewer than two vertices, so the apex construction cannot
    /// produce a cycle of length three.  Callers special-case these sizes before
    /// reaching here.
    GraphTooSmall(usize),
    /// A segment handed back for translation was open, so it is not a cycle of
    /// the reduced graph at all.  Without this check the translation would
    /// silently succeed and return a plausible-looking path.
    CycleNotClosed,
    /// A cycle handed back for translation did not pass through the apex, so it
    /// did not come from this instance.
    ApexNotInCycle,
    /// A cycle handed back for translation did not span the reduced graph.
    CycleNotSpanning,
}

impl fmt::Display for ReductionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReductionError::GraphTooSmall(n) => {
                write!(
                    f,
                    "graph has {n} vertices, need at least 2 for the apex construction"
                )
            }
            ReductionError::CycleNotClosed => {
                write!(f, "segment is open, so it is not a cycle")
            }
            ReductionError::ApexNotInCycle => {
                write!(f, "cycle does not pass through the apex vertex")
            }
            ReductionError::CycleNotSpanning => {
                write!(f, "cycle does not span the reduced graph")
            }
        }
    }
}

impl std::error::Error for ReductionError {}

/// A Hamiltonian path query rewritten as a Hamiltonian cycle query.
///
/// Owns the reduced graph *G′* and remembers where the apex went, so that a
/// cycle found in *G′* can be translated back into a path in *G*.
#[derive(Debug, Clone)]
pub(crate) struct CycleInstance {
    graph: UnGraph<(), ()>,
    apex: NodeIndex,
    original_order: usize,
}

impl CycleInstance {
    /// Builds *G′* from *G* by adding the apex vertex and joining it to every
    /// original vertex.
    ///
    /// The apex is added after every original vertex, and its edges after every
    /// original edge, preserving both index spaces.
    ///
    /// This function is deliberately independent of the solver: it can and
    /// should be tested on its own.  Reductions with an off-by-one fail far
    /// downstream, where the symptom is an answer that merely looks wrong.
    pub(crate) fn new(graph: &UnGraph<(), ()>) -> Result<Self, ReductionError> {
        let original_order = graph.node_count();
        if original_order < 2 {
            return Err(ReductionError::GraphTooSmall(original_order));
        }

        let mut reduced = graph.clone();
        // Capture the original vertices before adding the apex, so the apex
        // itself is never joined to itself.
        let originals: Vec<NodeIndex> = reduced.node_indices().collect();
        let apex = reduced.add_node(());
        for v in originals {
            reduced.add_edge(v, apex, ());
        }

        Ok(CycleInstance {
            graph: reduced,
            apex,
            original_order,
        })
    }

    /// The reduced graph, the one actually handed to the CEGAR engine.
    pub(crate) fn graph(&self) -> &UnGraph<(), ()> {
        &self.graph
    }

    /// Translates a Hamiltonian cycle of *G′* into the corresponding
    /// Hamiltonian path of *G*.
    ///
    /// Rotate the cycle so the apex comes first, drop it, and what remains is an
    /// open segment over the original vertices in their original indices.  The
    /// result is returned canonicalised.
    pub(crate) fn path_from_cycle(
        &self,
        cycle: &Segment,
    ) -> Result<Segment, ReductionError> {
        if !cycle.is_closed() {
            return Err(ReductionError::CycleNotClosed);
        }

        let vertices = cycle.vertices();
        let apex_pos = vertices
            .iter()
            .position(|&v| v == self.apex)
            .ok_or(ReductionError::ApexNotInCycle)?;
        if vertices.len() != self.original_order + 1 {
            return Err(ReductionError::CycleNotSpanning);
        }

        let mut rotated = vertices.to_vec();
        rotated.rotate_left(apex_pos);
        // `rotated[0]` is now the apex; the rest is the path in order.
        let path_vertices = rotated[1..].to_vec();

        // The cycle's own construction already guarantees distinct vertices,
        // and `original_order >= 2`, so this cannot fail.
        let mut path = Segment::new_open(path_vertices)
            .expect("dropping the apex leaves a non-empty set of distinct vertices");
        path.canonicalize();
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{graph_of, v};
    use claim::{assert_err, assert_ok};

    /// Building the reduced graph, and the index-preservation invariant that
    /// the SAT encoding downstream depends on.
    mod new {
        use super::*;

        #[test]
        fn rejects_order_zero() {
            let err = assert_err!(CycleInstance::new(&graph_of(0, &[])));
            assert_eq!(err, ReductionError::GraphTooSmall(0));
        }

        #[test]
        fn rejects_order_one() {
            let err = assert_err!(CycleInstance::new(&graph_of(1, &[])));
            assert_eq!(err, ReductionError::GraphTooSmall(1));
        }

        #[test]
        fn accepts_order_two() {
            let instance = assert_ok!(CycleInstance::new(&graph_of(2, &[(0, 1)])));
            assert_eq!(instance.original_order, 2);
            // Two originals plus the apex, and the apex joined to both: the
            // reduced graph is a triangle, the shortest legal cycle.
            assert_eq!(instance.graph().node_count(), 3);
            assert_eq!(instance.graph().edge_count(), 3);
        }

        #[test]
        fn apex_is_index_n() {
            for order in 2..6 {
                let instance = CycleInstance::new(&graph_of(order, &[(0, 1)])).unwrap();
                assert_eq!(
                    instance.apex,
                    v(order),
                    "apex should be index n for order {order}"
                );
            }
        }

        /// The load-bearing one.  A SAT variable is derived from an edge index,
        /// so an original edge index must denote the same edge in G and in G′.
        #[test]
        fn preserves_original_edge_indices() {
            let graph = graph_of(5, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 0), (1, 3)]);
            let before: Vec<_> = graph
                .edge_indices()
                .map(|e| (e, graph.edge_endpoints(e).unwrap()))
                .collect();

            let instance = CycleInstance::new(&graph).unwrap();

            for (edge, endpoints) in before {
                assert_eq!(
                    instance.graph().edge_endpoints(edge),
                    Some(endpoints),
                    "edge index {} moved during reduction",
                    edge.index()
                );
            }
        }

        #[test]
        fn preserves_original_node_indices() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3)]);
            let instance = CycleInstance::new(&graph).unwrap();
            for i in 0..graph.node_count() {
                assert!(instance.graph().node_weight(v(i)).is_some());
            }
            assert_eq!(instance.graph().node_count(), graph.node_count() + 1);
        }

        #[test]
        fn adds_exactly_n_edges() {
            let graph = graph_of(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
            let m = graph.edge_count();
            let n = graph.node_count();
            let instance = CycleInstance::new(&graph).unwrap();
            assert_eq!(instance.graph().edge_count(), m + n);
        }

        #[test]
        fn apex_is_adjacent_to_every_original_vertex() {
            let graph = graph_of(4, &[(0, 1)]);
            let instance = CycleInstance::new(&graph).unwrap();
            for i in 0..4 {
                assert!(
                    instance.graph().find_edge(v(i), instance.apex).is_some(),
                    "apex not joined to vertex {i}"
                );
            }
            // And never to itself.
            assert!(instance
                .graph()
                .find_edge(instance.apex, instance.apex)
                .is_none());
        }

        /// The reduction does not care about connectivity; the apex makes G′
        /// connected regardless.  This is why the precondition check in phase 4
        /// must run on G′ rather than on G.
        #[test]
        fn disconnected_graph_still_reduces() {
            let graph = graph_of(4, &[(0, 1), (2, 3)]);
            let instance = assert_ok!(CycleInstance::new(&graph));
            assert_eq!(instance.graph().node_count(), 5);
            for i in 0..4 {
                assert!(instance.graph().find_edge(v(i), instance.apex).is_some());
            }
        }

        /// The test that proves the reduction does anything at all: the path
        /// graph has a Hamiltonian path but no Hamiltonian cycle, and G′ has a
        /// Hamiltonian cycle.
        #[test]
        fn path_graph_gains_a_hamiltonian_cycle() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3)]);

            // G has no Hamiltonian cycle: vertex 0 has degree 1, and a cycle
            // needs degree at least 2 everywhere.
            assert_eq!(graph.edges(v(0)).count(), 1);

            let instance = CycleInstance::new(&graph).unwrap();
            let apex = instance.apex;

            // G′ does: apex → 0 → 1 → 2 → 3 → apex.
            let cycle = Segment::new_closed(vec![apex, v(0), v(1), v(2), v(3)]).unwrap();
            assert!(
                cycle.is_hamiltonian_cycle(instance.graph()),
                "not a Hamiltonian cycle of the reduced graph: {cycle}"
            );
        }
    }

    /// Translating a cycle of G′ back into a path of G.
    mod path_from_cycle {
        use super::*;

        /// Both fixtures below are genuine Hamiltonian cycles of G′ for the
        /// path graph 0-1-2-3, differing only in where the apex sits in the
        /// sequence, so together they show the rotation is doing its job.
        fn instance() -> CycleInstance {
            CycleInstance::new(&graph_of(4, &[(0, 1), (1, 2), (2, 3)])).unwrap()
        }

        #[test]
        fn round_trips_with_apex_first() {
            let instance = instance();
            let apex = instance.apex;
            let cycle = Segment::new_closed(vec![apex, v(0), v(1), v(2), v(3)]).unwrap();

            let path = assert_ok!(instance.path_from_cycle(&cycle));
            assert!(!path.is_closed());
            assert_eq!(path.vertices(), &[v(0), v(1), v(2), v(3)]);
        }

        #[test]
        fn round_trips_with_apex_in_the_middle() {
            let instance = instance();
            let apex = instance.apex;
            // 1-0-apex-3-2-1 is the same cycle traversed from a different
            // start, so it must translate to the same canonical path.
            let cycle = Segment::new_closed(vec![v(1), v(0), apex, v(3), v(2)]).unwrap();
            assert!(
                cycle.is_hamiltonian_cycle(instance.graph()),
                "not a Hamiltonian cycle: {cycle}"
            );

            let path = assert_ok!(instance.path_from_cycle(&cycle));
            assert!(!path.is_closed());
            assert_eq!(path.vertices(), &[v(0), v(1), v(2), v(3)]);
        }

        #[test]
        fn result_is_a_real_path_of_the_original_graph() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3)]);
            let instance = CycleInstance::new(&graph).unwrap();
            let apex = instance.apex;
            let cycle = Segment::new_closed(vec![apex, v(0), v(1), v(2), v(3)]).unwrap();

            let path = instance.path_from_cycle(&cycle).unwrap();
            assert!(
                path.is_hamiltonian_path(&graph),
                "not a Hamiltonian path of the original graph: {path}"
            );
        }

        /// An open segment of the right length containing the apex would
        /// otherwise translate successfully and return a plausible-looking
        /// path, which is the failure this variant exists to prevent.
        #[test]
        fn rejects_an_open_segment() {
            let instance = instance();
            let apex = instance.apex;
            let open = Segment::new_open(vec![apex, v(0), v(1), v(2), v(3)]).unwrap();
            assert_eq!(open.len(), instance.original_order + 1);

            let err = assert_err!(instance.path_from_cycle(&open));
            assert_eq!(err, ReductionError::CycleNotClosed);
        }

        #[test]
        fn rejects_cycle_without_the_apex() {
            let instance = instance();
            let cycle = Segment::new_closed(vec![v(0), v(1), v(2)]).unwrap();
            let err = assert_err!(instance.path_from_cycle(&cycle));
            assert_eq!(err, ReductionError::ApexNotInCycle);
        }

        #[test]
        fn rejects_cycle_that_does_not_span() {
            let instance = instance();
            let apex = instance.apex;
            // Contains the apex, but only 3 of the 5 vertices of G′.
            let cycle = Segment::new_closed(vec![apex, v(0), v(1)]).unwrap();
            let err = assert_err!(instance.path_from_cycle(&cycle));
            assert_eq!(err, ReductionError::CycleNotSpanning);
        }
    }

    mod display {
        use super::*;

        #[test]
        fn reduction_error() {
            assert_eq!(
                ReductionError::GraphTooSmall(1).to_string(),
                "graph has 1 vertices, need at least 2 for the apex construction"
            );
            assert_eq!(
                ReductionError::CycleNotClosed.to_string(),
                "segment is open, so it is not a cycle"
            );
            assert_eq!(
                ReductionError::ApexNotInCycle.to_string(),
                "cycle does not pass through the apex vertex"
            );
            assert_eq!(
                ReductionError::CycleNotSpanning.to_string(),
                "cycle does not span the reduced graph"
            );
        }
    }
}

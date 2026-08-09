//! The two public answer types: [`HamiltonianCycle`] and [`HamiltonianPath`].
//!
//! Both are thin newtypes over [`crate::segment::Segment`], constructed only
//! by the search itself — there is no public constructor, so a value of
//! either type came out of a completed search rather than being assembled by
//! a caller.  Splitting the answer in two, rather than handing back
//! one `Segment`, means each type exposes only what makes sense for the
//! answer it holds: a cycle has no endpoints to report and a path has no
//! closedness flag to ask about.
//!
//! The internal sharing is unaffected by this split: `Segment` and
//! `Decomposition` remain the ordered-sequence view used throughout
//! refinement, and both `HamiltonianCycle` and `HamiltonianPath` convert into
//! a [`Decomposition`] so the renderers can treat a final answer and a
//! mid-refinement counterexample uniformly.

use petgraph::graph::{NodeIndex, UnGraph};
use std::fmt;

use crate::segment::{Decomposition, Segment};

/// A Hamiltonian cycle: a closed tour visiting every vertex of a graph
/// exactly once.
///
/// Produced only by [`crate::find_hamiltonian_cycle`]; there is no public
/// constructor, so a `HamiltonianCycle` always came out of a completed search
/// rather than being assembled by a caller.  Use
/// [`is_valid_for`](Self::is_valid_for) to re-check one against a graph.
///
/// Deliberately has no `len()`: `vertices().len()` already answers that, and
/// adding `len()` would trip the standard `clippy::len_without_is_empty`
/// lint, which fires on exported types and would want a companion
/// `is_empty()` that can only ever return `false`.  This is the ordinary Rust
/// lint, not a convention local to this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HamiltonianCycle {
    inner: Segment,
}

impl HamiltonianCycle {
    /// Wraps a closed segment as a Hamiltonian cycle.
    ///
    /// `debug_assert`s that `inner` really is closed; the search is this
    /// type's only caller, so a release build trusts it rather than paying
    /// for a check that can only fail if this crate's own invariants are
    /// already broken.
    pub(crate) fn new(inner: Segment) -> Self {
        debug_assert!(
            inner.is_closed(),
            "HamiltonianCycle::new given an open segment"
        );
        HamiltonianCycle { inner }
    }

    /// The vertices in traversal order.
    pub fn vertices(&self) -> &[NodeIndex] {
        self.inner.vertices()
    }

    /// The edges of the cycle, as consecutive vertex pairs, including the
    /// wrap-around edge from the last vertex back to the first — `n` edges
    /// for `n` vertices.
    pub fn edges(&self) -> Vec<(NodeIndex, NodeIndex)> {
        self.inner.edges()
    }

    /// Whether this is a genuine Hamiltonian cycle of `graph`: every vertex
    /// of `graph` visited exactly once, by real edges of `graph`.
    ///
    /// Safe to call with a cycle built for a different or incompatible
    /// graph: this cannot panic on an out-of-range or foreign `NodeIndex`, it
    /// simply returns `false`.
    pub fn is_valid_for(&self, graph: &UnGraph<(), ()>) -> bool {
        self.inner.is_hamiltonian_cycle(graph)
    }
}

impl fmt::Display for HamiltonianCycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

/// A Hamiltonian path: an open tour visiting every vertex of a graph exactly
/// once.
///
/// Produced only by [`crate::find_hamiltonian_path`]; there is no public
/// constructor, so a `HamiltonianPath` always came out of a completed search
/// rather than being assembled by a caller.  Use
/// [`is_valid_for`](Self::is_valid_for) to re-check one against a graph.
///
/// Deliberately has no `len()`, for the same reason as [`HamiltonianCycle`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HamiltonianPath {
    inner: Segment,
}

impl HamiltonianPath {
    /// Wraps an open segment as a Hamiltonian path.
    ///
    /// `debug_assert`s that `inner` really is open; see
    /// [`HamiltonianCycle::new`] for why a release build does not repeat the
    /// check.
    pub(crate) fn new(inner: Segment) -> Self {
        debug_assert!(
            !inner.is_closed(),
            "HamiltonianPath::new given a closed segment"
        );
        HamiltonianPath { inner }
    }

    /// The vertices in traversal order.
    pub fn vertices(&self) -> &[NodeIndex] {
        self.inner.vertices()
    }

    /// The edges of the path, as consecutive vertex pairs — `n - 1` edges for
    /// `n` vertices.
    pub fn edges(&self) -> Vec<(NodeIndex, NodeIndex)> {
        self.inner.edges()
    }

    /// The two ends of the path.
    ///
    /// A single-vertex path reports that vertex as both ends, matching
    /// `Segment::endpoints`.  Unlike `Segment::endpoints`, this never
    /// returns `None`: a `HamiltonianPath` is always open by construction.
    pub fn endpoints(&self) -> (NodeIndex, NodeIndex) {
        self.inner
            .endpoints()
            .expect("a HamiltonianPath's inner segment is always open")
    }

    /// Whether this is a genuine Hamiltonian path of `graph`: every vertex of
    /// `graph` visited exactly once, by real edges of `graph`.
    ///
    /// Safe to call with a path built for a different or incompatible graph:
    /// this cannot panic on an out-of-range or foreign `NodeIndex`, it simply
    /// returns `false`.
    pub fn is_valid_for(&self, graph: &UnGraph<(), ()>) -> bool {
        self.inner.is_hamiltonian_path(graph)
    }
}

impl fmt::Display for HamiltonianPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

/// Builds the degenerate one-segment decomposition.
///
/// Sound, not optimistic: `Decomposition::new` fails only with
/// `OverlappingSegments`, which needs two segments to share a vertex, and
/// there is exactly one segment here.
fn single_segment_decomposition(segment: Segment) -> Decomposition {
    Decomposition::new(vec![segment]).expect("a single segment cannot overlap itself")
}

impl From<HamiltonianCycle> for Decomposition {
    fn from(cycle: HamiltonianCycle) -> Self {
        single_segment_decomposition(cycle.inner)
    }
}

impl From<&HamiltonianCycle> for Decomposition {
    fn from(cycle: &HamiltonianCycle) -> Self {
        single_segment_decomposition(cycle.inner.clone())
    }
}

impl From<HamiltonianPath> for Decomposition {
    fn from(path: HamiltonianPath) -> Self {
        single_segment_decomposition(path.inner)
    }
}

impl From<&HamiltonianPath> for Decomposition {
    fn from(path: &HamiltonianPath) -> Self {
        single_segment_decomposition(path.inner.clone())
    }
}

/// A clone, not a rebuild through `Decomposition::new`.  This does **not**
/// conflict with std's reflexive `impl<T> From<T> for T`: that blanket impl
/// yields `From<&Decomposition> for &Decomposition`, a different `Self` than
/// the one implemented here.  The blanket impl is also what makes
/// `render::dot(&graph, decomposition)` (by value) work for free, so only the
/// by-reference case needs writing.
impl From<&Decomposition> for Decomposition {
    fn from(decomposition: &Decomposition) -> Self {
        decomposition.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{graph_of, v};

    fn vs(indices: &[usize]) -> Vec<NodeIndex> {
        indices.iter().copied().map(v).collect()
    }

    mod cycle {
        use super::*;

        #[test]
        fn vertices_returns_traversal_order() {
            let segment = Segment::new_closed(vs(&[0, 3, 5])).unwrap();
            let cycle = HamiltonianCycle::new(segment);
            assert_eq!(cycle.vertices(), &vs(&[0, 3, 5]));
        }

        #[test]
        fn edges_includes_wrap_around_edge() {
            let segment = Segment::new_closed(vs(&[0, 3, 5])).unwrap();
            let cycle = HamiltonianCycle::new(segment);
            assert_eq!(
                cycle.edges(),
                vec![(v(0), v(3)), (v(3), v(5)), (v(5), v(0))]
            );
        }

        #[test]
        fn is_valid_for_true_on_real_cycle() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
            let segment = Segment::new_closed(vs(&[0, 1, 2, 3])).unwrap();
            let cycle = HamiltonianCycle::new(segment);
            assert!(cycle.is_valid_for(&graph));
        }

        /// Vertex indices out of range for `graph` must report `false`, not
        /// panic, mirroring `Segment::is_hamiltonian_cycle`'s guarantee.
        #[test]
        fn is_valid_for_false_on_incompatible_graph() {
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);
            let segment = Segment::new_closed(vs(&[10, 11, 12])).unwrap();
            let cycle = HamiltonianCycle::new(segment);
            assert!(!cycle.is_valid_for(&graph));
        }

        #[test]
        fn display_matches_arrow_form() {
            let segment = Segment::new_closed(vs(&[0, 3, 5])).unwrap();
            let cycle = HamiltonianCycle::new(segment);
            assert_eq!(cycle.to_string(), "0 \u{2192} 3 \u{2192} 5 \u{2192} 0");
        }
    }

    mod path {
        use super::*;

        #[test]
        fn vertices_returns_traversal_order() {
            let segment = Segment::new_open(vs(&[0, 3, 5])).unwrap();
            let path = HamiltonianPath::new(segment);
            assert_eq!(path.vertices(), &vs(&[0, 3, 5]));
        }

        #[test]
        fn edges_excludes_wrap_around() {
            let segment = Segment::new_open(vs(&[0, 3, 5])).unwrap();
            let path = HamiltonianPath::new(segment);
            assert_eq!(path.edges(), vec![(v(0), v(3)), (v(3), v(5))]);
        }

        #[test]
        fn endpoints_multi_vertex_reports_first_and_last() {
            let segment = Segment::new_open(vs(&[0, 3, 5])).unwrap();
            let path = HamiltonianPath::new(segment);
            assert_eq!(path.endpoints(), (v(0), v(5)));
        }

        #[test]
        fn endpoints_single_vertex_reports_it_twice() {
            let segment = Segment::new_open(vs(&[7])).unwrap();
            let path = HamiltonianPath::new(segment);
            assert_eq!(path.endpoints(), (v(7), v(7)));
        }

        #[test]
        fn is_valid_for_true_on_real_path() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3)]);
            let segment = Segment::new_open(vs(&[0, 1, 2, 3])).unwrap();
            let path = HamiltonianPath::new(segment);
            assert!(path.is_valid_for(&graph));
        }

        /// Vertex indices out of range for `graph` must report `false`, not
        /// panic, mirroring `Segment::is_hamiltonian_path`'s guarantee.
        #[test]
        fn is_valid_for_false_on_incompatible_graph() {
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);
            let segment = Segment::new_open(vs(&[10, 11, 12])).unwrap();
            let path = HamiltonianPath::new(segment);
            assert!(!path.is_valid_for(&graph));
        }

        #[test]
        fn display_matches_arrow_form() {
            let segment = Segment::new_open(vs(&[0, 3, 5])).unwrap();
            let path = HamiltonianPath::new(segment);
            assert_eq!(path.to_string(), "0 \u{2192} 3 \u{2192} 5");
        }
    }

    /// The five `From` impls that let a caller hand a final answer straight
    /// to a renderer.
    mod conversions {
        use super::*;
        use claim::assert_ok;

        #[test]
        fn cycle_by_value_produces_single_segment_decomposition() {
            let segment = Segment::new_closed(vs(&[0, 1, 2])).unwrap();
            let cycle = HamiltonianCycle::new(segment);
            let decomposition: Decomposition = cycle.into();
            assert_eq!(decomposition.covered_vertices(), 3);
        }

        #[test]
        fn cycle_by_reference_produces_single_segment_decomposition() {
            let segment = Segment::new_closed(vs(&[0, 1, 2])).unwrap();
            let cycle = HamiltonianCycle::new(segment);
            let decomposition: Decomposition = (&cycle).into();
            assert_eq!(decomposition.covered_vertices(), 3);
        }

        #[test]
        fn path_by_value_produces_single_segment_decomposition() {
            let segment = Segment::new_open(vs(&[0, 1, 2])).unwrap();
            let path = HamiltonianPath::new(segment);
            let decomposition: Decomposition = path.into();
            assert_eq!(decomposition.covered_vertices(), 3);
        }

        #[test]
        fn path_by_reference_produces_single_segment_decomposition() {
            let segment = Segment::new_open(vs(&[0, 1, 2])).unwrap();
            let path = HamiltonianPath::new(segment);
            let decomposition: Decomposition = (&path).into();
            assert_eq!(decomposition.covered_vertices(), 3);
        }

        #[test]
        fn decomposition_by_reference_clones() {
            let segment = Segment::new_closed(vs(&[0, 1, 2])).unwrap();
            let original = assert_ok!(Decomposition::new(vec![segment]));
            let cloned: Decomposition = (&original).into();
            assert_eq!(cloned, original);
        }
    }
}

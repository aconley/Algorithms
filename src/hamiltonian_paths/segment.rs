//! Ordered vertex sequences: the representation shared by solutions and by the
//! spurious models produced during refinement.
//!
//! A [`Segment`] is an ordered run of distinct vertices, either open (a path)
//! or closed (a cycle).  A [`Decomposition`] is a set of vertex-disjoint
//! segments — what a model of the abstraction decodes to.
//!
//! A genuine Hamiltonian cycle is the degenerate decomposition: a single closed
//! segment covering every vertex.  Using one type for both means the renderers
//! and the validation logic serve final answers and intermediate counterexamples
//! alike.
//!
//! These types are the *ordered-sequence view* of a cycle cover, not the CEGAR
//! loop's working data structure — that is `CycleCover`, which holds Knuth's
//! `SUCC`/`PRED`/`CID` arrays and is what refinement actually consumes.  A
//! `Decomposition` is derived from it on request; see
//! `.agents/overview.md`, "What `Decomposition` is *not*".

use petgraph::graph::NodeIndex;
use std::collections::HashSet;
use std::fmt;

/// Failure to construct a well-formed [`Segment`] or [`Decomposition`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SegmentError {
    /// A segment was constructed with no vertices.
    Empty,
    /// A vertex appeared more than once within a single segment.
    RepeatedVertex(NodeIndex),
    /// A closed segment had fewer than three vertices, so it cannot be a cycle
    /// in a simple graph.
    ClosedTooShort(usize),
    /// Two segments of a decomposition shared a vertex.
    OverlappingSegments(NodeIndex),
}

impl fmt::Display for SegmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SegmentError::Empty => write!(f, "segment has no vertices"),
            SegmentError::RepeatedVertex(v) => {
                write!(
                    f,
                    "vertex {} appears more than once in a segment",
                    v.index()
                )
            }
            SegmentError::ClosedTooShort(n) => {
                write!(f, "closed segment has only {n} vertices, need at least 3")
            }
            SegmentError::OverlappingSegments(v) => {
                write!(f, "vertex {} appears in more than one segment", v.index())
            }
        }
    }
}

impl std::error::Error for SegmentError {}

/// An ordered run of distinct vertices, either open (a path) or closed (a cycle).
///
/// The order is the content: it is what distinguishes a solution from the mere
/// set of selected edges, and it is what every renderer needs in order to draw
/// visit numbers, traversal direction, or endpoint markers.  The edge set is
/// recoverable from the order in linear time (see [`Segment::edges`]); the
/// order is not cheaply recoverable from the edge set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    vertices: Vec<NodeIndex>,
    closed: bool,
}

impl Segment {
    /// Builds a segment from a vertex sequence.
    ///
    /// Rejects an empty sequence, a repeated vertex, and a closed segment of
    /// length less than three.  For a closed segment the wrap-around edge from
    /// the last vertex back to the first is implied and must *not* be encoded by
    /// repeating the first vertex at the end.
    pub fn new(vertices: Vec<NodeIndex>, closed: bool) -> Result<Self, SegmentError> {
        if vertices.is_empty() {
            return Err(SegmentError::Empty);
        }
        let mut seen = HashSet::with_capacity(vertices.len());
        for &v in &vertices {
            if !seen.insert(v) {
                return Err(SegmentError::RepeatedVertex(v));
            }
        }
        if closed && vertices.len() < 3 {
            return Err(SegmentError::ClosedTooShort(vertices.len()));
        }
        Ok(Segment { vertices, closed })
    }

    /// The vertices in traversal order.
    pub fn vertices(&self) -> &[NodeIndex] {
        &self.vertices
    }

    /// Number of vertices, which for a closed segment is also its number of edges.
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Whether this segment is a cycle rather than a path.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// The two ends of an open segment, or `None` if it is closed.
    ///
    /// A single-vertex open segment reports that vertex as both ends.
    pub fn endpoints(&self) -> Option<(NodeIndex, NodeIndex)> {
        if self.closed {
            None
        } else {
            // Construction guarantees `vertices` is non-empty.
            Some((self.vertices[0], *self.vertices.last().unwrap()))
        }
    }

    /// The edges traversed, as consecutive vertex pairs, wrapping from last back
    /// to first when the segment is closed.
    ///
    /// Returned as a `Vec` rather than an iterator purely to keep this skeleton
    /// simple; it may become a borrowing iterator once implemented.
    pub fn edges(&self) -> Vec<(NodeIndex, NodeIndex)> {
        let mut edges: Vec<(NodeIndex, NodeIndex)> =
            self.vertices.windows(2).map(|w| (w[0], w[1])).collect();
        if self.closed {
            // Construction guarantees at least 3 vertices when closed.
            edges.push((*self.vertices.last().unwrap(), self.vertices[0]));
        }
        edges
    }

    /// Rewrites this segment into its canonical orientation, so that two
    /// representations of the same undirected path or cycle compare equal.
    ///
    /// On an undirected graph a segment and its reversal denote the same object,
    /// and a closed segment additionally has no distinguished starting point.
    /// The canonical form is:
    ///
    /// - **open**: oriented so the lower-indexed endpoint comes first;
    /// - **closed**: rotated so the lowest-indexed vertex comes first, then
    ///   oriented so that the lower-indexed of its two neighbours comes second.
    ///
    /// Canonicalising is what makes solutions comparable, deduplicable, and
    /// usable in test assertions.
    pub fn canonicalize(&mut self) {
        if self.closed {
            // Rotate so the lowest-indexed vertex comes first.
            let min_pos = self
                .vertices
                .iter()
                .enumerate()
                .min_by_key(|(_, v)| v.index())
                .map(|(i, _)| i)
                .unwrap();
            self.vertices.rotate_left(min_pos);

            // The minimum's two neighbours are now at position 1 and at the
            // end (the wrap-around edge).  Orient so the lower-indexed of the
            // two comes second.
            let second = self.vertices[1];
            let last = *self.vertices.last().unwrap();
            if last.index() < second.index() {
                self.vertices[1..].reverse();
            }
        } else if let (Some(&first), Some(&last)) =
            (self.vertices.first(), self.vertices.last())
        {
            if last.index() < first.index() {
                self.vertices.reverse();
            }
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, v) in self.vertices.iter().enumerate() {
            if i > 0 {
                write!(f, " \u{2192} ")?;
            }
            write!(f, "{}", v.index())?;
        }
        if self.closed {
            write!(f, " \u{2192} {}", self.vertices[0].index())?;
        }
        Ok(())
    }
}

/// A set of vertex-disjoint segments: what a model of the abstraction decodes to.
///
/// During refinement this holds the spurious structure — typically several
/// disjoint cycles where a single covering cycle was wanted.  Each segment that
/// falls short is what a refinement clause is built from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decomposition {
    segments: Vec<Segment>,
}

impl Decomposition {
    /// Builds a decomposition, rejecting segments that share a vertex.
    pub fn new(mut segments: Vec<Segment>) -> Result<Self, SegmentError> {
        let mut seen = HashSet::new();
        for segment in &segments {
            for &v in segment.vertices() {
                if !seen.insert(v) {
                    return Err(SegmentError::OverlappingSegments(v));
                }
            }
        }
        for segment in &mut segments {
            segment.canonicalize();
        }
        segments.sort_by_key(|segment| segment.vertices()[0]);
        Ok(Decomposition { segments })
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    /// Number of segments.
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Total number of vertices covered across all segments.
    pub fn covered_vertices(&self) -> usize {
        self.segments.iter().map(Segment::len).sum()
    }

    /// The single closed segment covering all `order` vertices, if this
    /// decomposition is one.
    ///
    /// This is the abstraction check: `Some` means the model is a genuine
    /// Hamiltonian cycle and the search is done; `None` means the model is
    /// spurious and must be refined away.
    pub fn as_hamiltonian_cycle(&self, order: usize) -> Option<&Segment> {
        match self.segments.as_slice() {
            [segment] if segment.is_closed() && segment.len() == order => Some(segment),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    fn v(i: u32) -> NodeIndex {
        NodeIndex::new(i as usize)
    }

    fn vs(indices: &[u32]) -> Vec<NodeIndex> {
        indices.iter().copied().map(v).collect()
    }

    /// Construction and the two derived views, `endpoints` and `edges`.
    mod segment {
        use super::*;

        #[test]
        fn new_rejects_empty() {
            let err = assert_err!(Segment::new(vec![], false));
            assert_eq!(err, SegmentError::Empty);

            let err = assert_err!(Segment::new(vec![], true));
            assert_eq!(err, SegmentError::Empty);
        }

        #[test]
        fn new_rejects_repeated_vertex_open() {
            let err = assert_err!(Segment::new(vs(&[0, 1, 0]), false));
            assert_eq!(err, SegmentError::RepeatedVertex(v(0)));
        }

        #[test]
        fn new_rejects_repeated_vertex_closed() {
            let err = assert_err!(Segment::new(vs(&[0, 1, 2, 1]), true));
            assert_eq!(err, SegmentError::RepeatedVertex(v(1)));
        }

        #[test]
        fn new_rejects_closed_too_short() {
            let err = assert_err!(Segment::new(vs(&[0]), true));
            assert_eq!(err, SegmentError::ClosedTooShort(1));

            let err = assert_err!(Segment::new(vs(&[0, 1]), true));
            assert_eq!(err, SegmentError::ClosedTooShort(2));
        }

        #[test]
        fn new_accepts_minimal_closed_triangle() {
            let seg = assert_ok!(Segment::new(vs(&[0, 1, 2]), true));
            assert!(seg.is_closed());
            assert_eq!(seg.len(), 3);
        }

        #[test]
        fn new_accepts_single_vertex_open() {
            let seg = assert_ok!(Segment::new(vs(&[0]), false));
            assert!(!seg.is_closed());
            assert_eq!(seg.len(), 1);
        }

        #[test]
        fn endpoints_open_reports_first_and_last() {
            let seg = Segment::new(vs(&[0, 3, 5]), false).unwrap();
            assert_eq!(seg.endpoints(), Some((v(0), v(5))));
        }

        #[test]
        fn endpoints_single_vertex_open_reports_it_twice() {
            let seg = Segment::new(vs(&[7]), false).unwrap();
            assert_eq!(seg.endpoints(), Some((v(7), v(7))));
        }

        #[test]
        fn endpoints_closed_is_none() {
            let seg = Segment::new(vs(&[0, 1, 2]), true).unwrap();
            assert_eq!(seg.endpoints(), None);
        }

        #[test]
        fn edges_open_are_consecutive_pairs() {
            let seg = Segment::new(vs(&[0, 3, 5]), false).unwrap();
            assert_eq!(seg.edges(), vec![(v(0), v(3)), (v(3), v(5))]);
        }

        #[test]
        fn edges_closed_include_wrap_around() {
            let seg = Segment::new(vs(&[0, 3, 5]), true).unwrap();
            assert_eq!(seg.edges(), vec![(v(0), v(3)), (v(3), v(5)), (v(5), v(0))]);
        }
    }

    /// The canonical-orientation rule.  Every later phase asserts against
    /// canonicalized segments, so this group carries more weight than its size
    /// suggests.
    mod canonicalize {
        use super::*;

        #[test]
        fn open_reorients_toward_lower_endpoint() {
            let mut seg = Segment::new(vs(&[5, 3, 0]), false).unwrap();
            seg.canonicalize();
            assert_eq!(seg.vertices(), &vs(&[0, 3, 5]));
        }

        #[test]
        fn open_leaves_already_oriented_segment() {
            let mut seg = Segment::new(vs(&[0, 3, 5]), false).unwrap();
            seg.canonicalize();
            assert_eq!(seg.vertices(), &vs(&[0, 3, 5]));
        }

        #[test]
        fn closed_rotates_and_orients() {
            let mut seg = Segment::new(vs(&[3, 5, 0]), true).unwrap();
            seg.canonicalize();
            assert_eq!(seg.vertices(), &vs(&[0, 3, 5]));
        }

        #[test]
        fn maps_every_rotation_to_same_form() {
            let base = vs(&[0, 3, 5, 9]);
            let expected = {
                let mut seg = Segment::new(base.clone(), true).unwrap();
                seg.canonicalize();
                seg
            };

            for start in 0..base.len() {
                let mut rotated = base.clone();
                rotated.rotate_left(start);
                let mut seg = Segment::new(rotated, true).unwrap();
                seg.canonicalize();
                assert_eq!(
                    seg, expected,
                    "rotation starting at {start} did not canonicalize the same"
                );
            }
        }

        #[test]
        fn maps_reversal_to_same_form() {
            let mut forward = Segment::new(vs(&[0, 3, 5, 9]), true).unwrap();
            forward.canonicalize();

            let mut reversed_vertices = vs(&[0, 3, 5, 9]);
            reversed_vertices.reverse();
            let mut backward = Segment::new(reversed_vertices, true).unwrap();
            backward.canonicalize();

            assert_eq!(forward, backward);
        }

        /// The two tests above cover rotation and reversal separately.  Every later
        /// phase asserts against canonicalized segments, so the property that
        /// actually matters is that *all* 2n representations of one cycle — both
        /// orientations, every rotation — collapse to a single form.
        #[test]
        fn maps_whole_dihedral_orbit_to_same_form() {
            let base = vs(&[4, 1, 7, 2, 6]);
            let expected = {
                let mut seg = Segment::new(base.clone(), true).unwrap();
                seg.canonicalize();
                seg
            };

            for reversed in [false, true] {
                let mut oriented = base.clone();
                if reversed {
                    oriented.reverse();
                }
                for start in 0..oriented.len() {
                    let mut rotated = oriented.clone();
                    rotated.rotate_left(start);
                    let mut seg = Segment::new(rotated, true).unwrap();
                    seg.canonicalize();
                    assert_eq!(
                        seg, expected,
                        "reversed={reversed}, rotation={start} did not canonicalize the same"
                    );
                }
            }
        }

        #[test]
        fn is_idempotent() {
            for verts in [vs(&[3, 5, 0, 9]), vs(&[9, 0, 5, 3])] {
                let mut seg = Segment::new(verts, true).unwrap();
                seg.canonicalize();
                let once = seg.clone();
                seg.canonicalize();
                assert_eq!(seg, once);
            }

            let mut open = Segment::new(vs(&[5, 0, 3]), false).unwrap();
            open.canonicalize();
            let once = open.clone();
            open.canonicalize();
            assert_eq!(open, once);
        }
    }

    /// `Display` for [`Segment`] and [`SegmentError`].
    mod display {
        use super::*;

        #[test]
        fn open_segment() {
            let seg = Segment::new(vs(&[0, 3, 5]), false).unwrap();
            assert_eq!(seg.to_string(), "0 \u{2192} 3 \u{2192} 5");
        }

        #[test]
        fn closed_segment() {
            let seg = Segment::new(vs(&[0, 3, 5]), true).unwrap();
            assert_eq!(seg.to_string(), "0 \u{2192} 3 \u{2192} 5 \u{2192} 0");
        }

        #[test]
        fn segment_error() {
            assert_eq!(
                SegmentError::RepeatedVertex(v(2)).to_string(),
                "vertex 2 appears more than once in a segment"
            );
            assert_eq!(
                SegmentError::ClosedTooShort(2).to_string(),
                "closed segment has only 2 vertices, need at least 3"
            );
        }
    }

    /// Disjointness, the normal form, and `as_hamiltonian_cycle` — which is the
    /// abstraction check the CEGAR loop is built around.
    mod decomposition {
        use super::*;

        #[test]
        fn new_rejects_overlapping_segments() {
            let a = Segment::new(vs(&[0, 1, 2]), true).unwrap();
            let b = Segment::new(vs(&[2, 3, 4]), true).unwrap();
            let err = assert_err!(Decomposition::new(vec![a, b]));
            assert_eq!(err, SegmentError::OverlappingSegments(v(2)));
        }

        #[test]
        fn new_rejects_overlap_between_open_segments() {
            // A single segment cannot itself contain a repeat; verify decomposition
            // catches overlap across otherwise-valid segments sharing a vertex.
            let a = Segment::new(vs(&[0, 1]), false).unwrap();
            let b = Segment::new(vs(&[1, 2]), false).unwrap();
            let err = assert_err!(Decomposition::new(vec![a, b]));
            assert_eq!(err, SegmentError::OverlappingSegments(v(1)));
        }

        #[test]
        fn new_canonicalizes_and_sorts_segments() {
            // Second segment's first vertex (5) is lower than the first segment's
            // once canonicalized (6), and each segment is given in a non-canonical
            // orientation, so `new` must fix both before sorting.
            let a = Segment::new(vs(&[8, 6]), false).unwrap(); // canonicalizes to 6 -> 8
            let b = Segment::new(vs(&[7, 5]), false).unwrap(); // canonicalizes to 5 -> 7
            let decomposition = assert_ok!(Decomposition::new(vec![a, b]));

            assert_eq!(decomposition.len(), 2);
            assert_eq!(decomposition.segments()[0].vertices(), &vs(&[5, 7]));
            assert_eq!(decomposition.segments()[1].vertices(), &vs(&[6, 8]));
        }

        #[test]
        fn covered_vertices_sums_segment_lengths() {
            let a = Segment::new(vs(&[0, 1, 2]), true).unwrap();
            let b = Segment::new(vs(&[3, 4]), false).unwrap();
            let decomposition = Decomposition::new(vec![a, b]).unwrap();
            assert_eq!(decomposition.covered_vertices(), 5);
        }

        #[test]
        fn as_hamiltonian_cycle_accepts_single_closed_covering_segment() {
            let seg = Segment::new(vs(&[0, 1, 2, 3]), true).unwrap();
            let decomposition = Decomposition::new(vec![seg]).unwrap();
            let cycle = decomposition.as_hamiltonian_cycle(4);
            assert!(cycle.is_some());
            assert_eq!(cycle.unwrap().len(), 4);
        }

        #[test]
        fn as_hamiltonian_cycle_rejects_single_open_segment() {
            let seg = Segment::new(vs(&[0, 1, 2, 3]), false).unwrap();
            let decomposition = Decomposition::new(vec![seg]).unwrap();
            assert_eq!(decomposition.as_hamiltonian_cycle(4), None);
        }

        #[test]
        fn as_hamiltonian_cycle_rejects_two_segments() {
            let a = Segment::new(vs(&[0, 1, 2]), true).unwrap();
            let b = Segment::new(vs(&[3, 4, 5]), true).unwrap();
            let decomposition = Decomposition::new(vec![a, b]).unwrap();
            assert_eq!(decomposition.as_hamiltonian_cycle(6), None);
        }

        #[test]
        fn as_hamiltonian_cycle_rejects_covers_too_few() {
            let seg = Segment::new(vs(&[0, 1, 2]), true).unwrap();
            let decomposition = Decomposition::new(vec![seg]).unwrap();
            assert_eq!(decomposition.as_hamiltonian_cycle(4), None);
        }
    }
}

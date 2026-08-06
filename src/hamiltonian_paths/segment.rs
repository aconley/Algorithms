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
//! alike, and cannot drift apart from what the refinement step consumes.

use petgraph::graph::NodeIndex;
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
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("SegmentError display")
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
    pub fn new(_vertices: Vec<NodeIndex>, _closed: bool) -> Result<Self, SegmentError> {
        todo!("validate and construct")
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
        todo!("first and last, when open")
    }

    /// The edges traversed, as consecutive vertex pairs, wrapping from last back
    /// to first when the segment is closed.
    ///
    /// Returned as a `Vec` rather than an iterator purely to keep this skeleton
    /// simple; it may become a borrowing iterator once implemented.
    pub fn edges(&self) -> Vec<(NodeIndex, NodeIndex)> {
        todo!("consecutive pairs, plus the wrap-around edge when closed")
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
        todo!("canonical orientation")
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("segment display")
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
    pub fn new(_segments: Vec<Segment>) -> Result<Self, SegmentError> {
        todo!("validate disjointness and construct")
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
        todo!("sum of segment lengths")
    }

    /// The single closed segment covering all `order` vertices, if this
    /// decomposition is one.
    ///
    /// This is the abstraction check: `Some` means the model is a genuine
    /// Hamiltonian cycle and the search is done; `None` means the model is
    /// spurious and must be refined away.
    pub fn as_hamiltonian_cycle(&self, _order: usize) -> Option<&Segment> {
        todo!("exactly one segment, closed, covering every vertex")
    }
}

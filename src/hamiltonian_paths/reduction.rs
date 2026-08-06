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
    /// A cycle handed back for translation did not pass through the apex, so it
    /// did not come from this instance.
    ApexNotInCycle,
    /// A cycle handed back for translation did not span the reduced graph.
    CycleNotSpanning,
}

impl fmt::Display for ReductionError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("ReductionError display")
    }
}

impl std::error::Error for ReductionError {}

/// A Hamiltonian path query rewritten as a Hamiltonian cycle query.
///
/// Owns the reduced graph *G′* and remembers where the apex went, so that a
/// cycle found in *G′* can be translated back into a path in *G*.
#[derive(Debug, Clone)]
pub struct CycleInstance {
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
    pub fn new(_graph: &UnGraph<(), ()>) -> Result<Self, ReductionError> {
        todo!("add apex last, join to every vertex")
    }

    /// The reduced graph, the one actually handed to the CEGAR engine.
    pub fn graph(&self) -> &UnGraph<(), ()> {
        &self.graph
    }

    /// The apex vertex of the reduced graph.
    pub fn apex(&self) -> NodeIndex {
        self.apex
    }

    /// Number of vertices in the original graph.
    pub fn original_order(&self) -> usize {
        self.original_order
    }

    /// Translates a Hamiltonian cycle of *G′* into the corresponding
    /// Hamiltonian path of *G*.
    ///
    /// Rotate the cycle so the apex comes first, drop it, and what remains is an
    /// open segment over the original vertices in their original indices.  The
    /// result is returned canonicalised.
    pub fn path_from_cycle(&self, _cycle: &Segment) -> Result<Segment, ReductionError> {
        todo!("rotate to apex, drop it, reopen as a path")
    }
}

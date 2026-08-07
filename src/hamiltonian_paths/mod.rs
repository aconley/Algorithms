//! Hamiltonian cycles and paths, found by CEGAR (counterexample-guided
//! abstraction refinement) over an incremental SAT solver.
//!
//! The whole public surface is the two functions below.  The engine searches
//! for Hamiltonian **cycles**; paths are answered by reducing them to a cycle
//! query on a graph with one extra vertex (see the private `reduction` module).
//!
//! Both functions distinguish three outcomes:
//!
//! - `Ok(Some(_))` — a witness.
//! - `Ok(None)` — a *proof* that none exists.  The abstraction went
//!   unsatisfiable, which is conclusive; this is an ordinary answer and not an
//!   error condition.
//! - `Err(_)` — no answer was obtained: a solver failure, or a resource limit.
//!
//! Keeping "gave up" in [`Error`] rather than folding it into `Ok(None)` is what
//! lets `Ok(None)` mean something strong.
//!
//! See `AGENTS.md` in this directory for the conventions this code is written
//! to; `.agents/overview.md` for the design decisions behind the
//! representations used here, and for the constraints that must not be
//! relitigated; `.agents/algorithm.md` for Knuth's Algorithm C; and
//! `.agents/plan.md` for the phase-by-phase work order.

// TODO: remove once the module is implemented; the skeleton's unused items would
// otherwise bury real warnings.
#![allow(dead_code)]

mod cycles;
mod driver;
mod encoding;
mod precheck;
mod reduction;
mod segment;
#[cfg(test)]
mod testing;

pub use cycles::CoverError;
pub use reduction::ReductionError;
pub use segment::{Segment, SegmentError};

use petgraph::graph::UnGraph;
use std::fmt;

/// Something went wrong, or the search gave up.
///
/// Note what is *not* here: the absence of a Hamiltonian cycle or path is
/// reported as `Ok(None)`, never as an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The SAT solver reported a failure.
    Solver(String),
    /// A configured round or conflict limit was reached, so the question is
    /// unsettled — neither a witness nor a proof of nonexistence.
    LimitExceeded,
    /// The path-to-cycle reduction rejected the query.
    Reduction(ReductionError),
    /// A model decoded to something that is not a valid vertex sequence, which
    /// means the encoding and the decoder disagree.
    Malformed(SegmentError),
    /// A model decoded to something that is not a cycle cover, which likewise
    /// means the encoding and the decoder disagree.
    MalformedCover(CoverError),
}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Error display")
    }
}

impl std::error::Error for Error {}

impl From<ReductionError> for Error {
    fn from(err: ReductionError) -> Self {
        Error::Reduction(err)
    }
}

impl From<SegmentError> for Error {
    fn from(err: SegmentError) -> Self {
        Error::Malformed(err)
    }
}

impl From<CoverError> for Error {
    fn from(err: CoverError) -> Self {
        Error::MalformedCover(err)
    }
}

/// Searches for a Hamiltonian cycle.
///
/// This is the engine's native question; [`find_hamiltonian_path`] is a wrapper
/// over it.  It is public in its own right because some instances are genuinely
/// cycle problems — a *closed* knight's tour, for one.
///
/// Returns the cycle as a closed [`Segment`] in canonical orientation, or
/// `Ok(None)` if the graph has no Hamiltonian cycle.
pub fn find_hamiltonian_cycle(
    _graph: &UnGraph<(), ()>,
) -> Result<Option<Segment>, Error> {
    todo!("run a CegarSearch to completion")
}

/// Searches for a Hamiltonian path, with both endpoints unconstrained.
///
/// Internally this adds an apex vertex joined to every vertex of `graph`,
/// searches for a Hamiltonian cycle in that larger graph, and deletes the apex
/// from the cycle it finds.  The returned segment is open and indexes the
/// vertices of `graph`, not of the reduced graph.
///
/// There is deliberately no way to ask for a path with particular endpoints —
/// "any path" is the question CEGAR answers.  See the `reduction` module.
///
/// Returns `Ok(None)` if no Hamiltonian path exists.
pub fn find_hamiltonian_path(_graph: &UnGraph<(), ()>) -> Result<Option<Segment>, Error> {
    todo!("reduce, search for a cycle, translate back")
}

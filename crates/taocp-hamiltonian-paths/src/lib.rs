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

mod cycles;
mod driver;
mod encoding;
pub mod generators;
mod precheck;
mod reduction;
mod refinement;
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Solver(msg) => write!(f, "solver error: {}", msg),
            Error::LimitExceeded => write!(f, "resource limit exceeded"),
            Error::Reduction(err) => write!(f, "reduction error: {}", err),
            Error::Malformed(err) => write!(f, "malformed segment: {}", err),
            Error::MalformedCover(err) => {
                write!(f, "malformed cycle cover: {}", err)
            }
        }
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
pub fn find_hamiltonian_cycle(graph: &UnGraph<(), ()>) -> Result<Option<Segment>, Error> {
    // No simple graph on fewer than 3 vertices has a cycle.
    if graph.node_count() < 3 {
        return Ok(None);
    }

    let mut search = driver::CegarSearch::new(graph, driver::Config::default())?;
    let result = search.run()?;

    match result {
        driver::Step::Found(segment) => Ok(Some(segment)),
        driver::Step::NoCycle => Ok(None),
        driver::Step::LimitReached => Err(Error::LimitExceeded),
        driver::Step::Spurious { .. } => {
            // The run() loop consumes all Spurious steps and only returns
            // when settled, so Spurious should never surface here. If it
            // does, that is a broken invariant.
            panic!("run() returned Spurious, which should not happen")
        }
    }
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
pub fn find_hamiltonian_path(graph: &UnGraph<(), ()>) -> Result<Option<Segment>, Error> {
    match graph.node_count() {
        0 => return Ok(None),
        1 => {
            // A single vertex is a trivial open path.
            let vertex = graph.node_indices().next().unwrap();
            let segment = Segment::new_open(vec![vertex])
                .expect("a single vertex is a valid open segment");
            return Ok(Some(segment));
        }
        _ => {
            // n >= 2: reduce to a cycle query.
        }
    }

    // Build the reduced graph with the apex.
    let instance = reduction::CycleInstance::new(graph)?;

    // Search for a Hamiltonian cycle in the reduced graph.
    let mut search =
        driver::CegarSearch::new(instance.graph(), driver::Config::default())?;
    let result = search.run()?;

    // Translate the result back to a path in the original graph.
    match result {
        driver::Step::Found(cycle) => {
            let path = instance.path_from_cycle(&cycle)?;
            debug_assert!(
                path.is_hamiltonian_path(graph),
                "find_hamiltonian_path is about to return an answer that is \
                 not actually a Hamiltonian path of the original graph: {path}"
            );
            Ok(Some(path))
        }
        driver::Step::NoCycle => Ok(None),
        driver::Step::LimitReached => Err(Error::LimitExceeded),
        driver::Step::Spurious { .. } => {
            panic!("run() returned Spurious, which should not happen")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::graph_of;
    use claim::assert_ok;

    mod basic_entry_points {
        use super::*;

        #[test]
        fn path_graph_has_path_but_no_cycle() {
            // The path graph 0-1-2-3: a Hamiltonian path but no cycle.
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3)]);

            // Path should succeed.
            let path_result = assert_ok!(find_hamiltonian_path(&graph));
            let path = path_result.expect("path graph has a Hamiltonian path");
            assert!(
                path.is_hamiltonian_path(&graph),
                "not a Hamiltonian path: {path}"
            );

            // Cycle should fail (conclusively).
            let cycle_result = assert_ok!(find_hamiltonian_cycle(&graph));
            assert!(cycle_result.is_none());
        }

        #[test]
        fn knight_5x5_has_path_but_no_cycle() {
            let graph = crate::generators::knight_graph(5, 5).0;

            // Path should succeed.
            let path_result = assert_ok!(find_hamiltonian_path(&graph));
            let path = path_result.expect("5x5 knight's graph has an open tour");
            assert!(
                path.is_hamiltonian_path(&graph),
                "not a Hamiltonian path: {path}"
            );

            // Cycle should fail.
            let cycle_result = assert_ok!(find_hamiltonian_cycle(&graph));
            assert!(cycle_result.is_none());
        }

        #[test]
        fn triangle_has_cycle() {
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);

            let cycle_result = assert_ok!(find_hamiltonian_cycle(&graph));
            let cycle = cycle_result.expect("triangle has a Hamiltonian cycle");
            assert!(
                cycle.is_hamiltonian_cycle(&graph),
                "not a Hamiltonian cycle: {cycle}"
            );
        }
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn cycle_empty_graph() {
            let graph = graph_of(0, &[]);
            let result = assert_ok!(find_hamiltonian_cycle(&graph));
            assert!(result.is_none());
        }

        #[test]
        fn cycle_single_vertex() {
            let graph = graph_of(1, &[]);
            let result = assert_ok!(find_hamiltonian_cycle(&graph));
            assert!(result.is_none());
        }

        #[test]
        fn cycle_two_vertices() {
            let graph = graph_of(2, &[(0, 1)]);
            let result = assert_ok!(find_hamiltonian_cycle(&graph));
            assert!(result.is_none());
        }

        #[test]
        fn path_empty_graph() {
            let graph = graph_of(0, &[]);
            let result = assert_ok!(find_hamiltonian_path(&graph));
            assert!(result.is_none());
        }

        #[test]
        fn path_single_vertex() {
            let graph = graph_of(1, &[]);
            let result = assert_ok!(find_hamiltonian_path(&graph));
            let path = result.expect("a single vertex is a trivial open path");
            assert!(!path.is_closed());
            assert_eq!(path.len(), 1);
        }

        #[test]
        fn path_two_vertices() {
            let graph = graph_of(2, &[(0, 1)]);
            let result = assert_ok!(find_hamiltonian_path(&graph));
            let path = result.expect("two connected vertices form a trivial open path");
            assert!(
                path.is_hamiltonian_path(&graph),
                "not a Hamiltonian path: {path}"
            );
        }
    }

    mod error_display {
        use super::*;

        #[test]
        fn display_solver_error() {
            let err = Error::Solver("test message".to_string());
            let display = err.to_string();
            assert!(!display.is_empty());
            assert!(display.contains("solver error"));
            assert!(display.contains("test message"));
        }

        #[test]
        fn display_limit_exceeded() {
            let err = Error::LimitExceeded;
            let display = err.to_string();
            assert!(!display.is_empty());
            assert!(display.contains("limit"));
        }

        #[test]
        fn display_reduction_error() {
            let err = Error::Reduction(ReductionError::GraphTooSmall(0));
            let display = err.to_string();
            assert!(!display.is_empty());
            assert!(display.contains("reduction"));
        }

        #[test]
        fn display_malformed_error() {
            let err = Error::Malformed(SegmentError::Empty);
            let display = err.to_string();
            assert!(!display.is_empty());
            assert!(display.contains("malformed"));
        }

        #[test]
        fn display_malformed_cover_error() {
            let err = Error::MalformedCover(CoverError::NoOutArc(
                petgraph::graph::NodeIndex::new(0),
            ));
            let display = err.to_string();
            assert!(!display.is_empty());
            assert!(display.contains("cover"));
        }
    }
}

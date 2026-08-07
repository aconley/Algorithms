//! `CycleCover`: the state Knuth's Algorithm C carries between steps C4, C6
//! and C8.
//!
//! This module transcribes Knuth's array names — `succ`/`pred`/`cid`/`cyc`/
//! `cloc`/`head` — so the code can be read directly against
//! `.agents/algorithm.md`.  `CycleCover` is the CEGAR loop's actual working
//! data structure; [`super::segment::Decomposition`] is the ordered-sequence
//! *view* of it, derived on request by [`CycleCover::to_decomposition`].
//!
//! This part of the module (phase 5) implements only step C4, decoding a SAT
//! model into a cycle cover.  Step C6 (merging, phase 9) adds
//! `CycleCover::merge` later; it is not here.

use super::encoding::ArcVars;
use super::segment::{Decomposition, Segment, SegmentError};
use petgraph::graph::{NodeIndex, UnGraph};
use rustsat::types::{Assignment, TernaryVal, Var};
use std::fmt;

/// A model that does not describe a cycle cover.
///
/// The encoding's at-least-one and at-most-one clauses guarantee that every
/// vertex of a genuine model has exactly one true out-arc and exactly one true
/// in-arc.  So none of these variants is a legitimate outcome to be repaired:
/// each means the encoding and this decoder disagree with each other, which is
/// a bug in one of the two.
///
/// This is deliberately *not* a [`SegmentError`], which is scoped to failures
/// of `Segment` and `Decomposition` construction.  Arc-permutation bookkeeping
/// is a different kind of malformedness, and reporting a vertex with no
/// out-arc as `SegmentError::RepeatedVertex` would render as "vertex 0 appears
/// more than once in a segment" — the opposite of what happened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverError {
    /// No true arc leaves this vertex.
    NoOutArc(NodeIndex),
    /// No true arc enters this vertex.
    NoInArc(NodeIndex),
    /// More than one true arc leaves this vertex.
    DuplicateOutArc(NodeIndex),
    /// More than one true arc enters this vertex.
    DuplicateInArc(NodeIndex),
}

impl fmt::Display for CoverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoverError::NoOutArc(v) => {
                write!(f, "no arc leaves vertex {}", v.index())
            }
            CoverError::NoInArc(v) => {
                write!(f, "no arc enters vertex {}", v.index())
            }
            CoverError::DuplicateOutArc(v) => {
                write!(f, "more than one arc leaves vertex {}", v.index())
            }
            CoverError::DuplicateInArc(v) => {
                write!(f, "more than one arc enters vertex {}", v.index())
            }
        }
    }
}

impl std::error::Error for CoverError {}

/// Knuth's cycle-cover bookkeeping (step C4, and later C6/C8).
///
/// `cid` is 1-based: `0` means "not yet assigned".  `cloc` and `head` are
/// indexed by 1-based cycle id and sized `n + 1`, leaving slot `0` unused —
/// the least error-prone way to carry Knuth's 1-based `CLOC`/`HEAD` indexing
/// into Rust without an off-by-one at every access.
/// The arrays a sibling module walks — `SUCC`, `CID`, `HEAD` — are readable
/// throughout this directory, so a transcription of Algorithm C can use
/// Knuth's names directly.  The rest stay private: `PRED`, `CYC` and `CLOC`
/// are bookkeeping for C6's merge, which lives in this file, and `CYC` in
/// particular must be read through [`active`](Self::active).
#[derive(Debug)]
pub(super) struct CycleCover {
    pub(super) succ: Vec<NodeIndex>, // SUCC
    pred: Vec<NodeIndex>,            // PRED
    pub(super) cid: Vec<usize>,      // CID, 1-based cycle ids; 0 means unassigned
    cyc: Vec<usize>,                 // CYC, the sparse set of active cycles
    cloc: Vec<usize>,                // CLOC, location of cycle c in CYC
    pub(super) head: Vec<NodeIndex>, // HEAD, an arbitrary vertex of each cycle
    t: usize,                        // number of active cycles
}

impl CycleCover {
    /// Step C4: decodes a solved model into `SUCC`/`PRED`/`CID`/`CYC`/`CLOC`/
    /// `HEAD`.
    ///
    /// Malformed models are reported as a [`CoverError`] rather than repaired;
    /// see that type for why none of its cases can arise from a genuine model.
    pub(super) fn from_model(
        graph: &UnGraph<(), ()>,
        vars: &ArcVars,
        model: &Assignment,
    ) -> Result<Self, CoverError> {
        let n = graph.node_count();
        let mut out: Vec<Option<NodeIndex>> = vec![None; n];
        let mut inn: Vec<Option<NodeIndex>> = vec![None; n];

        for var in 0..vars.n_vars() {
            if model.var_value(Var::new(var)) != TernaryVal::True {
                continue;
            }
            let (u, v) = vars.arc(var);
            if out[u.index()].is_some() {
                return Err(CoverError::DuplicateOutArc(u));
            }
            out[u.index()] = Some(v);
            if inn[v.index()].is_some() {
                return Err(CoverError::DuplicateInArc(v));
            }
            inn[v.index()] = Some(u);
        }

        // Checking both directions vertex by vertex, rather than draining
        // `out` first and then `inn`, is what keeps `NoInArc` reachable: once
        // every vertex is known to have exactly one out-arc there are exactly
        // `n` true arcs, and the duplicate-in check above then forces every
        // vertex to have exactly one in-arc as well.
        let mut succ = Vec::with_capacity(n);
        let mut pred = Vec::with_capacity(n);
        for (i, (s, p)) in out.into_iter().zip(inn).enumerate() {
            let node = NodeIndex::new(i);
            succ.push(s.ok_or(CoverError::NoOutArc(node))?);
            pred.push(p.ok_or(CoverError::NoInArc(node))?);
        }

        // `succ` is now total and injective, so it permutes the vertices.
        // That is what makes every `succ` walk — the one below, and the one
        // in `to_decomposition` — guaranteed to return to where it started.
        //
        // CID[v] = 0 for 0 <= v < n; t = v = 0.
        let mut cid = vec![0usize; n];
        let mut cyc = Vec::new();
        let mut cloc = vec![0usize; n + 1];
        let mut head = vec![NodeIndex::new(0); n + 1];
        let mut t = 0usize;

        for v_idx in 0..n {
            let v = NodeIndex::new(v_idx);
            if cid[v.index()] != 0 {
                continue;
            }
            // A new cycle, discovered at v.
            t += 1;
            cyc.push(t);
            cloc[t] = cyc.len() - 1;
            head[t] = v;
            cid[v.index()] = t;
            let mut u = succ[v.index()];
            while u != v {
                cid[u.index()] = t;
                u = succ[u.index()];
            }
        }

        Ok(CycleCover {
            succ,
            pred,
            cid,
            cyc,
            cloc,
            head,
            t,
        })
    }

    /// Number of active cycles, Knuth's `t`.  `t == 1` means `succ` defines a
    /// single spanning cycle: a genuine Hamiltonian cycle.
    pub(super) fn t(&self) -> usize {
        self.t
    }

    /// The active cycle ids, Knuth's `CYC[0..t]`.
    ///
    /// The only way to see `CYC` from outside this file, deliberately:
    /// merging (phase 9) reduces `t` by absorbing one cycle into another, and
    /// Knuth's `CYC` is a fixed-size array whose entries past `t` are stale
    /// rather than removed, so iterating the whole vector would walk cycles
    /// that no longer exist.
    pub(super) fn active(&self) -> &[usize] {
        &self.cyc[..self.t]
    }

    /// Walks each active cycle from its `HEAD` and builds the ordered-segment
    /// view of this cover.
    ///
    /// Every segment produced here is closed: this decoder only ever builds
    /// cycles, never open paths.  Asymmetry clauses forbid length-2 cycles and
    /// the graph has no self-loops (rejected in `precheck`), so a genuine
    /// model can only produce cycles of length >= 3 — but a model malformed in
    /// a way [`CoverError`] cannot see can still produce a too-short cycle,
    /// which surfaces here as `SegmentError::ClosedTooShort` rather than in
    /// `from_model`: a 2-cycle is a perfectly well-formed arc permutation, so
    /// only walking it reveals how short it is.
    pub(super) fn to_decomposition(&self) -> Result<Decomposition, SegmentError> {
        let mut segments = Vec::with_capacity(self.t);
        for &c in self.active() {
            let start = self.head[c];
            let mut vertices = vec![start];
            let mut v = self.succ[start.index()];
            while v != start {
                vertices.push(v);
                v = self.succ[v.index()];
            }
            segments.push(Segment::new_closed(vertices)?);
        }
        Decomposition::new(segments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{graph_of, knuth_cover, model_of, v};
    use claim::{assert_err, assert_ok};

    mod from_model {
        use super::*;

        #[test]
        fn single_four_cycle_has_one_active_cycle() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
            let model = model_of(&graph, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
            let vars = ArcVars::new(&graph);
            let cover = assert_ok!(CycleCover::from_model(&graph, &vars, &model));

            assert_eq!(cover.t(), 1);
            assert_eq!(cover.succ, vec![v(1), v(2), v(3), v(0)]);
            assert_eq!(cover.pred, vec![v(3), v(0), v(1), v(2)]);
            assert_eq!(cover.cid, vec![1, 1, 1, 1]);
            assert_eq!(cover.cyc, vec![1]);
            assert_eq!(cover.cloc[1], 0);
            assert_eq!(cover.head[1], v(0));
        }

        #[test]
        fn two_disjoint_triangles_have_two_active_cycles() {
            let graph = graph_of(6, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]);
            let model =
                model_of(&graph, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]);
            let vars = ArcVars::new(&graph);
            let cover = assert_ok!(CycleCover::from_model(&graph, &vars, &model));

            assert_eq!(cover.t(), 2);
            assert_eq!(cover.succ, vec![v(1), v(2), v(0), v(4), v(5), v(3)]);
            assert_eq!(cover.pred, vec![v(2), v(0), v(1), v(5), v(3), v(4)]);
            assert_eq!(cover.cid, vec![1, 1, 1, 2, 2, 2]);
            assert_eq!(cover.cyc, vec![1, 2]);
            assert_eq!(cover.cloc[1], 0);
            assert_eq!(cover.cloc[2], 1);
            assert_eq!(cover.head[1], v(0));
            assert_eq!(cover.head[2], v(3));
        }

        #[test]
        fn thirteen_vertex_example_matches_the_book() {
            let cover = knuth_cover();

            // NAME[v] = A B C D E F G H I J K L M, v = 0..12.
            assert_eq!(
                cover.succ,
                vec![
                    v(2),
                    v(11),
                    v(6),
                    v(0),
                    v(8),
                    v(10),
                    v(3),
                    v(5),
                    v(1),
                    v(4),
                    v(12),
                    v(9),
                    v(7),
                ]
            );
            assert_eq!(
                cover.pred,
                vec![
                    v(3),
                    v(8),
                    v(0),
                    v(6),
                    v(9),
                    v(7),
                    v(2),
                    v(12),
                    v(4),
                    v(11),
                    v(5),
                    v(1),
                    v(10),
                ]
            );
            assert_eq!(cover.cid, vec![1, 2, 1, 1, 2, 3, 1, 3, 2, 2, 3, 2, 3]);
            assert_eq!(cover.t(), 3);
            assert_eq!(cover.cyc, vec![1, 2, 3]);
            assert_eq!(cover.cloc[1], 0);
            assert_eq!(cover.cloc[2], 1);
            assert_eq!(cover.cloc[3], 2);
            assert_eq!(cover.head[1], v(0));
            assert_eq!(cover.head[2], v(1));
            assert_eq!(cover.head[3], v(5));
        }

        #[test]
        fn rejects_a_vertex_with_two_true_out_arcs() {
            // A triangle with all three arcs of the cycle true, plus 0->2 —
            // the *reverse* of the already-true 2->0.  Vertex 0 then has two
            // out-arcs, which the at-most-one clauses forbid in a genuine
            // model (as do the asymmetry clauses, for this arc pair).
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);
            let model = model_of(&graph, &[(0, 1), (1, 2), (2, 0), (0, 2)]);
            let vars = ArcVars::new(&graph);
            let err = assert_err!(CycleCover::from_model(&graph, &vars, &model));
            assert_eq!(err, CoverError::DuplicateOutArc(v(0)));
        }

        #[test]
        fn rejects_a_vertex_with_two_true_in_arcs() {
            // As above, but the duplicate is an in-arc: both 1->0 and 2->0
            // true means vertex 0 has two predecessors. Neither arc's source
            // has a conflicting out-arc yet, so this hits the pred check
            // specifically, before the succ side ever sees a duplicate.
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);
            let model = model_of(&graph, &[(1, 0), (2, 0)]);
            let vars = ArcVars::new(&graph);
            let err = assert_err!(CycleCover::from_model(&graph, &vars, &model));
            assert_eq!(err, CoverError::DuplicateInArc(v(0)));
        }

        #[test]
        fn rejects_a_vertex_with_no_out_arc() {
            // Only two of the triangle's three arcs are true: 1->0 and 2->1.
            // Vertex 0 never appears as a source, so it has no out-arc; it is
            // also the lowest index with anything missing (vertex 2's
            // missing in-arc would only be found by continuing the scan), so
            // this is the case actually reported.
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);
            let model = model_of(&graph, &[(1, 0), (2, 1)]);
            let vars = ArcVars::new(&graph);
            let err = assert_err!(CycleCover::from_model(&graph, &vars, &model));
            assert_eq!(err, CoverError::NoOutArc(v(0)));
        }

        #[test]
        fn rejects_a_vertex_with_no_in_arc() {
            // Only two of the triangle's three arcs are true: 0->1 and 1->2.
            // Vertex 0 never appears as a target, so it has no in-arc, and it
            // is checked (and found complete on the out-arc side) before the
            // scan would otherwise reach vertex 2's missing out-arc.
            let graph = graph_of(3, &[(0, 1), (1, 2), (2, 0)]);
            let model = model_of(&graph, &[(0, 1), (1, 2)]);
            let vars = ArcVars::new(&graph);
            let err = assert_err!(CycleCover::from_model(&graph, &vars, &model));
            assert_eq!(err, CoverError::NoInArc(v(0)));
        }
    }

    mod to_decomposition {
        use super::*;

        #[test]
        fn single_four_cycle_yields_one_closed_segment() {
            let graph = graph_of(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
            let model = model_of(&graph, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
            let vars = ArcVars::new(&graph);
            let cover = assert_ok!(CycleCover::from_model(&graph, &vars, &model));

            let decomposition = assert_ok!(cover.to_decomposition());
            assert_eq!(decomposition.len(), 1);
            assert!(decomposition.segments()[0].is_closed());
            assert_eq!(decomposition.covered_vertices(), 4);
            assert_eq!(
                decomposition.as_hamiltonian_cycle(4).unwrap().vertices(),
                &[v(0), v(1), v(2), v(3)]
            );
        }

        #[test]
        fn two_disjoint_triangles_yield_two_closed_segments() {
            let graph = graph_of(6, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]);
            let model =
                model_of(&graph, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]);
            let vars = ArcVars::new(&graph);
            let cover = assert_ok!(CycleCover::from_model(&graph, &vars, &model));

            let decomposition = assert_ok!(cover.to_decomposition());
            assert_eq!(decomposition.len(), 2);
            assert!(decomposition.segments().iter().all(Segment::is_closed));
            assert_eq!(decomposition.as_hamiltonian_cycle(6), None);
        }

        #[test]
        fn thirteen_vertex_example_walks_to_the_three_stated_cycles() {
            let cover = knuth_cover();
            let decomposition = assert_ok!(cover.to_decomposition());
            assert_eq!(decomposition.covered_vertices(), 13);
            assert!(decomposition.segments().iter().all(Segment::is_closed));

            // The book's three cycles A-C-G-D, B-L-J-E-I and F-K-M-H, each in
            // the canonical orientation `Decomposition::new` imposes: rotated
            // to start at the lowest index, then oriented so its lower-indexed
            // neighbour comes second.  Asserting the sequences rather than
            // their lengths is the point of this fixture — a decoder that
            // produced right-sized wrong cycles would pass on lengths alone.
            let cycles: Vec<&[NodeIndex]> = decomposition
                .segments()
                .iter()
                .map(Segment::vertices)
                .collect();
            assert_eq!(
                cycles,
                vec![
                    &[v(0), v(2), v(6), v(3)][..],
                    &[v(1), v(8), v(4), v(9), v(11)][..],
                    &[v(5), v(7), v(12), v(10)][..],
                ]
            );
        }

        #[test]
        fn rejects_a_genuine_two_cycle() {
            // Two 2-cycles, 0 <-> 1 and 2 <-> 3.  Every vertex has exactly one
            // out-arc and one in-arc, so this is a well-formed permutation and
            // `from_model` has nothing to object to; it is the asymmetry
            // clauses that rule it out of a real model.  Only walking a cycle
            // reveals that it is too short, so `to_decomposition` is where it
            // is caught.
            let graph = graph_of(4, &[(0, 1), (2, 3), (3, 0), (1, 2)]);
            let model = model_of(&graph, &[(0, 1), (1, 0), (2, 3), (3, 2)]);
            let vars = ArcVars::new(&graph);
            let cover = assert_ok!(CycleCover::from_model(&graph, &vars, &model));
            let err = assert_err!(cover.to_decomposition());
            assert_eq!(err, SegmentError::ClosedTooShort(2));
        }
    }
}

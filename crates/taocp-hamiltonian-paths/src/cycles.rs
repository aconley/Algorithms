//! `CycleCover`: the state Knuth's Algorithm C carries between steps C4, C6
//! and C8.
//!
//! This module transcribes Knuth's array names — `succ`/`pred`/`cid`/`cyc`/
//! `cloc`/`head` — so the code can be read directly against
//! `.agents/algorithm.md`.  `CycleCover` is the CEGAR loop's actual working
//! data structure; [`super::segment::Decomposition`] is the ordered-sequence
//! *view* of it, derived on request by [`CycleCover::to_decomposition`].
//!
//! Two steps of Algorithm C live here: C4, which decodes a SAT model into a
//! cycle cover ([`CycleCover::from_model`]), and C6, which merges adjacent
//! cycles of that cover into one another ([`CycleCover::merge`]).

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
pub(crate) struct CycleCover {
    pub(crate) succ: Vec<NodeIndex>, // SUCC
    pred: Vec<NodeIndex>,            // PRED
    pub(crate) cid: Vec<usize>,      // CID, 1-based cycle ids; 0 means unassigned
    cyc: Vec<usize>,                 // CYC, the sparse set of active cycles
    cloc: Vec<usize>,                // CLOC, location of cycle c in CYC
    pub(crate) head: Vec<NodeIndex>, // HEAD, an arbitrary vertex of each cycle
    t: usize,                        // number of active cycles
}

impl CycleCover {
    /// Step C4: decodes a solved model into `SUCC`/`PRED`/`CID`/`CYC`/`CLOC`/
    /// `HEAD`.
    ///
    /// Malformed models are reported as a [`CoverError`] rather than repaired;
    /// see that type for why none of its cases can arise from a genuine model.
    pub(crate) fn from_model(
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
    pub(crate) fn t(&self) -> usize {
        self.t
    }

    /// The active cycle ids, Knuth's `CYC[0..t]`.
    ///
    /// The only way to see `CYC` from outside this file, deliberately:
    /// merging (phase 9) reduces `t` by absorbing one cycle into another, and
    /// Knuth's `CYC` is a fixed-size array whose entries past `t` are stale
    /// rather than removed, so iterating the whole vector would walk cycles
    /// that no longer exist.
    pub(crate) fn active(&self) -> &[usize] {
        &self.cyc[..self.t]
    }

    /// Step C6: absorbs cycles into one another, reducing `t`.
    ///
    /// Two cycles can be merged whenever two vertices that are adjacent *in*
    /// one of them are adjacent *to* two vertices that are adjacent in the
    /// other: splicing along that pair of edges joins both into a single
    /// cycle.  Every branch below is guarded by an adjacency test, so a merged
    /// cycle only ever uses real edges — it is a genuine cycle of the graph,
    /// and if merging reaches `t == 1` the result is a Hamiltonian cycle even
    /// though it is not the cover the solver returned.
    ///
    /// Merging cannot change the answer, only how many refinement rounds
    /// reaching it takes.  Cut clauses built from merged cycles still exclude
    /// the current model, because a merged cycle is a union of whole model
    /// cycles and so no model arc crosses it.
    ///
    /// Transcribed from `.agents/algorithm.md`, steps C6.1 to C6.13, whose
    /// labels appear as comments below.  Knuth's `ADJ[u][v] != 0` becomes
    /// `graph.find_edge(u, v).is_some()`; the variable number that `ADJ`
    /// otherwise carries is not wanted here, which is why this takes no
    /// [`ArcVars`].
    ///
    /// **One correction to that transcription**, which is load-bearing.  C6.9
    /// splices `v' .. w'` in between `v` and `w`, leaving `SUCC[v] == v'`
    /// rather than `w`; C6.11 then continues over the remaining neighbours of
    /// `v` with a `w` that is no longer `SUCC[v]`.  A second merge at the same
    /// `v` would splice its cycle in between `v` and that same stale `w`,
    /// giving `w` two predecessors and orphaning the cycle absorbed first.
    /// Setting `w = v'` at the end of C6.9 restores the step's own invariant
    /// that `w == SUCC[v]`, and has the further merit of extending C6.12's
    /// walk over the newly absorbed vertices.
    pub(crate) fn merge(&mut self, graph: &UnGraph<(), ()>) {
        // C6.1 [Begin loop on j.]
        let mut j = 0usize;

        // C6.13's `return to C6.2 if j < t` is this loop's bound.
        while j < self.t {
            // C6.2 [Choose c.]  We'll try to absorb other cycles into c.
            let c = self.cyc[j];

            // C6.3 [Begin loop on v.]
            let mut v = self.head[c];
            let mut w = self.succ[v.index()];

            loop {
                // C6.4 [Begin loop on v'.], with C6.11 [Advance v'.] as this
                // loop's increment.  Merging never touches the graph, so the
                // neighbour iterator stays valid across a merge.
                for v_prime in graph.neighbors(v) {
                    // C6.5 [Is v' in c?]
                    let c_prime = self.cid[v_prime.index()];
                    if c_prime == c {
                        continue;
                    }

                    // C6.6 [Check PRED[v'].]
                    let mut w_prime = self.pred[v_prime.index()];
                    if graph.find_edge(w_prime, w).is_none() {
                        // C6.7 [Check SUCC[v'].]
                        w_prime = self.succ[v_prime.index()];
                        if graph.find_edge(w_prime, w).is_none() {
                            continue;
                        }

                        // C6.8 [Reverse subpath.]  Splicing at SUCC[v']
                        // rather than PRED[v'] means c' has to be traversed
                        // the other way round, so reverse it.  This leaves
                        // SUCC[w'] stale, which C6.9 immediately overwrites.
                        let mut u = w_prime;
                        let mut u_prime = self.succ[u.index()];
                        while u != v_prime {
                            let u_dprime = self.succ[u_prime.index()];
                            self.succ[u_prime.index()] = u;
                            self.pred[u.index()] = u_prime;
                            u = u_prime;
                            u_prime = u_dprime;
                        }
                    }

                    // C6.9 [Merge.]
                    self.succ[v.index()] = v_prime;
                    self.succ[w_prime.index()] = w;
                    self.pred[v_prime.index()] = v;
                    self.pred[w.index()] = w_prime;
                    let mut u = v_prime;
                    while u != w {
                        self.cid[u.index()] = c;
                        u = self.succ[u.index()];
                    }
                    // Restore w == SUCC[v]; see this method's doc comment.
                    w = v_prime;

                    // C6.10 [Delete c'.]  Knuth checks t == 1 before touching
                    // CYC and CLOC, since C7 reads its answer straight out of
                    // SUCC and never consults them again.  Maintaining them
                    // first instead costs nothing and leaves `active()`
                    // naming the surviving cycle in that case too.
                    self.t -= 1;
                    let mut k = self.cloc[c_prime];
                    if k > j {
                        self.cyc[k] = self.cyc[self.t];
                        self.cloc[self.cyc[k]] = k;
                    } else {
                        // CYC[j] is c and c' != c, so k != j; reaching here
                        // means k < j, and hence that j is at least 1.
                        j -= 1;
                        while k < self.t {
                            self.cyc[k] = self.cyc[k + 1];
                            self.cloc[self.cyc[k]] = k;
                            k += 1;
                        }
                    }
                    if self.t == 1 {
                        return;
                    }
                }

                // C6.12 [Advance v.]
                if w == self.head[c] {
                    break;
                }
                v = w;
                w = self.succ[w.index()];
            }

            // C6.13 [Advance j.]
            j += 1;
        }
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
    pub(crate) fn to_decomposition(&self) -> Result<Decomposition, SegmentError> {
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
    use std::collections::HashSet;

    /// The vertices of `start`'s cycle, in `SUCC` order beginning at `start`.
    fn walk(cover: &CycleCover, start: NodeIndex) -> Vec<NodeIndex> {
        let mut vertices = vec![start];
        let mut u = cover.succ[start.index()];
        while u != start {
            vertices.push(u);
            u = cover.succ[u.index()];
        }
        vertices
    }

    /// Everything a cover must satisfy whatever has been done to it: `SUCC`
    /// follows real edges, `PRED` inverts it, and the active cycles partition
    /// the vertices in a way `CID` and `HEAD` agree with.
    ///
    /// Merging rewrites all six arrays at once, so checking them against each
    /// other is what catches a bookkeeping slip that a single-array assertion
    /// would let through.
    fn assert_cover_invariants(graph: &UnGraph<(), ()>, cover: &CycleCover) {
        for i in 0..graph.node_count() {
            let u = v(i);
            let next = cover.succ[i];
            assert!(
                graph.find_edge(u, next).is_some(),
                "SUCC[{i}] = {} is not an edge of the graph",
                next.index()
            );
            assert_eq!(
                cover.pred[next.index()],
                u,
                "PRED does not invert SUCC at {i}"
            );
        }

        let mut seen = vec![false; graph.node_count()];
        for &c in cover.active() {
            for u in walk(cover, cover.head[c]) {
                assert!(!seen[u.index()], "vertex {} is on two cycles", u.index());
                seen[u.index()] = true;
                assert_eq!(cover.cid[u.index()], c, "CID disagrees with the walk");
            }
        }
        assert!(
            seen.iter().all(|&covered| covered),
            "the active cycles do not cover every vertex"
        );
    }

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

    mod merge {
        use super::*;

        /// The graph the book's merge example runs on: its three cycles
        /// A-C-G-D, B-L-J-E-I and F-K-M-H, plus the two edges the merge
        /// consumes, A-B (0-1) and C-I (2-8).
        ///
        /// A subset of `knuth_graph`, which is the point.  Restricting the
        /// graph to the edges the example actually uses is what makes the
        /// merge it finds independent of the order neighbours are visited in,
        /// and so lets this test pin the book's stated numbers exactly.  The
        /// cover itself is the phase 5 fixture unchanged — a `CycleCover`
        /// holds no reference to a graph, only `SUCC` and `PRED`.
        fn merge_example_graph() -> UnGraph<(), ()> {
            graph_of(
                13,
                &[
                    (0, 2),
                    (2, 6),
                    (6, 3),
                    (3, 0), // A-C-G-D
                    (1, 11),
                    (11, 9),
                    (9, 4),
                    (4, 8),
                    (8, 1), // B-L-J-E-I
                    (5, 10),
                    (10, 12),
                    (12, 7),
                    (7, 5), // F-K-M-H
                    (0, 1), // A-B, the first merge edge
                    (2, 8), // C-I, the second
                ],
            )
        }

        #[test]
        fn thirteen_vertex_example_matches_the_book() {
            let graph = merge_example_graph();
            let mut cover = knuth_cover();
            assert_eq!(cover.t(), 3);

            cover.merge(&graph);

            // The values `.agents/algorithm.md` states for this example.
            assert_eq!(cover.succ[0], v(1));
            assert_eq!(cover.succ[8], v(2));
            assert_eq!(cover.pred[1], v(0));
            assert_eq!(cover.pred[2], v(8));
            assert_eq!(cover.t(), 2);
            assert_eq!(cover.active(), &[1, 3]);
            assert_eq!(cover.cloc[1], 0);
            assert_eq!(cover.cloc[3], 1);

            // Cycle 2 (B L J E I) was absorbed into cycle 1; cycle 3 is
            // untouched.
            assert_eq!(
                cover.cid,
                vec![1, 1, 1, 1, 1, 3, 1, 3, 1, 1, 3, 1, 3],
                "every CID of 2 should now read 1"
            );

            assert_eq!(
                walk(&cover, v(0)),
                vec![v(0), v(1), v(11), v(9), v(4), v(8), v(2), v(6), v(3),],
                "the merged cycle should be 0 1 11 9 4 8 2 6 3"
            );
            assert_cover_invariants(&graph, &cover);
        }

        #[test]
        fn reverses_the_subpath_when_only_succ_is_adjacent() {
            // Triangles 0-1-2 and 3-4-5 joined by 0-3 and 1-4.  Merging at
            // v = 0, w = 1 finds v' = 3, whose PRED is 5 — and 5-1 is not an
            // edge, so C6.6 fails and C6.7 takes SUCC[3] = 4 instead, whose
            // edge 4-1 is present.  Splicing there runs the second cycle the
            // other way round, so C6.8 has to reverse it.
            let graph = graph_of(
                6,
                &[
                    (0, 1),
                    (1, 2),
                    (2, 0),
                    (3, 4),
                    (4, 5),
                    (5, 3),
                    (0, 3),
                    (1, 4),
                ],
            );
            let vars = ArcVars::new(&graph);
            let model =
                model_of(&graph, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]);
            let mut cover = assert_ok!(CycleCover::from_model(&graph, &vars, &model));
            assert_eq!(cover.t(), 2);

            cover.merge(&graph);

            assert_eq!(cover.t(), 1);
            // 3-4-5 now runs backwards: 3 -> 5 -> 4, spliced between 0 and 1.
            assert_eq!(walk(&cover, v(0)), vec![v(0), v(3), v(5), v(4), v(1), v(2)]);
            assert_cover_invariants(&graph, &cover);
        }

        #[test]
        fn splices_at_most_once_per_vertex() {
            // Triangles 0-1-2, 3-4-5 and 6-7-8.  Vertex 0 is adjacent to both
            // of the other two cycles, and both of them have a vertex
            // adjacent to 1 — so with w left at its stale value of 1 after
            // the first merge, the second would splice in between 0 and 1 as
            // well, giving 1 two predecessors and orphaning 3-4-5.
            //
            // Keeping w == SUCC[v] instead makes the second merge look for an
            // edge to whichever cycle was spliced in first.  There is none
            // either way round, so exactly one merge happens and the third
            // triangle is left for the next pass.
            let graph = graph_of(
                9,
                &[
                    (0, 1),
                    (1, 2),
                    (2, 0),
                    (3, 4),
                    (4, 5),
                    (5, 3),
                    (6, 7),
                    (7, 8),
                    (8, 6),
                    (0, 6),
                    (1, 8),
                    (0, 3),
                    (1, 5),
                ],
            );
            let vars = ArcVars::new(&graph);
            let model = model_of(
                &graph,
                &[
                    (0, 1),
                    (1, 2),
                    (2, 0),
                    (3, 4),
                    (4, 5),
                    (5, 3),
                    (6, 7),
                    (7, 8),
                    (8, 6),
                ],
            );
            let mut cover = assert_ok!(CycleCover::from_model(&graph, &vars, &model));
            assert_eq!(cover.t(), 3);

            cover.merge(&graph);

            assert_eq!(cover.t(), 2);
            assert_cover_invariants(&graph, &cover);
            assert_eq!(walk(&cover, v(0)).len(), 6);
        }

        /// A deterministic xorshift.  A handful of small random graphs does
        /// not justify a `rand` dependency, and a fixed seed means a failure
        /// reproduces.
        struct Xorshift(u32);

        impl Xorshift {
            fn next_u32(&mut self) -> u32 {
                self.0 ^= self.0 << 13;
                self.0 ^= self.0 >> 17;
                self.0 ^= self.0 << 5;
                self.0
            }

            fn below(&mut self, bound: usize) -> usize {
                self.next_u32() as usize % bound
            }
        }

        /// A random graph paired with a random cycle cover of it.
        ///
        /// Built the other way round from the solver's: a random permutation
        /// of the vertices is cut into cycles of length >= 3, the graph is
        /// given those edges so the cover is genuinely one of its covers, and
        /// then extra random edges are added for merging to find.
        fn random_cover(rng: &mut Xorshift) -> (UnGraph<(), ()>, CycleCover) {
            let n = 6 + rng.below(8); // 6 ..= 13

            let mut order: Vec<usize> = (0..n).collect();
            for i in (1..n).rev() {
                order.swap(i, rng.below(i + 1));
            }

            // Cut into cycles, never leaving a tail too short to be one.
            let mut arcs = Vec::new();
            let mut start = 0;
            while start < n {
                let remaining = n - start;
                let len = if remaining < 6 {
                    remaining
                } else {
                    3 + rng.below(remaining - 5)
                };
                for i in 0..len {
                    arcs.push((order[start + i], order[start + (i + 1) % len]));
                }
                start += len;
            }

            // `present` keeps the graph simple: parallel edges and self-loops
            // both break the arc-variable mapping, and `precheck` rejects
            // them rather than letting them reach a cover.
            let mut present = HashSet::new();
            let mut edges = Vec::new();
            let mut add = |a: usize, b: usize, edges: &mut Vec<(usize, usize)>| {
                if a != b && present.insert((a.min(b), a.max(b))) {
                    edges.push((a, b));
                }
            };
            for &(a, b) in &arcs {
                add(a, b, &mut edges);
            }
            for _ in 0..n {
                add(rng.below(n), rng.below(n), &mut edges);
            }

            let graph = graph_of(n, &edges);
            let vars = ArcVars::new(&graph);
            let model = model_of(&graph, &arcs);
            let cover = assert_ok!(CycleCover::from_model(&graph, &vars, &model));
            (graph, cover)
        }

        #[test]
        fn random_covers_stay_well_formed() {
            let mut rng = Xorshift(0x2545_f491);
            for _ in 0..200 {
                let (graph, mut cover) = random_cover(&mut rng);
                assert_cover_invariants(&graph, &cover);

                let before = cover.t();
                cover.merge(&graph);

                assert!(
                    cover.t() <= before,
                    "merging raised t from {before} to {}",
                    cover.t()
                );
                assert_cover_invariants(&graph, &cover);
            }
        }
    }
}

# Implementation Plan

This file is the work order for implementing the CEGAR Hamiltonian cycle finder.
It is written to be executed **phase by phase**, in order, by an implementing
agent.  Each phase is a single commit with its own tests.

Read `overview.md` (design decisions, what was deliberately rejected) and
`algorithm.md` (Knuth's Algorithm C) first.  This file resolves the places where
those two documents contradict each other; **where this file and `overview.md`
disagree, this file wins**, and the disagreement is called out explicitly so you
know it was deliberate rather than an oversight.

---

## Decisions made before writing this plan

These were settled with the repository owner.  Do not relitigate them.

1. **Two SAT variables per edge, not one.**  `overview.md` says a SAT variable
   *is* an edge index; `algorithm.md` requires 2*m* variables, one per directed
   arc, with the ⊕1 pairing property.  The algorithm wins.  See "Variable
   mapping" below.
2. **`Endpoints` is deleted.**  The engine answers "is there *any* Hamiltonian
   path", which is the only question wanted.  `find_hamiltonian_path` takes no
   endpoint argument, the `Endpoints` enum goes away, and the reduction has
   exactly one form: an apex adjacent to every vertex.
3. **At-most-one is encoded pairwise** (Knuth's `d choose 2` binary clauses), not
   via `rustsat::encodings::am1`.  `overview.md` recommends the library, but that
   section was written before the algorithm was known and `algorithm.md` is
   explicit that the binary representation works well.  The library encoding is a
   reasonable second knob for a future benchmark; it is not the default.
4. **Cycle merging (step C6) lands in phase 9, behind a `Config` flag.**  Phases
   1–8 implement a complete and correct algorithm without it.  Having both gives
   an A/B benchmark, and isolates the trickiest transcription in its own review.
5. **A failed precondition returns `Ok(None)`**, which is honest: each check
   genuinely proves no Hamiltonian cycle exists.  A `Config` flag skips the checks
   so tests can drive rejected graphs through the SAT path.
6. **Scope is phases 0–11.**  The Criterion benchmark (phase 12) is a follow-up.

`cargo build` has been confirmed to work — CaDiCaL compiles in about 47 s on this
machine.  You do not need to re-verify the toolchain.

---

## Variable mapping

Let `m = graph.edge_count()`.  For undirected edge index `e`, with
`graph.edge_endpoints(e) == Some((a, b))`:

```
var(a → b) = 2 * e.index()
var(b → a) = 2 * e.index() + 1
```

so `var(u → v) ^ 1 == var(v → u)` for every arc.  Variables occupy `0 .. 2*m`.

Knuth numbers variables from 2 so that `0` can be a sentinel in his dense `ADJ`
matrix.  We have no such need — `rustsat`'s `Var` is 0-based and we look arcs up
through petgraph — so the offset is dropped.  **The ⊕1 pairing survives, and it
is the part that matters**: step C8's `l ⊕ 2` trick is, in our numbering, simply
`Lit::positive(v ^ 1)`.

Literals are built with `rustsat::types::Lit::{positive, negative}` taking a
`u32` variable index.

Do not build Knuth's dense `ADJ[u][v]` matrix.  Use `graph.find_edge(u, v)`,
which is O(deg) and already handles the "not adjacent" case by returning `None`.
If profiling ever shows this matters, revisit it then.

**This mapping is only valid because edges are never removed.**  Never call
`remove_edge` or `remove_node` on an instance graph.

---

## Module layout

`overview.md` names only *encoding* and *refinement* as the modules still to be
written.  This plan adds four more.  That is a deliberate departure, agreed with
the owner:

| File | Visibility | Phase | Contents |
|---|---|---|---|
| `mod.rs` | public surface | 8 | the two entry points, `Error` |
| `segment.rs` | `pub` types | 1 | `Segment`, `Decomposition`, `SegmentError` |
| `reduction.rs` | private | 2 | `CycleInstance`, `ReductionError` |
| `encoding.rs` | private | 3 | arc variable map, C1/C2 clause generation, DIMACS dump |
| `precheck.rs` | private | 4 | connectivity, degree, bridge and articulation checks |
| `cycles.rs` | private | 5, 9 | `CycleCover`: C4 decode, C6 merge |
| `refinement.rs` | private | 6 | C8 cut clauses |
| `driver.rs` | private | 7 | `CegarSearch`, `Step`, `Stats`, `Config` |
| `generators.rs` | **`pub`** | 10 | knight, grid, random, Petersen, with side tables |
| `render.rs` | **`pub`** | 11 | ASCII board, DOT, SVG |

`generators` and `render` are public because a `[[bin]]` or a bench is a separate
crate and cannot reach `pub(crate)` items.  This does not widen the *solver*
surface, which stays private — consistent with `overview.md`'s "visualisation
lives in-crate".

---

## Conventions for every phase

See **`../AGENTS.md`** — test organisation, rustfmt, assertions, warnings.  It is
short, it applies to every phase below, and no phase is complete until it is
satisfied.  It is kept separate from this file so that it still applies once the
phases are done.

Note in particular that it is *authoritative for this directory*: the rest of
`src/` is an accumulation of separate exercises, so do not copy conventions from
neighbouring modules.

---

## Phase 0 — Correct the skeleton's doc comments

Most of this phase is **already done**, before you start:

- `overview.md` and `algorithm.md` have been brought into line with this plan.
  Do not re-edit them; they are the current design, not stale prose.
- The `Endpoints` enum is **gone** from `reduction.rs` and `mod.rs`, along with
  the `EndpointNotInGraph` and `DegenerateEndpoints` error variants, the module
  doc's three-mode table, and the `endpoints` parameter on
  `find_hamiltonian_path`.  `CycleInstance::new` now takes only the graph.
  `cargo build` succeeds in this state.
- `AGENTS.md` now exists in this directory and holds the working conventions;
  `mod.rs` points at it and at the three `.agents/` files.

What is left is two doc comments in `driver.rs`, both describing the abandoned
one-variable-per-edge design:

- on `next_var`: it says edge variables occupy `0..edge_count`.  They occupy
  `0..2*edge_count`.
- on `CegarSearch::new`: it says the constructor "freezes every edge variable".
  It is every *arc* variable, all 2*m* of them.

**Done when:** no doc comment states the one-variable-per-edge mapping.  No test
changes.

---

## Phase 1 — `segment.rs`

Pure data.  No graph, no solver.  Fill in every `todo!()`.

- `Segment::new(vertices, closed)` — reject empty (`Empty`), a repeated vertex
  (`RepeatedVertex`), and a closed segment shorter than 3 (`ClosedTooShort`).
  For a closed segment the wrap-around edge is implied; the first vertex must
  **not** be repeated at the end.
- `Segment::endpoints` — `Some((first, last))` when open, `None` when closed.  A
  one-vertex open segment reports that vertex twice.
- `Segment::edges` — consecutive pairs, plus `(last, first)` when closed.
- `Segment::canonicalize` — **open**: reverse if the last vertex index is lower
  than the first.  **Closed**: rotate so the lowest index is first, then reverse
  the tail if the last vertex is lower than the second (i.e. so the lower-indexed
  of the two neighbours of the minimum comes second).
- `Display` for `Segment` and `SegmentError`.  Suggested segment form:
  `0 → 3 → 5 → 0` for closed, `0 → 3 → 5` for open.
- `Decomposition::new` — reject segments sharing a vertex (`OverlappingSegments`).
  Canonicalize each segment and sort segments by their first vertex, so the type
  has a normal form and tests can assert on it.
- `Decomposition::covered_vertices`, `Decomposition::as_hamiltonian_cycle(order)`
  — the latter returns `Some` only when there is exactly one segment, it is
  closed, and it covers `order` vertices.

**Tests:** every rejection case; `canonicalize` idempotent; `canonicalize` maps a
cycle and its reversal and every rotation to the same representation;
`as_hamiltonian_cycle` rejects one-open-segment, two-segment, and
covers-too-few cases.

**Done when:** `segment.rs` has no `todo!()` and its tests pass.

---

## Phase 2 — `reduction.rs`

The `Endpoints` enum is already gone (see phase 0); `ReductionError`'s variants
are `GraphTooSmall`, `CycleNotClosed`, `ApexNotInCycle` and `CycleNotSpanning`,
and the signatures below are the ones already in the file.  This phase fills in
the two `todo!()` bodies and implements `Display for ReductionError`.

(`CycleNotClosed` was added during phase 2.  Without it, an *open* segment of the
right length containing the apex translates successfully and returns a
plausible-looking path — a silent wrong answer rather than an error.)

- `CycleInstance::new(graph) -> Result<Self, ReductionError>` — clone the graph,
  then add the apex vertex, then add an edge from the apex to every original
  vertex.  **The apex is added last, and its edges after every original edge**,
  so `NodeIndex` `0..n` are unchanged and the apex is index `n`, and original
  `EdgeIndex` values are unchanged.  Reject `n < 2` with `GraphTooSmall(n)`;
  callers special-case `n <= 1` before reaching here.
- `path_from_cycle(cycle)` — reject an open segment (`CycleNotClosed`), a cycle
  that does not contain the apex (`ApexNotInCycle`), and one that does not have
  `n + 1` vertices (`CycleNotSpanning`).  Otherwise rotate so the apex is first,
  drop it, build an open `Segment` over the remaining vertices, and return it
  canonicalized.

**Tests** (these are the cases that break reductions):

- order 0 and 1 → `GraphTooSmall`; order 2 → succeeds.
- Apex index is exactly `n`; every original edge index still resolves to the same
  endpoint pair; `graph().edge_count() == m + n`.
- A disconnected graph still reduces (the reduction does not care); the apex makes
  G′ connected, which is why the precondition check must run on G′.
- A graph with a Hamiltonian **path** but no Hamiltonian **cycle** — e.g. the
  path graph `0-1-2-3`.  This is the test that proves the reduction does anything
  at all: G has no cycle, G′ does.
- `path_from_cycle` round-trips: build a cycle through the apex by hand, translate
  it, check the vertex order and that the result is open.  Do this twice, with the
  apex first in the sequence and with it in the middle, so the rotation is
  actually exercised.
- `path_from_cycle` rejects an open segment, a cycle without the apex, and a
  short cycle.
- Assert that a witness cycle is genuinely traversable in G′, and that a
  translated path is genuinely traversable in G.  A test that only checks vertex
  *sequences* will pass on a sequence that corresponds to no real walk.

**Done when:** `reduction.rs` has no `todo!()`, no reference to `Endpoints`
remains anywhere, and its tests pass.

---

## Phase 3 — `encoding.rs`

Steps C1 and C2.  Generates CNF; does **not** talk to a solver, so it is testable
by asserting on clause sets.

```rust
pub(super) struct ArcVars<'g> { graph: &'g UnGraph<(), ()> }

impl ArcVars<'_> {
    fn var(&self, from: NodeIndex, to: NodeIndex) -> Option<u32>;  // None if not adjacent
    fn arc(&self, var: u32) -> (NodeIndex, NodeIndex);             // inverse
    fn n_vars(&self) -> u32;                                       // 2 * m
}

pub(super) fn cycle_cover_cnf(graph: &UnGraph<(), ()>) -> Cnf;
pub(super) fn write_dimacs<W: Write>(cnf: &Cnf, w: &mut W) -> io::Result<()>;
```

`cycle_cover_cnf` emits, in this order:

1. **Asymmetry**, one per edge `e`: `(¬2e ∨ ¬(2e+1))`.  Forbids 2-cycles.
2. **At-least-one**, two per vertex `v` of degree `d`, with incident edges
   `e_1..e_d` and neighbours `u_1..u_d`:
   - out: `(var(v→u_1) ∨ … ∨ var(v→u_d))`
   - in:  `(var(u_1→v) ∨ … ∨ var(u_d→v))`
   A degree-1 vertex yields a unit clause, which is correct — it forces the arc,
   and combined with the asymmetry clause makes the formula unsatisfiable, which
   is the right answer.
3. **At-most-one**, pairwise, for each vertex and each `1 ≤ i < j ≤ d`:
   - out: `(¬var(v→u_i) ∨ ¬var(v→u_j))`
   - in:  `(¬var(u_i→v) ∨ ¬var(u_j→v))`

Total clause count is `m + 2n + 2·Σ_v C(d_v, 2)`.  Assert this in a test; it is a
cheap tripwire for a whole class of encoding bugs.

`write_dimacs` is a debugging aid only, per `overview.md`.  Dumping round *k* and
running a standalone solver on it separates "my encoding is wrong" from "my
refinement is wrong".  There is no DIMACS *input* path and none should be added.

**Tests:**

- The path graph `A–B–C` from `algorithm.md` §"Clauses encoding the cycle cover".
  n=3, m=2 → exactly 10 clauses: 2 asymmetry, 6 at-least-one, 2 at-most-one.
  Assert the exact clause *set* (as sorted literal vectors), not just the count —
  this is the one place where the whole numbering scheme is pinned down against a
  worked example.
- A triangle: n=3, m=3 → 3 + 6 + 6 = 15 clauses.
- `var` / `arc` round-trip over every arc; `var(u,v) ^ 1 == var(v,u)`;
  `var` returns `None` for a non-adjacent pair.

**Done when:** the A–B–C clause set matches `algorithm.md` literally.

---

## Phase 4 — `precheck.rs`

```rust
pub(super) enum Obstruction {
    Empty,
    SelfLoop(EdgeIndex),
    ParallelEdges(NodeIndex, NodeIndex),
    Disconnected,
    LowDegree(NodeIndex),        // degree < 2
    Bridge(EdgeIndex),
    ArticulationPoint(NodeIndex),
}

pub(super) fn obstruction(graph: &UnGraph<(), ()>) -> Option<Obstruction>;
```

Each of these genuinely precludes a Hamiltonian cycle, so `Some(_)` means the
driver returns `Ok(None)`.  Record the variant in `Stats` so a run can report
*why* it answered no.

Use petgraph directly — `overview.md` explicitly permits this, and all three are
present in 0.8.3:

- `petgraph::algo::connected_components(graph) == 1`
- `petgraph::algo::bridges(graph)` — returns an iterator of `EdgeRef`
- `petgraph::algo::articulation_points(graph)` — returns a `HashSet<NodeId>`

**Self-loops and parallel edges are checked here because neither design document
mentions them and both break the encoding**: a self-loop gets two arc variables
that both refer to one vertex, and parallel edges get independent variables for
what is one adjacency.  `UnGraph` permits both, so they must be rejected rather
than assumed away.

**Tests:** one minimal graph per variant, plus a 2-connected graph that passes.
Note that a graph failing an early check may also fail a later one; assert on the
specific variant returned and fix the check order in the code, do not weaken the
test.

**Done when:** each variant is produced by at least one test.

---

## Phase 5 — `cycles.rs`, part 1 (step C4)

The state Knuth's algorithm carries between C4, C6 and C8.  Transcribe the array
names, so the code can be read against the book.

```rust
pub(super) struct CycleCover {
    succ: Vec<NodeIndex>,   // SUCC
    pred: Vec<NodeIndex>,   // PRED
    cid:  Vec<usize>,       // CID, 1-based cycle ids; 0 means unassigned
    cyc:  Vec<usize>,       // CYC, the sparse set of active cycles
    cloc: Vec<usize>,       // CLOC, location of cycle c in CYC
    head: Vec<NodeIndex>,   // HEAD, an arbitrary vertex of each cycle
    t: usize,               // number of active cycles
}

impl CycleCover {
    pub(super) fn from_model(graph, vars: &ArcVars, model: &Assignment)
        -> Result<Self, SegmentError>;
    pub(super) fn t(&self) -> usize;
    pub(super) fn to_decomposition(&self) -> Result<Decomposition, SegmentError>;
}
```

`from_model` is step C4: for each true arc variable `uv`, set `SUCC[u] = v` and
`PRED[v] = u`; then set `CID[v] = 0` for all `v`, `t = v = 0`, and walk as
written.  Note Knuth's `CYC[t-1] = t` / `CLOC[t] = t-1` indexing: `CYC` is
0-indexed by *position*, `CLOC` and `HEAD` are indexed by *cycle id*, which runs
from 1.  Sizing `cloc` and `head` as `n + 1` and leaving slot 0 unused is the
least error-prone way to do this in Rust.

The encoding guarantees exactly one out-arc and one in-arc per vertex, so a
missing or duplicated arc means the encoding and the decoder disagree — return
`SegmentError`, which `Error::Malformed` wraps.  Do not silently repair it.

`to_decomposition` walks each active cycle from its `HEAD` and builds a closed
`Segment`.  **Every segment from a cycle cover is closed**; the open case of
`Decomposition` is unreachable from this encoding, and an open segment coming out
of here is a bug.  Asymmetry clauses plus the self-loop rejection in phase 4
guarantee every cycle has length ≥ 3, matching `Segment`'s `ClosedTooShort`.

**Tests:** hand-write arc-to-bool maps — no solver needed.  A single 4-cycle
(t=1); two disjoint triangles (t=2); the 13-vertex three-cycle configuration from
`algorithm.md` §"Merging cycle covers", asserting `SUCC`, `PRED`, `CID`, `CYC`,
`CLOC`, `HEAD` and `t` against the values printed there.  That example is the best
fixture in either document; build it once here and reuse it in phase 9.

**Done when:** the 13-vertex fixture reproduces every array in `algorithm.md`.

---

## Phase 6 — `refinement.rs` (step C8)

```rust
pub(super) enum Cut {
    Clauses(Vec<Clause>),
    /// Fewer than two edges cross some cut, so no Hamiltonian cycle exists.
    NoCycle,
}

pub(super) fn cut_clauses(graph, vars: &ArcVars, cover: &CycleCover) -> Cut;
```

For each active cycle `c = CYC[j]`, walk `v` from `HEAD[c]` around the cycle via
`SUCC`.  For each neighbour `u` of `v` with `CID[u] != c`, collect
`Lit::positive(var(v→u))`.  Call that list `l_1..l_k`.  Emit **two** clauses:

- `l_1 … l_k` — at least one arc *leaves* the cycle.
- `l_1^1 … l_k^1` — at least one arc *enters* it.  This is Knuth's `l ⊕ 2`; in
  our 0-based numbering it is the ⊕1 partner variable, i.e. the reverse arc.
  The second clause is exactly `Cut(complement C_j)`.

Loop control, verbatim from C8: **if `t > 2`, advance `j` normally; otherwise
stop after `j = 0`.**  When `t == 2`, `v ∈ CYC[0] ⟺ v ∉ CYC[1]`, so `Cut(C_1)` is
literally the clause already emitted as `Cut(complement C_0)` and emitting it
again is pure duplication.

**If `k < 2` for any cycle, return `Cut::NoCycle`.**  This is not a bug and must
not be "fixed" into a unit clause: fewer than two crossing edges means a cycle
cannot both leave the set and return to it, so no Hamiltonian cycle exists, and
the answer is conclusive.

Each crossing edge contributes exactly one literal (the pair `v ∈ C`, `u ∉ C` is
unique per edge), so no deduplication is needed.

**Tests:** given a graph and a hand-built `CycleCover`, assert the exact literal
sets.  Cover: `t == 2` emits two clauses total, not four; `t == 3` emits six; a
cycle with one crossing edge yields `NoCycle`; the two clauses of a pair are ⊕1
images of each other.

**Done when:** the `t == 2` and `k < 2` cases are each pinned by a test.

---

## Phase 7 — `driver.rs`

Wire it together.  No merging yet.

`Config` gains two fields beyond the existing `max_rounds` and `max_conflicts`:

```rust
pub(super) struct Config {
    pub max_rounds: Option<usize>,
    pub max_conflicts: Option<usize>,
    pub skip_preconditions: bool,   // phase 7
    pub merge_cycles: bool,         // phase 9; default true once implemented
}
```

`Default` is no limits, preconditions on.

`CegarSearch::new`:

1. Run `precheck::obstruction` unless `skip_preconditions`.  If it fires, record
   it in `Stats` and put the search into a state where the first `step` returns
   `Step::NoCycle`.
2. Create `CaDiCaL::default()`, `reserve` up to `2m`, and **`freeze_var` every
   one of the `2m` arc variables.**  Not `m` — the doc comment in the skeleton is
   wrong and phase 0 fixes it.  Refinement clauses are built from arc variables,
   and CaDiCaL's inprocessing will otherwise eliminate them and have to restore
   clauses on every round.  `overview.md` calls this the single most likely
   source of unexplained slowdown, and it is invisible when omitted — the code
   still returns correct answers, just slowly.
3. Add `encoding::cycle_cover_cnf(graph)`; record `initial_clauses`.
4. Set `next_var = 2 * m`.  Nothing allocates auxiliary variables under a
   pairwise encoding; the field exists for a future `am1` variant.
5. Apply `max_conflicts` via `LimitConflicts::limit_conflicts`.

`CegarSearch::step` is C3–C5 and C8:

1. `solve()`.  `SolverResult::Unsat` → `Step::NoCycle` (conclusive).
   `SolverResult::Interrupted` → `Step::LimitReached`.
2. `full_solution()`, then `CycleCover::from_model` (C4).
3. If `t == 1` (C5) return `Step::Found` with the canonicalized cycle.
4. Otherwise `refinement::cut_clauses` (C8).  `Cut::NoCycle` → `Step::NoCycle`.
   Add the clauses, update `Stats`, retain the cover on `self`, and return
   `Step::Spurious { cycles: t }` — **having already added the clauses**, so a
   caller that ignores the round still makes progress.

   Note what does *not* happen here: no `Decomposition` is built.  The loop is a
   transcription of Algorithm C and runs on `SUCC`/`PRED`/`CID` throughout;
   `cycles` is Knuth's `t`, which is free, and `Stats::segments_per_round` takes
   the same number.  `CegarSearch::decomposition()` derives the ordered-segment
   view from the retained cover when a renderer or a test asks for one, and
   returns `None` before the first `step`.  This is not an optimisation — the
   cost either way is negligible against a SAT solve — it is about keeping the
   core loop free of presentation concerns.
5. Check `max_rounds` and return `Step::LimitReached` if exceeded.

`run` loops `step` until it returns something other than `Spurious`.

`Stats` gains an `obstruction: Option<String>` field alongside the existing
counters.  Fill in `rounds`, `initial_clauses`, `refinement_clauses`,
`clauses_per_round`, `segments_per_round`, `conflicts` and `solve_time` as you
go — `overview.md` is emphatic that the statistics are part of the deliverable,
not instrumentation to be retrofitted.  Read conflicts from
`GetInternalStats::conflicts()`; note that this is **not** in the trait list in
`overview.md`, which is inaccurate on this point (`SolveStats` does not carry it).

**Tests, in increasing order of what they exercise:**

| Graph | Expected | Exercises |
|---|---|---|
| Path `A–B–C`, `skip_preconditions: true` | `NoCycle` at round 0 | UNSAT with no refinement.  Degree-1 units plus asymmetry are contradictory |
| Triangle | `Found`, 0 rounds | Smallest satisfiable case |
| K4 | `Found`, 0 rounds | 4 vertices admit no cycle cover but the Hamiltonian one, so still no refinement |
| **Petersen graph** | `NoCycle`, `rounds >= 1` | The important one.  Petersen is 3-regular and 3-connected so it clears every precondition, it is non-Hamiltonian, and it *has* a cycle cover (outer pentagon plus inner pentagram) — so the loop is **forced** through at least one refinement round before proving UNSAT.  This is the deterministic refinement test that `overview.md` asks for |
| Triangular prism | `Found` | Two triangles plus a perfect matching between them; the two triangles are a valid spurious cover, so this usually refines, though which cover CaDiCaL returns is not guaranteed.  Assert the answer, not the round count |
| 6×6 knight's graph | `Found` | End-to-end at real size.  **Not 5×5** — 5×5 has no closed tour |

Validate every `Found` result independently: all `n` vertices, no repeats, and
every consecutive pair (including the wrap-around) a real edge of the graph.
Write that validator once as a test helper; phases 8–11 all reuse it.

**Done when:** Petersen proves UNSAT after at least one refinement round, and the
6×6 knight tour validates.

---

## Phase 8 — `mod.rs`

- Implement `Display for Error`.
- `find_hamiltonian_cycle(graph)` — return `Ok(None)` for `n < 3` (no simple
  graph on fewer than 3 vertices has a cycle), otherwise run a `CegarSearch` to
  completion and map `Step` onto the result.  `Step::LimitReached` becomes
  `Err(Error::LimitExceeded)`, never `Ok(None)`: `Ok(None)` means *proved*, and
  folding "gave up" into it would destroy that reading.
- `find_hamiltonian_path(graph)` — no endpoint argument.  Special-case `n == 0`
  (`Ok(None)`) and `n == 1` (`Ok(Some)` with the single vertex as an open
  segment); those are the only sizes the reduction cannot handle, since `n == 2`
  produces a triangle and flows through normally.  Otherwise build a
  `CycleInstance`, search, and translate back with `path_from_cycle`.
- Remove `#![allow(dead_code)]`, and fix whatever warnings that surfaces.

**Tests:** integration-level, using `claim::{assert_ok, assert_err}` per
`../AGENTS.md`.  A path graph has a Hamiltonian path but no cycle — assert
both entry points on the same graph and get different answers.  A 5×5 knight's
graph has an open tour but no closed one: same pair of assertions, at real size.
Assert `n == 0` and `n == 1` for both entry points.

**Done when:** no `todo!()` remains in the module tree and `cargo build` is
warning-clean.

---

## Phase 9 — `cycles.rs`, part 2 (step C6, merging)

Add `CycleCover::merge(&mut self, graph, vars)`, called from `step` between C5
and C8 when `config.merge_cycles`.  After merging, re-check `t == 1` (step C7)
before falling through to C8.

**Transcribe C6.1 through C6.13 literally, with the step labels as comments.**
The `CLOC`/`CYC` bookkeeping in C6.10 — where the branch on `k > j` decides
between swapping the last cycle into the hole and shifting everything down while
decrementing `j` — is exactly the kind of code that gets "simplified" into a bug.
This repository already has a commit history of ring-maintenance bugs in the DPLL
work; do not add another.  Match the book, then test.

Two facts that make the design sound, so you do not have to rederive them:

- Merging only ever joins vertices along **real edges** (every branch is guarded
  by an `ADJ` lookup, i.e. `find_edge`), so a merged cycle is a genuine cycle of
  the graph.  If merging reaches `t == 1` the result is a valid Hamiltonian cycle
  even though it is not the model CaDiCaL returned.
- Cut clauses computed from *merged* cycles still exclude the current model.  A
  merged cycle is a union of whole model cycles, so no model arc crosses it, so
  the model violates the cut clause.  Progress is still guaranteed.

`Step::Spurious { cycles }` reports the **post-merge** `t`, and
`CegarSearch::decomposition()` likewise derives from the post-merge cover, since
that is what refinement acts on and what the round-by-round renderer should show.
Record both counts in `Stats`: capture `t` before calling `merge` into a new
`segments_before_merge_per_round`, alongside the existing `segments_per_round`.
Both are plain integer reads; neither requires building a `Decomposition`.

**Tests:**

- **The worked example from `algorithm.md`.**  Build a 13-vertex graph whose edges
  are the three cycles — `0-2, 2-6, 6-3, 3-0` / `1-11, 11-9, 9-4, 4-8, 8-1` /
  `5-10, 10-12, 12-7, 7-5` — plus the two merge edges `0-1` and `2-8`.  Load the
  `CycleCover` fixture from phase 5, run one merge pass, and assert exactly what
  the book states: `SUCC[0] == 1`, `SUCC[8] == 2`, `PRED[1] == 0`, `PRED[2] == 8`,
  `t == 2`, `CYC == [1, 3]`, `CLOC[1] == 0`, `CLOC[3] == 1`, and every former
  `CID` of 2 now reads 1.  Walking `SUCC` from 0 must give the 9-vertex cycle
  `0 1 11 9 4 8 2 6 3`.
- **A case that forces the C6.8 subpath reversal.**  The example above takes the
  `PRED[v']` branch at C6.6 and never reverses.  Construct a configuration where
  `ADJ[PRED[v']][w] == 0` but `ADJ[SUCC[v']][w] != 0`, so C6.7 falls through to
  C6.8.  Assert the reversed subpath's `SUCC`/`PRED` are mutually consistent.
- **A property test over random small graphs.**  Generate a random cycle cover,
  merge, and assert: every `SUCC` arc is a real edge; `SUCC` and `PRED` are
  mutual inverses; the cycles are vertex-disjoint and cover everything; `t` never
  increased; `CID` agrees with the actual walk from each `HEAD`.  This catches
  reversal and bookkeeping bugs that the two fixtures miss.
- **An A/B test**: Petersen and the 6×6 knight give the same *answers* with
  `merge_cycles` on and off.  Merging is an optimization; it must not change what
  the algorithm decides.

**Done when:** the book's worked example reproduces exactly, and the A/B answers
match.

---

## Phase 10 — `generators.rs`

Each generator returns `(UnGraph<(), ()>, Vec<Meta>)` where `Meta` is specific to
that family and indexed by node index.  **Per-problem data lives outside the
graph** — the graph stays `UnGraph<(), ()>`, because the solver has no use for
coordinates and putting them in node weights would push a presentation concern
into the solver's data structure.

- `knight_graph(ranks, files) -> (UnGraph<(), ()>, Vec<Square>)` where
  `Square { rank: usize, file: usize }`.
- `grid_graph(rows, cols) -> (UnGraph<(), ()>, Vec<Cell>)`.
- `petersen() -> UnGraph<(), ()>` — no side table; it has no natural geometry.
- `random_graph(n, edge_prob, seed) -> UnGraph<(), ()>` — deterministic from the
  seed.  Write a small xorshift rather than adding a `rand` dependency for this.

**Tests:** assert order and size against known values (6×6 knight: 36 vertices,
80 edges; Petersen: 10 vertices, 15 edges, 3-regular), and that node index `i`
agrees with side-table entry `i`.

---

## Phase 11 — `render.rs`

Plain functions, called directly.  **No `Renderer` trait and no dispatch enum** —
the caller always knows which family it generated, so there is no runtime dispatch
problem to solve.

Implement in this order; each is independently useful and the first is the one
that pays for itself immediately.

1. `board_ascii(segment, squares, ranks, files) -> String`.  Print the board with
   each square showing its visit order.  A wrong tour is instantly obvious, and
   this is what the tests assert against.
2. `dot(graph, segment) -> String`, via
   `petgraph::dot::Dot::with_attr_getters`, highlighting the solution edges.
   Render with Graphviz externally.  Output only — petgraph has no DOT reader and
   none is wanted.
3. `svg(graph, segment, coords) -> String` for geometric instances: a background
   grid, a circle per vertex, a styled polyline.  For a *solution* view draw only
   the path, not the graph's several hundred other edges.

All three take a `Decomposition` as well as a `Segment`, so they serve
intermediate counterexamples too.  That is the point of the shared type: emitting
one image per refinement round produces a flipbook of the abstraction tightening,
which costs only a round number in the filename and is probably the most
illuminating artifact this project can produce.

**Tests:** golden-string assertion on a known 6×6 tour for the ASCII renderer;
smoke tests that DOT and SVG output is non-empty and contains one entry per
solution edge.

---

## Phase 12 — deferred

Criterion benchmark comparing `merge_cycles` on versus off across the knight
family, plus a stats report.  Out of scope for this task; the `Stats` plumbing
from phase 7 is what makes it a small job later.

Note for whoever picks it up: a bench is a separate crate, so it cannot reach
`pub(super)` items.  It will need either a `pub` entry point returning `Stats`
alongside the segment, or the bench moved in-crate.  Decide then, not now.

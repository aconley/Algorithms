# Hamiltonian Paths — Agent Reference

This project implements Hamiltonian path search by **CEGAR**
(counterexample-guided abstraction refinement), following Knuth TAOCP.

**The engine searches for Hamiltonian _cycles_, not paths.**  Despite the
directory name, every path query is answered by reducing it to a cycle query on
a graph with one extra vertex.  Do not try to encode paths directly; see
"Path-to-cycle reduction" below.

This file records the design decisions that were made *before* any code was
written, so that an implementing agent does not have to re-derive them or
re-litigate them.  It covers representations, library choices, and — most
importantly — the things that were deliberately **rejected**.

The CEGAR algorithm itself is described in algorithm.md in this directory, and
the phase-by-phase work order is in plan.md.

---

## Status

A skeleton exists: every type, signature and doc comment is in place, and every
body is `todo!()`.  This is the `taocp-hamiltonian-paths` member of the
workspace; its crate root is `src/lib.rs`.

The job is to fill in the bodies — not to redesign the interfaces, which were
settled deliberately and are documented here and in the doc comments.

**Read plan.md before starting.**  It is the work order: phases 0 through 11,
each a commit with its own tests, with the fixtures and expected values spelled
out, plus a deferred phase 12 for benchmarking.  This file explains *why* the
design is what it is; plan.md says what to do.

`lib.rs` carries a crate-level `#![allow(dead_code)]` so the unfinished skeleton
does not bury real warnings.  **Remove it once the crate is implemented.**

### Where the skeleton is known to be wrong

The skeleton was written before Knuth's algorithm was transcribed, so parts of it
described a design that did not survive contact with it.  The `Endpoints` enum
has already been removed from `reduction.rs` and `lib.rs`.  What is left for
phase 0 of plan.md, and should not be trusted until then:

- `driver.rs`, on `next_var`: it says edge variables occupy `0..edge_count`.
  They occupy `0..2*edge_count` — see "Graph representation" below.
- `driver.rs`, on `CegarSearch::new`: it says "freezes every edge variable".  It
  is every *arc* variable, all 2*m* of them.

---

## Non-goals — read this first

These are the mistakes most likely to be made by a well-meaning implementer.
Each was considered and rejected for a stated reason.

| Do **not** | Why |
|---|---|
| Substitute **kissat** for CaDiCaL | kissat is non-incremental by design.  CEGAR depends on carrying learned clauses across rounds; using kissat silently discards the entire point of the exercise. |
| Reuse the DPLL solver in the `taocp-sat` crate as the backend | It is not a CDCL solver, so there are no learned clauses to preserve between rounds.  It may optionally be used as a slow cross-check oracle on tiny instances, but never as the CEGAR engine. |
| Give the graph node or edge weights | All per-problem data lives in side tables (see below).  The graph type is `Graph<(), ()>`. |
| Add a DOT/GraphML/graph6 **parser** | Instances are constructed programmatically in Rust.  Output only.  No import path is needed and none should be added. |
| Read a DIMACS *graph* file | Same reason.  (DIMACS **CNF** *output* is a separate and permitted debugging aid — see below.) |
| Introduce a `Renderer` trait or dispatch enum | The caller always knows which problem family it generated, so it can call the right rendering function directly.  There is no runtime dispatch problem to solve. |
| Represent a solution as a subgraph / edge set | The order is the answer.  See "Solution representation". |
| Remove nodes or edges from the graph | Index stability is load-bearing (see below). |
| Encode Hamiltonian *paths* directly | The engine does cycles only.  Paths reduce to cycles; encoding them directly means fighting the endpoint asymmetry for nothing. |
| Use one SAT variable per undirected edge | The algorithm is defined over *arcs*: two variables per edge, paired so that `var(u→v) ^ 1 == var(v→u)`.  The asymmetry clauses and the cut clauses both depend on that pairing.  An earlier draft of this file said otherwise; it was wrong. |
| Let the caller constrain which path is found | Only "is there *any* Hamiltonian path" is wanted — that is the point of CEGAR.  An earlier draft had an `Endpoints` enum for pinning one or both ends; it has been cut.  See "Path-to-cycle reduction" for the one case where it might come back, and why it probably will not. |
| Widen the public surface | Only the two entry points, `Segment`, `Error`, and their error types are `pub`, plus `generators` and `render`.  The driver, encoding, refinement and reduction internals stay private to this module tree. |
| Report "no Hamiltonian path exists" as an error | It is an ordinary answer: `Ok(None)`.  `Err` means the question was *not settled*. |
| Make anything generic over `EdgeType` | Concretely `UnGraph<(), ()>`.  The algorithm is now known and takes an undirected graph — it derives its own arc orientation internally — so there is still nothing to generalise over.  Changing one concrete type later is easy; unwinding premature generics is not. |

---

## Dependencies

Added to `Cargo.toml`, versions pinned as verified against docs.rs:

```toml
petgraph = "0.8.3"
rustsat = "0.7.5"
rustsat-cadical = "0.7.5"
```

`rustsat-cadical` vendors and statically links CaDiCaL's C++ source, so a C++
toolchain is required to build.  This has been confirmed to work in this repo —
`cargo build` succeeds, CaDiCaL compiling in about 47 s from cold.  No toolchain
investigation is needed.

`rustsat` provides the types and solver traits; `rustsat-cadical` provides the
`CaDiCaL` struct that implements them.  Both are needed.

---

## Module layout

| File | Visibility | Contents |
|---|---|---|
| `lib.rs` | public surface | the two entry points, `Error`, re-exports |
| `segment.rs` | `pub` types, re-exported | `Segment`, `Decomposition`, `SegmentError` |
| `reduction.rs` | private module | `CycleInstance`, `ReductionError` |
| `encoding.rs` | private module | arc variable map, cycle-cover CNF (Algorithm C steps C1–C2), DIMACS dump |
| `precheck.rs` | private module | connectivity, degree, bridge and articulation checks |
| `cycles.rs` | private module | `CycleCover`: Knuth's SUCC/PRED/CID/CYC/CLOC/HEAD, decoding (C4) and merging (C6) |
| `refinement.rs` | private module | cut clauses (C8) |
| `driver.rs` | private module | `CegarSearch`, `Step`, `Stats`, `Config` — all `pub(crate)` |
| `generators.rs` | **`pub`** | knight, grid, random, Petersen, each with its side table |
| `render.rs` | **`pub`** | ASCII board, DOT, SVG |

The original skeleton named only *encoding* and *refinement* as the modules still
to be written.  The four others were added once the algorithm was known:
`cycles.rs` and `precheck.rs` because Algorithm C needs them, `generators.rs` and
`render.rs` because a `[[bin]]` or a bench is a separate crate and cannot reach
`pub(crate)` items.  Making those two public does not widen the *solver* surface,
which stays private.

---

## Public API

The entire public surface:

```rust
pub fn find_hamiltonian_cycle(graph: &UnGraph<(), ()>)
    -> Result<Option<Segment>, Error>;

pub fn find_hamiltonian_path(graph: &UnGraph<(), ()>)
    -> Result<Option<Segment>, Error>;
```

Neither takes a parameter saying *which* cycle or path is wanted.  "Any" is the
only question asked, and it is the question CEGAR answers.

Three outcomes, and the distinction between them matters:

| Result | Meaning |
|---|---|
| `Ok(Some(seg))` | a witness |
| `Ok(None)` | a **proof** that none exists — the abstraction went UNSAT, which is conclusive |
| `Err(_)` | no answer: solver failure, or a limit was hit |

"Gave up" belongs in `Error`, not in `Ok(None)`.  Folding the two together would
destroy the strong reading of `Ok(None)` and force every caller to re-derive the
distinction.

`find_hamiltonian_cycle` is the engine's native question and is public in its own
right — a *closed* knight's tour is genuinely a cycle problem, not a path problem
in disguise.

---

## Path-to-cycle reduction

Add one new **apex** vertex *u* to *G*, joined to **every** vertex of *G*.  *G′*
then has a Hamiltonian cycle exactly when *G* has a Hamiltonian path: the cycle
enters *u* from one end and leaves towards the other, so deleting *u* recovers
the path.

That is the whole reduction.  There is one construction, it takes no parameters,
and the CEGAR code never learns that paths exist at all.

An earlier draft of this file had an `Endpoints` enum offering three modes —
both ends free, one end pinned to *s*, both ends pinned to *s* and *t* — realised
by shrinking the apex's neighbourhood.  It has been cut, for two reasons.  The
first is that constraining the answer is not a question anyone here wants to ask.
The second is that the `From(s)` row was simply wrong: it joined the apex to
"*s*, and every vertex", which is just "every vertex", which is the free case.
Pinning one end needs a genuinely different construction (a second added vertex
adjacent only to the apex and to *s*), and nobody noticed because nothing had
been implemented against it.

**Invariant: the apex is added last.**  Then `NodeIndex` 0..n−1 of *G′* are
exactly the vertices of *G* and the apex is index *n*; original edge indices are
likewise preserved, so a SAT variable denotes the same edge in both graphs.  No
remapping table is needed and none should be introduced.  Translating an answer
back is then just: rotate the cycle so the apex is first, drop it, reopen as a
path.

### What the apex costs

The apex has degree *n*, so its degree constraint is an *n*-way exactly-one, and
its pairwise at-most-one alone is 2·C(*n*,2) clauses.  For the 8×8 knight's graph
(64 vertices, 168 edges, degrees 4×2, 8×3, 20×4, 16×6, 16×8):

| Query | Vars | Clauses | From the apex's at-most-one |
|---|---|---|---|
| Closed tour — `find_hamiltonian_cycle` | 336 | 1,968 | — |
| Open tour — `find_hamiltonian_path` | 464 | 6,738 | 4,032 (60%) |
| Open tour with both ends pinned (hypothetical) | 340 | ~1,982 | 2 |

Three things follow, and they matter for deciding whether this is worth
optimising:

1. **Cycle queries pay nothing.**  A closed knight's tour is a Hamiltonian cycle
   question on the knight's graph itself; no apex is ever built.  The cost is
   confined to *open* tours.
2. **The blowup is entirely binary clauses**, which is the cheapest kind CaDiCaL
   handles — dedicated watch structures, propagated without clause traversal.
   6,738 clauses over 464 variables is a small formula by any modern standard.
   Clause count is a poor proxy for difficulty here.
3. **Refinement rounds are the likelier bottleneck.**  Each round is a fresh
   solve, and knight's-graph cycle covers fragment badly.  There is a plausible
   mechanism by which the apex makes *this* worse rather than the encoding: it is
   adjacent to everything, so it has *n* symmetric choices of predecessor and *n*
   of successor, which is a large space of equally-good spurious covers to wander
   through.

So the apex is not obviously worth optimising away, and the way to find out is to
measure — `Stats` records rounds, clauses per round and solve time separately,
which distinguishes the two hypotheses directly.  Restoring `Endpoints::Between`
would shrink the apex to degree 2 (≈1,982 clauses, the third row of that table),
but it answers a *different* question: recovering "any open tour" from it means
enumerating all C(64,2) = 2,016 endpoint pairs, almost certainly worse than one
large solve.  And if the problem turns out to be round count rather than formula
size, it does not help at all.

If open tours do turn out slow, try merging (`Config::merge_cycles`) first, then
symmetry-breaking.  The knight's graph is bipartite and a 64-vertex path must
have opposite-coloured ends, so requiring the apex's successor to be a black
square loses no tours and halves the apex's freedom, for one 32-literal clause.
That belongs in a generator, not in the public API.

`CycleInstance::new` is deliberately independent of the solver and **must be
tested on its own**.  An off-by-one in a reduction surfaces far downstream as
"the solver returned a wrong-looking answer", which is expensive to diagnose from
that end.

---

## Graph representation

Use petgraph with **unit weights on both nodes and edges**:

```rust
use petgraph::graph::{UnGraph, EdgeIndex, NodeIndex};

type Instance = UnGraph<(), ()>;   // or Graph<(), ()> if a directed variant is wanted
```

Everything the solver needs is positional:

| Concept | Representation |
|---|---|
| Vertex | `NodeIndex` (0-based) |
| Edge | `EdgeIndex` (0-based) |
| SAT variable | one per directed **arc**, two per edge — see below |

The graph is undirected, but the algorithm reasons about *arcs*, because it is
constructing an oriented cycle.  So there are 2*m* variables, not *m*.  For edge
index `e` with `edge_endpoints(e) == Some((a, b))`:

```rust
var(a → b) = 2 * e.index()
var(b → a) = 2 * e.index() + 1        // so var(u→v) ^ 1 == var(v→u)
```

`rustsat`'s `Lit`/`Var` are **0-based** (internally `idx << 1` with the low bit
as the negation flag), and petgraph's indices are 0-based, so this needs no
offset:

```rust
let lit = rustsat::types::Lit::positive(2 * edge.index() as u32);
```

**The ⊕1 pairing is load-bearing, not a convenience.**  The asymmetry clauses
pair `2e` with `2e+1` directly, and step C8 builds the "arc enters this cycle"
clause from the "arc leaves this cycle" clause by flipping that bit.  Knuth
numbers variables from 2 instead of 0, purely so that `0` can be a sentinel in
his dense `ADJ` matrix; we look arcs up through `graph.find_edge` and have no
such need, so the offset is dropped and the pairing kept.

**The mapping is only valid if edges are never removed.**  Never call
`remove_edge` or `remove_node` on an instance graph.  If a mutable graph is
ever genuinely required, switch to `StableGraph` and reconsider this mapping
explicitly — do not silently rely on it.

Self-loops and parallel edges break it too: a self-loop gets two arc variables
naming one vertex, and parallel edges get independent variables for a single
adjacency.  `UnGraph` permits both, so `precheck.rs` rejects both rather than
assuming they cannot happen.

### Per-problem data lives outside the graph

Problem families carry different metadata: a knight's-tour instance knows each
vertex is a `(rank, file)` board square; a grid graph knows `(row, col)`; a
random graph knows nothing.  **None of this belongs in the graph.**  The CEGAR
solver has no use for it — only the renderers do — so putting it in node
weights would push a presentation concern into the solver's data structure.

Instead, each generator returns its graph alongside a side table indexed by
node index, with a type specific to that problem family.  Renderers take the
graph, the solution, and that side table.  Different problem families are free
to define entirely different associated data types.

---

## SAT layer

### Verified API surface

Checked against docs.rs for rustsat / rustsat-cadical 0.7.5.  Signatures below
are accurate as of that version; anything **not** listed here should be
re-checked against the docs rather than guessed.

Construction — `CaDiCaL` implements `Default` and has two lifetime parameters
(`'term`, `'learn`) for terminator and learner callbacks:

```rust
use rustsat_cadical::CaDiCaL;
let mut solver = CaDiCaL::default();
```

From `rustsat::solvers::Solve` (all results are `anyhow::Result`):

```rust
fn solve(&mut self) -> Result<SolverResult>
fn lit_val(&self, lit: Lit) -> Result<TernaryVal>
fn var_val(&self, var: Var) -> Result<TernaryVal>
fn add_clause(&mut self, clause: Clause) -> Result<()>
fn add_clause_ref<C>(&mut self, clause: &C) -> Result<()> where C: AsRef<Cl> + ?Sized
fn add_unit(&mut self, lit: Lit) -> Result<()>
fn add_binary(&mut self, lit1: Lit, lit2: Lit) -> Result<()>
fn add_ternary(&mut self, lit1: Lit, lit2: Lit, lit3: Lit) -> Result<()>
fn add_cnf(&mut self, cnf: Cnf) -> Result<()>
fn reserve(&mut self, max_var: Var) -> Result<()>
fn solution(&self, high_var: Var) -> Result<Assignment>
fn full_solution(&self) -> Result<Assignment>   // requires Self: SolveStats
```

From `rustsat::solvers::SolveIncremental`:

```rust
fn solve_assumps(&mut self, assumps: &[Lit]) -> Result<SolverResult>
fn core(&mut self) -> Result<Vec<Lit>>
```

From `rustsat::solvers::SolveStats` — feeds the statistics reporting described
below:

```rust
fn n_sat_solves(&self) -> usize
fn n_unsat_solves(&self) -> usize
fn n_solves(&self) -> usize
fn n_clauses(&self) -> usize
fn max_var(&self) -> Option<Var>
fn n_vars(&self) -> usize
fn cpu_solve_time(&self) -> Duration
```

Note what is **not** on `SolveStats`: conflicts, decisions and propagations.
Those are on a different trait, verified against the vendored 0.7.5 source:

```rust
use rustsat::solvers::{GetInternalStats, LimitConflicts};

fn conflicts(&self) -> usize        // GetInternalStats
fn decisions(&self) -> usize
fn propagations(&self) -> usize

fn limit_conflicts(&mut self, limit: Option<u32>) -> Result<()>   // LimitConflicts
```

`Stats::conflicts` reads the first; `Config::max_conflicts` is applied with the
last.

CaDiCaL-specific, on the struct itself:

```rust
fn freeze_var(&mut self, var: Var) -> Result<()>
fn melt_var(&mut self, var: Var) -> Result<()>
fn is_frozen(&mut self, var: Var) -> Result<bool>
fn set_configuration(&mut self, config: Config) -> Result<()>
fn set_limit(&mut self, limit: Limit) -> Result<()>
fn get_statistic(&self, statistic: Statistic) -> u64
fn trace_proof<P: AsRef<Path>>(&mut self, path: P, format: ProofFormat) -> Result<(), NulError>
```

Literal construction (`rustsat::types`), 0-based:

```rust
Lit::positive(idx: u32) -> Lit
Lit::negative(idx: u32) -> Lit
Lit::new(idx: u32, negated: bool) -> Lit    // panics if idx > Var::MAX_IDX
lit.var() -> Var
lit.is_neg() -> bool
```

### Freeze the arc variables

CaDiCaL performs inprocessing, including variable elimination.  Adding a clause
that mentions an already-eliminated variable forces the solver to restore
clauses to stay sound — correct, but a performance cliff if it happens every
refinement round.

Since every refinement clause is built from arc variables, **freeze all 2*m* arc
variables immediately after creating the solver** via `freeze_var`.  Note 2*m*,
not *m*.  This is cheap insurance and is the single most likely source of
unexplained slowdown if omitted — and it is invisible when omitted, because the
answers stay correct and only the clock changes.

### Why plain incremental solving suffices

CEGAR refinement here is **monotone**: each round only ever *adds* clauses, and
never retracts one.  Consequently every clause CaDiCaL learned in round *k*
remains sound in round *k+1*, and the ordinary IPASIR-style incremental
interface gives the desired "restart using already-derived state" behaviour for
free.  No clause-retraction machinery is needed, and none should be built.

Assumptions (`solve_assumps` + `core`) are for *temporary* constraints.  Nothing
in the current design needs them: refinement is monotone, so every constraint is
permanent.  They are listed here because they are the tool to reach for if a
future variant ever wants to ask a family of related questions while reusing one
solver's accumulated learning.

### At-most-one encodings

The degree constraints are at-most-one constraints, encoded **pairwise**: for a
vertex of degree *d*, C(*d*,2) binary clauses for the outgoing arcs and C(*d*,2)
for the incoming.  This is what algorithm.md specifies, and Knuth notes the
binary representation works well in practice.

An earlier draft of this file recommended `rustsat::encodings::am1` over
"hand-rolling pairwise encodings".  That was written before the algorithm was
known and is now overruled — but `am1` remains the obvious second arm of a
benchmark, since the apex's degree-*n* at-most-one is the one place where a
ladder or commander encoding might pay for itself.  If you go looking, the exact
struct and method names were never verified; consult
<https://docs.rs/rustsat/0.7.5/rustsat/encodings/> rather than guessing.

### DIMACS CNF output

Permitted and encouraged as a debugging aid only.  Dumping the formula at round
*k* and running a standalone `cadical`/`kissat` binary on it separates "my
encoding is wrong" from "my refinement is wrong", which is the first question
to ask whenever the loop misbehaves.  This is an *output* path; it does not
imply any CNF input path.

---

## Solution representation

### Ordered sequences, not edge sets

A solution is an **ordered vertex sequence**, not a set of selected edges.
Deriving the edge set from the sequence is a trivial O(n) zip over consecutive
pairs; deriving the order from an edge set requires finding an endpoint and
walking.  The sequence strictly dominates, and it is what every renderer needs
(visit numbering, traversal direction, gradient colouring, endpoint markers).

### One type for solutions and counterexamples

A spurious model from an intermediate round decomposes into disjoint paths and
cycles covering the vertices.  A genuine Hamiltonian cycle is the degenerate
case: exactly one *closed* segment covering all *n* vertices.  That predicate is
`Decomposition::as_hamiltonian_cycle`, and it is the abstraction check — naming
it makes the driver read the way the algorithm reads.

Therefore define a **segment** — an ordered `Vec<NodeIndex>` plus an open/closed
flag — and let both an intermediate counterexample and a final answer be a
*collection* of segments.

This is deliberate and load-bearing:

- One renderer implementation serves both final solutions and per-round
  counterexamples, so the round-by-round visualisation comes free.
- Validation is one function: segments disjoint, every consecutive pair a real
  edge, then the Hamiltonicity predicate on top.
### What `Decomposition` is *not*

It is not the CEGAR loop's working data structure.  An earlier draft of this file
claimed "the refinement step consumes this same type, so the thing that is
rendered and the thing that is refined on cannot drift apart".  That was written
before the algorithm was known, and it is false: refinement consumes
`CycleCover` — Knuth's `SUCC`/`PRED`/`CID`/`CYC`/`CLOC`/`HEAD` arrays — because
step C8 needs `CID` lookups and `SUCC` walks that a `Vec<Segment>` cannot
provide.

The whole loop runs on those arrays.  C4 decodes the model straight into them,
C5 and C7 test `t == 1`, C6 merges by rewriting them in place, and C8 walks them.
`Segment` and `Decomposition` are the **ordered-sequence view** of that state,
derived from it, and they exist for four consumers: the answer handed back to
the caller, the renderers, per-round observation, and test assertions.

Consequently a `Decomposition` is materialised **on request, not on every
round** — `CegarSearch::decomposition()`.  The cost is small (an O(*n*) walk plus
a sort, against a SAT solve), so this is not an optimisation; it is about keeping
the core loop a transcription of Algorithm C rather than something that pauses
each round to build a presentation object.  `Stats` gets its per-round segment
count from `t` directly, which is free.

One consequence of the cycle-cover encoding: **every segment a model decodes to
is closed**.  `Decomposition`'s open case is unreachable from this abstraction,
and an open segment coming out of the decoder is a bug, not a case to handle.
The open case exists because a *final* path answer uses it, and because a future
abstraction over paths might.

### Canonicalisation

On an undirected graph a path and its reversal are the same solution, and a cycle
additionally has no distinguished starting point.  The rule, settled and
implemented in `Segment::canonicalize`:

- **open** — orient so the lower-indexed endpoint comes first;
- **closed** — rotate so the lowest-indexed vertex comes first, then orient so
  the lower-indexed of its two neighbours comes second.

`Decomposition` canonicalises each segment and then sorts segments by their first
vertex, so it too has a normal form.  This is what makes solutions comparable,
deduplicable, and assertable in tests.

### Outcome and statistics

Wrap the solution in an outcome (found / no path exists / resource limit
reached) and carry statistics alongside it: refinement rounds, clauses added
per round, solver conflicts and propagations, wall time per round.

For this project the statistics are as much the deliverable as the path is —
they are what shows whether one refinement strategy beats another.  Do not
treat them as optional instrumentation to be added later.

### Node indices and serialisation

The sequence holds `NodeIndex`, which is only meaningful paired with the graph
it came from; renderers therefore take both.  If a solution is ever serialised,
map through the generator's side table to stable labels on the way out.

---

## Rendering

Plain functions, called directly by each generator's driver.  No trait, no
dispatch enum — the caller knows what it generated.

Implement in this order:

1. **ASCII board output.**  For an 8×8 knight's tour, printing the board with
   each square showing its visit order is a `println!` loop, needs no
   dependency, and makes a wrong tour instantly obvious.  This is also what
   tests should assert against.  Build this first.
2. **DOT output** for graphs with no natural geometry.  petgraph 0.8.3 provides:

   ```rust
   Dot::with_attr_getters(
       graph: G,
       config: &'a [Config],
       get_edge_attributes: &'a dyn Fn(G, G::EdgeRef) -> String,
       get_node_attributes: &'a dyn Fn(G, G::NodeRef) -> String,
   ) -> Self
   ```

   Render with Graphviz externally.  Output only — petgraph has no DOT reader
   and none is wanted.
3. **SVG output** for geometric instances, if and when it is worth it.  With
   coordinates already known from the side table this is a background grid, a
   circle per vertex and a styled polyline; Graphviz adds nothing here.  Note
   that for a solution view you usually want to draw *only* the path, not the
   graph's several hundred other edges.

Emitting one image per refinement round produces a flipbook of the abstraction
tightening.  That is probably the most illuminating artifact this project can
generate, and costs only a round number in the filename.

---

## Testing conventions

Mechanics — test submodules, `claim`, rustfmt — are in `../AGENTS.md`, which is
authoritative for this directory.  Work red/green, as the DPLL work in this repo
did.

What follows is not mechanics but the *instances*: this crate needs canonical
test graphs, playing the role that the "R'" problem plays in `taocp-sat`.

- **5×5 knight's graph** — the smallest square board admitting an *open* knight's
  tour.  Use it for the path entry point only: 5×5 has **no closed tour** (25
  squares, and the knight's graph is bipartite with unequal parts), so it is the
  wrong fixture for the cycle engine.  It is a good fixture for exactly that
  reason — the two entry points must give different answers on it.
- **6×6 knight's graph** — the cycle-engine end-to-end test, since it does admit
  a closed tour.
- **The Petersen graph** — the instance whose first abstraction round is known to
  be spurious.  It is 3-regular and 3-connected, so it clears every precondition;
  it is non-Hamiltonian; and it *has* a cycle cover (outer pentagon plus inner
  pentagram).  So the loop is forced through at least one refinement round before
  proving UNSAT, and that is guaranteed rather than dependent on which model
  CaDiCaL happens to return.
- **The path graph A–B–C** from algorithm.md — degree-1 vertices force unit
  clauses which contradict the asymmetry clause, so the formula is unsatisfiable
  at round zero.  It doubles as the fixture pinning the whole clause-numbering
  scheme, since algorithm.md writes out all ten of its clauses.

The reduction needs its own tests, independent of the solver.  The cases that
break reductions:

- graphs of order 0, 1 and 2;
- a disconnected graph — the reduction does not care, which is precisely why the
  precondition check must run on *G′* rather than on *G*;
- a graph with a Hamiltonian path but **no** Hamiltonian cycle.  This is the
  test that proves the reduction is doing anything at all.

Merging (step C6) has a ready-made fixture: algorithm.md prints a 13-vertex,
three-cycle configuration together with every array before and after the merge.
Reproduce those values exactly.  Note that the printed example never takes the
subpath-reversal branch at C6.8, so it needs a companion case that does.

---

## Still to be decided

Deliberately left open; do not resolve these unilaterally:

- Whether an SVG helper crate is worth a dependency, or hand-formatted output
  suffices.

Resolved since this file was first written:

- **How the benchmark reaches `Stats`:** a `pub` entry point returning `Stats`
  alongside the segment.  A bench is a separate crate and still cannot see
  `pub(crate)` items, but since the workspace split `pub` widens only this
  solver crate rather than the whole repository, which makes it the cheap
  answer rather than a trade-off.  plan.md phase 12 records it.

- **The algorithm** is Knuth's Algorithm C, transcribed in algorithm.md.
- **Refinement strategy comparison** is cycle merging on versus off
  (`Config::merge_cycles`), which is why merging is a flag rather than
  unconditional.  A second arm comparing pairwise against `am1` for the apex's
  degree constraint is available if wanted.

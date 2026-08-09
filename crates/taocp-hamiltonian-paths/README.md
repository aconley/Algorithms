# taocp-hamiltonian-paths

Finding Hamiltonian paths and cycles in undirected graphs, with algorithms from
Knuth's *The Art of Computer Programming*.

A **Hamiltonian cycle** visits every vertex exactly once and returns to where it
started; a **Hamiltonian path** does the same without returning.  Both are
NP-complete, so every method here is a search with some structure to it rather
than a formula.

* [CEGAR](#cegar) is a SAT-based solver for finding a single Hamiltonian
path or cycle or proving none exists.

## CEGAR

### Usage

Graphs are `petgraph::graph::UnGraph<(), ()>` — no weights; per-problem data
lives in side tables the generators hand back.

```rust
use taocp_hamiltonian_paths::{
    find_hamiltonian_cycle, generators::knight_graph, render, Config,
};

let (graph, squares) = knight_graph(6, 6);

match find_hamiltonian_cycle(&graph, Config::default())? {
    Some(cycle) => {
        // A closed knight's tour, as a board with each square's visit number.
        print!("{}", render::board_ascii(&cycle, &squares, 6, 6));
        println!("{cycle}");
    }
    None => println!("no closed tour exists"),
}
```

Three outcomes, and the difference between them is the point:

| Result | Meaning |
|---|---|
| `Ok(Some(_))` | a witness — here it is |
| `Ok(None)` | a **proof** that none exists |
| `Err(_)` | the question was *not settled*: solver failure, or a limit was hit |

"Gave up" lives in `Err`, never in `Ok(None)`.  That is what lets `Ok(None)`
mean something strong.

### Examples

```bash
cargo run --example knights_tour         # closed and open tours, and a proof
cargo run --example petersen             # no cycle, but a path; writes an SVG
cargo run --example refinement_flipbook  # one SVG per refinement round
```

### CEGAR Algoirithm

**Counterexample-guided abstraction refinement**, over an incremental SAT
solver: Algorithm C from *TAOCP* Fascicle 8a.

Asking a SAT solver directly for "a single cycle through every vertex" needs
clauses saying the chosen edges form *one* cycle rather than several, and there
is no small way to say that.  CEGAR sidesteps it by asking a weaker question
and then repairing the answer:

1. **Abstract.**  Ask instead for a **cycle cover**: every vertex has exactly
   one incoming and one outgoing arc.  That is a small, local set of clauses.
2. **Check.**  A cycle cover may be a single spanning cycle — the real
   answer — or it may be several disjoint cycles, which is not.
3. **Refine.**  If it fragmented, add clauses forbidding *that* fragmentation,
   and solve again.  The solver is incremental, so learned clauses carry over
   between rounds.

Repeat until one spanning cycle survives, or until the formula goes
unsatisfiable — which is a **proof** that no Hamiltonian cycle exists.

The engine only ever searches for **cycles**.  A path query is answered by
adding an *apex* vertex joined to every vertex, finding a cycle in that larger
graph, and deleting the apex from it.  That reduction is not specific to CEGAR;
any cycle-only method can use it.

### Watching the rounds

`Search` is the same loop with the lid off: one refinement round per `step`,
with that round's cycle cover available to hand straight to a renderer.

```rust
use taocp_hamiltonian_paths::{generators::petersen, Config, Progress, Search};

let graph = petersen();
let mut search = Search::new(&graph, Config::default())?;

loop {
    let progress = search.step()?;
    if let Some(cover) = search.cover() {
        println!("cover has {} cycle(s)", cover?.len());
    }
    match progress {
        Progress::Refining { .. } => continue,
        _ => break,
    }
}
```

### Refinement rarely engages

Worth knowing before reading much into the round counts, and it surprised us.
Measured across this crate's generators, every satisfiable knight board up to
10×10, every grid, and every moderate-density random graph was settled by the
*first* solve, with zero refinement rounds — CaDiCaL finds a spanning cycle
directly.  Of ~210 random graphs swept near the Hamiltonicity threshold, only
three needed even one round.

The Petersen graph is the one instance here that reliably fragments, which is
why the flipbook example uses it.  Any benchmark comparing refinement
strategies needs instance families that actually fragment; finding them is the
first piece of that work, not an afterthought.

## Components

Brief descriptions of the components present in this sub-crate.

### General components

| Item | What it is for |
|---|---|
| `HamiltonianCycle`, `HamiltonianPath` | the two answers, as separate types so each exposes only what makes sense for it — a cycle has no endpoints to report, a path has no closedness to ask about |
| `Error` and friends | the question was not settled.  Absence of a tour is not in here |
| `generators` | instance families: knight, grid, Petersen, random — each with the side table mapping vertices back to squares or cells |
| `render` | `board_ascii`, `dot`, `svg` |
| `segment.rs` | `Segment`, an ordered run of vertices.  The order *is* the answer: it is what renderers need and what an edge set does not give |
| `reduction.rs` | `CycleInstance`: the apex construction turning a path query into a cycle query, and translating the answer back |
| `precheck.rs` | cheap structural rejections before any solver is troubled — connectivity, degree, bridges, articulation points |

### CEGAR-specific

| Item | What it is for |
|---|---|
| `find_hamiltonian_cycle`, `find_hamiltonian_path` | the one-shot entry points: run a search, hand back the answer |
| `driver.rs` | the CEGAR loop itself — `CegarSearch`, and Algorithm C's step structure |
| `encoding.rs` | steps C1–C2: the arc-variable map and the cycle-cover CNF |
| `cycles.rs` | `CycleCover` — Knuth's `SUCC`/`PRED`/`CID` arrays, decoding a model (C4) and merging adjacent cycles (C6).  This, not `Decomposition`, is what the loop actually runs on |
| `refinement.rs` | step C8: the cut clauses that forbid a fragmentation |
| `search.rs` | `Search` and `Progress`: the loop watched round by round |
| `Config` | knobs on *how* the search runs — limits, whether cycle merging is on.  Never *which* answer comes back.  Implements `Default` |
| `Stats` | rounds, clause growth, solver conflicts, time in the solver.  As much the deliverable as the tour is |
| `Decomposition` | a set of vertex-disjoint cycles: what one round's model decodes to.  A finished answer is the degenerate one-cycle case, which is why one renderer serves both |

### Design notes

`.agents/` holds the reasoning: `overview.md` for the decisions and — more
useful — the alternatives that were **rejected** and why, `algorithm.md` for
Knuth's Algorithm C transcribed, and `original_plan.md` for the work order the
crate was first built to, kept as a historical record rather than a plan to
follow.  `AGENTS.md` covers how to write code here.

Read `overview.md` before changing an interface; most of the obvious
"improvements" are in its rejected list with a reason attached.

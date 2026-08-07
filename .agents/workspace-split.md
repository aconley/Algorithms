# Workspace Split — Work Order

Convert `taocp` from one crate holding four unrelated exercises into a Cargo
workspace of four member crates.  This file is the work order; it is written to
be executed by an implementing agent in the three commits below.

## Why

Each top-level directory under `src/` is a separate exercise with its own
conventions, and `src/hamiltonian_paths/AGENTS.md` already says so in prose:
"the rest of `src/` is an accumulation of separate exercises rather than a
coherent body of work, so do not infer conventions from it."  A crate boundary
makes that structural instead of advisory, and buys three concrete things:

1. **Visibility means what it says.**  `pub(crate)` becomes "this exercise".
   The `pub(super)` spelling used throughout `hamiltonian_paths` is a
   workaround for a directory that wants to be a crate but is not.
2. **Build time.**  `rustsat-cadical` takes ~47 s to compile and is needed only
   by `hamiltonian_paths`.  Today every exercise, every bench and both binaries
   pay for it because they share one crate.  After the split they do not.
3. **Per-crate tooling.**  `cargo fmt -p taocp-hamiltonian-paths` and
   `cargo test -p taocp-sat` become exact, replacing the scoped-`rustfmt`
   workaround documented in `hamiltonian_paths/AGENTS.md`.

## What makes this safe

Measured before writing this file, and worth re-confirming rather than trusting:

- **No cross-directory coupling.**  Every `use crate::…` in the repository
  refers to the same top-level directory it appears in.  In particular
  `hamiltonian_paths` references nothing outside itself — there is no shared
  code to extract, duplicate, or re-export.
- **Each bench and binary belongs to exactly one directory**, so nothing needs
  to be split across members or given a home by judgement call.
- The only shared items are *dev-dependencies* (`claim`, `criterion`), which
  each member simply declares for itself.

## Version control

**This repository is managed with [jj](https://jj-vcs.github.io/) (0.44.0),
colocated with git.**  Use jj for every version-control operation; do not run
`git add` or `git commit`, which in a colocated repo creates state that jj then
has to reconcile.

Two consequences for a task that is mostly file moves:

- **There is no staging area and no `git mv`.**  jj snapshots the working copy
  automatically, so a plain `mv` is the whole operation.  Nothing records a
  rename — jj, like git, detects them by content when producing a diff — so
  there is no history to preserve by using a special command.
- **`jj commit -m "…"` finishes the current change and starts a new empty one
  on top.**  That is the whole per-commit workflow; there is nothing to add
  first.

`jj status`, `jj diff` and `jj log` inspect state.  If a step goes wrong,
`jj undo` reverses the last operation — prefer it to hand-repairing the tree.

## Baseline to preserve

Captured on the commit before this work starts.  The split must not change any
of these numbers.

| Scope | Tests |
|---|---|
| `backtracking` | 92 |
| `basic_combinations` | 30 |
| `hamiltonian_paths` | 82 |
| `sat` | 144 |
| `sat_solve` binary | 6 |
| **total** | **354** |

`cargo build --all-targets` is currently **warning-clean**.  Note that this
contradicts `hamiltonian_paths/AGENTS.md`, which claims two pre-existing
warnings in `sat_solve` and `pentominoes_box`; that note is stale and is
corrected in commit 3.

## Target layout

```
taocp/
├── Cargo.toml              # [workspace] only — a virtual manifest, no [package]
├── rustfmt.toml            # unchanged, stays at the workspace root
├── .agents/workspace-split.md
└── crates/
    ├── taocp-backtracking/
    │   ├── Cargo.toml
    │   ├── src/lib.rs      # was src/backtracking/mod.rs
    │   ├── src/*.rs
    │   ├── src/bin/pentominoes_box.rs
    │   └── benches/{dancing,langford,nqueens,sudoku}_benchmark.rs
    ├── taocp-basic-combinations/
    │   ├── src/lib.rs      # was src/basic_combinations/mod.rs
    │   └── benches/combinatoric_benchmark.rs
    ├── taocp-hamiltonian-paths/
    │   ├── AGENTS.md       # moves with the crate
    │   ├── .agents/{overview,algorithm,plan}.md
    │   └── src/lib.rs      # was src/hamiltonian_paths/mod.rs
    └── taocp-sat/
        ├── AGENTS.md
        ├── .agents/
        ├── src/lib.rs      # was src/sat/mod.rs
        ├── src/bin/sat_solve.rs
        └── benches/dpll_benchmark.rs
```

`src/lib.rs` is deleted.  `src/sat/target/` is a stray gitignored
rust-analyzer artifact — delete it rather than moving it.

### Naming

Package names carry a `taocp-` prefix: `taocp-backtracking`,
`taocp-basic-combinations`, `taocp-hamiltonian-paths`, `taocp-sat`.  Cargo
derives each lib name by replacing hyphens with underscores, so call sites read
`use taocp_sat::…` and `use taocp_hamiltonian_paths::…`.

Rust has no crate namespacing, so a literal `taocp::sat` path is not available
without a facade crate named `taocp` that re-exports the members.  That was
considered and rejected: after the split the only call sites naming a crate are
the six benches and two binaries, and each of those lives inside its member and
depends on it directly, so a facade would have no users — it would add a fifth
crate and a feature matrix to serve nobody.  The prefix puts `taocp` in those
same paths for the cost of four `name =` fields.

## Commit 1 — the split

The working tree must compile and all 354 tests must pass at the end of this
commit.  Keep it to moves plus the minimum edits needed to build; visibility
and prose changes belong to commits 2 and 3.

### 1. Root manifest

Replace `Cargo.toml` with a virtual manifest.  Centralise versions in
`[workspace.dependencies]` so the four members cannot drift apart:

```toml
[workspace]
resolver = "2"
members = ["crates/*"]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Alex Conley <alexanderconley@gmail.com>"]

[workspace.dependencies]
lazy_static = "1.4.0"
petgraph = "0.8.3"
rustsat = "0.7.5"
rustsat-cadical = "0.7.5"
criterion = "0.4"
claim = "0.5"
```

`resolver = "2"` must be stated explicitly: a virtual manifest does not inherit
the resolver from its members' edition.

### 2. Move the files

Each `src/<dir>/mod.rs` becomes `crates/<crate>/src/lib.rs`; every other `.rs`
file in the directory keeps its name.  `AGENTS.md` and `.agents/` move with
their crate.

Plain `mv` is correct here — see "Version control" below for why there is no
`git mv` step.

Binaries move into their crate's `src/bin/`, which makes them
auto-discovered — so the explicit `[[bin]]` stanza with a `path` that
`pentominoes_box` needs today can be dropped entirely.

### 3. Per-member manifests

Dependencies, measured per directory:

| Crate | Dependencies | Dev-dependencies | Bins | Benches |
|---|---|---|---|---|
| `taocp-backtracking` | `lazy_static` | `claim`, `criterion` | `pentominoes_box` | 4 |
| `taocp-basic-combinations` | — | `criterion` | — | 1 |
| `taocp-hamiltonian-paths` | `petgraph`, `rustsat`, `rustsat-cadical` | `claim` | — | — |
| `taocp-sat` | — | `claim`, `criterion` | `sat_solve` | 1 |

Each member inherits the shared fields, e.g. for `taocp-hamiltonian-paths`:

```toml
[package]
name = "taocp-hamiltonian-paths"
version.workspace = true
edition.workspace = true
authors.workspace = true

[dependencies]
petgraph.workspace = true
rustsat.workspace = true
rustsat-cadical.workspace = true

[dev-dependencies]
claim.workspace = true
```

Every bench keeps its `harness = false`, now in its own member's manifest:

```toml
[[bench]]
name = "dpll_benchmark"
harness = false
```

### 4. Rewrite the imports

This is the only non-mechanical part, and it is a narrow one.  A module that
becomes a crate root loses one path segment:

| Location | Before | After |
|---|---|---|
| within a member | `use crate::sat::sample_problems::langford;` | `use crate::sample_problems::langford;` |
| within a member | `use crate::backtracking::dancing_links::{…};` | `use crate::dancing_links::{…};` |
| within a member | `use crate::hamiltonian_paths::testing::{…};` | `use crate::testing::{…};` |
| bench or bin | `use taocp::sat::{…};` | `use taocp_sat::{…};` |

Do **not** blanket-`sed` this.  `sat` has a module *named* `backtracking`
(`src/sat/backtracking.rs`), so the string `crate::backtracking` is legitimate
inside the `sat` crate and must survive; a repository-wide substitution would
silently break it.  Rewrite per crate, and let `cargo build` confirm.

Note that `src/sat/mod.rs`'s `pub use` re-exports need no change: as a
`lib.rs` they re-export from the crate root, which is what they already did.

### 5. Verify

```
cargo build --workspace --all-targets    # no warnings
cargo test --workspace                   # 354 passing, per the table above
cargo fmt --all --check                  # see the caveat in commit 3
```

## Commit 2 — `pub(super)` becomes `pub(crate)`

Confined to `crates/taocp-hamiltonian-paths/`, and mechanical now that the
directory is a crate: `pub(super)` on an item in a top-level module already
*means* `pub(crate)`, so this changes no behaviour and the test count must not
move.  It is a separate commit precisely so that reviewers can see commit 1 is
a pure move.

Two things to preserve rather than sweep up:

- `CycleCover`'s field-by-field split (`succ`/`cid`/`head` visible, `pred`/
  `cyc`/`cloc`/`t` private) is deliberate and must survive verbatim.  Only the
  *keyword* changes.
- `CycleCover::active()` stays.  It is not a field accessor; it returns the
  `cyc[..t]` prefix and is the only way to read `CYC` from outside `cycles.rs`,
  which is what stops phase 9's merging from walking stale cycle ids.

## Commit 3 — documentation

### `crates/taocp-hamiltonian-paths/AGENTS.md`

The "Formatting" section is largely obsolete and should be rewritten:

- The scoped `rustfmt --edition 2021 src/hamiltonian_paths/*.rs` command is
  replaced by `cargo fmt -p taocp-hamiltonian-paths`.
- The reason for avoiding bare `cargo fmt` narrows but does not vanish: 21
  files in the other three crates are still not clean at 90 columns, so
  `cargo fmt --all` would still drag in unrelated churn.  (The file says
  "seventeen"; it is 21 today.  Prefer dropping the count over updating it.)
- The warning "do not add a nested `rustfmt.toml`" is now wrong for a crate
  directory — a member crate *is* a crate root, so rustfmt would honour one
  there.  Keeping the config at the workspace root is still right, because all
  four crates want the same width; say that instead.
- The claim about "two pre-existing warnings in `sat_solve` and
  `pentominoes_box`" is stale — the build is warning-clean.  Verify, then
  drop it.

The "Shared test fixtures" section stays accurate, but its reasoning improves:
`testing.rs` can now cite `pub(crate)` rather than `pub(super)` as the reason
`tests/common/` will not work.

### `crates/taocp-hamiltonian-paths/.agents/plan.md`

- Update the module-layout table: paths become crate-relative and `mod.rs`
  becomes `lib.rs`.
- Replace `pub(super)` with `pub(crate)` in the prose and signature blocks.
- **Phases 10 and 11** justify making `generators` and `render` `pub` because
  "a `[[bin]]` or a bench is a separate crate and cannot reach `pub(crate)`
  items."  That reasoning is still *correct* after the split — a bench in
  `crates/taocp-hamiltonian-paths/benches/` still links the crate externally.
  What changes is the cost: `pub` now widens a focused solver crate rather
  than the whole `taocp` grab-bag.  Reframe; do not delete.
- **Phase 12** defers a decision that is now cheap to make: "a bench is a
  separate crate, so it cannot reach `pub(super)` items.  It will need either a
  `pub` entry point returning `Stats` alongside the segment, or the bench moved
  in-crate.  Decide then, not now."  Record the answer: a `pub` entry point
  returning `Stats`, since `pub` on this crate is no longer a wide claim.

### `crates/taocp-sat/AGENTS.md`

Audited: it carries no formatting or tooling guidance, so the concerns above
do not apply to it.  It needs exactly two path corrections, both from the
`mod.rs` → `lib.rs` rename:

- "All types are re-exported from `mod.rs`" → `lib.rs`.
- Step 6 of "Implementing a new algorithm", which says to register the module
  in `src/sat/mod.rs` → `crates/taocp-sat/src/lib.rs`.

Its `use super::SatProblem` examples stay correct unchanged: `super` from a
top-level module is the crate root either way.

The four files in `crates/taocp-sat/.agents/` contain no path references and
need no edits at all.

## Out of scope

- **Reformatting the 21 unclean files.**  `AGENTS.md` already records that as
  its own separate change, and folding it in would bury the split's diff.  The
  split does make it easier afterwards: it can be done one crate at a time with
  `cargo fmt -p <crate>`.
- **Any behaviour change.**  No logic, no test, and no public API is altered by
  any of the three commits.
- **The `hamiltonian_paths` plan itself.**  Phase 7 and onward continue
  unchanged, in the new location.

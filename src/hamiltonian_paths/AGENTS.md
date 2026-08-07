# Hamiltonian Paths — Working Conventions

How to work in this directory.  These conventions are **local and authoritative**:
the rest of `src/` is an accumulation of separate exercises rather than a coherent
body of work, so do not infer conventions from it.  Where a neighbouring module
does something differently, that is not a precedent — follow this file.

## Where things are

| File | What it is for |
|---|---|
| `.agents/overview.md` | The design decisions, and — more importantly — the alternatives that were considered and **rejected**, with reasons.  Read before changing an interface. |
| `.agents/algorithm.md` | Knuth's Algorithm C (Fascicle 8a), transcribed, with implementation notes where the transcription needed adapting. |
| `.agents/plan.md` | The phase-by-phase work order.  Each phase is one commit with its own tests. |
| `AGENTS.md` | This file: how to write the code, as opposed to what to write. |

## Formatting

**Run rustfmt.**  Every file here is expected to be rustfmt-clean, and a phase is
not complete until it is:

```bash
rustfmt --edition 2021 src/hamiltonian_paths/*.rs
```

Add `--check` to report without rewriting.  This is not a matter of taste —
unformatted code makes every subsequent diff noisier than the change it carries.

Use that scoped command rather than bare `cargo fmt` **for now**.  Seventeen
files elsewhere in the repository predate the width setting below and are not yet
clean under it; `cargo fmt` would reformat them all and drag unrelated churn into
whatever you are working on.  That reformat is planned as its own separate
change.  Once it lands, `cargo fmt` becomes the better command and this note
should go.

The scoped command still picks up the repository-root `rustfmt.toml` — rustfmt
walks up from each input file's directory to find it — so both commands format
this directory identically.

### Line width

`rustfmt.toml` at the **repository root** sets `max_width = 90`, against
rustfmt's default of 100.

It is at the root deliberately.  `cargo fmt` passes the crate root to rustfmt,
so rustfmt resolves its config from *there* and **silently ignores a nested
`rustfmt.toml`** in a subdirectory.  A per-directory config appears to work when
you invoke `rustfmt` on the files directly, then gets quietly undone by the next
`cargo fmt`.  Do not add one.

90 rather than 80 because at 80 rustfmt splits an ordinary `write!` call across
four lines, which is worse than the long line it replaced.  90 breaks the
genuinely long function signatures without that side effect.

### Comments are not formatted

**rustfmt on stable does not reflow comments at all** — `wrap_comments` and
`comment_width` are nightly-only.  In practice this is where most over-long lines
come from: measured on this directory, 30 of 33 lines exceeding 80 columns were
doc comments, and only 3 were code.

So prose is a manual discipline: **hand-wrap doc comments and `//` comments at 80
columns**, matching the markdown in `.agents/`.  `max_width` will not do it for
you and `cargo fmt` will not flag it.

If you want the tool to do it, nightly can, for this directory only:

```bash
rustfmt +nightly --edition 2021 --unstable-features \
    --config wrap_comments=true,comment_width=80 src/hamiltonian_paths/*.rs
```

That is a deliberate one-off, not part of the normal loop: those options in
`rustfmt.toml` would emit a warning on every `cargo fmt` for anyone on stable.

## Test organisation

Group tests into submodules inside `#[cfg(test)] mod tests`, **one per major
chunk of functionality**, with shared helpers in the parent module and
`use super::*;` at the top of each submodule.

Group by what is being tested, not strictly by type: a piece of logic subtle
enough to carry its own risk deserves its own group even if it is one method of a
larger type.  `segment.rs` is the worked example — `mod segment`,
`mod canonicalize`, `mod display`, `mod decomposition` — where `canonicalize` is
split out from `segment` precisely because everything downstream asserts against
canonical form, so its tests are load-bearing out of proportion to their number.

Name tests without repeating the group.  Inside `mod decomposition` a test is
`new_rejects_overlapping_segments`, not
`decomposition_new_rejects_overlapping_segments` — the module path already says
it, and `cargo test` prints the full path anyway:

```
hamiltonian_paths::segment::tests::decomposition::new_rejects_overlapping_segments
```

The grouping is what makes `cargo test segment::tests::canonicalize` useful.

## Shared test fixtures

A few helpers — `v`, `graph_of`, and the 13-vertex Knuth example
(`knuth_graph`, `knuth_cover_arcs`, `knuth_cover`) — are needed by the test
modules of more than one file.  These live in `testing.rs`, declared in
`mod.rs` as `#[cfg(test)] mod testing;`, with every item `pub(super)` so
sibling modules' `mod tests` can reach them via
`use crate::hamiltonian_paths::testing::{...};`.

`tests/common/` (the usual place for cross-module test helpers in a Cargo
crate) does not work here: these fixtures traffic in `pub(super)` types —
`ArcVars`, `CycleCover` — and an integration-test crate, compiled outside
`taocp`, cannot see anything less than `pub`.  `testing.rs` lives inside the
module tree instead, specifically so it can reach those types.

A helper used by only one module's tests still belongs in that module's own
`mod tests`, per "Test organisation" above.  `testing.rs` is only for
fixtures that are genuinely shared *between* modules — do not move a
single-use helper there, and do not add a fixture nothing uses yet.

## Assertions

Use `claim::{assert_ok, assert_err}` for constructors and other functions
returning `Result`; `claim` is already a dev-dependency.  Assert on the specific
error variant, not merely that an error occurred — several of the types here have
error enums whose variants are easy to confuse, and a test that accepts any error
will not notice when the wrong one is returned.

## Warnings

`mod.rs` carries `#![allow(dead_code)]` while the skeleton is incomplete; it is
removed in phase 8.  Until then, a phase must still introduce **no new warnings**
of its own.  Two pre-existing warnings in the unrelated `sat_solve` and
`pentominoes_box` binaries are not yours and should be left alone.

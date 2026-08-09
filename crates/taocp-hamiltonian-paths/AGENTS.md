# Hamiltonian Paths — Working Conventions

How to work in this crate.  These conventions are **local and authoritative**:
the other crates in this workspace implement algorithms from different sections
of TAOCP and were written at different times, so do not infer conventions from
them.  Where a neighbouring crate does something differently, that is not a
precedent — follow this file.

## Where things are

| File | What it is for |
|---|---|
| `.agents/overview.md` | The design decisions, and — more importantly — the alternatives that were considered and **rejected**, with reasons.  Read before changing an interface. |
| `.agents/algorithm.md` | Knuth's Algorithm C (Fascicle 8a), transcribed, with implementation notes where the transcription needed adapting. |
| `.agents/original_plan.md` | The phased work order the crate was first built to, with the fixtures and expected values each step was checked against.  A **historical record, not a plan to follow** — new work is scoped on its own terms, not fitted into a numbered phase. |
| `AGENTS.md` | This file: how to write the code, as opposed to what to write. |

## Formatting

**Run rustfmt.**  Every file here is expected to be rustfmt-clean, and a change
is not complete until it is:

```bash
cargo fmt -p taocp-hamiltonian-paths
```

Add `--check` to report without rewriting.  This is not a matter of taste —
unformatted code makes every subsequent diff noisier than the change it carries.

Use that per-package command rather than `cargo fmt --all` **for now**.  A
number of files in the other three crates predate the width setting below and
are not yet clean under it; `--all` would reformat them and drag unrelated churn
into whatever you are working on.  That reformat is planned as its own separate
change — the workspace split makes it doable one crate at a time — and once it
lands, `cargo fmt --all` becomes safe and this note should go.

### Line width

`rustfmt.toml` at the **workspace root** sets `max_width = 90`, against
rustfmt's default of 100.

It is at the root deliberately.  This crate is a crate root, so a `rustfmt.toml`
*here* would be honoured — that is exactly why not to add one.  All four crates
want the same width, and a single file at the workspace root says that once
instead of four times, with no way for the copies to drift apart.

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

If you want the tool to do it, nightly can, for this crate only:

```bash
rustfmt +nightly --edition 2021 --unstable-features \
    --config wrap_comments=true,comment_width=80 src/*.rs
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
segment::tests::decomposition::new_rejects_overlapping_segments
```

The grouping is what makes `cargo test segment::tests::canonicalize` useful.

## Shared test fixtures

A few helpers — `v`, `graph_of`, and the 13-vertex Knuth example
(`knuth_graph`, `knuth_cover_arcs`, `knuth_cover`) — are needed by the test
modules of more than one file.  These live in `testing.rs`, declared in
`lib.rs` as `#[cfg(test)] mod testing;`, with every item `pub(crate)` so
sibling modules' `mod tests` can reach them via
`use crate::testing::{...};`.

`tests/common/` (the usual place for cross-module test helpers in a Cargo
crate) does not work here: these fixtures traffic in `pub(crate)` types —
`ArcVars`, `CycleCover` — and an integration-test crate, compiled outside
`taocp-hamiltonian-paths`, cannot see anything less than `pub`.  `testing.rs`
lives inside the module tree instead, specifically so it can reach those types.

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

There is no crate-level `#![allow(dead_code)]`; the `#[allow]` attributes that
remain are targeted at one item each and carry a comment saying why it is
there.  A change must introduce **no new warnings** of its own.  `cargo build
--workspace --all-targets` is warning-clean, so any build warning you see is
one you added.

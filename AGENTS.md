# taocp — Repository Conventions

A Cargo workspace of four member crates.  Each implements algorithms from a
different section of Knuth's *The Art of Computer Programming*.  The crates
share no code and are independent of one another.

This file holds only what is true **repo-wide**.  How to write code is a
per-crate matter — see "Per-crate conventions" below.

## Version control

**This repository is managed with [jj](https://jj-vcs.github.io/) (0.44.0),
colocated with git.**  Use jj for every version-control operation; do not run
`git add` or `git commit`, which in a colocated repo creates state that jj then
has to reconcile.  Read-only git commands (`git diff`, `git log`) are harmless,
but the jj equivalents are usually what you want.

- **There is no staging area and no `git mv`.**  jj snapshots the working copy
  automatically, so a plain `mv` is the whole operation.  Nothing records a
  rename — jj, like git, detects them by content when producing a diff — so
  there is no history to preserve by using a special command.  A plain `rm` is
  likewise the whole of a deletion.
- **`jj commit -m "…"` finishes the current change and starts a new empty one
  on top.**  That is the whole per-commit workflow; there is nothing to add
  first.
- `jj status`, `jj diff` and `jj log` inspect state.  If a step goes wrong,
  `jj undo` reverses the last operation — prefer it to hand-repairing the tree.

## Formatting

`rustfmt.toml` at the workspace root sets `max_width = 90` for every crate.  It
is at the root deliberately: all four crates want the same width, and one file
says that once instead of four times with no way for the copies to drift apart.

Prefer the per-package form, `cargo fmt -p <crate>`, over `cargo fmt --all`.
Several files in the other crates predate the width setting and are not yet
clean under it, so `--all` drags unrelated churn into whatever you are working
on.  Reformatting those is planned as its own change, one crate at a time.

## Per-crate conventions

Each crate's own `AGENTS.md` is **authoritative for that crate**, and overrides
anything here that conflicts.  The crates cover different sections of the books
and were written at different times, so do not infer one crate's conventions
from another: what a neighbouring crate does is not a precedent.

| Crate | Conventions |
|---|---|
| `crates/taocp-hamiltonian-paths` | `AGENTS.md`, plus design docs in `.agents/` |
| `crates/taocp-sat` | `AGENTS.md` |
| `crates/taocp-backtracking` | none recorded |
| `crates/taocp-basic-combinations` | none recorded |

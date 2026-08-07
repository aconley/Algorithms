# SAT Solver — Agent Reference

This directory implements SAT solvers following Knuth TAOCP Vol. 4B §7.2.2.2,
written in Rust.  This file explains the shared representations and conventions
so that future agents can implement new algorithms without re-reading every
existing file.

---

## Literal encoding

Variables are 1-indexed: x_1 … x_n.

| Concept    | Encoding        | Example              |
|------------|-----------------|----------------------|
| x_i        | `2 * i`         | x_1 → 2, x_3 → 6    |
| ¬x_i       | `2 * i + 1`     | ¬x_1 → 3, ¬x_3 → 7  |

Literals 0 and 1 are invalid and rejected by the constructors.

The complement of any literal `l` is `l ^ 1`.
The variable index for literal `l` is `l / 2`.

---

## Core types (`sat_problem.rs`)

All types are re-exported from `mod.rs`, so consumers just `use super::SatProblem` etc.

### `Clause`

```rust
pub struct Clause { literals: Box<[u32]> }
```

- Created with `Clause::new(&[u32]) -> Result<Self, ClauseError>`.
  Rejects invalid literals (< 2) and duplicates.  Stores literals **sorted ascending**.
- `clause.literals() -> &[u32]`  — the sorted literal slice.
- `clause.len() -> usize`, `clause.is_empty() -> bool`.
- `Display` / `Debug` both print `{1̅ 2 3}` style (Unicode combining overline U+0305
  on each digit of the variable number for negated literals).
- Derives `PartialEq`, `Eq`, `Hash`, `Clone`.

### `SatProblem`

```rust
pub struct SatProblem { clauses: Vec<Clause> }
```

Three constructors:
- `SatProblem::new(Vec<Clause>)` — takes ownership.
- `SatProblem::from_clauses(&[Clause])` — clones each clause.
- `SatProblem::from_literals(&[Vec<u32>]) -> Result<Self, SatProblemError>` — builds
  clauses from raw literal vecs; propagates `ClauseError` as `SatProblemError`.

If any clause is empty the problem collapses to a single empty clause (immediately
unsatisfiable); this is handled inside every constructor so algorithm code never
needs to re-check.

Accessors: `clauses() -> &[Clause]`, `clause_count() -> usize`.

`display_latex()` renders a LaTeX formula string.

---

## Implementing a new algorithm

Follow the pattern established in `backtracking.rs`:

### 1. Create a new file, e.g. `lazy_backtracking.rs`

### 2. Define a private state enum/struct

```rust
use super::SatProblem;

struct MyAlgorithmData { /* algorithm-specific fields */ }

enum MyAlgorithm {
    Trivial,        // no clauses → always SAT
    Unsatisfiable,  // contains an empty clause → immediately UNSAT
    Active(MyAlgorithmData),
}
```

### 3. Implement `new` (reads from `SatProblem`)

```rust
impl MyAlgorithm {
    fn new(problem: &SatProblem) -> Self {
        let clauses = problem.clauses();
        let m = clauses.len();
        if m == 0 { return Self::Trivial; }
        if clauses.iter().any(|c| c.is_empty()) { return Self::Unsatisfiable; }

        // Compute n = max variable index across all literals.
        let n = clauses.iter()
            .flat_map(|c| c.literals())
            .map(|&l| l / 2)
            .max()
            .unwrap_or(0);

        // Build data structure, iterate clauses via:
        //   clauses[i].literals()  → &[u32] of sorted literal values
        //   clauses[i].len()       → number of literals in clause i

        Self::Active(MyAlgorithmData { /* ... */ })
    }
}
```

### 4. Implement `solve` (self-consuming, returns `Option<Vec<bool>>`)

```rust
impl MyAlgorithm {
    fn solve(self) -> Option<Vec<bool>> {
        match self {
            Self::Trivial        => Some(vec![]),
            Self::Unsatisfiable  => None,
            Self::Active(data)   => data.solve(),
        }
    }
}
```

The returned `Vec<bool>` is **0-indexed**: `assignment[i]` is the value of x_{i+1}.
Return `Some(vec![false; n])` or any valid assignment for the trivial case; the
caller only checks `is_some()`.

### 5. Expose a public wrapper function

```rust
pub fn solve_via_my_algorithm(problem: &SatProblem) -> Option<Vec<bool>> {
    MyAlgorithm::new(problem).solve()
}
```

### 6. Register the module in `mod.rs`

Add to `src/sat/mod.rs`:

```rust
pub mod my_algorithm;
pub use my_algorithm::solve_via_my_algorithm;
```

---

## Iterating over a SatProblem inside `new`

Clause indices in TAOCP algorithms are 1-based.  A typical initialisation loop:

```rust
let clauses = problem.clauses(); // &[Clause], 0-indexed
let m = clauses.len();

for i in 1..=m {
    let clause = &clauses[i - 1];
    let size = clause.len();
    for &lit in clause.literals() {
        // lit is already a u32 in the 2i / 2i+1 encoding
    }
}
```

Algorithms may store clause cells in **reverse clause order**
(clause m at lower cell indices, clause 1 at higher), so that iterating
`p` downward from `total_cells - 1` processes clause 1 first.  See
`backtracking.rs:new` for the pattern.

---

## Testing conventions

- Use `claim::{assert_ok, assert_err}` for `Result`-returning constructors.
- A helper like `fn make(clauses: &[Vec<u32>]) -> SatProblem` (calls
  `SatProblem::from_literals(...).unwrap()`) is idiomatic for test setup.
- Tests verify the internal data structure via `match MyAlgorithm::new(&p)`
  before testing `solve()`.
- The canonical test instance is "R'" — a 4-variable, 7-clause problem whose
  expected solution from Algorithm A is `[false, true, false, true]`.  Its
  literal encoding appears in both `backtracking.rs` and `lazy.md`.
- Adding the clause `vec![2, 5, 9]` (x_1 ∨ ¬x_2 ∨ ¬x_4) to R' makes it
  unsatisfiable, providing a standard UNSAT test case.

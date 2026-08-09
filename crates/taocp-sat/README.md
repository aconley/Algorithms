This is a repository for playing with basic SAT implementations,
following Knuth Volume 4B 7.2.2.2.

## SAT CLI

You can run sample SAT problems from the command line with the `sat_solve` binary.

### Usage

```bash
cargo run --bin sat_solve -- <solver> <problem> <problem-params>
```

Available solvers:

- `backtracking`
- `lazy_backtracking`

Available problems:

- `langford <n>`
- `waerden <j> <k> <n>`

### Examples

Satisfiable Waerden instance:

```bash
cargo run --quiet --bin sat_solve -- backtracking waerden 3 3 8
```

Example output:

```text
Solution found.
Waerden bitstring: 01011010
```

Unsatisfiable Waerden instance:

```bash
cargo run --quiet --bin sat_solve -- lazy_backtracking waerden 3 3 9
```

Example output:

```text
No solution exists.
```

Satisfiable Langford instance:

```bash
cargo run --quiet --bin sat_solve -- backtracking langford 4
```

Example output:

```text
Solution found.
Langford arrangement: [2 3 4 2 1 3 1 4]
```

Unsatisfiable Langford instance:

```bash
cargo run --quiet --bin sat_solve -- lazy_backtracking langford 5
```

Example output:

```text
No solution exists.
```

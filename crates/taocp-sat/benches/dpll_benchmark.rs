#[macro_use]
extern crate criterion;
extern crate taocp_sat;

use criterion::{BenchmarkId, Criterion};
use taocp_sat::{langford, waerden, SatProblem};
use taocp_sat::dpll_alternatives::dpll_claude::solve_via_dpll as solve_via_dpll_claude;
use taocp_sat::dpll_alternatives::dpll_codex::solve_via_dpll_codex;
use taocp_sat::dpll_alternatives::dpll_gemini::solve_via_dpll as solve_via_dpll_gemini;

fn r_prime() -> SatProblem {
    SatProblem::from_literals(&[
        vec![2, 4, 7],
        vec![4, 6, 9],
        vec![2, 6, 8],
        vec![3, 4, 8],
        vec![3, 5, 6],
        vec![5, 7, 8],
        vec![3, 7, 9],
    ])
    .unwrap()
}

fn r_prime_unsat() -> SatProblem {
    SatProblem::from_literals(&[
        vec![2, 4, 7],
        vec![4, 6, 9],
        vec![2, 6, 8],
        vec![3, 4, 8],
        vec![3, 5, 6],
        vec![5, 7, 8],
        vec![3, 7, 9],
        vec![2, 5, 9],
    ])
    .unwrap()
}

type Solver = fn(&SatProblem) -> Option<Vec<bool>>;

const SOLVERS: &[(&str, Solver)] = &[
    ("claude", solve_via_dpll_claude),
    ("codex", solve_via_dpll_codex),
    ("gemini", solve_via_dpll_gemini),
];

fn bench_r_prime(c: &mut Criterion) {
    let problem = r_prime();
    let mut group = c.benchmark_group("r_prime_sat");
    for (name, solver) in SOLVERS {
        group.bench_with_input(BenchmarkId::new("dpll", name), &problem, |b, p| {
            b.iter(|| solver(p))
        });
    }
    group.finish();
}

fn bench_r_prime_unsat(c: &mut Criterion) {
    let problem = r_prime_unsat();
    let mut group = c.benchmark_group("r_prime_unsat");
    for (name, solver) in SOLVERS {
        group.bench_with_input(BenchmarkId::new("dpll", name), &problem, |b, p| {
            b.iter(|| solver(p))
        });
    }
    group.finish();
}

fn bench_waerden(c: &mut Criterion) {
    let cases: &[(u8, u8, u8, &str)] = &[
        (3, 3, 8, "w(3,3,8)_sat"),
        (3, 3, 9, "w(3,3,9)_unsat"),
        (3, 3, 12, "w(3,3,12)_unsat"),
    ];

    for (j, k, n, label) in cases {
        let problem = waerden(*j, *k, *n).unwrap();
        let mut group = c.benchmark_group(*label);
        for (name, solver) in SOLVERS {
            group.bench_with_input(BenchmarkId::new("dpll", name), &problem, |b, p| {
                b.iter(|| solver(p))
            });
        }
        group.finish();
    }
}

fn bench_langford(c: &mut Criterion) {
    // langford(n) is SAT when n ≡ 0 or 3 (mod 4)
    let cases: &[(u8, &str)] = &[
        (3, "langford(3)_sat"),
        (4, "langford(4)_sat"),
        (5, "langford(5)_unsat"),
    ];

    for (n, label) in cases {
        let problem = langford(*n).unwrap();
        let mut group = c.benchmark_group(*label);
        for (name, solver) in SOLVERS {
            group.bench_with_input(BenchmarkId::new("dpll", name), &problem, |b, p| {
                b.iter(|| solver(p))
            });
        }
        group.finish();
    }
}

criterion_group!(
    benches,
    bench_r_prime,
    bench_r_prime_unsat,
    bench_waerden,
    bench_langford,
);
criterion_main!(benches);

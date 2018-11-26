#[macro_use]

extern crate criterion;
extern crate taocp;

use criterion::Criterion;
use taocp::backtracking::nqueens;

fn bench_bitwise(c: &mut Criterion) {
  c.bench_function("Bitwise 10 queens", |b| b.iter(|| bitwise(10)));
}

fn bitwise(n:u8) -> u64 {
  let mut nq = nqueens::BitwiseNQueensSolver::new(n);
  count_n(&mut nq)
}

fn bench_walker(c: &mut Criterion) {
  c.bench_function("Walker 10 queens", |b| b.iter(|| walker(10)));
}

fn walker(n: u8) -> u64 {
  let mut nq = nqueens::WalkerNQueensSolver::new(n);
  count_n(&mut nq)
}

fn count_n(nq: &mut nqueens::NQueensSolver) -> u64 {
  let mut cv = nqueens::CountingVisitor::new();
  nq.visit(&mut cv);
  cv.n_solutions
}

criterion_group!(benches, bench_bitwise, bench_walker);
criterion_main!(benches);

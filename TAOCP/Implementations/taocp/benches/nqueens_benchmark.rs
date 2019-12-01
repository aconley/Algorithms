#[macro_use]

extern crate criterion;
extern crate taocp;

use criterion::Criterion;
use taocp::backtracking::nqueens;

fn bench_bitwise(c: &mut Criterion) {
  c.bench_function("Bitwise 10 queens", |b| b.iter(|| bitwise(10)));
}

fn bitwise(n:u8) -> usize {
  nqueens::NQueensIterator::new(n).count()
}

criterion_group!(benches, bench_bitwise);
criterion_main!(benches);

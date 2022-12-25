#[macro_use]
extern crate criterion;
extern crate taocp;

use criterion::Criterion;
use taocp::backtracking::nqueens;

fn bench_bitwise(c: &mut Criterion) {
    c.bench_function("Bitwise 10 queens", |b| b.iter(|| bitwise(10)));
}

fn bitwise(n: u8) -> usize {
    nqueens::NQueensIterator::new(n).count()
}

fn bench_array(c: &mut Criterion) {
    c.bench_function("Array based 10 queens", |b| b.iter(|| array(10)));
}

fn array(n: u8) -> usize {
    nqueens::NQueensIteratorAlt::new(n).count()
}

criterion_group!(benches, bench_array, bench_bitwise);
criterion_main!(benches);

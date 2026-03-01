#[macro_use]

extern crate criterion;
extern crate taocp;

use criterion::Criterion;
use taocp::basic_combinations::combinations;

fn combinations_basic_benchmark(c: &mut Criterion) {
  let mut cv = combinations::CountingVisitor::new();
  c.bench_function("Basic choose 20 10", 
    |b| b.iter(|| combinations::basic_generate(20, 10, &mut cv)));
}

fn combinations_basic_benchmark_large_t(c: &mut Criterion) {
  let mut cv = combinations::CountingVisitor::new();
  c.bench_function("Basic choose 20 18", 
    |b| b.iter(|| combinations::basic_generate(20, 18, &mut cv)));
}

fn combinations_benchmark(c: &mut Criterion) {
  let mut cv = combinations::CountingVisitor::new();
  c.bench_function("Optimized choose 20 10", 
    |b| b.iter(|| combinations::combinations(20, 10, &mut cv)));
}

fn combinations_benchmark_large_t(c: &mut Criterion) {
  let mut cv = combinations::CountingVisitor::new();
  c.bench_function("Optimized choose 20 18", 
    |b| b.iter(|| combinations::combinations(20, 18, &mut cv)));
}

criterion_group!(benches, 
  combinations_basic_benchmark, 
  combinations_benchmark,
  combinations_basic_benchmark_large_t,
  combinations_benchmark_large_t);
criterion_main!(benches);
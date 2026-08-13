#[macro_use]
extern crate criterion;
extern crate taocp_basic_combinations;

use criterion::Criterion;
use lending_iterator::prelude::*;
use taocp_basic_combinations::combinations;

fn combinations_basic_benchmark(c: &mut Criterion) {
    let mut cv = combinations::CountingVisitor::new();
    c.bench_function("Visitor choose 20 10", |b| {
        b.iter(|| combinations::basic_generate(20, 10, &mut cv))
    });
}

fn combinations_basic_benchmark_large_t(c: &mut Criterion) {
    let mut cv = combinations::CountingVisitor::new();
    c.bench_function("Visitor choose 20 18", |b| {
        b.iter(|| combinations::basic_generate(20, 18, &mut cv))
    });
}

fn combinations_benchmark(c: &mut Criterion) {
    let mut cv = combinations::CountingVisitor::new();
    c.bench_function("Visitor optimized choose 20 10", |b| {
        b.iter(|| combinations::combinations(20, 10, &mut cv))
    });
}

fn combinations_benchmark_large_t(c: &mut Criterion) {
    let mut cv = combinations::CountingVisitor::new();
    c.bench_function("Visitor optimized choose 20 18", |b| {
        b.iter(|| combinations::combinations(20, 18, &mut cv))
    });
}

fn basic_generate_iter_benchmark(c: &mut Criterion) {
    c.bench_function("Iter choose 20 10", |b| {
        b.iter(|| combinations::BasicGenerateIter::new(20, 10).count())
    });
}

fn basic_generate_iter_benchmark_large_t(c: &mut Criterion) {
    c.bench_function("Iter choose 20 18", |b| {
        b.iter(|| combinations::BasicGenerateIter::new(20, 18).count())
    });
}

fn combinations_iter_benchmark(c: &mut Criterion) {
    c.bench_function("Iter optimized choose 20 10", |b| {
        b.iter(|| combinations::CombinationsIter::new(20, 10).count())
    });
}

fn combinations_iter_benchmark_large_t(c: &mut Criterion) {
    c.bench_function("Iter optimized choose 20 18", |b| {
        b.iter(|| combinations::CombinationsIter::new(20, 18).count())
    });
}

fn basic_generate_lending_benchmark(c: &mut Criterion) {
    c.bench_function("Lending choose 20 10", |b| {
        b.iter(|| combinations::BasicGenerateLendingIter::new(20, 10).count())
    });
}

fn basic_generate_lending_benchmark_large_t(c: &mut Criterion) {
    c.bench_function("Lending choose 20 18", |b| {
        b.iter(|| combinations::BasicGenerateLendingIter::new(20, 18).count())
    });
}

fn combinations_lending_benchmark(c: &mut Criterion) {
    c.bench_function("Lending optimized choose 20 10", |b| {
        b.iter(|| combinations::CombinationsLendingIter::new(20, 10).count())
    });
}

fn combinations_lending_benchmark_large_t(c: &mut Criterion) {
    c.bench_function("Lending optimized choose 20 18", |b| {
        b.iter(|| combinations::CombinationsLendingIter::new(20, 18).count())
    });
}

criterion_group!(
    benches,
    combinations_basic_benchmark,
    combinations_benchmark,
    combinations_basic_benchmark_large_t,
    combinations_benchmark_large_t,
    basic_generate_iter_benchmark,
    basic_generate_iter_benchmark_large_t,
    combinations_iter_benchmark,
    combinations_iter_benchmark_large_t,
    basic_generate_lending_benchmark,
    basic_generate_lending_benchmark_large_t,
    combinations_lending_benchmark,
    combinations_lending_benchmark_large_t,
);
criterion_main!(benches);

#[macro_use]
extern crate criterion;
extern crate taocp_basic_combinations;

use criterion::Criterion;
use lending_iterator::prelude::*;
use taocp_basic_combinations::subsets::{
    SubsetsIterator, SubsetsLendingIter, SubsetsRefLendingIter,
};

// 64-byte heap strings: an element type whose clone is a real allocation.
fn strings(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("{i:064}")).collect()
}

fn subsets_iter_benchmark(c: &mut Criterion) {
    let v: Vec<u32> = (0..16).collect();
    c.bench_function("Iter subsets 16", |b| {
        b.iter(|| SubsetsIterator::new(&v).count())
    });
}

fn subsets_iter_benchmark_large(c: &mut Criterion) {
    let v: Vec<u32> = (0..20).collect();
    c.bench_function("Iter subsets 20", |b| {
        b.iter(|| SubsetsIterator::new(&v).count())
    });
}

fn subsets_lending_benchmark(c: &mut Criterion) {
    let v: Vec<u32> = (0..16).collect();
    c.bench_function("Lending subsets 16", |b| {
        b.iter(|| SubsetsLendingIter::new(&v).count())
    });
}

fn subsets_lending_benchmark_large(c: &mut Criterion) {
    let v: Vec<u32> = (0..20).collect();
    c.bench_function("Lending subsets 20", |b| {
        b.iter(|| SubsetsLendingIter::new(&v).count())
    });
}

fn subsets_ref_lending_benchmark(c: &mut Criterion) {
    let v: Vec<u32> = (0..16).collect();
    c.bench_function("RefLending subsets 16", |b| {
        b.iter(|| SubsetsRefLendingIter::new(&v).count())
    });
}

fn subsets_ref_lending_benchmark_large(c: &mut Criterion) {
    let v: Vec<u32> = (0..20).collect();
    c.bench_function("RefLending subsets 20", |b| {
        b.iter(|| SubsetsRefLendingIter::new(&v).count())
    });
}

fn subsets_string_iter_benchmark(c: &mut Criterion) {
    let v = strings(16);
    c.bench_function("Iter subsets String 16", |b| {
        b.iter(|| SubsetsIterator::new(&v).count())
    });
}

fn subsets_string_lending_benchmark(c: &mut Criterion) {
    let v = strings(16);
    c.bench_function("Lending subsets String 16", |b| {
        b.iter(|| SubsetsLendingIter::new(&v).count())
    });
}

fn subsets_string_ref_lending_benchmark(c: &mut Criterion) {
    let v = strings(16);
    c.bench_function("RefLending subsets String 16", |b| {
        b.iter(|| SubsetsRefLendingIter::new(&v).count())
    });
}

criterion_group!(
    benches,
    subsets_iter_benchmark,
    subsets_iter_benchmark_large,
    subsets_lending_benchmark,
    subsets_lending_benchmark_large,
    subsets_ref_lending_benchmark,
    subsets_ref_lending_benchmark_large,
    subsets_string_iter_benchmark,
    subsets_string_lending_benchmark,
    subsets_string_ref_lending_benchmark,
);
criterion_main!(benches);

#[macro_use]
extern crate criterion;
extern crate taocp_basic_combinations;

use criterion::Criterion;
use lending_iterator::prelude::*;
use taocp_basic_combinations::permutations::{
    PermutationsIterator, PermutationsLendingIter,
};

fn permutations_iter_benchmark(c: &mut Criterion) {
    let v: Vec<u32> = (0..8).collect();
    c.bench_function("Iter permutations 8", |b| {
        b.iter(|| PermutationsIterator::new(&v).unwrap().count())
    });
}

fn permutations_iter_benchmark_large(c: &mut Criterion) {
    let v: Vec<u32> = (0..9).collect();
    c.bench_function("Iter permutations 9", |b| {
        b.iter(|| PermutationsIterator::new(&v).unwrap().count())
    });
}

fn permutations_lending_benchmark(c: &mut Criterion) {
    let v: Vec<u32> = (0..8).collect();
    c.bench_function("Lending permutations 8", |b| {
        b.iter(|| PermutationsLendingIter::new(&v).unwrap().count())
    });
}

fn permutations_lending_benchmark_large(c: &mut Criterion) {
    let v: Vec<u32> = (0..9).collect();
    c.bench_function("Lending permutations 9", |b| {
        b.iter(|| PermutationsLendingIter::new(&v).unwrap().count())
    });
}

criterion_group!(
    benches,
    permutations_iter_benchmark,
    permutations_iter_benchmark_large,
    permutations_lending_benchmark,
    permutations_lending_benchmark_large,
);
criterion_main!(benches);

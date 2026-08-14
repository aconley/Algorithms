#[macro_use]
extern crate criterion;
extern crate taocp_basic_combinations;

use criterion::Criterion;
use lending_iterator::prelude::*;
use taocp_basic_combinations::partitions::{
    IntegerPartitions, IntegerPartitionsIntoParts, IntegerPartitionsIntoPartsLending,
    IntegerPartitionsLending,
};

fn partitions_iter_benchmark(c: &mut Criterion) {
    c.bench_function("Iter partitions 30", |b| {
        b.iter(|| IntegerPartitions::new(30).count())
    });
}

fn partitions_iter_benchmark_large(c: &mut Criterion) {
    c.bench_function("Iter partitions 40", |b| {
        b.iter(|| IntegerPartitions::new(40).count())
    });
}

fn partitions_lending_benchmark(c: &mut Criterion) {
    c.bench_function("Lending partitions 30", |b| {
        b.iter(|| IntegerPartitionsLending::new(30).count())
    });
}

fn partitions_lending_benchmark_large(c: &mut Criterion) {
    c.bench_function("Lending partitions 40", |b| {
        b.iter(|| IntegerPartitionsLending::new(40).count())
    });
}

fn partitions_into_parts_iter_benchmark(c: &mut Criterion) {
    c.bench_function("Iter partitions into parts 30 10", |b| {
        b.iter(|| IntegerPartitionsIntoParts::new(30, 10).count())
    });
}

fn partitions_into_parts_iter_benchmark_large(c: &mut Criterion) {
    c.bench_function("Iter partitions into parts 40 10", |b| {
        b.iter(|| IntegerPartitionsIntoParts::new(40, 10).count())
    });
}

fn partitions_into_parts_lending_benchmark(c: &mut Criterion) {
    c.bench_function("Lending partitions into parts 30 10", |b| {
        b.iter(|| {
            IntegerPartitionsIntoPartsLending::new(30, 10)
                .unwrap()
                .count()
        })
    });
}

fn partitions_into_parts_lending_benchmark_large(c: &mut Criterion) {
    c.bench_function("Lending partitions into parts 40 10", |b| {
        b.iter(|| {
            IntegerPartitionsIntoPartsLending::new(40, 10)
                .unwrap()
                .count()
        })
    });
}

criterion_group!(
    benches,
    partitions_iter_benchmark,
    partitions_iter_benchmark_large,
    partitions_lending_benchmark,
    partitions_lending_benchmark_large,
    partitions_into_parts_iter_benchmark,
    partitions_into_parts_iter_benchmark_large,
    partitions_into_parts_lending_benchmark,
    partitions_into_parts_lending_benchmark_large,
);
criterion_main!(benches);

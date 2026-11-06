#[macro_use]
extern crate criterion;
extern crate taocp_basic_combinations;

use std::hint::black_box;

use criterion::Criterion;
use lending_iterator::prelude::*;
use taocp_basic_combinations::set_partitions::{
    SetPartitionIndices, SetPartitionIndicesLending, SetPartitions, SetPartitionsLending,
};

// The partition benchmarks all do the same small piece of work per partition
// (sum of squared block sizes, read through the blocks) so that building the
// blocks cannot be optimized away.

fn indices_iter_benchmark(c: &mut Criterion) {
    c.bench_function("Iter set partition indices 11", |b| {
        b.iter(|| SetPartitionIndices::new(11).count())
    });
}

fn indices_lending_benchmark(c: &mut Criterion) {
    c.bench_function("Lending set partition indices 11", |b| {
        b.iter(|| SetPartitionIndicesLending::new(11).count())
    });
}

fn partitions_iter_benchmark(c: &mut Criterion) {
    let v: Vec<u32> = (0..10).collect();
    c.bench_function("Iter set partitions 10", |b| {
        b.iter(|| {
            let mut total = 0;
            for p in SetPartitions::new(&v) {
                total += p.iter().map(|blk| blk.len() * blk.len()).sum::<usize>();
            }
            black_box(total)
        })
    });
}

fn partitions_lending_benchmark(c: &mut Criterion) {
    let v: Vec<u32> = (0..10).collect();
    c.bench_function("Lending set partitions 10", |b| {
        b.iter(|| {
            let mut total = 0;
            let mut it = SetPartitionsLending::new(&v);
            while let Some(p) = it.next() {
                total += p.blocks().map(|blk| blk.len() * blk.len()).sum::<usize>();
            }
            black_box(total)
        })
    });
}

fn partitions_iter_benchmark_large(c: &mut Criterion) {
    let v: Vec<u32> = (0..11).collect();
    c.bench_function("Iter set partitions 11", |b| {
        b.iter(|| {
            let mut total = 0;
            for p in SetPartitions::new(&v) {
                total += p.iter().map(|blk| blk.len() * blk.len()).sum::<usize>();
            }
            black_box(total)
        })
    });
}

fn partitions_lending_benchmark_large(c: &mut Criterion) {
    let v: Vec<u32> = (0..11).collect();
    c.bench_function("Lending set partitions 11", |b| {
        b.iter(|| {
            let mut total = 0;
            let mut it = SetPartitionsLending::new(&v);
            while let Some(p) = it.next() {
                total += p.blocks().map(|blk| blk.len() * blk.len()).sum::<usize>();
            }
            black_box(total)
        })
    });
}

criterion_group!(
    benches,
    indices_iter_benchmark,
    indices_lending_benchmark,
    partitions_iter_benchmark,
    partitions_lending_benchmark,
    partitions_iter_benchmark_large,
    partitions_lending_benchmark_large,
);
criterion_main!(benches);

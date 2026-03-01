#[macro_use]
extern crate criterion;
extern crate taocp;

use criterion::Criterion;
use std::time::Duration;
use taocp::backtracking::dancing_langford::DancingLangfordIterator;
use taocp::backtracking::dancing_queens::DancingQueensIterator;
use taocp::backtracking::dancing_sudoku::{DancingSudokuIterator, SudokuEntry};

// Benchmarks for Dancing Links

// Sudoku benchmarks.
fn create_sudoku_iterator(pos: &[u8; 81]) -> DancingSudokuIterator {
    let initial_position = SudokuEntry::create_from_values(pos);
    DancingSudokuIterator::new(initial_position).unwrap()
}

#[rustfmt::skip]
const FULLY_SOLVED_SUDOKU: [u8; 81] = [
  5, 3, 4, 6, 7, 8, 9, 1, 2, 
  6, 7, 2, 1, 9, 5, 3, 4, 8, 
  1, 9, 8, 3, 4, 2, 5, 6, 7, 
  8, 5, 9, 7, 6, 1, 4, 2, 3, 
  4, 2, 6, 8, 5, 3, 7, 9, 1, 
  7, 1, 3, 9, 2, 4, 8, 5, 6, 
  9, 6, 1, 5, 3, 7, 2, 8, 4, 
  2, 8, 7, 4, 1, 9, 6, 3, 5, 
  3, 4, 5, 2, 8, 6, 1, 7, 9,
];

fn bench_already_solved_sudoku(c: &mut Criterion) {
    c.bench_function("DancingLinks/Sudoku: Already solved problem", |b| {
        b.iter(|| create_sudoku_iterator(&FULLY_SOLVED_SUDOKU).count())
    });
}

#[rustfmt::skip]
const MEDIUM_SUDOKU : [u8; 81] = [
  0, 2, 0, 0, 6, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 1, 9, 5, 2,
  9, 0, 0, 8, 5, 2, 4, 7, 0,
  0, 0, 6, 4, 0, 0, 0, 0, 9,
  0, 0, 0, 0, 2, 0, 8, 0, 0,
  1, 0, 0, 0, 0, 8, 3, 6, 7,
  0, 0, 9, 7, 3, 0, 6, 0, 0,
  7, 0, 0, 0, 0, 0, 5, 9, 0,
  0, 0, 0, 6, 8, 9, 7, 0, 4
];

fn bench_medium_sudoku(c: &mut Criterion) {
    c.bench_function("DancingLinks/Sudoku: Medium problem", |b| {
        b.iter(|| create_sudoku_iterator(&MEDIUM_SUDOKU).count())
    });
}

#[rustfmt::skip]
const HARD_SUDOKU : [u8; 81] = [
  4, 0, 0, 0, 0, 0, 0, 1, 0,
  0, 0, 0, 4, 0, 2, 3, 0, 0,
  8, 3, 6, 0, 1, 0, 0, 0, 0,
  2, 0, 0, 0, 6, 0, 0, 5, 7,
  0, 9, 0, 5, 0, 0, 6, 0, 1,
  0, 0, 7, 1, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 8, 6, 0, 0, 3,
  7, 0, 0, 0, 0, 0, 0, 0, 0,
  6, 4, 0, 0, 7, 0, 0, 0, 2
];

fn bench_hard_sudoku(c: &mut Criterion) {
    c.bench_function("DancingLinks/Sudoku: Hard problem", |b| {
        b.iter(|| create_sudoku_iterator(&HARD_SUDOKU).count())
    });
}

#[rustfmt::skip]
const VERY_HARD_SUDOKU : [u8; 81] = [
  1, 2, 0, 3, 0, 0, 4, 0, 0,
  4, 0, 0, 1, 0, 0, 0, 0, 0,
  0, 0, 5, 0, 6, 0, 0, 0, 0,
  3, 0, 0, 0, 0, 0, 0, 1, 0,
  0, 7, 0, 0, 0, 0, 2, 3, 0,
  0, 0, 0, 0, 0, 0, 6, 0, 8,
  0, 4, 0, 2, 0, 0, 0, 7, 0,
  0, 0, 9, 0, 8, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 5, 0, 0, 6
];

fn bench_very_hard_sudoku(c: &mut Criterion) {
    let mut group = c.benchmark_group("DancingLinks Sudoku: very hard problem");
    group.measurement_time(Duration::from_secs(25));
    group.bench_function("DancingLinks/Sudoku: Very hard problem", |b| {
        b.iter(|| create_sudoku_iterator(&VERY_HARD_SUDOKU).count())
    });
}

#[rustfmt::skip]
const SUDOKU_WITH_MULTIPLE_SOLUTIONS : [u8; 81] = [
  0, 3, 0, 0, 1, 0, 0, 0, 0,
  0, 0, 0, 4, 0, 0, 1, 0, 0,
  0, 5, 0, 0, 0, 0, 0, 9, 0,
  2, 0, 0, 0, 0, 0, 6, 0, 4,
  0, 0, 0, 0, 3, 5, 0, 0, 0,
  1, 0, 0, 0, 0, 0, 0, 0, 0,
  4, 0, 0, 6, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 5, 0,
  0, 9, 0, 0, 0, 0, 0, 0, 0
];

fn bench_sudoku_with_multiple_solutions(c: &mut Criterion) {
    c.bench_function(
        "DancingLinks/Sudoku: Problem with multiple solutions",
        |b| b.iter(|| create_sudoku_iterator(&SUDOKU_WITH_MULTIPLE_SOLUTIONS).count()),
    );
}

criterion_group!(
    sudoku_benches,
    bench_already_solved_sudoku,
    bench_medium_sudoku,
    bench_hard_sudoku,
    bench_very_hard_sudoku,
    bench_sudoku_with_multiple_solutions
);

// Langford pairs.
fn langford7_benchmark(c: &mut Criterion) {
    c.bench_function("DancingLinks/Langford 7", |b| {
        b.iter(|| DancingLangfordIterator::new(7).unwrap().count())
    });
}

fn langford8_benchmark(c: &mut Criterion) {
    c.bench_function("DancingLinks/Langford 8", |b| {
        b.iter(|| DancingLangfordIterator::new(8).unwrap().count())
    });
}

fn langford12_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("DancingLinks Langford 12");
    group.measurement_time(Duration::from_secs(25));
    group.bench_function("DancingLinks/Langford 12", |b| {
        b.iter(|| DancingLangfordIterator::new(12).unwrap().count())
    });
}

criterion_group!(
    langford_benches,
    langford7_benchmark,
    langford8_benchmark,
    langford12_benchmark
);

// N Queens
fn bench_8_queens(c: &mut Criterion) {
    c.bench_function("DancingLinks NQueens=8", |b| {
        b.iter(|| DancingQueensIterator::new(8).unwrap().count())
    });
}

fn bench_10_queens(c: &mut Criterion) {
    c.bench_function("DancingLinks NQueens=10", |b| {
        b.iter(|| DancingQueensIterator::new(10).unwrap().count())
    });
}

fn bench_12_queens(c: &mut Criterion) {
    let mut group = c.benchmark_group("DancingLinks NQueens=12");
    group.measurement_time(Duration::from_secs(5));
    group.bench_function("DancingLinks NQueens=12", |b| {
        b.iter(|| DancingQueensIterator::new(12).unwrap().count())
    });
}

fn bench_13_queens(c: &mut Criterion) {
    let mut group = c.benchmark_group("DancingLinks NQueens=13");
    group.measurement_time(Duration::from_secs(12));
    group.bench_function("DancingLinks NQueens=13", |b| {
        b.iter(|| DancingQueensIterator::new(13).unwrap().count())
    });
}

criterion_group!(
    nqueens_benches,
    bench_8_queens,
    bench_10_queens,
    bench_12_queens,
    bench_13_queens
);

criterion_main!(sudoku_benches, langford_benches, nqueens_benches);

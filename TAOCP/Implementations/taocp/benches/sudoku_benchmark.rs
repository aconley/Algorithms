#[macro_use]
extern crate criterion;
extern crate taocp;

use criterion::Criterion;
use std::time::Duration;
use taocp::backtracking::sudoku::{InitialPosition, SudokuIterator};

fn create_iterator(pos: &[u8; 81]) -> SudokuIterator {
  let initial_position = InitialPosition::create_from_values(pos);
  SudokuIterator::create(initial_position)
}

#[rustfmt::skip]
const FULLY_SOLVED: [u8; 81] = [
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

fn bench_already_solved(c: &mut Criterion) {
  c.bench_function("Sudoku: Already solved problem", |b| {
    b.iter(|| create_iterator(&FULLY_SOLVED).count())
  });
}

#[rustfmt::skip]
const MEDIUM_PROBLEM : [u8; 81] = [
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

fn bench_medium_problem(c: &mut Criterion) {
  c.bench_function("Sudoku: Medium problem", |b| {
    b.iter(|| create_iterator(&MEDIUM_PROBLEM).count())
  });
}

#[rustfmt::skip]
const HARD_PROBLEM : [u8; 81] = [
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

fn bench_hard_problem(c: &mut Criterion) {
  c.bench_function("Sudoku: Hard problem", |b| {
    b.iter(|| create_iterator(&HARD_PROBLEM).count())
  });
}

#[rustfmt::skip]
const VERY_HARD_PROBLEM : [u8; 81] = [
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

fn bench_very_hard_problem(c: &mut Criterion) {
  let mut group = c.benchmark_group("Sudoku: very hard problem");
  group.measurement_time(Duration::from_secs(10));
  group.bench_function("Sudoku: Very hard problem", |b| {
    b.iter(|| create_iterator(&VERY_HARD_PROBLEM).count())
  });
}

#[rustfmt::skip]
const PROBLEM_WITH_MULTIPLE_SOLUTIONS : [u8; 81] = [
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

fn bench_problem_with_multiple_solutions(c: &mut Criterion) {
  c.bench_function("Sudoku: Problem with multiple solutions", |b| {
    b.iter(|| create_iterator(&PROBLEM_WITH_MULTIPLE_SOLUTIONS).count())
  });
}

criterion_group!(
  benches,
  bench_already_solved,
  bench_medium_problem,
  bench_hard_problem,
  bench_very_hard_problem,
  bench_problem_with_multiple_solutions
);
criterion_main!(benches);

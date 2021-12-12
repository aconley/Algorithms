#[macro_use]
extern crate criterion;
extern crate taocp;

use criterion::Criterion;
use taocp::backtracking::langford::LangfordIterator;

fn langford7_benchmark(c: &mut Criterion) {
  c.bench_function("Langford 7", |b| {
    b.iter(|| LangfordIterator::new(7).count())
  });
}

fn langford12_benchmark(c: &mut Criterion) {
  c.bench_function("Langford 12", |b| {
    b.iter(|| LangfordIterator::new(12).count())
  });
}

criterion_group!(benches, langford7_benchmark, langford12_benchmark);
criterion_main!(benches);

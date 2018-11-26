#[macro_use]

extern crate criterion;
extern crate taocp;

use criterion::Criterion;
use taocp::basic_combinations::combinations;

fn basic_generate_benchmark(c: &mut Criterion) {
  let mut cv = combinations::CountingVisitor::new();
  c.bench_function("choose 20 10", 
    |b| b.iter(|| combinations::basic_generate(20, 10, &mut cv)));
}

criterion_group!(benches, basic_generate_benchmark);
criterion_main!(benches);
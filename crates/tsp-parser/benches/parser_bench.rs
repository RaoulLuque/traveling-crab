use criterion::{Criterion, criterion_group, criterion_main};
use tsp_parser::parse_tsp_instance;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Parse \"first.tsp\"", |b| {
        b.iter(|| parse_tsp_instance("../../instances/bench/first.tsp").unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

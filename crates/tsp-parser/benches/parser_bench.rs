use criterion::{Criterion, criterion_group, criterion_main};
use tsp_parser::parse_tsp_instance;

fn a280_parsing_benchmark(c: &mut Criterion) {
    c.bench_function("Parse \"a280.tsp\"", |b| {
        b.iter(|| parse_tsp_instance("../../instances/bench/a280.tsp").unwrap())
    });
}

fn d18512_parsing_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("d18512_parsing");
    group.sample_size(10);
    group.bench_function("Parse \"d18512.tsp\"", |b| {
        b.iter(|| parse_tsp_instance("../../instances/bench/d18512.tsp").unwrap())
    });
    group.finish();
}

criterion_group!(a280, a280_parsing_benchmark);
criterion_group!(d18512, d18512_parsing_benchmark);
criterion_main!(a280, d18512);

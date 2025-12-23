use criterion::{BatchSize::SmallInput, Criterion, criterion_group, criterion_main};
use tsp_core::instance::edge::data::EdgeDataMatrixSym;
use tsp_parser::parse_tsp_instance;
use tsp_solvers::held_karp::{
    EdgeState, fixed_point_arithmetic::ScaledDistance, trees::min_one_tree as min_one_tree_function,
};

fn min_one_tree_benchmark(c: &mut Criterion) {
    let tsp_instance = parse_tsp_instance("../../instances/bench/a280.tsp").unwrap();
    let scaled_distances = EdgeDataMatrixSym {
        dimension: tsp_instance.distances().dimension,
        data: tsp_instance
            .distances()
            .data
            .iter()
            .map(|&d| ScaledDistance::from_distance(d))
            .collect::<Vec<_>>(),
    };
    let edge_states = EdgeDataMatrixSym {
        data: vec![EdgeState::Available; scaled_distances.data.len()],
        dimension: scaled_distances.dimension,
    };
    let node_penalties = vec![ScaledDistance(0); scaled_distances.dimension];

    c.bench_function("Compute min one tree", |b| {
        b.iter_batched_ref(
            || node_penalties.clone(),
            |node_penalties| min_one_tree_function(&scaled_distances, &edge_states, node_penalties),
            SmallInput,
        )
    });
}

criterion_group!(min_one_tree, min_one_tree_benchmark);
criterion_main!(min_one_tree);

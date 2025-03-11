use criterion::{Criterion, criterion_group, criterion_main};
use graph_api_test::bench_suite;
use petgraph::stable_graph::StableGraph;

fn criterion_benchmark(c: &mut Criterion) {
    // Run benchmarks using PetGraph (StableGraph) implementation
    bench_suite!(c, || StableGraph::new());
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(25);
    targets = criterion_benchmark
);
criterion_main!(benches);

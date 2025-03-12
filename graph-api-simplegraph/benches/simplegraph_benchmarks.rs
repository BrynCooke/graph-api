use criterion::{Criterion, criterion_group, criterion_main};
use graph_api_benches::bench_suite;
use graph_api_simplegraph::SimpleGraph;

fn criterion_benchmark(c: &mut Criterion) {
    // Run benchmarks using SimpleGraph implementation
    bench_suite!(c, SimpleGraph::new);
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(25);
    targets = criterion_benchmark
);
criterion_main!(benches);

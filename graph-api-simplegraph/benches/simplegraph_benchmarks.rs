use criterion::{criterion_group, criterion_main, Criterion};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::bench_suite;

fn criterion_benchmark(c: &mut Criterion) {
    // Run benchmarks using SimpleGraph implementation
    bench_suite!(c, || SimpleGraph::new());
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(25);
    targets = criterion_benchmark
);
criterion_main!(benches);

use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(_c: &mut Criterion) {
    // This is just a placeholder. Each graph implementation will need to
    // provide its own benchmark suite that calls the bench_suite macro
    // with the appropriate graph setup function.
    //
    // Example:
    // bench_suite!(c, || MyGraph::new());
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

# Graph-API Benchmarks

**Welcome to graph-api-benches**  the performance measurement toolkit for the Graph-API ecosystem!

This crate provides comprehensive benchmarking utilities to assess and compare the performance of different graph implementations. Whether you're optimizing your own graph implementation or evaluating which implementation best suits your needs, these benchmarks offer valuable insights.

## Benchmark Categories

This suite includes performance tests for all critical graph operations:

* **Construction**: Measure how quickly graphs can be built
* **Insertion**: Evaluate vertex and edge addition performance
* **Traversal**: Time how efficiently graphs can be navigated
* **Query**: Benchmark lookup operations with different index types
* **Mutation**: Test the performance of graph modifications
* **Scale**: Assess how implementations handle graphs of different sizes

## Feature Toggles

Like other Graph-API crates, benchmarks can be selectively enabled based on the features your implementation supports:

```toml
[dev-dependencies]
graph-api-benches = { version = "0.1.0", features = [
    "vertex-hash-index",
    "vertex-range-index",
    "edge-label-index"
]}
```

## Using the Benchmarks

To benchmark your own graph implementation:

1. Create a benchmark file in your project
2. Import the benchmark functions from this crate
3. Create instances of your graph for testing
4. Run the benchmarks with Criterion

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use graph_api_benches::{bench_vertex_insertion, bench_traversal};
use your_crate::YourGraph;

fn vertex_insertion(c: &mut Criterion) {
    bench_vertex_insertion::<YourGraph>(c, "your_graph");
}

fn traversal(c: &mut Criterion) {
    bench_traversal::<YourGraph>(c, "your_graph");
}

criterion_group!(benches, vertex_insertion, traversal);
criterion_main!(benches);
```

## Performance Comparison

The benchmark suite is used to produce comparative metrics between different Graph-API implementations, helping you choose the right implementation for your specific use case.

## Contributions

Have ideas for additional benchmarks? We welcome contributions that help evaluate graph performance for different scenarios and workloads.

Optimize your graph operations with confidence using graph-api-benches!

Learn more in the [graph-api book](https://bryncooke.github.io/graph-api/).
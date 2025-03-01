# Graph API Performance Benchmarks

****This document describes the benchmarking utilities provided by graph-api-test for performance testing of graph implementations.

## Overview

The benchmarking suite is designed to compare different graph implementations across a wide range of operations, from basic vertex/edge manipulation to complex traversals. The benchmarks measure performance in a standardized way, allowing graph implementations to identify bottlenecks and optimize critical paths.

## Setting Up Benchmarks for Your Graph Implementation

### Adding Dependencies

First, ensure your graph implementation has the necessary dependencies in your `Cargo.toml`:

```toml
[dev-dependencies]
graph-api-test = { path = "../graph-api-test" }
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "your_graph_benchmarks"
harness = false
```

### Creating Benchmark Entrypoint

Create a benchmark file at `benches/your_graph_benchmarks.rs` with the following structure:

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use graph_api_test::bench_suite;
use your_graph_crate::YourGraph;

fn criterion_benchmark(c: &mut Criterion) {
    // Run benchmarks using your graph implementation
    bench_suite!(c, || YourGraph::new());
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

## Running Benchmarks

Run benchmarks using:

```bash
cargo bench --package your-graph-crate
```

For specific benchmark categories:

```bash
cargo bench --package your-graph-crate -- vertex_operations
```

## Benchmark Categories

The benchmarking suite tests several key aspects of graph performance:

1. **Vertex Operations** - Adding, retrieving, removing vertices and manipulating their properties
2. **Edge Operations** - Adding, retrieving, removing edges and accessing their properties
3. **Traversal Operations** - Basic and complex graph traversals including filtering and multi-hop traversals
4. **Query Operations** - Count, first, collect and other query operations
5. **Mutation Operations** - Modifications during traversal
6. **Construction** - Building graphs of different sizes and topologies
7. **Index Operations** - Performance of various indexing strategies
8. **Scaling** - Measuring how performance changes as graph size increases

## Data Generation

The benchmarking suite includes several data generators for creating realistic test graphs:

- **Random Graphs** - General-purpose graphs with mixed vertex and edge types
- **Social Graphs** - Person-centric graphs with community structure
- **Project Graphs** - Project dependency graphs

## Customizing Benchmarks

You can customize the benchmark behavior by modifying the benchmark parameters in your implementation:

```rust
fn criterion_benchmark(c: &mut Criterion) {
    // Use a custom builder for your graph setup
    bench_suite!(c, || {
        let mut graph = YourGraph::with_capacity(10_000);
        graph.set_index_strategy(IndexStrategy::HashMapBased);
        graph
    });
}
```

## Interpreting Results

Criterion.rs generates detailed reports including:

- Average execution time
- Throughput measurements
- Statistical significance of differences between benchmark runs
- HTML reports with performance graphs

Look for the generated reports in `target/criterion/` after running benchmarks.

## Best Practices

1. **Run on idle systems** - Ensure minimal background processes are running
2. **Consistent hardware** - Compare benchmarks on the same machine
3. **Multiple runs** - Use criterion's statistical analysis over multiple runs
4. **Profile bottlenecks** - Use the results to identify performance issues 
5. **Compare implementations** - Benchmark multiple graph implementations to identify strengths and weaknesses

## Common Performance Considerations

When optimizing, pay special attention to:

- Index creation and maintenance
- Memory allocation patterns
- Traversal algorithms
- Cache efficiency
- Thread safety mechanisms (if applicable)

By following these guidelines, you can effectively benchmark your graph implementation and identify areas for optimization.
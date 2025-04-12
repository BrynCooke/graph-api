# Benchmarking

Benchmarking is essential for measuring and comparing the performance of your graph implementation. This chapter covers
how to set up and run benchmarks, interpret results, and use benchmarking to guide optimization.

## Setting Up Benchmarks

The Graph API provides built-in benchmarking tools to help you evaluate your implementation's performance.

### Adding Benchmarking Dependencies

First, add the necessary dependencies to your `Cargo.toml`:

```toml
[dev-dependencies]
criterion = "0.5"
graph-api-benches = { version = "0.1.0", features = ["vertex-hash-index", "vertex-label-index", "vertex-full-text-index", "vertex-range-index", "edge-label-index"] }

[[bench]]
name = "my_graph_benchmarks"
harness = false
```

### Creating a Basic Benchmark Suite

Create a benchmark file in your project's `benches` directory:

```rust
// benches/my_graph_benchmarks.rs
use criterion::{criterion_group, criterion_main, Criterion};
use graph_api_benches::bench_suite;
use my_graph::MyGraph;

fn criterion_benchmark(c: &mut Criterion) {
    bench_suite!(c, || MyGraph::new());
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

The `bench_suite!` macro runs a standardized set of benchmarks against your graph implementation.

## Standard Benchmark Suite

The standard benchmark suite in `graph-api-benches` tests several aspects of graph performance:

### 1. Construction Benchmarks

Measures the performance of creating graph elements:

- Creating vertices
- Creating edges
- Creating graphs of different sizes

### 2. Query Benchmarks

Evaluates lookup and traversal performance:

- Vertex retrieval by ID
- Vertex retrieval by index
- Edge traversal
- Path finding

### 3. Mutation Benchmarks

Tests the efficiency of modifying the graph:

- Adding vertices and edges
- Removing vertices and edges
- Modifying vertex and edge properties

### 4. Traversal Benchmarks

Measures the performance of walking the graph:

- Simple steps (vertices, edges)
- Filter operations
- Map operations
- Complex traversals

### 5. Scale Benchmarks

Assesses how performance scales with graph size:

- Small graphs (10s of vertices)
- Medium graphs (100s of vertices)
- Large graphs (1000s of vertices)
- Huge graphs (10,000s of vertices)

## Running Benchmarks

To run the benchmarks:

```bash
# Run all benchmarks
cargo bench

# Run a specific benchmark
cargo bench -- vertex_insertion
```

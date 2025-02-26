# Graph API Performance Benchmarks Specification

## Overview
This document outlines a performance benchmarking suite for the graph-api project using Criterion.rs. The benchmarks will be implemented in the graph-api-test package and reused by different graph implementations, similar to how tests are currently shared.

## Implementation Details

### Structure
- Create a new module `bench` in `graph-api-test`
- Add Criterion as a dev-dependency
- Implement benchmarks in a modular way, similar to tests
- Expose benchmark functions through a macro similar to `test_suite!`

### Criterion Setup
```rust
// Example benchmark group structure
pub fn bench_group<G: Graph<Vertex = Vertex, Edge = Edge>>(c: &mut Criterion, setup: impl Fn() -> G) {
    // Group for vertex operations
    let mut vertex_group = c.benchmark_group("vertex_operations");
    vertex_operations(&mut vertex_group, setup);
    vertex_group.finish();
    
    // Group for edge operations
    let mut edge_group = c.benchmark_group("edge_operations");
    edge_operations(&mut edge_group, setup);
    edge_group.finish();
    
    // Group for traversal operations
    let mut traversal_group = c.benchmark_group("traversal_operations");
    traversal_operations(&mut traversal_group, setup);
    traversal_group.finish();

    // Other benchmark groups...
}
```

## Benchmark Categories

### 1. Graph Construction
- **Small Graph Creation**: Benchmark creating a small graph (10-100 vertices)
- **Medium Graph Creation**: Benchmark creating a medium graph (1,000-10,000 vertices)
- **Large Graph Creation**: Benchmark creating a large graph (100,000+ vertices)
- **Batch Vertex Insertion**: Adding multiple vertices in a single operation
- **Batch Edge Insertion**: Adding multiple edges in a single operation

### 2. Vertex Operations
- **Vertex Addition**: Add a single vertex
- **Vertex Retrieval by ID**: Retrieve vertices by ID
- **Vertex Removal**: Remove a vertex
- **Vertex Property Access**: Access properties of vertices
- **Vertex Property Update**: Update properties of vertices

### 3. Edge Operations
- **Edge Addition**: Add a single edge between vertices
- **Edge Retrieval**: Retrieve edges by ID
- **Edge Removal**: Remove an edge
- **Edge Property Access**: Access properties of edges
- **Edge Property Update**: Update properties of edges

### 4. Index Operations
- **Vertex Label Index Lookup**: Lookup vertices by label
- **Vertex Property Index Lookup**: Lookup vertices by property values
- **Vertex Full-Text Index Lookup**: Search vertices using full-text search
- **Vertex Ordered Index Range Queries**: Find vertices with property values in a range
- **Edge Label Index Lookup**: Lookup edges by label

### 5. Graph Traversal
- **Basic Traversal**: Traverse from vertex to connected vertices
- **Filter Traversal**: Traverse and filter by properties
- **Path Traversal**: Find paths between vertices
- **Complex Traversal**: Combine multiple traversal steps (filter, limit, head, etc.)
- **Deep Traversal**: Traverse several hops from starting vertices

### 6. Graph Queries
- **Count Vertices**: Count vertices matching criteria
- **Count Edges**: Count edges matching criteria
- **Find First**: Find first vertex matching criteria
- **Collect Results**: Collect vertices/edges into collection
- **Detection**: Check if vertices/edges exist with certain criteria

### 7. Graph Mutations During Traversal
- **Vertex Update During Traversal**: Update vertices during traversal
- **Edge Addition During Traversal**: Add edges during traversal
- **Edge Removal During Traversal**: Remove edges during traversal

### 8. Data Scale Benchmarks
- **Scaling Vertex Count**: Measure how performance scales with number of vertices
- **Scaling Edge Count**: Measure how performance scales with number of edges
- **Scaling Connectivity**: Measure how performance scales with graph connectivity

## Benchmark Configurations

### Test Data Generation
- Create realistic datasets of various sizes
- Include both random and structured graph topologies
- Generate synthetic data that mirrors real-world graph structures

### Performance Metrics
- **Throughput**: Operations/second
- **Latency**: Time per operation
- **Memory Usage**: Heap allocations
- **Scaling Characteristics**: How performance changes with graph size

## Implementation Plan

1. Add Criterion dependency to `graph-api-test`
2. Create benchmark module structure
3. Implement benchmark helper functions
4. Create benchmark macro for graph implementations
5. Develop data generators for different graph scales
6. Implement benchmarks for each category
7. Create reporting and visualization tools

## Usage by Graph Implementations

Graph implementations will be able to use these benchmarks similarly to how they use tests:

```rust
use graph_api_test::bench_suite;

fn criterion_benchmark(c: &mut Criterion) {
    bench_suite!(c, || MyGraph::new());
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

This specification provides a comprehensive framework for benchmarking different aspects of graph implementations using the graph-api, allowing for consistent performance comparison across implementations.
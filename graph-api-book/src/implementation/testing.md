# Testing Your Implementation

Testing a graph implementation thoroughly is essential to ensure correctness, reliability, and performance. The Graph API provides a comprehensive test suite to verify that your implementation meets all the requirements of the Graph API traits.

## Using the Test Suite

The `graph-api-test` crate provides a test suite that can be used to verify your graph implementation. The test suite covers:

1. Basic graph operations (adding/removing vertices and edges)
2. Graph traversal and walker steps
3. Index functionality
4. Edge cases and error handling
5. Fuzzing tests for robustness

### Setting Up the Test Suite

To test your implementation:

1. Add `graph-api-test` as a dev-dependency in your `Cargo.toml`:

```toml
[dev-dependencies]
graph-api-test = { version = "0.1.0", path = "../graph-api-test" }
```

2. Create a test module in your crate:

```rust
#[cfg(test)]
mod test {
    use crate::MyGraph;
    use graph_api_test::test_suite;

    test_suite!(MyGraph::new());
}
```

The `test_suite!` macro generates test cases for all the functionality supported by the Graph API.

## Understanding Test Coverage

The test suite covers several areas:

### Basic Graph Operations

- Adding vertices and edges
- Removing vertices and edges
- Retrieving vertices and edges by ID
- Modifying vertex and edge properties

### Walker Steps

Tests for all walker steps:
- `vertices`, `vertices_by_id`, `edges`
- `filter`, `map`, `limit`, `first`
- `head`, `tail`, `detour`
- `collect`, `count`, `into_iter`
- `probe`, `mutate`, `dbg`
- Context operations

### Index Tests

If your graph supports indexes, tests for:
- Vertex and edge label indexes
- Hash indexes
- Range indexes
- Full-text indexes

### Conditional Feature Testing

The test suite adapts to the features your graph implementation supports. For example, if your graph doesn't support range indexes, those tests will be skipped.

## Writing Additional Tests

While the test suite covers most functionality, you should write additional tests for:

1. **Implementation-specific features**: Any custom functionality your graph provides.
2. **Edge cases**: Unusual usage patterns specific to your implementation.
3. **Performance tests**: Verify that your implementation meets performance requirements.

### Example: Testing a Custom Feature

```rust
#[test]
fn test_my_custom_feature() {
    let mut graph = MyGraph::new();
    
    // Test setup
    let vertex = graph.add_vertex(Vertex::Person { name: "Test".to_string(), ... });
    
    // Test the custom feature
    let result = graph.my_custom_feature(vertex);
    
    // Assertions
    assert_eq!(result, expected_value);
}
```

## Fuzzing Tests

The test suite includes fuzzing tests that apply random sequences of operations to your graph to find edge cases and bugs. These tests help ensure your implementation is robust against unexpected usage patterns.

To run the fuzzing tests with extended iterations:

```bash
PROPTEST_CASES=1000 cargo test fuzz_test
```

## Testing Performance

Performance testing is crucial for graph implementations. The test suite includes basic performance tests, but you should also write specific benchmarks:

1. Add a benchmarking framework like `criterion`:

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "my_graph_benchmarks"
harness = false
```

2. Create benchmarks for performance-critical operations:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use my_graph::{MyGraph, Vertex, Edge};
use graph_api_lib::{Graph, VertexSearch};

fn vertex_insertion_benchmark(c: &mut Criterion) {
    c.bench_function("insert_vertices_1000", |b| {
        b.iter(|| {
            let mut graph = MyGraph::new();
            for i in 0..1000 {
                graph.add_vertex(Vertex::Person {
                    name: format!("Person {}", i),
                    // Other fields...
                });
            }
        })
    });
}

criterion_group!(benches, vertex_insertion_benchmark);
criterion_main!(benches);
```

3. Use the benchmarking utilities provided by `graph-api-benches`:

```rust
use criterion::{Criterion, criterion_group, criterion_main};
use graph_api_benches::bench_suite;
use my_graph::MyGraph;

fn criterion_benchmark(c: &mut Criterion) {
    bench_suite!(c, || MyGraph::new());
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

## Debugging Test Failures

When a test fails, the test suite provides detailed error messages to help diagnose the issue:

1. **Element mismatches**: Shows which elements were expected but missing, and which extra elements were found.
2. **Type errors**: Indicates when type constraints are not satisfied.
3. **Panic information**: Provides context when your implementation panics.

### Common Test Failure Causes

1. **Incorrect Iterator Implementation**: Many failures stem from incorrect iterator implementations for `VertexIter` and `EdgeIter`.
2. **Index Inconsistencies**: Failures in index tests often indicate that indexes are not being updated correctly during mutations.
3. **Missing Direction Handling**: Edge traversal tests may fail if direction filtering isn't correctly implemented.
4. **Lifetime Issues**: Reference handling problems can cause compiler errors in tests.

## Test-Driven Development Approach

A recommended approach for implementing a graph backend:

1. Start with a minimal implementation that passes basic vertex and edge tests.
2. Add traversal capabilities and ensure walker step tests pass.
3. Implement indexes one at a time, testing each thoroughly before moving on.
4. Run the fuzzing tests to catch any edge cases.
5. Optimize and benchmark once functionality is correct.

## Conclusion

Thorough testing is essential for a robust graph implementation. Use the provided test suite as your primary testing tool, but also write custom tests for implementation-specific features and performance characteristics. A well-tested implementation will be more reliable and easier to maintain as the Graph API evolves.
# Testing Your Implementation

Testing a graph implementation thoroughly is essential to ensure correctness, reliability, and performance. The Graph
API provides a comprehensive test suite to verify that your implementation meets all the requirements of the Graph API
traits.

## Using the Test Suite

The `graph-api-test` crate provides a test suite that can be used to verify your graph implementation. The test suite
covers:

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
graph-api-test = { version = "0.1.0", features = ["vertex-hash-index", "vertex-label-index", "vertex-full-text-index", "vertex-range-index", "edge-label-index"] }

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

The test suite adapts to the features your graph implementation supports. For example, if your graph doesn't support
range indexes, those tests will be skipped.

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

The test suite includes fuzzing tests that apply random sequences of operations to your graph to find edge cases and
bugs. These tests help ensure your implementation is robust against unexpected usage patterns.



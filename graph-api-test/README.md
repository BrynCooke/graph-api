# Testing your graph

This crate contains a test suite to ensure that your graph behaves as expected.

Simply include the following code in your library:

```rust
#[cfg(test)]
mod test {
    use graph_api_test::test_suite;
    test_suite!(YourGraph::new());
}
```

The `test_suite!` macro will expand to cover all basic operations and traversals.

When importing this crate make sure to use the features that match your graph implementation.

For instance:

```rust
impl Graph for MyGraph {
    type SupportsVertexLabelIndex: Supported;
}
```

make sure to include feature `vertex-label-index`

The full list of features are:

* `SupportsVertexLabelIndex`: `vertex-label-index`
* `SupportsEdgeLabelIndex`: `edge-label-index`
* `SupportsVertexIndex`: `vertex-index`
* `SupportsEdgeIndex`: `edge-index`
* `SupportsVertexOrderedIndex`: `vertex-ordered-index`
* `SupportsEdgeOrderedIndex`: `edge-ordered-index`
* `SupportsVertexFullTextIndex`: `vertex-full-text-index`


# Writing Tests for graph-api

This document explains how to write tests for the graph-api library using the test utilities provided in this package.

## Test Graph Structure

Most tests in this library use a standard test graph created by the `populate_graph` function. This function creates a small graph with the following elements:

### Vertices
- **Bryn**: A Person vertex with age 45, username "bryn", and biography "Did some graph stuff"
- **Julia**: A Person vertex with age 48, username "julia", and biography "Mastered the English language"
- **GraphApi**: A Project vertex with name "GraphApi"
- **Rust**: A Rust vertex (no properties)

### Edges
- **bryn_knows_julia**: Bryn knows Julia (since 1999)
- **julia_knows_bryn**: Julia knows Bryn (since 1999)
- **bryn_created_graph_api**: Bryn created GraphApi
- **graph_api_language_rust**: GraphApi is written in Rust

The `populate_graph` function returns a `Refs` struct containing IDs for all these elements, which you can use in your tests.

## Writing a Test

Tests in graph-api follow a standard pattern:

1. Create a test function that accepts a generic graph type
2. Use `populate_graph` to create the standard test graph and get references
3. Perform operations on the graph and assert expected results

Here's a simple example based on `first.rs`:

```rust
pub fn test_my_feature<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Populate the graph and get references to elements
    let refs = populate_graph(graph);
    
    // Test your functionality
    assert_eq!(
        graph
            .walk_mut()
            .vertices_by_id(vec![refs.bryn, refs.julia])
            .my_operation(),
        expected_result
    );
}
```

## Using the Test Macros

The library provides several macros to make test creation easier:

### General tests

```rust
general_test!{$setup, test_name, path::to::test_function}
```

### Feature-specific tests

For features that might not be supported in all implementations:

```rust
vertex_index_label_test!{$setup, test_name, path::to::test_function}
edge_index_label_test!{$setup, test_name, path::to::test_function}
```

### Running the Full Test Suite

To run all standard tests for a graph implementation:

```rust
test_suite!{my_graph_setup_function()}
```

## Assertion Helpers

The library provides helper functions for comparing collections of elements:

- `assert_elements_eq!($graph, $actual, $expected)`: Assert that two collections contain the same elements
- `assert_elements_one_of!($graph, $actual, $expected)`: Assert that a collection contains exactly one element from the expected collection
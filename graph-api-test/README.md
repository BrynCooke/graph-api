# Graph-API Test Suite

**Welcome to graph-api-test** — your friendly companion for ensuring your graph implementation behaves exactly as
expected!

Implementing the Graph-API traits for your custom graph structure? This comprehensive test suite takes the guesswork out
of compatibility testing, giving you confidence that your graph behaves correctly.

## Effortless Testing

Testing your graph implementation couldn't be easier:

```rust
#[cfg(test)]
mod test {
    use graph_api_test::test_suite;

    // One line to test everything!
    test_suite!(YourGraph::new());
}
```

With this simple macro, you'll validate that your graph implementation properly supports all the operations and
behaviors expected by the Graph-API ecosystem. No need to write tedious test cases for each feature!

## Feature-Specific Testing

Implementing only a subset of Graph-API's optional capabilities? No problem! Simply include the features that match your
implementation:

```toml
[dev-dependencies]
graph-api-test = { version = "0.1.0", features = [
    "vertex-label-index",
    "edge-label-index",
    "vertex-range-index"
] }
```

## Available Feature Tests

* `vertex-label-index`: Label-based vertex lookups
* `edge-label-index`: Label-based edge traversals
* `vertex-hash-index`: Hash-indexed vertex properties
* `edge-hash-index`: Hash-indexed edge properties
* `vertex-range-index`: Range queries for vertex properties
* `edge-range-index`: Range queries for edge properties
* `vertex-full-text-index`: Full-text search for vertex properties
* `graph-clear`: Graph clearing operations

## Standard Test Graph

Most tests use a consistent, well-defined graph with:

* **Vertices**: People (Bryn, Julia), Projects (GraphApi), and Technologies (Rust)
* **Edges**: Knowledge relationships, creation attribution, and technology connections

This standardized structure ensures that all graph implementations are tested against the same scenarios.

## Writing Custom Tests

Need to test specific aspects of your implementation? The suite provides helpful utilities:

* Ready-to-use test graph population
* Element comparison helpers
* Reference tracking for standard test elements
* Test patterns for different graph features

Make your graph implementation rock-solid with graph-api-test!

Learn more in the [graph-api book](https://bryncooke.github.io/graph-api/).

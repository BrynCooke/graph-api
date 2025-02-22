# Graph Testing Suite

The Graph API provides a comprehensive test suite to validate your graph implementation's correctness. The suite covers both basic graph operations and the walker API functionality.

## Setup

1. Add `graph-api-test` to your `Cargo.toml` dev-dependencies:

```toml
[dev-dependencies]
graph-api-test = { version = "0.1.0", features = [...] }
```

2. Implement the test suite in your crate:

```rust
#[cfg(test)]
mod test {
    use graph_api_test::test_suite;
    
    test_suite!(YourGraph::new());
}
```

## Feature Configuration

Enable features based on your graph's capabilities when including the `graph-api-test` crate:

| Graph Trait | Feature Flag | Status |
|------------|--------------|--------|
| `SupportsVertexLabelIndex` | `vertex-label-index` | ✓ |
| `SupportsEdgeLabelIndex` | `edge-label-index` | ✓ |
| `SupportsVertexIndex` | `vertex-index` | ✓ |
| `SupportsEdgeIndex` | `edge-index` | Not implemented |
| `SupportsVertexOrderedIndex` | `vertex-ordered-index` | ✓ |
| `SupportsEdgeOrderedIndex` | `edge-ordered-index` | Not implemented |
| `SupportsVertexFullTextIndex` | `vertex-full-text-index` | ✓ |
| `SupportsClear` | `graph-clear` | ✓ |

## Example Configuration

For a graph implementation that supports vertex labels and edge labels:

```toml
[dev-dependencies]
graph-api-test = { version = "0.1.0", features = ["vertex-label-index", "edge-label-index"] }
```

## Note

- The test suite ensures compatibility with the Graph API specification
- Each feature enables specific test cases related to that functionality
- Enabling features for unsupported functionality will result in test failures
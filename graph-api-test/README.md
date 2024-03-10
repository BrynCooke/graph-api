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

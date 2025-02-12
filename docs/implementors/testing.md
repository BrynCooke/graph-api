# Testing

Graph API gives you a test suite to make sure that your graph implementation is correct.

It includes tests basic graphs operations as well as the walker API.

## Usage

Include the `graph-api-test` crate in your dev dependencies and add the following code:

```rust
#[cfg(test)]
mod test {
    use graph_api_test::test_suite;
    test_suite!(YourGraph::new());
}
```

Depending on your graph supported features you will need to specify the relevant features then including the
`graph-api-test` crate.

The feature mappings are:

* `SupportsVertexLabelIndex`: `vertex-label-index`
* `SupportsEdgeLabelIndex`: `edge-label-index`
* `SupportsVertexIndex`: `vertex-index`
* `SupportsEdgeIndex`: `edge-index`; (not implemented yet)
* `SupportsVertexOrderedIndex`: `vertex-ordered-index`;
* `SupportsEdgeOrderedIndex`: `edge-ordered-index`; (not implemented yet)
* `SupportsVertexFullTextIndex`: `vertex-full-text-index`;
* `SupportsClear`: `graph-clear`



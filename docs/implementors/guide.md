# Graph Implementors Guide

Adding Graph API support is as simple as implementing the following traits:

* `Graph`: The main interface to your graph
* `VertexReference`: A reference to a vertex
* `VertexReferenceMut`: A mutable reference to a vertex
    * `MutationListener`: A listener that reacts to vertex mutations
* `EdgeReference`: A reference to an edge
* `EdgeReferenceMut`: A mutable reference to an edge
    * `MutationListener`: A listener that reacts to edge mutations
* `VertexIter`: An iterator over vertices
* `EdgeIter`: An iterator over edges

Implementing a graph without index support is very straight forward.

Implementing a graph with index support is more complicated as your implementation of `VertexIter` and `EdgeIter`
must return only elements that match the relevant search criteria.
You must also provide a `MutationListener` that updates indexes when an element is mutated.

## Graph Features

The `Graph` trait includes several associated types for index support:

* `SupportsVertexLabelIndex`: Vertices by label
* `SupportsEdgeLabelIndex`: Edges by label
* `SupportsVertexIndex`: Vertices by value lookup
* `SupportsEdgeIndex`: Edges by value lookup
* `SupportsVertexOrderedIndex`: Vertices by range lookups
* `SupportsEdgeOrderedIndex`:  Edges by range lookups
* `SupportsVertexFullTextIndex`: Vertices by full text index

When implementing a graph you must specify these as `Supported` or `Unsupported`.

## Testing

Graph API gives you a test suite to make sure that your graph implementation is correct.
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
* `SupportsEdgeIndex`: `edge-index`;
* `SupportsVertexOrderedIndex`: `vertex-ordered-index`;
* `SupportsEdgeOrderedIndex`: `edge-ordered-index`;
* `SupportsVertexFullTextIndex`: `vertex-full-text-index`;


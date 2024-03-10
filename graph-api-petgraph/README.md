# PetGraph

An implementation of Graph API for the popular [petgraph](https://docs.rs/petgraph/latest/petgraph/) crate.

This allows users of PetGraph's StableGraph to use Graph API's traversals.

```rust
fn test() {
    let graph = petgraph::stable_graph::StableGraph::new();
    // Populate the graph here

    // Find all people in the graph
    let people = graph
        .walk()
        .vertices(VertexIndex::person())
        .collect::<Vec<_>>();
}
```

## Future development

Other PetGraph Graph implementations will not be developed as part of this crate.
PetGraph support requires that it is included directly in the Graph API lib crate due to Rust's orphan rule which is
undesired.
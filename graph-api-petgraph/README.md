# Graph-API PetGraph Adapter

**Welcome to graph-api-petgraph** — bridging the gap between the popular [petgraph](https://docs.rs/petgraph/latest/petgraph/) crate and the Graph-API ecosystem!

Already using petgraph for your graph data structures? Love its efficiency but wish it had more expressive traversal capabilities? This adapter brings the best of both worlds to your Rust projects!

## The Perfect Combination

This crate allows you to keep using petgraph's excellent StableGraph implementation while gaining access to all the intuitive traversal patterns and type-safety features of Graph-API. It's the ideal way to enhance your existing petgraph-based code without rewriting your storage layer.

## Simple Integration

With just a few lines of code, unlock powerful new capabilities:

```rust
fn main() {
    // Your existing petgraph code
    let mut graph = petgraph::stable_graph::StableGraph::new();
    
    // Add some vertices and edges...
    
    // Now enjoy the rich Graph-API traversal system!
    let people = graph
        .walk()
        .vertices(VertexIndex::person())
        .edges(EdgeIndex::knows().direction(Direction::Outgoing))
        .tail()
        .filter_by_person(|person, _| person.age() > 30)
        .collect::<Vec<_>>();
}
```

## Benefits

* **Keep Your Existing Code**: No need to migrate your graph storage
* **Enhance Capabilities**: Add expressive traversals to your petgraph applications
* **Type-Safe Operations**: Enjoy compile-time checks for your graph operations
* **Performance**: Benefit from petgraph's efficient implementation with Graph-API's intuitive interface

## Implementation Note

This adapter implements the Graph-API traits for petgraph's StableGraph. Other petgraph graph types are not currently supported due to Rust's orphan rule limitations.

Explore the integration between two powerful graph libraries with graph-api-petgraph!

Learn more in the [graph-api book](https://bryncooke.github.io/graph-api/).
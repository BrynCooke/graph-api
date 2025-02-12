# Graph Implementors Guide

Graph API provides a set of traits that can be implemented for your graph that provide users with a consistent
API between graph implementations.

A graph implementation allows adding removing and mutating vertices and edges, and also provides search and iteration
capability.

```rust
fn example() {
    let graph = SimpleGraph::new();

    // Populate the graph
    let person = graph.add_vertex(Vertex::Person {
        name: "Bryn".to_string(),
        age: 45,
        unique_id: Uuid::from_u128(1),
        username: "bryn".to_string(),
        biography: "Did some graph stuff".to_string(),
    });
    let project = graph.add_vertex(Vertex::Project { name: "Graph API".to_string() });
    graph.add_edge(person, project, Edge::Created);
}
```

By using Graph API your users get a powerful walker API to traverse their
graph without writing endless loops. Think iterator for graphs:

```rust
fn example() {
    let graph = SimpleGraph::new();
    graph.walk()
        .vertices(VertexIndex::person_by_name("Bryn"))
        .out_edges(None)
        .collect::<Vec<_>>();
}
```

Finally Graph API comes with derive macro that further enhances the user experience when walking the graph
and also allows static definition of how elements are indexed.

## Implementing a graph

Implementing a graph is easy, and can be done in a few hours/days assuming good Rust knowledge:

1. [Create your Graph implementation](./graphs.md)
2. [Use the Graph API test suite](./testing.md)
3. [Enable features for your Graph](./features.md)
4. [Implement indexes](./indexes.md)

If in doubt take a look at the `SimpleGraph` reference implementation.

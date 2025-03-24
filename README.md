![CI](https://github.com/BrynCooke/graph-api/actions/workflows/ci.yml/badge.svg)

# Graph API

An ergonomic API for working with **in memory** graphs in Rust that provides a flexible and type-safe way to interact
with graph data
structures.

## Overview

This library offers a unified interface for working with different types of graphs while maintaining strong type safety
and ergonomic usage patterns. It includes features for graph traversal, modification, and custom extensions.

## Key Features

- Type-safe graph operations
- Flexible vertex and edge traversal
- Custom graph implementations support
- Derive macros for extending graph functionality
- Comprehensive testing utilities including fuzzing support

## Example usage

```rust
# use graph_api_derive::VertexExt;
# use graph_api_derive::EdgeExt;
# use uuid::Uuid;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use std::ops::Deref;
# use graph_api_lib::VertexSearch;
# use graph_api_lib::Direction;

#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index]
        name: String,
        #[index(range)]
        age: u64,
        #[index]
        unique_id: Uuid,
        #[index(range)]
        username: String,
        #[index(full_text)]
        biography: String,
    },
    Project {
        name: String
    },
    Rust,
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows { since: i32 },
    Created,
    Language {
        name: String
    },
}

# fn main() {
    // Create a new graph
    let mut graph = SimpleGraph::new();

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

    // Traverse the graph
    let all_vertices = graph.walk().vertices(VertexSearch::scan()).collect::<Vec<_>>();

    // A more complicated traversal
    // For up to two people that Bryn knows find all the people that they know and add
    // their ages to Bryn's age and collect them into a list.
    let complex = graph
        .walk()
        .vertices(VertexIndex::person_by_name("Bryn")) // Start at people named Bryn
        .filter_by_person(|v, _| v.username().ends_with("e")) // Filter by username ending with e
        .push_context(|v, ctx| v.project::<Person<_>>().unwrap().age()) // Stash the age in the context
        .edges(EdgeIndex::knows().direction(Direction::Outgoing)) // Traverse to knows
        .limit(2) // Limit the traversal to two elements
        .head() // Traverse to the head of the edge (the known person) 
        .detour(|v| { // Find the people that this person knows and collect their ages
            v.edges(EdgeIndex::knows().direction(Direction::Outgoing))
                .head()
                .push_context(|v, ctx| v.project::<Person<_>>().unwrap().age())
        })
        .map(|v, ctx| **ctx.parent() + *ctx) // Add the ages collected during the traversal 
        .collect::<Vec<_>>();
# }

```

## Documentation

### For API Users

- [Getting Started](docs/users/getting_started)
- [Defining A Model](docs/users/defining_a_model)
- [Basic Operations](docs/users/basic_operations)
- [Graph Traversal](docs/users/traversal.md)

### For Graph Implementors

- [Implementation Guide](docs/implementors/guide.md)

## License

[Apache 2.0](./LICENSE)

## Contributing

Contributions are welcome! Please check out our [contribution guidelines](CONTRIBUTING.md) for details on how to get
started.




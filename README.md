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
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index]
        name: String,
        #[index(ordered)]
        age: u64,
        #[index]
        unique_id: Uuid,
        #[index(ordered)]
        username: String,
        #[index(full_text)]
        biography: String,
    },
    Project(Project),
    Rust,
}

#[derive(Debug, Clone)]
pub struct Project {
    name: String,
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows { since: i32 },
    Created,
    Language(Language),
}
#[derive(Debug, Clone)]
pub struct Language {
    name: String,
}

fn main() {
    // Create a new graph
    let graph = SimpleGraph::new();

    // Populate the graph
    let person = graph.add_vertex(Vertex::Person { name: "Bryn".to_string(), age: 30 });
    let project = graph.add_vertex(Vertex::Project { name: "Graph API".to_string() });
    graph.add_edge(person, project, Edge::Created);

    // Traverse the graph
    graph.walk().vertices()
}
```

## Documentation

### For API Users

- [Getting Started](docs/users/getting-started.md)
- [Defining A Model](docs/users/defining-a-model.md)
- [Basic Operations](docs/users/basic-operations.md)
- [Graph Traversal](docs/users/traversal.md)

### For Graph Implementors

- [Implementation Guide](docs/implementors/guide.md)
- [Index support](docs/implementors/index-support.md)

## License

[Apache 2.0](LICENSE)

## Contributing

Contributions are welcome! Please check out our [contribution guidelines](CONTRIBUTING.md) for details on how to get
started.




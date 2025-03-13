# Introduction to Graph API

Graph API is an ergonomic library for working with **in memory** graphs in Rust that provides a flexible and type-safe way to interact with graph data structures.

## Overview

This library offers a unified interface for working with different types of graphs while maintaining strong type safety and ergonomic usage patterns. It includes features for graph traversal, modification, and custom extensions.

## Key Features

- **Type-safe graph operations**: Work with graph data in a strongly-typed manner
- **Flexible vertex and edge traversal**: Explore graph relationships with powerful walker API
- **Custom graph implementations support**: Adapt to various graph storage backends
- **Derive macros for extending graph functionality**: Automatically generate boilerplate code
- **Comprehensive testing utilities**: Including fuzzing support

## Example

```rust
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index]
        name: String,
        #[index(ordered)]
        age: u64,
        // Other fields...
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

// Create a graph and populate it
let mut graph = SimpleGraph::new();
let person = graph.add_vertex(Vertex::Person { 
    name: "Bryn".to_string(),
    age: 45,
    // Other fields...
});
let project = graph.add_vertex(Vertex::Project { name: "Graph API".to_string() });
graph.add_edge(person, project, Edge::Created);

// Perform a graph traversal
let people_who_created_projects = graph
    .walk()
    .vertices(VertexIndex::person_by_name("Bryn"))
    .edges(EdgeIndex::created().direction(Direction::Outgoing))
    .tail()
    .collect::<Vec<_>>();
```

## Book Organization

This book is organized into several sections:

- **User Guide**: How to use the Graph API to work with graph data
- **Implementation Guide**: How to implement the Graph API traits for your own graph types
- **Reference**: Detailed information about API components and functionality
- **Appendix**: Additional resources and reference materials

Whether you're a graph API user or implementing your own graph backend, this book provides comprehensive documentation to help you make the most of the Graph API library.
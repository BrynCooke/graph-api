# Getting Started

`graph-api` is an API that helps you work with graphs in Rust. Any implementation of the Graph trait will benefit from an ergonomic walker API that allows you to perform traversals.

For the purposes of documentation we will use `SimpleGraph`, but you may choose any other graph implementation.

## Installation

Add the Graph API to your project dependencies:

```toml
[dependencies]
graph-api-lib = "0.1.0"
graph-api-derive = "0.1.0"  # For derive macros
```

For a simple graph implementation:

```toml
[dependencies]
graph-api-simplegraph = "0.1.0"
```

## Your First Graph

Let's define a simple graph to play with:

```rust
use graph_api_derive::{VertexExt, EdgeExt};
use graph_api_simplegraph::SimpleGraph;
use graph_api_lib::{Graph, VertexSearch};

// Define vertex types
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index]
        name: String,
        age: u64,
    },
    Project {
        name: String,
    },
}

// Define edge types
#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows { since: i32 },
    Created,
}

// Create a graph
let mut graph = SimpleGraph::new();

// Add vertices
let alice = graph.add_vertex(Vertex::Person {
    name: "Alice".to_string(),
    age: 30,
});

let bob = graph.add_vertex(Vertex::Person {
    name: "Bob".to_string(),
    age: 28,
});

let project = graph.add_vertex(Vertex::Project {
    name: "Graph API".to_string(),
});

// Add edges
graph.add_edge(alice, bob, Edge::Knows { since: 2020 });
graph.add_edge(alice, project, Edge::Created);

// Find all vertices
let vertex_count = graph.walk()
    .vertices(VertexSearch::scan())
    .count::<Vec<_>>();

println!("Found {} vertices", vertex_count);
```

## Next Steps

Now that you have created your first graph, you can:

1. Learn about [defining a model](./defining_a_model.md) for your graph data
2. Explore [basic operations](./basic_operations.md) for working with graphs
3. Discover [graph traversal](./traversal.md) techniques using walkers
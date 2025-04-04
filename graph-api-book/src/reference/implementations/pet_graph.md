# PetGraph

`PetGraph` provides a Graph API adapter for the popular [petgraph](https://crates.io/crates/petgraph) Rust graph library. This implementation allows existing petgraph users to take advantage of the Graph API ergonomics while leveraging petgraph's established algorithms and performance characteristics.

## Overview

`PetGraph` is an adapter implementation that wraps `petgraph::stable_graph::StableGraph` to make it compatible with the Graph API. This implementation focuses on providing a bridge to petgraph's functionality rather than implementing all Graph API features.

```rust
use petgraph::stable_graph::StableGraph;
use graph_api_derive::{VertexExt, EdgeExt};
use graph_api_lib::Graph;

// Define vertex and edge types
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        name: String,
        age: u64,
    },
    Project {
        name: String
    },
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows { since: i32 },
    Created,
}

// Create a new petgraph StableGraph (which implements Graph)
let mut graph = StableGraph::new();

// Use the graph through the Graph API
let alice = graph.add_vertex(Vertex::Person { 
    name: "Alice".to_string(), 
    age: 30 
});
let project = graph.add_vertex(Vertex::Project { 
    name: "Graph API".to_string() 
});
graph.add_edge(alice, project, Edge::Created);
```

## Architecture

The PetGraph adapter:

1. **Maps Graph API concepts to petgraph**: Translates between Graph API's model and petgraph's model
2. **Provides wrapper types**: Wraps petgraph references to implement Graph API reference traits
3. **Adapts traversal patterns**: Adapts petgraph's traversal methods to match Graph API expectations

## Features

`PetGraph` supports a subset of Graph API features:

- ❌ Vertex label indexes
- ❌ Edge label indexes
- ❌ Vertex hash indexes
- ❌ Edge hash indexes
- ❌ Vertex range indexes
- ❌ Edge range indexes
- ❌ Vertex full-text indexes
- ❌ Edge adjacent label indexes
- ✅ Graph clearing

## Petgraph Integration

The primary advantage of `PetGraph` is access to petgraph's rich ecosystem:

### Graph Algorithms

Petgraph provides many graph algorithms that can be used alongside Graph API:

```rust
use petgraph::algo::dijkstra;
use petgraph::stable_graph::StableGraph;
use graph_api_lib::Graph;

// Create and populate graph using Graph API
let mut graph = StableGraph::new();
let a = graph.add_vertex(Vertex::Person { name: "A".to_string(), age: 30 });
let b = graph.add_vertex(Vertex::Person { name: "B".to_string(), age: 25 });
let c = graph.add_vertex(Vertex::Person { name: "C".to_string(), age: 40 });

graph.add_edge(a, b, Edge::Knows { since: 2010 });
graph.add_edge(b, c, Edge::Knows { since: 2015 });
graph.add_edge(a, c, Edge::Knows { since: 2020 });

// Use petgraph algorithms directly on the same graph
let path = dijkstra(&graph, a, Some(c), |_| 1);
```

### Visualization

Petgraph supports graph visualization with Graphviz:

```rust
use petgraph::dot::{Dot, Config};

// Create and populate graph using Graph API
let mut graph = StableGraph::new();
// ... add vertices and edges ...

// Use petgraph's Dot export
println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
```

## Performance Characteristics

`PetGraph` inherits performance characteristics from petgraph:

1. **Efficient traversal**: Petgraph's adjacency list representation enables fast traversal
2. **Fast mutations**: Quick addition and removal of vertices and edges
3. **Algorithm optimizations**: Petgraph's algorithms are optimized for performance

However, `PetGraph` lacks indexing support, which means:

- All label-based searches require full scans
- Property-based lookups require iterating through all vertices/edges

## Use Cases

`PetGraph` is well-suited for:

- **Existing petgraph users**: When you want to use Graph API ergonomics with existing petgraph code
- **Algorithm-heavy applications**: When you need access to petgraph's graph algorithms
- **Simple graph structures**: When you don't need indexing features

## Implementation Notes

The `PetGraph` adapter consists of several wrapper types:

- `VertexReferenceWrapper`: Wraps petgraph node references
- `VertexReferenceWrapperMut`: Wraps mutable petgraph node references
- `EdgeReferenceWrapper`: Wraps petgraph edge references
- `EdgeReferenceWrapperMut`: Wraps mutable petgraph edge references
- `VertexIter`: Adapts petgraph's vertex iteration
- `EdgeIter`: Adapts petgraph's edge iteration

These wrappers implement the corresponding Graph API traits to provide compatibility.

## Limitations

When using `PetGraph`, be aware of these limitations:

1. **No indexing support**: All lookups by label or property require full scans
2. **Limited filtering**: Edge and vertex filtering must be done after retrieval
3. **Compatibility gaps**: Some petgraph features may not map directly to Graph API concepts

## Source Code

The source code for the `PetGraph` adapter is available in the [graph-api-lib](https://github.com/BrynCooke/graph-api/tree/main/graph-api-lib/src/petgraph) crate under the `petgraph` module.

## Example Usage

Here's an example demonstrating `PetGraph` usage:

```rust
use petgraph::stable_graph::StableGraph;
use graph_api_derive::{VertexExt, EdgeExt};
use graph_api_lib::{Graph, VertexSearch, EdgeSearch, Direction};

// Define vertex and edge types
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        name: String,
        age: u64,
    },
    Project {
        name: String
    },
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows { since: i32 },
    Created,
}

// Create a new graph
let mut graph = StableGraph::new();

// Add vertices
let alice = graph.add_vertex(Vertex::Person { 
    name: "Alice".to_string(), 
    age: 30,
});

let bob = graph.add_vertex(Vertex::Person { 
    name: "Bob".to_string(), 
    age: 25,
});

let project = graph.add_vertex(Vertex::Project { 
    name: "Graph API".to_string() 
});

// Add edges
graph.add_edge(alice, bob, Edge::Knows { since: 2020 });
graph.add_edge(alice, project, Edge::Created);
graph.add_edge(bob, project, Edge::Created);

// Basic traversal (without indexing)
let all_vertices = graph.walk()
    .vertices(VertexSearch::scan())
    .collect::<Vec<_>>();
assert_eq!(all_vertices.len(), 3);

// Find all people (manual filtering since indexing isn't supported)
let people = graph.walk()
    .vertices(VertexSearch::scan())
    .filter_by_person(|_, _| true)
    .collect::<Vec<_>>();
assert_eq!(people.len(), 2);

// Find projects connected to Alice
let alices_projects = graph.walk()
    .vertices_by_id(vec![alice])
    .edges(EdgeSearch::scan())
    .filter_by_created(|_, _| true)
    .head()
    .collect::<Vec<_>>();
assert_eq!(alices_projects.len(), 1);
```

## Integrating with Petgraph Algorithms

One of the main advantages of using `PetGraph` is access to petgraph's algorithms:

```rust
use petgraph::stable_graph::StableGraph;
use petgraph::algo::{dijkstra, is_cyclic_directed};
use graph_api_lib::Graph;

// Create and populate graph using Graph API
let mut graph = StableGraph::new();
let a = graph.add_vertex(Vertex::Person { name: "A".to_string(), age: 30 });
let b = graph.add_vertex(Vertex::Person { name: "B".to_string(), age: 25 });
let c = graph.add_vertex(Vertex::Person { name: "C".to_string(), age: 40 });

graph.add_edge(a, b, Edge::Knows { since: 2010 });
graph.add_edge(b, c, Edge::Knows { since: 2015 });

// Use petgraph algorithms
let distances = dijkstra(&graph, a, None, |_| 1);
assert_eq!(distances[&c], 2); // Distance from A to C is 2

let is_cyclic = is_cyclic_directed(&graph);
assert_eq!(is_cyclic, false);

// Add an edge to create a cycle
graph.add_edge(c, a, Edge::Knows { since: 2020 });
let is_cyclic = is_cyclic_directed(&graph);
assert_eq!(is_cyclic, true);
```

## When to Choose PetGraph

Consider using `PetGraph` when:

1. You're already using petgraph in your project
2. You need access to petgraph's graph algorithms
3. You want to use the Graph API's ergonomic interface
4. You don't need indexing features

If you require index-based lookups or other advanced Graph API features, consider using `SimpleGraph` instead.
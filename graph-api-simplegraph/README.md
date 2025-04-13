# SimpleGraph

`SimpleGraph` is the reference implementation for the Graph API, designed primarily to showcase and test the full
capabilities of the API, including comprehensive indexing support for property graphs. While functional, it's not
optimized for high performance.

## Overview

`SimpleGraph` is a custom in-memory graph implementation built specifically to demonstrate and validate all Graph API
features, including every type of index. It serves as a clear example for developers implementing the Graph API traits
and is invaluable for testing Graph API consumers against a fully compliant backend. It handles property graph use cases
where elements have labels (enum variants) and indexed properties.

```rust
use graph_api_simplegraph::SimpleGraph;
use graph_api_derive::{VertexExt, EdgeExt};
use graph_api_lib::Graph;

// Define vertex and edge types
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index(hash)]
        name: String,
        #[index(range)]
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
let mut graph = SimpleGraph::new();

// Use the graph
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

`SimpleGraph` uses a custom data structure designed specifically for property graphs:

1. **Vertex Storage**: Vertices are stored in collections grouped by label (enum variant), allowing for efficient
   label-based filtering.

2. **Edge Storage**: Edges are stored with both head and tail connections, enabling fast traversal in both directions.

3. **Indexes**: Multiple index types are implemented to support different query patterns:
    - Label indexes for finding vertices/edges by label
    - Hash indexes for exact property matching
    - Range indexes for numeric and string range queries
    - Full-text indexes for text search

4. **Adjacency Lists**: Each vertex maintains an adjacency list for fast edge traversal.

## Features

`SimpleGraph` supports all Graph API features:

- ✅ Vertex label indexes
- ✅ Edge label indexes
- ✅ Vertex hash indexes
- ✅ Edge hash indexes
- ✅ Vertex range indexes
- ✅ Edge range indexes
- ✅ Vertex full-text indexes
- ✅ Edge adjacent label indexes
- ✅ Graph clearing

## Performance Characteristics

`SimpleGraph` is primarily designed for feature completeness and ease of understanding, not for high performance:

1. **Correctness over Speed**: Implementation prioritizes demonstrating API features correctly.
2. **Basic Optimizations**: Includes fundamental optimizations like adjacency lists but lacks advanced performance
   tuning.

## Use Cases

`SimpleGraph` is ideal for:

- **Testing**: Verifying code that uses the Graph API against a fully compliant implementation.
- **Reference**: Understanding how to implement the Graph API traits and features.
- **Learning**: Exploring the capabilities of the Graph API in a controlled environment.
- **Prototyping**: Quickly building graph-based applications where performance is not the primary concern initially.

## Limitations

While `SimpleGraph` is a robust implementation, it has some limitations:

- **In-memory only**: All graph data must fit in memory
- **Single-threaded**: No built-in support for concurrent access
- **No persistence**: No built-in support for saving/loading graphs

Learn more in the [graph-api book](https://bryncooke.github.io/graph-api/).
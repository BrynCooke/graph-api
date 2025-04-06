# SimpleGraph

`SimpleGraph` is the flagship implementation of the Graph API, designed specifically for property graphs with full
indexing support. It serves as both a reference implementation and a practical, high-performance graph for most use
cases.

## Overview

`SimpleGraph` is a custom in-memory graph implementation that fully supports all Graph API features, including all types
of indexes. It's designed for property graph use cases where elements are identified by labels (enum variants) and may
have properties that need indexing.

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

`SimpleGraph` is optimized for:

1. **Query Performance**: Fast lookups for indexed properties
2. **Memory Efficiency**: Compact representation of graph elements
3. **Traversal Speed**: Efficient adjacency lists for fast graph traversal

### Benchmarks

While performance will vary based on the specific workload, `SimpleGraph` generally provides:

- Fast insertion and removal of vertices/edges
- Near constant-time lookup for hash-indexed properties
- Logarithmic time lookup for range-indexed properties
- Efficient traversal performance for both incoming and outgoing edges

## Use Cases

`SimpleGraph` is well-suited for:

- **Property graphs**: Especially when using labels and properties extensively
- **Index-heavy applications**: When you need to efficiently query vertices/edges by properties
- **General-purpose graph processing**: For most in-memory graph processing needs

## Implementation Notes

`SimpleGraph` is implemented using several key components:

### Core Data Structures

- `LabelledVertices`: Stores vertices by label, with efficient access by ID
- `LabelledEdges`: Stores edges by label
- `VertexStorage`: Maintains vertex data and adjacency information
- `VertexIndexStorage`: Handles different index types (hash, range, full-text)

### Indexes

`SimpleGraph` implements several index types:

- `HashIndex`: Maps property values to sets of vertex/edge IDs
- `RangeIndex`: Uses ordered maps for range queries
- `FullTextIndex`: Implements inverted indexes for text search

### Mutation Handling

To maintain index consistency, `SimpleGraph` uses a mutation listener system:

```rust
// When a vertex property changes, the index is automatically updated
graph.vertex_mut(vertex_id)
.unwrap()
.project_mut::<PersonMut<_ > > ()
.unwrap()
.set_name("New Name");
```

## Limitations

While `SimpleGraph` is a robust implementation, it has some limitations:

- **In-memory only**: All graph data must fit in memory
- **Single-threaded**: No built-in support for concurrent access
- **No persistence**: No built-in support for saving/loading graphs

## Source Code

The source code for `SimpleGraph` is available in
the [graph-api-simplegraph](https://github.com/BrynCooke/graph-api/tree/main/graph-api-simplegraph) crate.

## Example Usage

Here's a complete example demonstrating key features of `SimpleGraph`:

```rust
use graph_api_simplegraph::SimpleGraph;
use graph_api_derive::{VertexExt, EdgeExt};
use graph_api_lib::{Graph, VertexSearch, EdgeSearch, Direction};

// Define vertex and edge types
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index(hash)]
        name: String,
        #[index(range)]
        age: u64,
        #[index(full_text)]
        bio: String,
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

// Add vertices
let alice = graph.add_vertex(Vertex::Person { 
    name: "Alice".to_string(), 
    age: 30,
    bio: "Graph enthusiast".to_string(),
});

let bob = graph.add_vertex(Vertex::Person { 
    name: "Bob".to_string(), 
    age: 25,
    bio: "Software developer".to_string(),
});

let project = graph.add_vertex(Vertex::Project { 
    name: "Graph API".to_string() 
});

// Add edges
graph.add_edge(alice, bob, Edge::Knows { since: 2020 });
graph.add_edge(alice, project, Edge::Created);
graph.add_edge(bob, project, Edge::Created);

// Query by label
let people = graph.walk()
    .vertices(Vertex::person_all())
    .collect::<Vec<_>>();
assert_eq!(people.len(), 2);

// Query by property (hash index)
let alice_found = graph.walk()
    .vertices(Vertex::person_by_name("Alice"))
    .collect::<Vec<_>>();
assert_eq!(alice_found.len(), 1);

// Query by range (range index)
let young_people = graph.walk()
    .vertices(Vertex::person_by_age(20..30))
    .collect::<Vec<_>>();
assert_eq!(young_people.len(), 1);

// Text search (full-text index)
let developers = graph.walk()
    .vertices(Vertex::person_by_bio("developer"))
    .collect::<Vec<_>>();
assert_eq!(developers.len(), 1);

// Traversal
let alices_creations = graph.walk()
    .vertices(Vertex::person_by_name("Alice"))
    .edges(Edge::created().direction(Direction::Outgoing))
    .head()
    .collect::<Vec<_>>();
assert_eq!(alices_creations.len(), 1);
```

## Advanced Features

`SimpleGraph` supports several advanced features:

### Custom Indexing

You can define custom indexes on vertex and edge properties:

```rust
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index(hash)]
        id: u64,
        #[index(range)]
        score: f64,
        #[index(full_text)]
        description: String,
    },
}
```

### Mutation with Index Updates

When properties are modified, indexes are automatically updated:

```rust
// Update an indexed property
graph.vertex_mut(vertex_id)
.unwrap()
.project_mut::<PersonMut<_ > > ()
.unwrap()
.set_age(31);

// The age index is automatically updated, so this query now works
let people = graph.walk()
.vertices(Vertex::person_by_age(31..32))
.collect::<Vec<_ > > ();
assert_eq!(people.len(), 1);
```
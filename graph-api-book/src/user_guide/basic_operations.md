# Basic Operations

## Overview

Once you've defined your graph model, you can perform basic operations like adding vertices and edges, querying the
graph, and modifying elements. This guide covers the fundamental operations available in Graph API.

## Creating a Graph

To start working with graphs, first create a new graph instance:

```rust,noplayground
{{#include basic_operations/create_graph.rs:create_graph}}
```

## Adding Vertices

Once you have a graph instance, you can add vertices to it:

```rust,noplayground
{{#include basic_operations/create_graph.rs:add_vertices}}
```

## Adding Edges

Edges connect vertices in your graph:

```rust,noplayground
{{#include basic_operations/create_graph.rs:add_edges}}
```

## Querying Vertices

You can query vertices using the walker API. Here are some examples:

### Full Scan

When you need to find all vertices in the graph:

```rust,noplayground
{{#include basic_operations/vertices_query.rs:scan_all}}
```

### Label-Based Lookup

For more efficient queries, use label-based indexes:

```rust,noplayground
{{#include basic_operations/vertices_query.rs:label_index}}
```

### Property Filtering

Filter vertices based on their properties:

```rust,noplayground
{{#include basic_operations/vertices_query.rs:property_filter}}
```

## Working with Edges

The walker API also provides powerful tools for working with edges:

### Finding Connected Edges

Get all edges connected to a vertex:

```rust,noplayground
{{#include basic_operations/edges_query.rs:all_edges}}
```

### Directional Edge Queries

Specify whether you want incoming or outgoing edges:

```rust,noplayground
{{#include basic_operations/edges_query.rs:directional}}
```

### Label-Based Edge Filtering

Filter edges by their label:

```rust,noplayground
{{#include basic_operations/edges_query.rs:label_filter}}
```

## Modifying the Graph

You can modify the graph during traversal using the mutation API:

### Updating Vertex Properties

```rust,noplayground
{{#include basic_operations/mutate_graph.rs:update_vertices}}
```

### Verifying Changes

After modification, you can verify the changes:

```rust,noplayground
{{#include basic_operations/mutate_graph.rs:verify_changes}}
```

## Next Steps

Now that you understand the basic operations, you can:

1. Learn about [Advanced Traversals](./traversal.md) using the Walker API
2. Explore [Context and State](./walker/context_system) in graph traversals
3. See [Examples of Walker Steps](./walker/steps.md) to build more complex queries
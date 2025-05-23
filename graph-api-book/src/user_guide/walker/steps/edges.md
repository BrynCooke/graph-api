# Edges Step

The `edges` step traverses from vertices to their connecting edges, allowing navigation along relationships in the
graph. This step shifts the walker's position from vertices to their adjacent edges, transforming a stream of vertices into a stream of edges.

<object type="image/svg+xml" data="edges/image.svg" title="Edges Step Diagram">
Edges step diagram showing traversal from a vertex to its outgoing edges
</object>

In this diagram:

- An **Input Stream** contains vertex **A**.
- Vertex **A** has outgoing edges: **A->B** (likes) and **A->C** (created).
- The **`.edges(EdgeSearch::scan())`** step processes vertex **A**.
- The **Output Stream** contains the edge elements **A->B** and **A->C** connected to vertex A.

## Syntax

```rust,noplayground
walker.edges(search_criteria)
```

Where `search_criteria` is an `EdgeSearch` object or a predefined search from an index.

## Parameters

- `search_criteria`: An `EdgeSearch` object that defines criteria for selecting edges, including:
    - Edge labels
    - Direction (incoming, outgoing, or both)
    - Property values (when supported)

## Return Value

Returns a new walker positioned at the edges matching the search criteria.

## Examples

### Finding All Connected Edges

Get all edges connected to a vertex:

```rust,noplayground
{{#include edges/edges_examples.rs:all_edges}}
```

### Directional Edge Queries

Specify whether you want incoming or outgoing edges:

```rust,noplayground
{{#include edges/edges_examples.rs:directional}}
```

### Label-Based Edge Filtering

Filter edges by their label:

```rust,noplayground
{{#include edges/edges_examples.rs:label_filter}}
```

### Combined Filtering

Combine direction and label filtering:

```rust,noplayground
{{#include edges/edges_examples.rs:combined_filter}}
```

## Best Practices

- Specify the direction when possible to limit the search space
- Use label-based indexes to avoid scanning all edges
- Follow edges step with head() or tail() to continue vertex-based traversals
- Consider the naming of relationships to match conceptual understanding

## Common Use Cases

- **Relationship navigation**: Moving from vertices to their connections
- **Filtered relationships**: Finding specific types of connections between vertices
- **Direction-specific queries**: Finding incoming or outgoing relationships
- **Relationship property examination**: Inspecting metadata on connections

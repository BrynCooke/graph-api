# Vertices Step

The `vertices` step starts a traversal by selecting a set of vertices from the graph based on specified criteria. This
is typically the first step in a traversal chain.

{{#include images/vertices.svg}}

## Syntax

```rust,noplayground
graph.walk().vertices(search_criteria)
```

Where `search_criteria` is a `VertexSearch` object or a predefined search from an index.

## Parameters

- `search_criteria`: A `VertexSearch` object that defines the criteria for selecting vertices

## Return Value

Returns a new walker positioned at the vertices matching the search criteria.

## Examples

### Full Scan

When you need to find all vertices in a graph:

```rust,noplayground
{{#include vertices/vertices_examples.rs:scan_all}}
```

### Using a Label Index

For more efficient queries, use label-based indexes:

```rust,noplayground
{{#include vertices/vertices_examples.rs:label_index}}
```

### Property-Based Filtering

Find vertices based on their properties:

```rust,noplayground
{{#include vertices/vertices_examples.rs:property_filter}}
```

### Combined Filtering

Chain multiple conditions for complex queries:

```rust,noplayground
{{#include vertices/vertices_examples.rs:combined_filter}}
```

## Best Practices

- Start with the most specific index when possible instead of scanning all vertices
- Use specialized indexes for frequently queried properties to improve performance
- Combine multiple search criteria to narrow results early in the traversal
- For ordered results, rely on range indexes rather than sorting later

## Common Use Cases

- **Entity retrieval**: Finding vertices of a specific type (e.g., all users, products, etc.)
- **Initial selection**: Starting traversals by selecting entry points based on criteria
- **Filtered starting sets**: Beginning with a targeted subset that matches complex conditions
- **Index-driven queries**: Leveraging custom indexes for specialized lookups based on specific properties

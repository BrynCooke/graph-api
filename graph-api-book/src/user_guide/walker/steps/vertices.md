# Vertices Step

The `vertices` step starts a traversal by selecting a set of vertices from the graph based on specified criteria.

## Syntax

```rust,noplayground
graph.walk().vertices(search_criteria)
```

Where `search_criteria` is a `VertexSearch` object or a predefined search from an index.

## Parameters

- `search_criteria`: A `VertexSearch` object that defines the criteria for selecting vertices

## Return Value

Returns a new walker positioned at the vertices matching the search criteria.

## Diagram

```bob
Before step:
  [A] --- [B] --- [C]
  Position: None (traversal not started)

After step:
  [A]* --- [B] --- [C]*
  Position: Vertices matching search criteria (marked with *)
```

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

## Notes

- The `vertices` step is typically the first step in a traversal
- Performance varies based on the search criteria:
    - Using a label index (e.g., `VertexIndex::person()`) is faster than a full scan
    - Property-based searches are efficient when properly indexed
    - Full scans with `VertexSearch::scan()` should be avoided for large graphs
- The order of vertices in the traversal is not guaranteed unless using an ordered index
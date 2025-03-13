# Vertices Step

The `vertices` step starts a traversal by selecting a set of vertices from the graph based on specified criteria.

## Syntax

```rust
graph.walk().vertices(search_criteria)
```

Where `search_criteria` is a `VertexSearch` object or a predefined search from an index.

## Parameters

- `search_criteria`: A `VertexSearch` object that defines the criteria for selecting vertices

## Return Value

Returns a new walker positioned at the vertices matching the search criteria.

## Diagram

```
Before:
  Graph: [A]---[B]---[C]
  Position: None (traversal not started)

After:
  Graph: [A]*---[B]---[C]
  Position: Vertices matching search criteria (marked with *)
```

## Example

{% include_fn ./examples/vertices.rs:vertices_step_example %}

## Notes

- The `vertices` step is typically the first step in a traversal
- Performance varies based on the search criteria:
  - Using a label index (e.g., `VertexIndex::person()`) is faster than a full scan
  - Property-based searches are efficient when properly indexed
  - Full scans with `VertexSearch::scan()` should be avoided for large graphs
- The order of vertices in the traversal is not guaranteed unless using an ordered index
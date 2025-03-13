# Edges Step

The `edges` step traverses from vertices to their connecting edges, allowing navigation along relationships in the graph.

## Syntax

```rust
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

## Diagram

```
Before:
  Graph: [A]*---likes--->[B]---created--->[C]
  Position: Vertex A (marked with *)

After (with outgoing edges):
  Graph: [A]---likes--->*[B]---created--->[C]
  Position: Edge 'likes' (marked with *)
```

## Example

{% include_fn ./examples/edges.rs:edges_step_example %}

## Notes

- The `edges` step changes the traversal position from vertices to edges
- Use `head()` or `tail()` steps afterward to navigate back to vertices
- Direction matters:
  - `.outgoing()`: Edges where the current vertex is the source
  - `.incoming()`: Edges where the current vertex is the target
  - Default (no direction specified): Both incoming and outgoing edges
- Using label-based indexes (e.g., `EdgeIndex::created()`) is much more efficient than scanning
- Filtering by edge properties is possible but requires those properties to be indexed
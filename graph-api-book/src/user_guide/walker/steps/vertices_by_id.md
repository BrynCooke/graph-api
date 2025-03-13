# Vertices By ID Step

The `vertices_by_id` step starts a traversal from a specific set of vertices identified by their IDs.

## Syntax

```rust
graph.walk().vertices_by_id(ids)
```

Where `ids` is an iterator yielding vertex IDs.

## Parameters

- `ids`: An iterator yielding vertex IDs to include in the traversal

## Return Value

Returns a new walker positioned at the vertices with the specified IDs.

## Diagram

```
Before:
  Graph: [A(id=1)]---[B(id=2)]---[C(id=3)]
  Position: None (traversal not started)

After (with ids=[2]):
  Graph: [A(id=1)]---[B(id=2)]*---[C(id=3)]
  Position: Vertex with ID=2 (marked with *)
```

## Example

{% include_fn ./examples/vertices_by_id.rs:vertices_by_id_example %}

## Notes

- This step is the most efficient way to start a traversal when you already know the exact vertex IDs
- The traversal will contain vertices in the same order as the IDs in the input iterator
- If an ID doesn't correspond to any vertex in the graph, it is simply skipped (no error is thrown)
- Commonly used for:
  - Following up on previous traversal results
  - Starting from known entry points (e.g., a user's profile vertex)
  - Building multi-hop traversals from a specific starting point
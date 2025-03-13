# Tail Step

The `tail` step navigates from edges to their target (destination) vertices, allowing traversal to where the edges point to.

## Syntax

```rust
walker.tail()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker positioned at the target vertices of the edges in the current traversal.

## Diagram

```
Before:
  Graph: [A]---follows--->*[B]---created--->[C]
  Position: Edge 'follows' (marked with *)

After:
  Graph: [A]---follows--->[B]*---created--->[C]
  Position: Vertex B, the target of the 'follows' edge (marked with *)
```

## Example

{% include_fn ./examples/head_tail.rs:head_tail_example %}

## Notes

- The `tail` step can only be used after an edge traversal step
- For directed edges, the "tail" is the target or destination vertex
- This step is essential for following relationships in the graph
- Common patterns:
  - `vertices(...).edges(...).tail()` follows relationships to find connected vertices
  - Chaining multiple sequences (`edges().tail().edges().tail()`) to traverse multiple hops
- In social graphs, a typical "friends of friends" query uses two edge-tail sequences
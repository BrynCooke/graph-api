# Head Step

The `head` step navigates from edges to their source (origin) vertices, allowing traversal back to where the edges start from.

## Syntax

```rust
walker.head()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker positioned at the source vertices of the edges in the current traversal.

## Diagram

```
Before:
  Graph: [A]---follows--->*[B]---created--->[C]
  Position: Edge 'follows' (marked with *)

After:
  Graph: [A]*---follows--->[B]---created--->[C]
  Position: Vertex A, the source of the 'follows' edge (marked with *)
```

## Example

{% include_fn ./examples/head_tail.rs:head_tail_example %}

## Notes

- The `head` step can only be used after an edge traversal step
- For directed edges, the "head" is the source or origin vertex
- This step is useful for finding:
  - Who created something (when traversing incoming "created" edges)
  - The starting point of a relationship
  - The author of an action
- Common pattern: `vertices(...).edges(...).head()` finds vertices with specific outgoing edges
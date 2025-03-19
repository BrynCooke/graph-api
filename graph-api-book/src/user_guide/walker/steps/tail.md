# Tail Step

The `tail` step navigates from edges to their target (destination) vertices, allowing traversal to where the edges point
to.

## Syntax

```rust,noplayground
walker.tail()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker positioned at the target vertices of the edges in the current traversal.

## Diagram

```bob
Before step:
  [A] --- follows* ---> [B] --- created ---> [C]
  Position: Edge 'follows' (marked with *)

After step:
  [A] --- follows ---> [B]* --- created ---> [C]
  Position: Vertex B, the target of the 'follows' edge
```

## Examples

### Basic Tail Step

Find projects created by following "created" edges to their target:

```rust,noplayground
{{#include head_tail/head_tail_examples.rs:tail_example}}
```

### Multi-Step Traversal

Traverse multiple relationships to find indirect connections:

```rust,noplayground
{{#include head_tail/head_tail_examples.rs:multi_step}}
```

## Notes

- The `tail` step can only be used after an edge traversal step
- For directed edges, the "tail" is the target or destination vertex
- This step is essential for following relationships in the graph
- Common patterns:
    - `vertices(...).edges(...).tail()` follows relationships to find connected vertices
    - Chaining multiple sequences (`edges().tail().edges().tail()`) to traverse multiple hops
- In social graphs, a typical "friends of friends" query uses two edge-tail sequences
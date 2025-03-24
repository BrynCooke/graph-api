# Tail Step

The `tail` step navigates from edges to their target (destination) vertices, allowing traversal to where the edges point
to.

```pikchr
# Graph structure - vertices with edge highlighted
A: box rad 10px width 0.5 height 0.3 "A" fill white
B: box same at 1.5 right of A "B" fill white

# Connect vertices with labeled edges, highlight follows edge
FOLLOWS: arrow thick color green from A.e to B.w "follows" above

text at 0.4 below FOLLOWS "Before tail(): Position at 'follows' edge (highlighted green)"

# Second diagram - after tail step, highlight target vertex
Aprime: box rad 10px width 0.5 height 0.3 at 1 below A "A" fill white
Bprime: box same at 1.5 right of Aprime "B" fill lightgreen

# Connect vertices with labeled edges
FOLLOWS: arrow from Aprime.e to Bprime.w "follows" above

text at 0.4 below FOLLOWS "After tail(): Position at target vertex B (highlighted green)"
```

## Syntax

```rust,noplayground
walker.tail()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker positioned at the target vertices of the edges in the current traversal.

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
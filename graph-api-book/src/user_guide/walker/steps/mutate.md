# Mutate Step

The `mutate` step allows you to modify vertices or edges during a traversal, enabling batch updates to graph elements.
Unlike most other steps, this requires a mutable traversal started with `walk_mut()`.

```pikchr
# Graph structure with all vertices active in traversal
A: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
B: box same at 1 right of A "B" fill lightgreen
C: box same at 1 right of B "C" fill lightgreen
D: box same at 1 right of C "D" fill lightgreen

# Connect vertices with edges
line from A.e to B.w
line from B.e to C.w
MID: line from C.e to D.w

# Show mutating operation
CollectionBox: box rad 10px at 0.5 below 1.0 right of D "Collect" bold "Vec[A,B,C,D]" fit fill lightyellow

# Show mutating operation
MutationBox: box rad 10px at 0.8 below CollectionBox "Mutate" bold "|&mut Graph, element, ctx|" "{ <Mutate elements> }" fit fill lightyellow

# Show arrows indicating the mutation operation
arrow from A.s down until even with CollectionBox then to CollectionBox.w rad 20px
arrow from B.s down until even with CollectionBox then to CollectionBox.w rad 20px
arrow from C.s down until even with CollectionBox then to CollectionBox.w rad 20px
arrow from D.s down until even with CollectionBox then to CollectionBox.w rad 20px

arrow from CollectionBox.s to MutationBox.n 

text at 0.4 below MutationBox "Use the Graph API for mutations (traversal terminates)"
```

## Syntax

```rust,noplayground
graph.walk_mut()
    .vertices(...)
    .mutate(|graph, element, context| {
        // modification logic
    })
```

## Parameters

- `modifier`: A function that takes:
    - A mutable reference to the current element (vertex or edge)
    - The element's context
    - Performs in-place modifications to the element

## Return Value

Returns the same traversal unchanged, allowing you to continue chaining steps.

## Examples

### Updating Vertex Properties

This example shows how to update properties of vertices during traversal:

```rust,noplayground
{{#include mutate/mutate_example.rs:update_vertex}}
```

### Adding Edges

This example demonstrates using the mutate step to add new connections to the graph:

```rust,noplayground
{{#include mutate/mutate_example.rs:add_edges}}
```

## Notes

- The mutate step requires a mutable graph traversal started with `walk_mut()`
- It allows for in-place modification of graph elements without rebuilding the graph
- Perfect for batch updates or incremental changes to graph properties
- Each element is processed independently - changes to one element don't affect traversal of others
- Use pattern matching to handle different vertex/edge types appropriately
- Can be combined with filter steps to selectively modify elements matching specific criteria
- Changes are immediately visible to subsequent steps in the traversal
- Mutations that affect graph structure (adding/removing edges) should generally be done separately
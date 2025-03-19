# Mutate Step

The `mutate` step allows you to modify vertices or edges during a traversal, enabling batch updates to graph elements.

## Syntax

```rust,noplayground
graph.walk_mut()
    .vertices(...)
    .mutate(|element, context| {
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

## Diagram

```bob
Before step:
  [A:age=30]* --- [B:age=25]* --- [C:age=40]*
  Position: All vertices in traversal

After step (increment age):
  [A:age=31]* --- [B:age=26]* --- [C:age=41]*
  Position: Same vertices, but now modified
```

## Example

```rust,noplayground
{{#include mutate/mutate_example.rs:all}}
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
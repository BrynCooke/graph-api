# Mutate Step

The `mutate` step allows you to modify vertices or edges during a traversal, enabling batch updates to graph elements.
Unlike most other steps, this requires a mutable traversal started with `walk_mut()`.

<object type="image/svg+xml" data="mutate/image.svg">
Mutate step diagram showing element properties being modified in-place
</object>

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

## Best Practices

- Always start with `walk_mut()` when planning to use the mutate step
- Use pattern matching to handle different vertex/edge types appropriately
- Combine with filter steps to selectively apply mutations to elements matching specific criteria
- Keep structural changes (adding/removing edges) separate from property updates when possible

## Common Use Cases

- **Batch updates**: Applying the same change to multiple elements matching criteria
- **Property maintenance**: Updating attributes based on computations or external data
- **Status changes**: Modifying state properties across a subset of graph elements
- **Relationship enhancement**: Adding metadata to edges based on vertex properties

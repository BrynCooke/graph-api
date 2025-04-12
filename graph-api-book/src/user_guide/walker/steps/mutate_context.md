# Mutate Context

The `mutate_context` step allows you to modify context data during a graph traversal without affecting the traversal position. This provides a way to update state or accumulate information as you move through the graph.

<object type="image/svg+xml" data="mutate_context/image.svg" title="Mutate Context Step Diagram">
Mutate context step diagram showing context being modified while elements remain unchanged
</object>

In this diagram:

- An **Input Stream** contains elements **A** and **B**, each associated with a context value.
- The **`.mutate_context(|e, ctx| { /* modify ctx */ })`** step is applied, allowing in-place modification of the context associated with each element.
- The resulting **Output Stream** shows the same elements (**A** and **B**) but with modified context values.
- This demonstrates that `mutate_context` affects only the context data, not the traversal elements or position.

## Syntax

```rust,noplayground
walker.mutate_context(|element, context| {
    // Modify context in-place via mutable reference
})
```

## Parameters

- `callback`: A function that takes:
    - A reference to the current element (vertex or edge)
    - A mutable reference to the current context
    - Performs in-place modifications to the context

## Return Value

Returns a new walker with the same position but modified context data.

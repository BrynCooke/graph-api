# Mutate Step

The `mutate` step iterates over elements in the traversal, applying a callback function to each one. This callback receives mutable access to the graph, allowing for **in-place modification** of vertices or edges. `mutate` is a terminal step that consumes the walker and returns the total count (`usize`) of elements processed.

<object type="image/svg+xml" data="mutate/image.svg" title="Mutate Step Diagram">
Mutate step diagram showing elements being modified in-place and the step returning a count
</object>

In this diagram:

- **Input Elements**: The walker starts with elements **A, B, C**, each having an initial value (e.g., `val=1`).
- The **`.mutate(callback)`** step processes each element. The callback function has access to modify the graph.
- **Mutated Elements (In Graph)**: The diagram shows the state of the elements *after* the mutation callback has been applied (e.g., **A'** with `val=10`). This highlights the in-place nature of the operation.
- **Returns: usize**: The step completes and returns the count of elements processed (3 in this case), terminating the walker chain.

## Syntax

```rust,noplayground
walker.mutate(|element, context, graph| {
    // mutation logic using graph access
})
```

## Parameters

- `callback`: A function that takes:
    - A reference to the current element (vertex or edge) - Note: Direct mutable access to the *element itself* might vary; mutation often happens via the `graph` reference using the element's ID.
    - The element's context
    - A mutable reference to the graph (`&mut G`)
    - Performs modifications using the mutable graph reference (e.g., `graph.vertex_mut(element.id())`).

## Return Value

Returns a `usize` representing the number of elements processed by the `mutate` step.

## Example

```rust,noplayground
{{#include mutate/mutate_example.rs:update_vertex}}
```

## Best Practices

- Use `mutate` specifically for modifying graph elements based on traversal results.
- Access mutable elements via the provided `graph` reference and the element's ID (e.g., `graph.vertex_mut(id)`).
- Be mindful that mutations are applied directly and immediately to the graph state.
- Consider using `filter` or `control_flow` *before* `mutate` to precisely target which elements should be modified.
- Understand the graph implementation's behavior regarding mutations during iteration (e.g., adding/removing elements might invalidate iterators in some graph types).

## Common Use Cases

- **Updating properties**: Modifying attributes of vertices or edges (e.g., incrementing age, changing status).
- **Graph cleaning**: Standardizing data or fixing inconsistencies found during traversal.
- **State transitions**: Updating element states based on workflow logic during traversal.
- **Bulk updates**: Applying a change to a set of elements identified by prior traversal steps.

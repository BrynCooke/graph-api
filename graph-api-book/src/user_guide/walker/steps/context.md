# Context

The `push_context` step allows you to carry information along a graph traversal, making it possible to access data from
previous steps while working with the current element. Context creates a typed value that travels with the traversal
without changing its position.

<object type="image/svg+xml" data="context/image.svg" title="Push Context Step Diagram">
Context step diagram showing context value traveling with the walker
</object>

In this diagram:

- **Before `push_context()`**: The walker is positioned at vertex **V1**.
- The **code snippet** on the left shows the `.push_context(|v| v.id)` step being applied.
- **After `push_context()`**: The walker is still at **V1**, but now has an associated **Context box** containing the value `"V1_ID"` (the result of the context function).
- **Later (at V2)**: The walker has moved to vertex **V2**. The **Context box** (still containing `"V1_ID"`) remains associated with the walker, demonstrating that the context value travels along with the traversal state.

## Methods for Adding Context

```rust,noplayground
// Adding context to a traversal
walker.push_context(|element, current_context| {
    // Create and return new context value
})

// Adding current vertex/edge as context
walker.push_default_context()
```

## Parameters

- `context_fn`: A function that takes:
    - A reference to the current element (vertex or edge)
    - The current context (if any)
    - Returns a new context value to be carried along the traversal

## Return Value

Returns a new walker with the context added, preserving the current traversal position.

## Accessing Context

Any step that accepts a function with context (like `map`, `filter`, etc.) will receive:

- The current element as the first parameter
- The context as the second parameter

```rust,noplayground
// Using context in a map step
walker
    .push_context(|v, _| v.id())  // Store vertex ID
    .map(|current, context| {
        // Use the stored vertex ID from context
        format!("Current: {}, Source: {}", current.id(), context)
    })
```

## Examples

### Vertex Context

Store information about the source vertex during a traversal:

```rust,noplayground
{{#include context/context_examples.rs:vertex_context}}
```

### Edge Context

Track edge types during traversal:

```rust,noplayground
{{#include context/context_examples.rs:edge_context}}
```

### Path Tracking

Build a representation of the path taken during traversal:

```rust,noplayground
{{#include context/context_examples.rs:path_tracking}}
```

## Type Safety

The context system is fully type-safe:

- Each context value has a concrete type
- Context transformation functions must return the correct type
- Closures that receive context are provided with the correctly typed context

## Common Use Cases

- **Path tracking**: Store the sequence of vertices or edges traversed
- **Metadata collection**: Gather information from different parts of the graph
- **Aggregation**: Build up composite results during traversal
- **Decision making**: Use information from earlier steps to influence later decisions

## Best Practices

- Keep contexts immutable - create new contexts rather than modifying existing ones
- Use `push_default_context()` when you simply need to track the current vertex/edge
- Chain multiple context operations to build complex data structures
- Consider type-safety when designing context pipelines

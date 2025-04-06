# Default Context

The `push_default_context` step provides a simplified way to track vertex and edge information during traversal, without
having to define custom context types. It automatically captures the current vertex or edge to make it available in
downstream steps.

<object type="image/svg+xml" data="default_context/image.svg" title="Push Default Context Step Diagram">
Default Context step diagram showing the context automatically tracking the current element
</object>

In this diagram:

- An **Input Stream** contains elements **A** and **B**.
- The **`.push_default_context()`** step is applied.
- In the **Output Stream**, each element is paired with a context that automatically contains *that specific element*.
  - Element **A** is paired with `Context: A`.
  - Element **B** is paired with `Context: B`.
- This shows how the default context dynamically reflects the current element being processed.

## Syntax

```rust,noplayground
walker.push_default_context()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker with the default context added, preserving the current traversal position.

## Examples

### Tracking Relationships

Use default context to describe relationships between people:

```rust,noplayground
{{#include default_context/default_context_example.rs:knows_example}}
```

### Working with Edge Properties

Combine edge properties with source vertex information:

```rust,noplayground
{{#include default_context/default_context_example.rs:edge_properties}}
```

## Default Context Structure

The default context automatically tracks:

- For vertices: The current vertex reference
- For edges: Both the source and target vertex references

## Accessing Default Context

You can access the context in subsequent steps like this:

```rust,noplayground
// After pushing default context
walker
    .push_default_context()
    .map(|current_element, ctx| {
        // Access vertex from context
        let context_vertex = ctx.vertex();
        
        // Work with the context vertex
        // ...
    })
```

## Best Practices

- Use default context for simple element tracking rather than creating custom context types
- Chain default contexts in multi-step traversals to maintain element history
- Access context values using the appropriate type-safe methods (e.g., `ctx.vertex()`, `ctx.edge()`)
- Consider default context before writing complex custom context functions for basic traversals

## Common Use Cases

- **Relationship description**: Building natural language descriptions (e.g., "Person A knows Person B")
- **Path tracking**: Recording the sequence of vertices and edges in a traversal
- **Element comparison**: Comparing properties between current and previous elements
- **Data collection**: Gathering information from connected elements in the graph

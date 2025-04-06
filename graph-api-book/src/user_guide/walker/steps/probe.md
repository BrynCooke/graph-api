# Probe Step

The `probe` step allows you to execute a callback function for each element in the traversal, primarily for **side effects** like logging, metrics collection, or debugging, without altering the elements themselves or the flow of the traversal.

<object type="image/svg+xml" data="probe/image.svg" title="Probe Step Diagram">
Probe step diagram showing elements flowing through, side effects, and unchanged output stream
</object>

In this diagram:

- **Input Elements**: The walker starts with elements **A, B, C**.
- The **`.probe(callback)`** step processes each element.
- **Side Effect**: As each element (A, B, C) passes through, the provided callback function is executed. This is shown triggering actions like logging or updating metrics.
- **Output Elements (Unchanged)**: The elements **A, B, C** continue to the next step in the traversal, completely unaffected by the `probe` operation.

## Syntax

```rust,noplayground
walker.probe(|element, context| {
    // inspection logic
})
```

## Parameters

- `inspector`: A function that takes:
    - A reference to the current element (vertex or edge)
    - The element's context
    - Performs some side effect like logging or debugging

## Return Value

Returns the same traversal unchanged, allowing you to continue chaining steps.

## Examples

### Inspecting Vertices

This example shows how to use the `probe` step to inspect and count vertices during traversal:

```rust,noplayground
{{#include probe/probe_example.rs:probe_vertices}}
```

### Inspecting Edges

This example demonstrates using the `probe` step to examine relationships between vertices:

```rust,noplayground
{{#include probe/probe_example.rs:probe_edges}}
```

## Best Practices

- Insert probe steps at key points in complex traversals to verify correct behavior
- Use descriptive logging within probes to make debugging output meaningful
- Add counters or statistics collection to understand traversal performance
- Keep probe functions simple and side-effect only; don't try to affect the traversal flow

## Common Use Cases

- **Debugging**: Inserting temporary probe steps to understand traversal behavior
- **Logging**: Recording traversal information during development or in production
- **Metrics collection**: Gathering statistics about traversal performance and results
- **Inspection**: Examining element properties at specific points in the traversal

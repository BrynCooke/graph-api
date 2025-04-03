# Probe Step

The `probe` step allows you to inspect or log vertices or edges during a traversal without affecting the traversal flow.
It's perfect for debugging or monitoring what's happening in your traversal.

<object type="image/svg+xml" data="probe/image.svg" title="Probe Step Diagram">
Probe step diagram showing side effects being triggered without affecting traversal
</object>

In this diagram:

- **Before `probe()`**: The walker contains highlighted elements **V1, V2, V3**.
- The **code snippet** on the left shows the `.probe(|v, _| ...)` step being applied.
- **After `probe()`**: The walker state remains **identical** to the "Before" state, with **V1, V2, V3** still highlighted.
- A separate **Side Effect box** (e.g., representing console output) is shown, indicating the action performed by the `probe` function for each element.
- The traversal continues to the next step, **unaffected** by the `probe`.

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

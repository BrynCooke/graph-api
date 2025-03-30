# Probe Step

The `probe` step allows you to inspect or log vertices or edges during a traversal without affecting the traversal flow.
It's perfect for debugging or monitoring what's happening in your traversal.

![Probe Step Diagram](images/probe.svg)

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

# Probe Step

The `probe` step allows you to inspect or log vertices or edges during a traversal without affecting the traversal flow.
It's perfect for debugging or monitoring what's happening in your traversal.

```pikchr
# Graph structure with all vertices active in traversal
A: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
B: box same at 1 right of A "B" fill lightgreen
C: box same at 1 right of B "C" fill lightgreen

# Connect vertices with edges
line from A.e to B.w
line from B.e to C.w

text at 0.4 below B "Before probe(): All vertices in traversal (highlighted green)"

# During probe with logging indicators
Aprobe: box rad 10px width 0.5 height 0.3 at 1 below A "A" fill lightgreen
Bprobe: box same at 1 right of Aprobe "B" fill lightgreen
Cprobe: box same at 1 right of Bprobe "C" fill lightgreen

line from Aprobe.e to Bprobe.w
MIDprobe: line from Bprobe.e to Cprobe.w

# Show log messages for each element
LogA: box height 0.2 width 0.8 fill lightyellow at 0.4 below Aprobe "println!(\"A\")"
LogB: box height 0.2 width 0.8 fill lightyellow at 0.4 below Bprobe "println!(\"B\")"
LogC: box height 0.2 width 0.8 fill lightyellow at 0.4 below Cprobe "println!(\"C\")"

arrow from Aprobe.s to LogA.n
arrow from Bprobe.s to LogB.n
arrow from Cprobe.s to LogC.n

text at 0.25 below LogB "During probe(): Side effects happen (logging, etc.)"
```

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
# Default Context

The `default_context` step provides a simplified way to track vertex and edge information during traversal, without
having to define custom context types.

## Syntax

```rust,noplayground
walker.push_default_context()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker with the default context added, preserving the current traversal position.

## Diagram

```bob
Before step:
  [Person:Alice]* --- knows ---> [Person:Bob]
  Context: None
  Position: At Alice vertex

After push_default_context step:
  [Person:Alice]* --- knows ---> [Person:Bob]
  Context: DefaultContext(Alice)
  Position: Still at Alice vertex, with default context
```

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

## Notes

- Default context is the simplest way to track graph elements during traversal
- No need to define custom context types or write context creation functions
- Especially useful for traversals where you need to reference the previous vertex
- Common use cases:
    - Building relationship descriptions (e.g., "Person A knows Person B")
    - Tracking traversal paths
    - Comparing current elements with previous elements
    - Accumulating data from multiple traversal steps
- Automatically updates as the traversal moves through the graph
- More efficient than manually tracking elements in custom contexts

# Default Context

The `default_context` step provides a simplified way to track vertex and edge information during traversal, without
having to define custom context types. It automatically captures the current vertex or edge to make it available in
downstream steps.

```pikchr
# Graph structure - starting at vertex A
A: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
B: box same at 1.5 right of A "B" fill white 
C: box same at 1.5 right of B "C" fill white

# Connect vertices with labeled edges
arrow from A.e to B.w "Follows" above
arrow from B.e to C.w "Created" above

# Initial state with no context
text at 0.4 below B "Before push_context: At A with no context"

# After push_context - position unchanged but context added
Aprime: box rad 10px width 0.5 height 0.3 at 1.4 below A "A" fill lightgreen
Bprime: box same at 1.5 right of Aprime "B" fill white
Cprime: box same at 1.5 right of Bprime "C" fill white

# Connect vertices with labeled edges
arrow from Aprime.e to Bprime.w "Follows" above
arrow from Bprime.e to Cprime.w "Created" above

# Show the context carried with the traversal
ContextBox: box rad 10px height 0.2 width 1.0 fill lightyellow at 0.4 below Aprime "element: A"

# Connect context to current position
arrow <-> from Aprime.s to ContextBox.n color black

text at 0.7 below Bprime "After push_context: Still at A, but with A element in context"

# Show how context follows the traversal to new positions
Aprime2: box rad 10px width 0.5 height 0.3 at 1.4 below Aprime "A" fill white
Bprime2: box same at 1.5 right of Aprime2 "B" fill lightgreen
Cprime2: box same at 1.5 right of Bprime2 "C" fill white

# Connect vertices with labeled edges
arrow from Aprime2.e to Bprime2.w "Follows" above
arrow from Bprime2.e to Cprime2.w "Created" above

# Show the context still carried with the traversal at new position
ContextBox2: box rad 10px height 0.2 width 1.0 fill lightyellow at 0.4 below Bprime2  "element: B" 

# Connect context to new position
arrow <-> from Bprime2.s to ContextBox2.n color black

text at 0.7 below Bprime2 "Later in traversal: Context moves with the traversal to B"
```

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

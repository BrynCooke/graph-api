# Context

The `context` step allows you to carry information along a graph traversal, making it possible to access data from previous steps while working with the current element. Context creates a typed value that travels with the traversal without changing its position.

```pikchr
# Graph structure - starting at vertex A
A: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
B: box same at 1.5 right of A "B" fill white 
C: box same at 1.5 right of B "C" fill white

# Connect vertices with labeled edges
arrow from A.e to B.w "Follows" above
arrow from B.e to C.w "Created" above

# Initial state with no context
text at 0.4 below B "Before context: At A with no context"

# After push_context - position unchanged but context added
Aprime: box rad 10px width 0.5 height 0.3 at 1.4 below A "A" fill lightgreen
Bprime: box same at 1.5 right of Aprime "B" fill white
Cprime: box same at 1.5 right of Bprime "C" fill white

# Connect vertices with labeled edges
arrow from Aprime.e to Bprime.w "Follows" above
arrow from Bprime.e to Cprime.w "Created" above

# Show the context carried with the traversal
ContextBox: box rad 10px height 0.2 width 1.0 fill lightyellow at 0.4 below Aprime "username: alice"

# Connect context to current position
arrow <-> from Aprime.s to ContextBox.n color black

text at 0.7 below Bprime "After push_context: Still at A, but with user data in context"

# Show how context follows the traversal to new positions
Aprime2: box rad 10px width 0.5 height 0.3 at 1.4 below Aprime "A" fill white
Bprime2: box same at 1.5 right of Aprime2 "B" fill lightgreen
Cprime2: box same at 1.5 right of Bprime2 "C" fill white

# Connect vertices with labeled edges
arrow from Aprime2.e to Bprime2.w "Follows" above
arrow from Bprime2.e to Cprime2.w "Created" above

# Show the context still carried with the traversal at new position
ContextBox2: box rad 10px height 0.2 width 1.0 fill lightyellow at 0.4 below Bprime2  "username: alice" 

# Connect context to new position
arrow <-> from Bprime2.s to ContextBox2.n color black

text at 0.7 below Bprime2 "Later in traversal: Context moves with the traversal to B"
```

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
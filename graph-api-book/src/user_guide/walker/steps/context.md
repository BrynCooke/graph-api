# Context

The `context` system in graph traversals allows you to carry information along the traversal path, making it possible to
reference data from previous steps while working with the current element. Context travels with the traversal without
changing its position.

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

## Syntax

```rust,noplayground
// Adding context to a traversal
walker.push_context(|element, current_context| {
    // Create and return new context value
})

// Accessing context later in the traversal
walker.map(|element, context| {
    // Use the context value
})
```

## Parameters

- `context_fn`: A function that takes:
    - A reference to the current element (vertex or edge)
    - The current context (if any)
    - Returns a new context value to be carried along the traversal

## Return Value

Returns a new walker with the context added, preserving the current traversal position.

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

## Notes

- Contexts are values carried along with each traversal element
- Each step in the traversal can access the latest context value
- You can push multiple contexts in sequence to build up a stack of information
- Context works with any type that can be stored in the traversal
- Common uses include:
    - Tracking the path taken during traversal
    - Carrying properties from earlier vertices/edges
    - Building up compound data structures during traversal
    - Storing metadata about the traversal process

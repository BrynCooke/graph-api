# Context

The `context` system in graph traversals allows you to carry information along the traversal path, making it possible to
reference data from previous steps while working with the current element.

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

## Diagram

```bob
Before step:
  [A]* --- [B] --- [C]
  Context: None
  Position: At vertex A

After push_context step:
  [A]* --- [B] --- [C]
  Context: Some(Data about A)
  Position: Still at vertex A, but with context
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

## Notes

- Contexts are immutable values carried along with each traversal element
- Each step in the traversal can access the latest context value
- You can push multiple contexts in sequence to build up a stack of information
- Context works with any type that can be stored in the traversal
- Common uses include:
    - Tracking the path taken during traversal
    - Carrying properties from earlier vertices/edges
    - Building up compound data structures during traversal
    - Storing metadata about the traversal process

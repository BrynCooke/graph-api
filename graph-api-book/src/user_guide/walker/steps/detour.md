# Detour Step

The `detour` step is a powerful operation that allows you to execute a sub-traversal for *each* element currently in the
main walker stream. The results yielded by each sub-traversal *do not replace* the original element in the main stream.
This enables complex conditional logic and exploration without losing the main traversal context.

<object type="image/svg+xml" data="detour/image.svg" title="Detour Step Diagram">
Detour step diagram showing input elements triggering sub-walkers, whose results form the output stream
</object>

In this diagram:

- **Input Elements**: The main walker stream arrives at the `detour` step with elements **A** and **X**.
- **`.detour(|sub| ...)`**: For each input element, a sub-walker is initiated:
    - **Sub-Walker (from A)**: Starts at A, executes the steps defined in the closure (e.g., `edges().head()`), and
      yields results **B** and **C**.
    - **Sub-Walker (from X)**: Starts at X, executes the same steps, and yields result **Y**.
- **Output Elements (Results)**: The main walker stream continues, now containing the *combined results* from all
  sub-walkers (**A, A, X**). The original elements (A, X) have been replaced.

## Syntax

```rust,noplayground
walker.detour(|sub_walker_builder| {
    // Configure the sub-walker with additional steps
    sub_walker_builder
        .edges(...) // Example step
        .head()     // Example step
        // ... more steps ...
})
```

## Parameters

- `detour_fn`: A closure that receives a `StartWalkerBuilder` positioned at the current element from the main stream.
  This closure defines the steps for the sub-traversal.

## Return Value

Returns a new walker where the elements are the combined results of all the sub-traversals executed within the `detour`.

## Examples

### Basic Detour

This example shows how to use a detour to find projects created by people who follow someone over 30:

```rust,noplayground
{{#include detour/detour_example.rs:basic_detour}}
```

### Nested Detours

This example demonstrates using nested detours for more complex traversal logic:

```rust,noplayground
{{#include detour/detour_example.rs:complex_detour}}
```

## When to use `detour`

Detour shines in several common graph traversal scenarios:

- **Conditional filtering based on connected elements**:  
  For example, "Find people who follow someone over 30" requires exploring connections before making filtering decisions

- **Looking ahead without changing position**:  
  When you need to check properties of elements further in the graph before deciding how to proceed

- **Complex multi-step conditions**:  
  When a condition requires checking multiple properties or relationships that would be awkward with chained filters

- **Collecting contextual information**:  
  When you need to gather information from related elements to inform decisions in the main traversal

- **Avoiding traversal state mutation**:  
  When you want to explore potential paths without permanently changing your traversal state

## Implementation Notes

- The detour closure receives a fresh walker starting at the current position
- All walker operations inside the detour are executed in a sub-traversal context
- Results from the detour replace the current elements in the traversal
- The main traversal continues from where it left off, but with the filtered/transformed elements
- For better performance, try to use indexed lookups within detours whenever possible
- Detours can be nested for extremely complex traversal logic
- Using detours improves code readability by logically grouping related traversal operations

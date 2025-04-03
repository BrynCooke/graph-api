# Detour Step

The `detour` step allows you to temporarily branch off from the main traversal to perform complex operations, and then
continue with the results. It's like taking a side trip during your graph journey to explore or evaluate conditions
before proceeding on your main path.

<object type="image/svg+xml" data="detour/image.svg" title="Detour Step Diagram">
Detour step diagram showing main path branching into a sub-walker and returning results
</object>

In this diagram:

- **Before `detour()`**: The main walker is positioned at vertex **A**.
- **During `detour()`**: The code snippet on the left shows the `.detour()` step being applied. A **Detour Sub-Walker** is initiated from A and explores a separate path (A → D → E). The sub-walker finishes at **E**.
- **After `detour()`**: The main walker's position is updated to the result of the sub-walker (**E**). The main traversal resumes from E, ready to proceed to the next step (e.g., to F).

## Syntax

```rust,noplayground
walker.detour(|sub_walker| {
    // Configure the sub-walker with additional steps
    sub_walker
        .edges(...)
        .head()
        .filter(...)
})
```

## Parameters

- `detour_fn`: A closure that takes the current walker at its current position and returns a modified walker.

## Return Value

Returns a new walker that continues from where the detour ends with the results of the detour applied.

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

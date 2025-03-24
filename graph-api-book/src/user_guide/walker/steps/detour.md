# Detour Step

The `detour` step allows you to temporarily branch off from the main traversal to perform complex operations, and then
continue with the results. It's like taking a side trip during your graph journey to explore or evaluate conditions
before proceeding on your main path.

```pikchr
# Before filter - all vertices 
A: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
B: box same at 1.5 right of A "B" fill white
C: box same at 1.5 right of B "C" fill white
D: box same at 0.5 s of B "D" fill lightgreen


# Connect vertices with edges
line from A.e to B.w
line from B.e to C.w
line from A.e to D.w

text at 0.4 below D "Detour to D"


# Before filter - all vertices 
APrime: box same at 1.5 below A "A" 
BPrime: box same at 1.5 right of APrime "B" 
CPrime: box same at 1.5 right of BPrime "C"
DPrime: box same at 0.5 s of BPrime "D" color gray fill white


# Connect vertices with edges
line from APrime.e to BPrime.w
line from BPrime.e to CPrime.w
line from APrime.e to DPrime.w


text at 0.4 below DPrime "Continue from A"
```

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
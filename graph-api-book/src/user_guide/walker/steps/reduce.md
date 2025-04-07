# Reduce Step

The `reduce` step combines all elements in a traversal using a binary operation, returning a single result element. It repeatedly applies a function to pairs of elements, keeping one from each pair, until only one element remains.

<object type="image/svg+xml" data="reduce/image.svg" title="Reduce Step Diagram">
Reduce step diagram showing pairwise comparison reducing elements to a single result
</object>

In this diagram:

- An **Input Stream** contains elements **A** (age=30), **B** (age=25), and **C** (age=40).
- The **`.reduce(|acc, v| ...)`** step is applied, using a reducer function that keeps the element with the maximum age.
- The **Reduction Process** visualization shows the pairwise comparisons:
    - **A vs B**: Element **A** is kept (30 > 25), **B** is discarded.
    - **(A) vs C**: Element **C** is kept (40 > 30), **A** is discarded.
- The **Result** of the reduction process is the single remaining element (**C**), which continues in the walker chain.

## Syntax

```rust,noplayground
walker.reduce(|accumulated, next_element, context| {
    // combine elements and return either accumulated or next_element
})
```

## Parameters

- `reducer`: A function that takes:
    - The accumulated element from previous reduction steps
    - The next element to be considered
    - The parent walker's context (immutable)
    - Returns a ControlFlow with either the accumulated or next element

## Return Value

Returns an `Option` containing the result element if the traversal is not empty, or `None` if the traversal is empty.

## Example

```rust,noplayground
{{#include reduce/reduce_example.rs:all}}
```

## Best Practices

- Design reducers that follow associative properties when possible
- Handle empty traversals by checking for None in the result
- The reducer can only select one of the elements, not create new ones
- Use `ControlFlow::Continue` to keep reducing, and `ControlFlow::Break` to halt early
- Consider `fold` instead when you need to build a new value rather than select among elements (note: `fold` terminates the walker)
- Remember that unlike the old API, the context is immutable in the reducer function

## Common Use Cases

- **Extrema finding**: Selecting maximum or minimum elements by some property
- **Best match selection**: Choosing the most relevant element from a set of results
- **Representative selection**: Picking a single representative element from similar options
- **Priority determination**: Finding highest/lowest priority elements in a graph

# Reduce Step

The `reduce` step combines all elements in a traversal using a binary operation, returning a single result element. It repeatedly applies a function to pairs of elements until only one remains.

<object type="image/svg+xml" data="reduce/image.svg" title="Reduce Step Diagram">
Reduce step diagram showing pairwise comparison reducing elements to a single result
</object>

In this diagram:

- The **Traversal Elements** are V1, V2, and V3, with their ages.
- The **Reduction** process (e.g., finding the element with the maximum age) happens in steps:
    - **V1 vs V2**: V1 (age 30) is kept, V2 (age 25) is discarded.
    - **V1 vs V3**: V3 (age 40) is kept, V1 (age 30) is discarded.
- The final **Result** is the single remaining element, **V3**.
- This step **terminates the traversal**.

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
- Consider fold instead when you need to build a new value rather than select among elements
- Remember that unlike the old API, the context is immutable in the reducer function

## Common Use Cases

- **Extrema finding**: Selecting maximum or minimum elements by some property
- **Best match selection**: Choosing the most relevant element from a set of results
- **Representative selection**: Picking a single representative element from similar options
- **Priority determination**: Finding highest/lowest priority elements in a graph

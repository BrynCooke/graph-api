# Reduce Step

The `reduce` step combines all elements in a traversal using a binary operation, returning a single result element.

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

## Diagram

```bob
Before step:
  [A]* --- [B]* --- [C]* --- [D]*
  Position: All vertices in traversal

During reduce execution:
  acc = [A] (initial accumulator)
  reducer([A], [B], ctx) → ControlFlow::Continue([A])
  reducer([A], [C], ctx) → ControlFlow::Continue([C])
  reducer([C], [D], ctx) → ControlFlow::Continue([C])

After step:
  [C]* --- ... ---> [More Traversal Steps]
  Result: [C] (single reduced vertex)
  Position: Ready for next step in the walker chain
```

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
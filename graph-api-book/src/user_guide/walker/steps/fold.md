# Fold Step

The `fold` step accumulates a result by processing each element in a traversal, operating like the standard Rust `fold` operation but specifically for graph traversals. It's a powerful terminal operation that builds a single value from all elements.

<object type="image/svg+xml" data="fold/image.svg">
Fold step diagram showing accumulation of values from traversal elements
</object>

## Syntax

```rust,noplayground
walker.fold(initial_value, |accumulator, element, context| {
    // accumulation logic
})
```

## Parameters

- `initial_value`: The starting value for the accumulator
- `f`: A closure that takes:
    - The current accumulator value
    - A reference to the current element (vertex or edge)
    - The current element's context
    - Returns the updated accumulator value

## Return Value

Returns the final accumulated value after processing all elements in the traversal.

## Example

```rust,noplayground
{{#include fold/fold_example.rs:all}}
```

## Best Practices

- Choose an appropriate initial value that handles edge cases (empty traversals)
- Design fold closures to be commutative when possible for predictable results
- Use type annotations for complex accumulator types to improve readability
- Consider specialized steps like `count()` when their behavior matches your needs

## Common Use Cases

- **Aggregation**: Calculating sums, averages, or other numerical aggregates
- **Collection building**: Creating custom collections or data structures from traversal results
- **State tracking**: Building a final state that incorporates data from all elements
- **Custom reductions**: Implementing specialized reduction operations not covered by built-in steps

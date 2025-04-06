# Fold Step

The `fold` step accumulates a result by processing each element in a traversal, operating like the standard Rust `fold` operation but specifically for graph traversals. It's a powerful terminal operation that builds a single value from all elements in the stream.

<object type="image/svg+xml" data="fold/image.svg" title="Fold Step Diagram">
Fold step diagram showing accumulation of values from traversal elements
</object>

In this diagram:

- An **Input Stream** contains elements **A** (age=30), **B** (age=25), and **C** (age=40).
- The **`.fold(0, |acc, v| acc + v.age())`** step is applied. It starts with an initial accumulator value of `0`.
- The **Accumulator** visualization shows the value being updated as each element is processed:
    - Initial: `0`
    - After A: `0 + 30 = 30`
    - After B: `30 + 25 = 55`
    - After C: `55 + 40 = 95`
- The **Final Result** box shows the final accumulated value (`95`).
- This step **Terminates Walker**, meaning no further Graph API steps can be chained after `fold`.

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

# Fold Step

The `fold` step accumulates a result by processing each element in a traversal, operating like the standard Rust `fold` operation but specifically for graph traversals.

## Syntax

```rust
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

## Diagram

```
Before:
  Graph: [A]---[B]---[C]---[D]
  Position: [A]*, [B]*, [C]*, [D]* (all vertices)

During fold:
  Accumulator: Initial → [A] → [B] → [C] → [D] → Final Result
```

## Example

{% include_fn ./fold.rs:vertex_example %}

## Notes

- The fold step is a terminal operation - it consumes the traversal and returns a value
- Unlike map, fold doesn't yield a stream of values but a single accumulated result
- The closure is called once for each element with the accumulator and element
- Can be used for many common operations like counting, summing, finding min/max, etc.
- More flexible than specialized steps like count when you need to calculate custom aggregates
- The accumulator can be any type that matches your needs
- Context is available if you need it for your calculations
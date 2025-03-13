# Reduce Step

The `reduce` step combines all elements in a traversal using a binary operation, returning a single result element.

## Syntax

```rust
walker.reduce(|element1, element2, context1, context2| {
    // combine elements and return one of them
})
```

## Parameters

- `reducer`: A function that takes:
  - Two elements from the traversal
  - The contexts for each element
  - Returns one of the elements as the accumulated result

## Return Value

Returns an `Option` containing the result element if the traversal is not empty, or `None` if the traversal is empty.

## Diagram

```
Before:
  Graph: [A]---[B]---[C]---[D]
  Position: [A]*, [B]*, [C]*, [D]* (all vertices)

During reduce:
  [A] + [B] = [A]
  [A] + [C] = [C]
  [C] + [D] = [C]
  Final Result: [C]
```

## Example

{% include_fn ./reduce.rs:reduce_example %}

## Notes

- The reduce step is a terminal operation - it consumes the traversal and returns a result
- Similar to fold, but works with the traversal elements themselves rather than an external accumulator
- Ideal for finding extreme values (max/min) within a traversal
- The reducer must return one of the input elements, not a new element
- Returns None if the traversal is empty
- If the traversal contains only one element, that element is returned without calling the reducer
- Unlike standard Rust reduce, graph-api-reduce provides the contexts of both elements
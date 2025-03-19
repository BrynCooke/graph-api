# Map Step

The `map` step transforms vertices or edges in the traversal by applying a mapping function, returning an iterator over
the transformed elements.

## Syntax

```rust,noplayground
walker.map(|element, context| {
    // transformation logic
})
```

## Parameters

- `mapping`: A function that takes:
    - A reference to the current element (vertex or edge)
    - The element's context
    - Returns a transformed value

## Return Value

Returns an iterator that yields the transformed elements. The type of the iterator items is determined by the return
type of the mapping function.

## Diagram

```bob
Before step:
  [Project A]* --- [Person B]* --- [Project C]*
  Position: All vertices in traversal

After step (map vertex -> name):
  Result: Iterator of "Project A", "Person B", "Project C"
  Position: Traversal converted to iterator of transformed values
```

## Example

```rust,noplayground
{{#include map/map_example.rs:all}}
```

## Notes

- The `map` step is terminal - it returns an iterator, not a traversal builder
- After mapping, you can continue with standard Rust iterator operations like filter or collect
- Common uses include extracting properties from vertices or edges (e.g., names, IDs, attributes)
- For building complex data structures, consider using pattern matching in the mapping function
- For more complex transformations, you can also access the context data
- When working with edges, you can access the connected vertices through the edge reference
# Map Step

The `map` step transforms vertices or edges in the traversal by applying a mapping function, returning an iterator over
the transformed elements. This is a terminal step that ends the traversal.

```pikchr
# Graph structure with all vertices active in traversal
A: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
B: box same at 1 right of A "B" fill lightgreen
C: box same at 1 right of B "C" fill lightgreen

# Connect vertices with edges
line from A.e to B.w
line from B.e to C.w

# Show mapping transformation with arrows
Transformation: box height 0.4 width 1.1 at 0.7 below B "Map Function" "|vertex, ctx| → name" fill lightyellow

arrow from A.s down 0.1 then right until even with B then down to Transformation.n rad 10px
arrow from B.s to Transformation.n
arrow from C.s down 0.1 then left until even with B then down to Transformation.n rad 10px

# Result of mapping as strings
Result: box height 0.5 width 2.5 at 0.7 below Transformation "Iterator<String>" "\"Project A\", \"Person B\", \"Project C\"" fill lightyellow

arrow from Transformation.s to Result.n

text at 0.5 below Result "After map(): Traversal converts to iterator of transformed values"
```

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
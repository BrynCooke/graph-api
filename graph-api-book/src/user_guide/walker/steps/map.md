# Map Step

The `map` step transforms vertices or edges in the traversal by applying a mapping function, returning an iterator over the transformed elements.

## Syntax

```rust
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

Returns an iterator that yields the transformed elements. The type of the iterator items is determined by the return type of the mapping function.

## Diagram

```
Before:
  Graph: [Project A]---[Person B]---[Project C]
  Position: [Project A]*, [Person B]*, [Project C]* (all vertices)

After (map vertex -> name):
  Iterator: "Project A", "Person B", "Project C"
```

## Example

{% include_fn ./map.rs:vertex_example %}

## Notes

- The `map` step is terminal - it returns an iterator, not a traversal builder
- After mapping, you can continue with standard Rust iterator operations like filter or collect
- Common uses include extracting properties from vertices or edges (e.g., names, IDs, attributes)
- For building complex data structures, consider using pattern matching in the mapping function
- For more complex transformations, you can also access the context data
- When working with edges, you can access the connected vertices through the edge reference
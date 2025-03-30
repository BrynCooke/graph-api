# Map Step

The `map` step transforms vertices or edges in the traversal by applying a mapping function, returning an iterator over
the transformed elements. This is a terminal step that ends the traversal.

{{#include images/map.svg}}

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

## Best Practices

- Structure your mapping logic to handle all expected element types and properties
- Use pattern matching in your mapping function to handle different vertex or edge types
- Leverage context data for transformations that require information from previous steps
- Chain standard iterator methods after mapping to further refine results

## Common Use Cases

- **Property extraction**: Transforming graph elements into specific properties or attributes
- **Type conversion**: Converting graph elements into domain-specific data structures
- **Data aggregation**: Creating composite values from multiple element properties
- **Format transformation**: Preparing graph data for serialization or external systems

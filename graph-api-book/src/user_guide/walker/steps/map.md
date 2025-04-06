# Map Step

The `map` step transforms vertices or edges in the traversal by applying a mapping function, returning a standard Rust iterator over the transformed elements. This is a terminal step that consumes the walker and ends the Graph API traversal chain.

<object type="image/svg+xml" data="map/image.svg" title="Map Step Diagram">
Map step diagram showing elements being transformed into values in an iterator
</object>

In this diagram:

- An **Input Stream** contains elements **A, B, C**.
- The **`.map(|v| v.name())`** step processes each element, applying the transformation function.
- The output is represented as a **Rust Iterator** box, containing the resulting values (e.g., `"NameA"`, `"NameB"`, `"NameC"`).
- The diagram indicates that this step **Terminates Walker**, meaning no further Graph API steps can be chained after `map`. Standard Rust iterator methods can be used on the result.

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

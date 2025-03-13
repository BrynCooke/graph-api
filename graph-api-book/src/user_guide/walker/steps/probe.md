# Probe Step

The `probe` step allows you to inspect or log vertices or edges during a traversal without affecting the traversal flow.

## Syntax

```rust
walker.probe(|element, context| {
    // inspection logic
})
```

## Parameters

- `inspector`: A function that takes:
  - A reference to the current element (vertex or edge)
  - The element's context
  - Performs some side effect like logging or debugging

## Return Value

Returns the same traversal unchanged, allowing you to continue chaining steps.

## Diagram

```
Before:
  Graph: [A]---[B]---[C]
  Position: [A]*, [B]*, [C]* (all vertices)

During probe (with logging):
  [A]* (logs "Element A") -> [B]* (logs "Element B") -> [C]* (logs "Element C")

After:
  Graph: [A]---[B]---[C]
  Position: [A]*, [B]*, [C]* (unchanged)
```

## Example

{% include_fn ./probe.rs:probe_example %}

## Notes

- The probe step is non-terminal - it allows the traversal to continue unchanged
- Perfect for debugging, logging, or statistics collection during traversal
- Can be inserted at any point in a step chain without affecting the traversal
- The inspector function isn't expected to return anything
- Unlike map or filter, probe doesn't transform elements or filter the traversal
- You can use multiple probe steps at different points in a complex traversal
- Particularly useful for understanding traversal flow in complex graph operations
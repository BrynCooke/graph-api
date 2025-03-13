# Filter Step

The `filter` step narrows a traversal by keeping only vertices or edges that match a specified predicate.

## Syntax

```rust
walker.filter(|element| /* predicate logic */)
```

## Parameters

- `predicate`: A function that takes a reference to a graph element and returns a boolean.
  - Returns `true` to keep the element in the traversal
  - Returns `false` to remove it from the traversal

## Return Value

Returns a new walker containing only the elements that match the predicate.

## Diagram

```
Before:
  Graph: [A]---[B]---[C]
  Position: [A]*, [B]*, [C]* (all vertices)

After (filter where age > 30):
  Graph: [A]---[B]---[C]
  Position: [A]*, [C]* (only vertices with age > 30)
```

## Example

{% include_fn ./examples/filter.rs:filter_example %}

## Notes

- The filter step does not modify the graph, only the traversal
- Filtering is lazy - predicates are only evaluated when traversal elements are accessed
- For complex filtering conditions:
  - Chain multiple filter steps for better readability
  - Use pattern matching for different vertex/edge types
- The filter predicate has access to the current element only, not the entire graph
- When possible, consider using indexed searches (like `VertexSearch` with properties) instead of filtering after retrieval
# First Step

The `first` step retrieves only the first element from a traversal and immediately terminates.

## Syntax

```rust
walker.first()
```

## Parameters

This step takes no parameters.

## Return Value

Returns an `Option` containing the first element from the traversal, or `None` if the traversal is empty.

## Example

{% include_fn ./examples/first.rs:first_example %}

## Notes

- The first step is a terminal operation that consumes the walker
- More efficient than `limit(1).collect()` when you only need one element
- Useful patterns:
  - Getting a single result when you know it exists
  - Checking if any elements match a condition (using `is_some()`)
  - Finding an example of a specific type of vertex or edge
- The "first" element depends on the graph implementation and traversal order
  - If you need a specific element, use filtering or ordering before first
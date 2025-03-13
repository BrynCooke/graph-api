# Count Step

The `count` step counts the number of elements in a traversal without collecting them.

## Syntax

```rust
walker.count()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a `usize` representing the number of elements in the traversal.

## Example

{% include_fn ./examples/count.rs:count_example %}

## Notes

- The count step is a terminal operation that consumes the walker
- It's more efficient than collecting to a vector just to get the length
- Count can be used to:
  - Check if a traversal contains any elements (if count > 0)
  - Get statistics about a graph (e.g., average connections per node)
  - Validate expected sizes during testing
- For very large graphs, counting might still require a full traversal
  - Some graph implementations might optimize this for better performance
  - Consider using indexed counts when available
# Count Step

The `count` step counts the number of elements in a traversal without collecting them.

## Syntax

```rust,noplayground
walker.count()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a `usize` representing the number of elements in the traversal.

## Diagram

```bob
Before step:
  [A]* --- [B]* --- [C]* --- [D]*
  Position: Traversal at multiple elements

After step:
  [A] --- [B] --- [C] --- [D]
  Result: 4  (Number of elements)
  Position: Traversal terminated
```

## Examples

### Basic Count

Count the number of people in the graph:

```rust,noplayground
{{#include count/count_example.rs:basic_count}}
```

### Filtered Count

Count elements that match specific criteria:

```rust,noplayground
{{#include count/count_example.rs:filtered_count}}
```

### Edge Count

Count relationships in the graph:

```rust,noplayground
{{#include count/count_example.rs:edge_count}}
```

### Analytics

Use counts to calculate graph analytics:

```rust,noplayground
{{#include count/count_example.rs:analytics}}
```

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
# Count Step

The `count` step consumes the walker and efficiently counts the number of elements that have passed through the traversal up to that point. It is a **terminal** step that returns a single `usize` value representing the total count.

<object type="image/svg+xml" data="count/image.svg" title="Count Step Diagram">
Count step diagram showing elements flowing into the step and a usize count as the output
</object>

In this diagram:

- **Input Elements**: The walker starts with elements **A, B, C, D**.
- The **`.count()`** step processes the stream and consumes the walker.
- **Returns: usize**: The step returns a single `usize` value, which is the total number of elements processed (4 in this case).
- **Terminates Walker**: This step ends the Graph API walker chain.

## Syntax

```rust,noplayground
walker.count()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a `usize` representing the number of elements in the traversal.

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

## Best Practices

- Use count directly rather than collecting results just to count them
- Consider indexed counts for large graphs when available in your implementation
- Combine with filter steps for specific counting queries
- Use count to validate expectations in tests and assertions

## Common Use Cases

- **Existence checking**: Determining if any elements match criteria (count > 0)
- **Graph analytics**: Calculating statistics like average connections per node
- **Validation**: Ensuring expected numbers of elements exist in certain conditions
- **Performance metrics**: Measuring graph size and density characteristics

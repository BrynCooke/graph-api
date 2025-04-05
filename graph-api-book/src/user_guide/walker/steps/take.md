# Take Step

The `take` step restricts a traversal to return at most a specified number of elements, helping to control result size
and improve performance.

<object type="image/svg+xml" data="take/image.svg" title="Take Step Diagram">
Take step diagram showing traversal stopping after a specified number of elements
</object>

In this diagram:

- **Before `take()`**: The walker contains highlighted elements **A, B, C, D**.
- The **`.take(2)` step** is applied.
- **After `take(2)`**: Only the first two elements, **A** and **B**, remain highlighted. Elements C and D are faded,
  indicating they were discarded.

## Syntax

```rust,noplayground
walker.take(n)
```

## Parameters

- `n`: A `usize` value specifying the maximum number of elements the traversal should return

## Return Value

Returns a new walker that will yield at most `n` elements.

## Examples

### Basic Usage

```rust,noplayground
{{#include take/take_example.rs:basic_usage}}
```

### With Filtering

```rust,noplayground
{{#include take/take_example.rs:with_filter}}
```

### Edge Traversal Example

```rust,noplayground
{{#include take/take_example.rs:edge_example}}
```

## Best Practices

- Use `take` early in the traversal chain to reduce computation on intermediate steps
- Combine with ordered indexes when sequence matters to ensure consistent results
- For single element retrieval, prefer the more idiomatic `first()` over `take(1)`
- Set conservative limits when exploring large graphs to prevent memory issues

## Common Use Cases

- **Performance optimization**: Restricting result size for large graph traversals
- **Pagination**: Implementing "page-by-page" data retrieval when combined with skip/offset mechanisms
- **Top-N queries**: Getting the first N elements matching certain criteria
- **Resource control**: Preventing excessive memory or processing use in production systems


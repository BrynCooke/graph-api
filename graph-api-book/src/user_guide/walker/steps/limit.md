# Limit Step

The `limit` step restricts a traversal to return at most a specified number of elements, helping to control result size
and improve performance.

<object type="image/svg+xml" data="limit/image.svg">
Limit step diagram showing traversal stopping after a specified number of elements
</object>

## Syntax

```rust,noplayground
walker.limit(n)
```

## Parameters

- `n`: A `usize` value specifying the maximum number of elements the traversal should return

## Return Value

Returns a new walker that will yield at most `n` elements.

## Example

```rust,noplayground
{{#include limit/limit_example.rs:all}}
```

## Best Practices

- Use `limit` early in the traversal chain to reduce computation on intermediate steps
- Combine with ordered indexes when sequence matters to ensure consistent results
- For single element retrieval, prefer the more idiomatic `first()` over `limit(1)`
- Set conservative limits when exploring large graphs to prevent memory issues

## Common Use Cases

- **Performance optimization**: Restricting result size for large graph traversals
- **Pagination**: Implementing "page-by-page" data retrieval when combined with skip/offset mechanisms
- **Top-N queries**: Getting the first N elements matching certain criteria
- **Resource control**: Preventing excessive memory or processing use in production systems

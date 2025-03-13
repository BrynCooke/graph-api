# Limit Step

The `limit` step restricts a traversal to return at most a specified number of elements.

## Syntax

```rust
walker.limit(n)
```

## Parameters

- `n`: A `usize` value specifying the maximum number of elements the traversal should return

## Return Value

Returns a new walker that will yield at most `n` elements.

## Diagram

```
Before:
  [A]*, [B]*, [C]*, [D]*, [E]*
  (5 vertices in traversal)

After (with limit 2):
  [A]*, [B]*
  (limited to 2 vertices)
```

## Example

{% include_fn ./examples/limit.rs:limit_example %}

## Notes

- The limit step doesn't guarantee which elements will be returned, just how many
- For ordered or predictable results, combine with sorted indexes or other ordering steps
- Applying limit can significantly improve performance by avoiding unnecessary traversal work
- Useful patterns:
  - `limit(1)` to get a single element (though `first()` is more idiomatic)
  - Setting reasonable limits on potentially large traversals
  - Implementing pagination when combined with other techniques
# Limit Step

The `limit` step restricts a traversal to return at most a specified number of elements.

## Syntax

```rust,noplayground
walker.limit(n)
```

## Parameters

- `n`: A `usize` value specifying the maximum number of elements the traversal should return

## Return Value

Returns a new walker that will yield at most `n` elements.

## Diagram

```bob
Before step:
  [A]* --- [B]* --- [C]* --- [D]* --- [E]*
  Position: All vertices in traversal

After step (with limit 2):
  [A]* --- [B]* --- [C] --- [D] --- [E]
  Position: Only the first 2 vertices
```

## Example

```rust,noplayground
{{#include limit/limit_example.rs:all}}
```

## Notes

- The limit step doesn't guarantee which elements will be returned, just how many
- For ordered or predictable results, combine with sorted indexes or other ordering steps
- Applying limit can significantly improve performance by avoiding unnecessary traversal work
- Useful patterns:
    - `limit(1)` to get a single element (though `first()` is more idiomatic)
    - Setting reasonable limits on potentially large traversals
    - Implementing pagination when combined with other techniques
# Limit Step

The `limit` step restricts a traversal to return at most a specified number of elements, helping to control result size
and improve performance.

```pikchr
# Graph structure with all vertices active in traversal
A: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
B: box same at 1.5 right of A "B" fill lightgreen
C: box same at 1.5 right of B "C" fill lightgreen
D: box same at 1.5 right of C "D" fill lightgreen

# Connect vertices with edges
line from A.e to B.w
MID: line from B.e to C.w
line from C.e to D.w

text at 0.4 below MID "Before limit(): All vertices in traversal (highlighted green)"

# Second diagram - after limit(2) step
Aprime: box rad 10px width 0.5 height 0.3 at 1 below A "A" fill lightgreen
Bprime: box same at 1.5 right of Aprime "B" fill lightgreen
Cprime: box same at 1.5 right of Bprime "C" fill white
Dprime: box same at 1.5 right of Cprime "D" fill white

# Connect vertices with edges
line from Aprime.e to Bprime.w
MID: line from Bprime.e to Cprime.w
line from Cprime.e to Dprime.w

text at 0.4 below MID "After limit(2): Only first 2 vertices remain (highlighted green)"
```

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

## Notes

- The limit step doesn't guarantee which elements will be returned, just how many
- For range or predictable results, combine with sorted indexes or other ordering steps
- Applying limit can significantly improve performance by avoiding unnecessary traversal work
- Useful patterns:
    - `limit(1)` to get a single element (though `first()` is more idiomatic)
    - Setting reasonable limits on potentially large traversals
    - Implementing pagination when combined with other techniques
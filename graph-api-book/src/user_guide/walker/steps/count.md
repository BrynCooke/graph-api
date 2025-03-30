# Count Step

The `count` step counts the number of elements in a traversal without collecting them. This is a terminal operation that
consumes the walker and returns a single numeric value.

```pikchr
# Graph structure with all vertices active in traversal
A: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
B: box same at 1 right of A "B" fill lightgreen
C: box same at 1 right of B "C" fill lightgreen
D: box same at 1 right of C "D" fill lightgreen

# Connect vertices with edges
line from A.e to B.w
line from B.e to C.w
MID: line from C.e to D.w


# Show counting operation
CountBox: box rad 10px height 0.4 width 0.4 at 0.5 below 1 right of D "4" fill lightyellow

# Show arrows indicating the count operation
arrow from A.s down until even with CountBox then to CountBox.w rad 20px
arrow from B.s down until even with CountBox then to CountBox.w rad 20px
arrow from C.s down until even with CountBox then to CountBox.w rad 20px
arrow from D.s down until even with CountBox then to CountBox.w rad 20px


text at 0.4 below CountBox "Returns total element count (traversal terminates)"
```

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
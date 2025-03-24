# Collect Step

The `collect` step gathers all elements from a traversal into a collection, allowing you to materialize the results.
This is a terminal operation that ends the traversal and returns the collected elements.

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
CollectBox: box rad 10px at 0.5 below 1 right of D "Vec[A, B, C, D]" fit height 0.4 fill lightyellow

# Show arrows indicating the count operation
arrow from A.s down until even with CollectBox then to CollectBox.w rad 20px
arrow from B.s down until even with CollectBox then to CollectBox.w rad 20px
arrow from C.s down until even with CollectBox then to CollectBox.w rad 20px
arrow from D.s down until even with CollectBox then to CollectBox.w rad 20px

text at 0.4 below CollectBox "Returns collected elements (traversal terminates)"
```

## Syntax

```rust,noplayground
walker.collect::<C>()
```

Where `C` is the collection type you want to gather results into.

## Parameters

This step takes no parameters, but uses a type parameter to specify the collection type.

## Return Value

Returns a collection of type `C` where `C` implements `FromIterator`.

## Examples

### Collecting into a Vec

The most common use case is collecting elements into a `Vec`:

```rust,noplayground
{{#include collect/collect_example.rs:collect_vec}}
```

### Collecting Unique Values

You can collect into a `HashSet` to get unique values:

```rust,noplayground
{{#include collect/collect_example.rs:collect_hashset}}
```

### Collecting into an Range Set

Use a `BTreeSet` when you need range unique values:

```rust,noplayground
{{#include collect/collect_example.rs:collect_btreeset}}
```

## Notes

- Collection is a terminal operation that consumes the walker
- The collection type must implement `FromIterator`
- For large graphs, consider using `limit` before `collect` to avoid memory issues
- Different collection types offer different properties:
    - `Vec`: Maintains traversal order, allows duplicates
    - `HashSet`: Removes duplicates, no guaranteed order
    - `BTreeSet`: Removes duplicates, orders elements
    - Custom collections are also supported
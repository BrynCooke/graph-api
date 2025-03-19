# Collect Step

The `collect` step gathers all elements from a traversal into a collection, allowing you to materialize the results.

## Syntax

```rust,noplayground
walker.collect::<C>()
```

Where `C` is the collection type you want to gather results into.

## Parameters

This step takes no parameters, but uses a type parameter to specify the collection type.

## Return Value

Returns a collection of type `C` where `C` implements `FromIterator`.

## Diagram

```bob
Before step:
  [A]* --- [B]* --- [C]* --- [D]*
  Position: Traversal at multiple elements

After step:
  [A] --- [B] --- [C] --- [D]
  Result: Collection[A,B,C,D]  (Collected elements)
  Position: Traversal terminated
```

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

### Collecting into an Ordered Set

Use a `BTreeSet` when you need ordered unique values:

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
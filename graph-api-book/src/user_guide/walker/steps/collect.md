# Collect Step

The `collect` step gathers all elements from a traversal into a collection, allowing you to materialize the results.
This is a terminal operation that ends the traversal and returns the collected elements.

{{#include images/collect.svg}}

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

## Best Practices

- Use `limit` before `collect` when working with large graphs to control memory usage
- Choose the right collection type for your needs:
  - `Vec`: When order matters and duplicates are acceptable
  - `HashSet`: When you need unique elements and don't care about order
  - `BTreeSet`: When you need unique elements in a specific order

## Common Use Cases

- **Result accumulation**: Collecting all vertices meeting specific criteria
- **Set operations**: Gathering unique elements via `HashSet` for later processing
- **Ordered results**: Using `BTreeSet` when elements need to be in a specific order
- **Custom collections**: Feeding traversal results into specialized data structures

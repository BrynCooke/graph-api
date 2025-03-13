# Collect Step

The `collect` step gathers all elements from a traversal into a collection, allowing you to materialize the results.

## Syntax

```rust
walker.collect::<C>()
```

Where `C` is the collection type you want to gather results into.

## Parameters

This step takes no parameters, but uses a type parameter to specify the collection type.

## Return Value

Returns a `Result` containing a collection of type `C` where `C` implements `FromIterator`.

## Example

{% include_fn ./examples/collect.rs:collect_example %}

## Notes

- Collection is a terminal operation that consumes the walker
- The collection type must implement `FromIterator`
- Results are returned in a `Result` to propagate any errors
- For large graphs, consider using `limit` before `collect` to avoid memory issues
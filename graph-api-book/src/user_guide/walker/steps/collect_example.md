# Collect Step Example

The `collect` step is used to gather the results of a traversal into a collection like a `Vec`.

## Example Usage

Here's how to use the `collect` step to gather vertex IDs into a vector:

```rust,noplayground
{{#include collect/collect_example.rs:collect_vec}}
```

You can also collect results into different collection types like a HashSet for unique values:

```rust,noplayground
{{#include collect/collect_example.rs:collect_hashset}}
```

Or into a BTreeSet for ordered values:

```rust,noplayground
{{#include collect/collect_example.rs:collect_btreeset}}
```

## Additional Features

The `collect` step works with any collection type that implements `FromIterator`, allowing you to collect into various
collection types.
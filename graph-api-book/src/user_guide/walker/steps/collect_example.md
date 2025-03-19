# Collect Step Example

The `collect` step is used to gather the results of a traversal into a collection like a `Vec`.

## Example Usage

Here's how to use the `collect` step to gather vertex IDs into a vector:

```rust,noplayground
#![function_body!("../../../../graph-api-lib/examples/collect.rs", example)]
```

## Additional Features

The `collect` step works with any collection type that implements `FromIterator`, allowing you to collect into various
collection types.
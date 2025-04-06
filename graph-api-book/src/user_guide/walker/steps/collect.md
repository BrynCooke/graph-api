# Collect Step

The `collect` step gathers the **IDs** of all elements from a traversal into a specified Rust collection type (e.g., `Vec`, `HashSet`, `BTreeSet`). This is a **terminal** operation that consumes the walker and returns the populated collection.

<object type="image/svg+xml" data="collect/image.svg" title="Collect Step Diagram">
Collect step diagram showing elements flowing into the step and a Rust collection of IDs as the output
</object>

In this diagram:

- **Input Elements**: The walker starts with elements **A, B, C**.
- The **`.collect::<Vec<_>>()`** step processes the stream and consumes the walker. The type parameter (`Vec<_>`) specifies the desired collection type.
- **Returns: Vec&lt;ID&gt;**: The step returns a Rust `Vec` containing the unique identifiers (IDs) of the elements that were processed (`[ID(A), ID(B), ID(C)]`).
- **Terminates Walker**: This step ends the Graph API walker chain.

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

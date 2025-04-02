# First Step

The `first` step retrieves only the first element from a traversal and immediately terminates. This is a terminal
operation that short-circuits the traversal for efficiency.

<object type="image/svg+xml" data="first/image.svg" title="First Step Diagram">
First step diagram showing retrieval of only the first element from a traversal
</object>

In this diagram:

- **Before `first()`**: The walker contains highlighted elements **A, B, C, D**.
- The **`.first()` step** is applied.
- **After `first()`**: Only element **A** remains highlighted. Elements B, C, and D are faded, indicating they are no longer part of the result set.
- The final **Result** is shown as `Some(A)`, representing the `Option` returned by `first()`.
- This step **terminates the traversal**.

## Syntax

```rust,noplayground
walker.first()
```

## Parameters

This step takes no parameters.

## Return Value

Returns an `Option` containing the first element from the traversal, or `None` if the traversal is empty.

## Examples

### Basic Usage

Retrieve the first person vertex from the graph:

```rust,noplayground
{{#include first/first_example.rs:basic_usage}}
```

### With Filtering

Get the first person matching specific criteria:

```rust,noplayground
{{#include first/first_example.rs:with_filter}}
```

### Existence Check

Check if any elements match a condition:

```rust,noplayground
{{#include first/first_example.rs:existence_check}}
```

## Best Practices

- Always handle the `None` case when using `first()` on potentially empty traversals
- Use filtering before `first()` to ensure you get the element you want
- For deterministic results, combine with ordered indexes or explicit sorting
- Prefer `first()` over `limit(1).collect::<Vec<_>>()` for better performance

## Common Use Cases

- **Single element retrieval**: Getting exactly one element matching specific criteria
- **Existence checking**: Determining if any elements match a condition with `is_some()`
- **Quick lookup**: Finding a representative example of a vertex or edge type
- **Early termination**: Efficiently stopping traversal after finding the first match

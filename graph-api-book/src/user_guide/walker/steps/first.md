# First Step

The `first` step consumes the walker and returns the **ID** of the very first element encountered in the traversal stream, wrapped in an `Option`. If the stream is empty, it returns `None`. This is a **terminal** operation that efficiently short-circuits the traversal as soon as the first element is found.

<object type="image/svg+xml" data="first/image.svg" title="First Step Diagram">
First step diagram showing elements flowing into the step, only the first being considered, and an Option<ID> as the output
</object>

In this diagram:

- **Input Elements**: The walker starts with elements **A, B, C, D**.
- The **`.first()`** step processes the stream, immediately takes element **A**, and consumes the walker. Elements B, C, and D are never processed or considered.
- **Returns: Option&lt;ID&gt;**: The step returns `Some(ID(A))`, containing the ID of the first element found.
- **Terminates Walker**: This step ends the Graph API walker chain.

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

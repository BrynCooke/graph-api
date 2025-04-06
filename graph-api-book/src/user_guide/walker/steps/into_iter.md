# Into Iterator

The `into_iter` step converts a walker traversal into a standard Rust iterator. This is a **terminal** step that consumes the walker and returns an iterator yielding the **IDs** of the vertices or edges that were in the traversal stream. It allows you to bridge the Graph API's walker system with Rust's standard iterator ecosystem.

<object type="image/svg+xml" data="into_iter/image.svg" title="Into Iterator Step Diagram">
Into Iterator step diagram showing elements flowing into the step and a Rust iterator yielding IDs as the output
</object>

In this diagram:

- **Input Elements**: The walker starts with elements **A, B, C**.
- The **`.into_iter()`** step processes the stream and consumes the walker.
- **Rust Iterator**: The step returns a standard Rust iterator. This iterator yields the unique identifiers (IDs) of the elements that were processed (ID(A), ID(B), ID(C)).
- **Terminates Walker**: This step ends the Graph API walker chain. Subsequent operations must use standard Rust iterator methods.

## Syntax

```rust,noplayground
walker.into_iter()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a standard Rust iterator that yields vertex or edge IDs from the traversal.

## Examples

### Basic Usage

Convert a traversal to an iterator and collect results:

```rust,noplayground
{{#include into_iter/into_iter_example.rs:basic_usage}}
```

### Filtering with Standard Iterators

Use standard Rust iterator methods:

```rust,noplayground
{{#include into_iter/into_iter_example.rs:filtering}}
```

### Comparing with Walker Methods

Walker methods vs standard iterator methods:

```rust,noplayground
{{#include into_iter/into_iter_example.rs:comparison}}
```

## Best Practices

- Use `into_iter` only when you need to leverage standard Rust iterators 
- Remember that standard iterator methods lose access to graph context
- Consider extracting essential data into context before converting to an iterator
- When working with large graphs, use the Graph API's lazy evaluation before converting to an iterator

## Common Use Cases

- **Integration with existing code**: Bridging Graph API traversals with existing iterator-based systems
- **Complex iterator chains**: Using specialized iterator adaptors not available as walker steps
- **ID-based operations**: Working with element IDs directly for memory efficiency
- **Ecosystem integration**: Connecting graph traversals with Rust's extensive iterator ecosystem

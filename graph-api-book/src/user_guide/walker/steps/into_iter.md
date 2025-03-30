# Into Iterator

The `into_iter` step converts a traversal into a standard Rust iterator, allowing you to use all of Rust's iterator
methods directly with graph traversals. This bridges the gap between the Graph API's Walker and Rust's standard
iterators.

{{#include images/into_iter.svg}}

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

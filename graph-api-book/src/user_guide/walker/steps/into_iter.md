# Into Iterator

The `into_iter` step converts a traversal into a standard Rust iterator, allowing you to use all of Rust's iterator
methods directly with graph traversals. This bridges the gap between the Graph API's Walker and Rust's standard
iterators.

```pikchr
# Graph structure with all vertices active in traversal
A: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
B: box same at 1 right of A "B" fill lightgreen
C: box same at 1 right of B "C" fill lightgreen

# Connect vertices with edges
line from A.e to B.w
line from B.e to C.w

# Show mapping transformation with arrows
Iter: box height 0.4 at 0.7 below B "Iterator<(Element, Ctx)>" fit fill lightyellow

arrow from A.s down 0.15 then right until even with B then down to Iter.n rad 10px
arrow from B.s to Iter.n
arrow from C.s down 0.15 then left until even with B then down to Iter.n rad 10px

text at 0.4 below Iter "After map(): Traversal converts to iterator of transformed values"
```

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

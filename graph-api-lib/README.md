# Graph-API-Lib

**Welcome to graph-api-lib** — a delightful and powerful general-purpose library for working with graph data structures in Rust!

**graph-api-lib** provides a standardized API for graph mutation and traversal, abstracting away the complexity of working with different graph implementations. It allows you to write clean, maintainable graph-related code that isn't tied to a specific backend.

With **graph-api-lib**, graph operations become intuitive and enjoyable. Add vertices, create edges, traverse your data, and analyze graph structures with a consistent, ergonomic interface.

## Key Features

1. **Type-Safe Graph Operations**: Leverage Rust's type system for safer, more maintainable code.
2. **Flexible Traversal System**: Navigate your graph data with intuitive, chainable operations.
3. **Implementation Agnostic**: Write code once that works across different graph backends.
4. **Ergonomic API Design**: Enjoy a clean, intuitive interface that makes graph code readable and maintainable.
5. **Rich Query Capabilities**: Find exactly what you're looking for with expressive queries.

## Example Usage

```rust
// Find all people who created projects written in Rust:
let rust_creators = graph
    .walk()
    .vertices(VertexIndex::person())
    .edges(EdgeIndex::created().direction(Direction::Outgoing))
    .tail()
    .edges(EdgeIndex::language().direction(Direction::Outgoing))
    .tail()
    .filter_rust()
    .head()
    .head()
    .collect::<Vec<_>>();
```

## Future Development

1. **Algorithm Support**: Shortest path, spanning trees, and more graph algorithms.
2. **Cycle Detection**: Prevent traversals from visiting the same element twice.
3. **API Refinements**: Continued improvements to make graph operations even more intuitive.

## Perfect For

- Social network analysis
- Dependency tracking
- Knowledge graphs
- Recommendation systems
- Any application dealing with connected data!

Dive into the [graph-api book](https://bryncooke.github.io/graph-api/) for a comprehensive guide to using this library.
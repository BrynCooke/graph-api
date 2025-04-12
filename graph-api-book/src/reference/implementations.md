# Existing Implementations

The Graph API ecosystem includes two ready-to-use graph implementations that cater to different use cases and
performance requirements. This section provides an overview of the existing implementations, their features, and
suitability for different applications.

## Core Implementations

The Graph API project provides two main graph implementations:

1. **[SimpleGraph](./implementations/simple_graph.md)**: A reference graph implementation designed specifically for
   the Graph API, with full support for all index types.

2. **[PetGraph](./implementations/pet_graph.md)**: An adapter implementation for the
   popular [petgraph](https://crates.io/crates/petgraph) crate, demonstrating Graph API compatibility with an
   established
   Rust graph library.

## Choosing an Implementation

When deciding which graph implementation to use, consider the following factors:

### Feature Support

Different implementations support different features:

| Feature                     | SimpleGraph | PetGraph |
|-----------------------------|-------------|----------|
| Vertex label indexes        | ✅           | ❌        |
| Edge label indexes          | ✅           | ❌        |
| Vertex hash indexes         | ✅           | ❌        |
| Edge hash indexes           | ✅           | ❌        |
| Vertex range indexes        | ✅           | ❌        |
| Edge range indexes          | ✅           | ❌        |
| Vertex full-text indexes    | ✅           | ❌        |
| Edge adjacent label indexes | ✅           | ❌        |
| Graph clearing              | ✅           | ✅        |

### Performance Characteristics

Performance varies between implementations:

- **SimpleGraph**: Supports all of GraphApi's features, but has not been optimized
- **PetGraph**: Established graph library that is widely used

### Memory Usage

- **SimpleGraph**: Supports all of GraphApi's features, but has not been optimized
- **PetGraph**: Uses petgraph's memory model, which may be more memory-efficient

### Integration

- **SimpleGraph**: Designed specifically for Graph API and fully supports all features
- **PetGraph**: Useful when you need to integrate with existing petgraph-based code

## Creating Your Own Implementation

If the existing implementations don't meet your needs, you can create your own by implementing the `Graph` trait. See
the [Implementation Guide](../implementation/guide) for detailed instructions on creating a custom graph implementation.

## Future Implementations

The Graph API is designed to support a variety of graph implementations.

Community contributions of new graph implementations are welcomed and encouraged!
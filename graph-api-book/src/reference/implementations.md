# Existing Implementations

The Graph API ecosystem includes several ready-to-use graph implementations that cater to different use cases and
performance requirements. This section provides an overview of the existing implementations, their features, and
suitability for different applications.

## Core Implementations

The Graph API project provides two main graph implementations:

1. **[SimpleGraph](./implementations/simple_graph.md)**: A custom-built graph implementation designed specifically for
   the Graph API, with full support for all index types and optimized for property graph use cases.

2. **[PetGraph](./implementations/pet_graph.md)**: An adapter implementation for the
   popular [petgraph](https://crates.io/crates/petgraph) crate, providing Graph API compatibility with an established
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

- **SimpleGraph**: Optimized for property graph operations with fast index lookups
- **PetGraph**: Leverages petgraph's established graph algorithms but lacks index support

### Memory Usage

- **SimpleGraph**: Uses a specialized data structure with efficient memory layout
- **PetGraph**: Uses petgraph's memory model, which may be more memory-efficient for specific use cases

### Integration

- **SimpleGraph**: Designed specifically for Graph API and fully supports all features
- **PetGraph**: Useful when you need to integrate with existing petgraph-based code

## Creating Your Own Implementation

If the existing implementations don't meet your needs, you can create your own by implementing the `Graph` trait. See
the [Implementation Guide](../implementation/guide) for detailed instructions on creating a custom graph implementation.

## Future Implementations

The Graph API is designed to support a variety of graph implementations. Future versions may include additional
implementations for:

- **Disk-backed graphs**: For handling graphs larger than memory
- **Distributed graphs**: For processing very large graphs across multiple machines
- **Specialized graphs**: For domain-specific optimization (e.g., social networks, knowledge graphs)

Community contributions of new graph implementations are welcomed and encouraged!
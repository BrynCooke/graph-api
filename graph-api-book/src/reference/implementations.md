# Existing Implementations

The Graph API ecosystem includes two ready-to-use graph implementations that cater to different use cases and
performance requirements. This section provides an overview of the existing implementations, their features, and
suitability for different applications.

## Core Implementations

The Graph API project provides two main graph implementations:

1. **[SimpleGraph](./implementations/simple_graph.md)**: A reference implementation built specifically for the Graph
   API. It fully supports all Graph API features, including all index types, making it ideal for testing Graph API
   functionality and serving as a blueprint for new implementations.

2. **[PetGraph](./implementations/pet_graph.md)**: An adapter for the excellent and widely-used
   [petgraph](https://crates.io/crates/petgraph) crate. This demonstrates Graph API compatibility with established
   Rust graph libraries and is a great choice for performance-sensitive applications or projects already using
   petgraph.

## Choosing an Implementation

When deciding which graph implementation to use, consider the following factors:

### Feature Support

Different implementations support different features:

| Feature                      | SimpleGraph | PetGraph |
|------------------------------|-------------|----------|
| Vertex label indexes         | ✅           | ❌        |
| Edge label indexes           | ✅           | ❌        |
| Vertex hash indexes          | ✅           | ❌        |
| Edge hash indexes            | ❌           | ❌        |
| Vertex range indexes         | ✅           | ❌        |
| Edge range indexes           | ❌           | ❌        |
| Vertex full-text indexes     | ✅           | ❌        |
| Edge adjacent label indexes* | ✅           | ❌        |
| Graph clearing               | ✅           | ✅        |

\* Edge adjacent indexes are not fully supported in graph-api yet.

### Performance Characteristics

Performance varies between implementations:

- **SimpleGraph**: Primarily designed for feature completeness and testing; not optimized for high performance.
- **PetGraph**: A mature, well-optimized, and widely-used graph library suitable for production use.

### Memory Usage

- **SimpleGraph**: Memory usage is straightforward but not heavily optimized.
- **PetGraph**: Leverages petgraph's memory model, which is generally efficient.

### Integration

- **SimpleGraph**: Best for demonstrating and testing Graph API features or as a starting point for custom
  implementations.
- **PetGraph**: Ideal for integrating Graph API capabilities into projects already using petgraph or when requiring
  petgraph's performance characteristics.

## Creating Your Own Implementation

If the existing implementations don't meet your needs, you can create your own by implementing the `Graph` trait. See
the [Implementation Guide](../implementation/guide.md) for detailed instructions on creating a custom graph
implementation.

## Future Implementations

The Graph API is designed to support a variety of graph implementations.

Community contributions of new graph implementations are welcomed and encouraged!

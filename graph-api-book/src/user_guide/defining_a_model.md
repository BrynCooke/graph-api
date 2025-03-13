# Defining a Model

Graph API provides a flexible way to define your graph data model using Rust's enum types and derive macros. This
approach gives you the benefits of Rust's type system while maintaining the flexibility of property graphs.

## Basic Model Definitions

A graph consists of two primary elements:

1. **Vertices** (nodes): The entities in your graph
2. **Edges**: The relationships connecting vertices

For each of these, you'll define a Rust enum that represents all possible types. Here's a simple example:

{% include_fn ./model_definition.rs:model_definition_example %}

## Using Derive Macros

The `VertexExt` and `EdgeExt` derive macros generate implementations for your model types that enable them to work with
Graph API's traversal and query features.

### VertexExt

This macro provides:

- Methods to access vertex properties
- Integration with the indexing system
- Serialization/deserialization support
- Type-safe accessors for properties
- Type-safe filters for traversals

### EdgeExt

This macro provides:

- Methods to access edge properties
- Integration with label-based indexing
- Serialization/deserialization support
- Type-safe accessors for properties
- Type-safe filters for traversals

## Indexing

You can define indexes for efficient lookups using attributes:

- `#[index]`: Creates a standard index for exact match lookups
- `#[index(ordered)]`: Creates an ordered index for range queries
- `#[index(full_text)]`: Creates a full-text index for text search

Example:

{% include_fn ./indexing_example.rs:indexing_example %}

## Best Practices

When defining your graph model:

1. **Use descriptive names** - Choose clear names for vertex and edge types
2. **Index strategically** - Only index fields used in frequent queries
3. **Use appropriate index types** - Match index types to query patterns
4. **Keep models focused** - Split large models into logical groupings
5. **Use Rust's type system** - Leverage Rust's types for property values
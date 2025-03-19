# Defining a Model

## Overview

Graph API provides a flexible way to define your graph data model using Rust's enum types and derive macros. This
approach gives you the benefits of Rust's type system while maintaining the flexibility of property graphs.

## Basic Concepts

A graph consists of two primary elements:

1. **Vertices** (nodes): The entities in your graph
2. **Edges**: The relationships connecting vertices

For each of these, you'll define a Rust enum that represents all possible types.

## Model Definition

Here's a simple example of a social media model:

```rust,noplayground
{{#include defining_a_model/social_media_model.rs:model_definition}}
```

This model defines vertex types for people, projects, and comments, along with edge types for the relationships between
them.

## Creating Instances

Once you've defined your model, you can create instances of vertices and edges:

```rust,noplayground
{{#include defining_a_model/social_media_model.rs:usage}}
```

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

For more details on indexes and examples, see the [Property Graphs](./property_graphs.md) section.

## Best Practices

When defining your graph model:

1. **Use descriptive names** - Choose clear names for vertex and edge types
2. **Index strategically** - Only index fields used in frequent queries
3. **Use appropriate index types** - Match index types to query patterns
4. **Keep models focused** - Split large models into logical groupings
5. **Use Rust's type system** - Leverage Rust's types for property values
# Property Graphs

A property graph is a powerful data model that allows you to represent complex, interconnected data in a highly
intuitive way. Graph API leverages Rust's type system to provide a strongly-typed property graph experience.

## What are Property Graphs?

Property graphs consist of two primary elements:

1. **Vertices** (nodes): Represent entities in your data model. Each vertex has a type (label) and can hold multiple
   key-value properties.

   <object type="image/svg+xml" data="property_graphs/vertex_with_properties.svg">
   Diagram showing a single vertex with properties
   </object>

2. **Edges** (relationships): Connect vertices to express relationships. Edges are directed, have a type (label), and
   can also hold properties.

   <object type="image/svg+xml" data="property_graphs/edge_with_properties.svg">
   Diagram showing two vertices connected by an edge with properties
   </object>

Both vertices and edges can have properties (attributes) that store additional information. In Graph API, these
properties are represented using Rust's enum types, giving you full type safety.

## Why Use Property Graphs?

Property graphs excel at representing highly connected data where relationships are as important as the entities
themselves. They're particularly useful for:

- Social networks (people, friendships, interests)
- Knowledge graphs (concepts, relationships)
- Recommendation systems (users, products, preferences)
- Network topologies (devices, connections)
- Dependency graphs (components, dependencies)

The key advantages of property graphs include:

- **Intuitive modeling**: Reflects how we naturally think about connected data
- **Relationship-centric**: Makes connections first-class citizens
- **Flexible schema**: Easily adapt to changing data models
- **Performance**: Efficient for traversal and relationship-based queries

## Graph API's Approach to Property Graphs

Graph API uses Rust's strong type system to create a safe, ergonomic property graph experience:

- **Enum-based modeling**: Define vertices and edges using Rust enums
- **Derive macros**: Generate boilerplate code for traversal and querying
- **Type-safe queries**: Leverage Rust's type checking for query correctness
- **Efficient indexing**: First-class support for various index types

## Property Graph Example

Here's a simple example of how Graph API models a property graph:

```rust,noplayground
{{#include ../standard_model.rs:model_definition}}
```

## Understanding Indexes

Indexes are a crucial part of an efficient graph database. They allow you to quickly locate vertices and edges without
scanning the entire graph.

In Graph API, indexes are defined as part of your model using derive macros. The following sections explore different
types of indexes and how to use them effectively:

- [Exploring Without Indexes](./property_graphs/no_index.md): Understand the challenges of graph traversal without
  indexes
- [Hash Indexes](./property_graphs/hash_index.md): Fast lookups by exact property value
- [Range Indexes](./property_graphs/range_index.md): Range queries for numeric and range properties
- [Full-text Indexes](./property_graphs/full_text_index.md): Text search capabilities for string properties

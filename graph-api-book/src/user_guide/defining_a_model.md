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

```pikchr
Eve:    box rad 10px "Person" bold "name: Eve" "username: eve789" "age: 31" fit; move
Julia:    box rad 10px at 1 below Eve.s "Person" bold "name: Julia" "username: julia456" "age: 34" "biography: An editor" fit
Bryn:  box rad 10px at 1.5 left of Julia.w "Person" bold "name: Bryn" "username: bryn123" "age: 28" "biography: A programmer"  fit       
GraphApi:  box rad 10px at 1.5 left of Bryn.w "Project" bold "name: GraphApi" fit         
Alpaca:  box rad 10px at 1 below Bryn.s "Project" bold "name: Alpaca" fit

arrow <-> from Bryn.n to Eve.w "Follows" above rjust rad 20px
arrow from Eve.s to Julia.n " Follows" ljust 
arrow from Julia.w to Bryn.e "Follows" above      
arrow from Bryn.w to GraphApi.e "Created" above
arrow from Bryn.s to Alpaca.n "Liked " rjust "Commented " rjust
arrow from Julia.s to Alpaca.ne "Created" ljust below
```

The equivalent definition using Graph API is:

```rust,noplayground
{{#include ../standard_model.rs:model_definition}}
```

This model defines vertex types for people, projects, and comments, along with edge types for the relationships between
them.

## Creating Instances

Once you've defined your model, you can create instances of vertices and edges:

```rust,noplayground
{{#include ../standard_model.rs:setup}}
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
- `#[index(range)]`: Creates an range index for range queries
- `#[index(full_text)]`: Creates a full-text index for text search

For more details on indexes and examples, see the [Property Graphs](./property_graphs.md) section.

## Best Practices

When defining your graph model:

1. **Use descriptive names** - Choose clear names for vertex and edge types
2. **Index strategically** - Only index fields used in frequent queries
3. **Use appropriate index types** - Match index types to query patterns
4. **Keep models focused** - Split large models into logical groupings
5. **Use Rust's type system** - Leverage Rust's types for property values
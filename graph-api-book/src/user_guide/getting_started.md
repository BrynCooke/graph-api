# Getting Started

## Overview

`graph-api` is an API that helps you work with graphs in Rust. Graphs are powerful data structures used to represent
relationships between entities. With the Graph API, you can create, manipulate, and traverse graphs with ease.

Any implementation of the Graph trait will benefit from an ergonomic walker API that allows you to perform complex
traversals. For the purposes of this documentation, we will use `SimpleGraph`, but you may choose any other graph
implementation.

## Basic Concepts

A graph consists of:

- **Vertices** (also called nodes): These represent entities in your data model
- **Edges** (also called links or connections): These represent relationships between entities
- **Properties**: Both vertices and edges can have properties that store data

The Graph API provides:

- A common interface for working with graph data
- Powerful traversal capabilities with the Walker API
- Index support for efficient lookups
- Derive macros for easy model definition

## Installation

Add the Graph API to your project dependencies:

```toml
[dependencies]
graph-api-lib = "{{lib_version}}"
graph-api-derive = "{{derive_version}}"  # For derive macros
```

For a simple graph implementation:

```toml
[dependencies]
graph-api-simplegraph = "{{simplegraph_version}}"
```

## Examples

### Your First Graph

Let's define a simple graph model with people and projects:

```rust,noplayground
{{#include getting_started/first_graph.rs:model}}
```

Now we can create a graph and add some data:

```rust,noplayground
{{#include getting_started/first_graph.rs:create_graph}}
```

And finally, we can query the graph:

```rust,noplayground
{{#include getting_started/first_graph.rs:query}}
```

## Advanced Usage

Graph API supports more advanced features that we'll explore in later sections:

- Complex traversals with the Walker API
- Different index types for optimized queries
- Transaction support (with appropriate graph implementations)
- Custom property types

## Next Steps

Now that you have created your first graph, you can:

1. Learn about [defining a model](./defining_a_model.md) for your graph data
2. Explore [basic operations](./basic_operations.md) for working with graphs
3. Discover [graph traversal](./traversal.md) techniques using walkers
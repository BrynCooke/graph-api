# Introduction to Graph API

Graph API is an ergonomic library for working with **in memory** graphs in Rust that provides a flexible and type-safe
way to interact with graph data structures.

## Overview

This library offers a unified interface for working with different types of graphs while maintaining strong type safety
and ergonomic usage patterns. It includes features for graph traversal, modification, and custom extensions.

## Key Features

- **Type-safe graph operations**: Work with graph data in a strongly-typed manner
- **Flexible vertex and edge traversal**: Explore graph relationships with powerful walker API
- **Custom graph implementations support**: Adapt to various graph storage backends
- **Derive macros for extending graph functionality**: Automatically generate boilerplate code
- **Comprehensive testing utilities**: Including fuzzing support

## Example

```rust,noplayground
{{#include introduction_example.rs:basic_example}}
```

## Book Organization

This book is organized into several sections:

- **User Guide**: How to use the Graph API to work with graph data
- **Implementation Guide**: How to implement the Graph API traits for your own graph types
- **Reference**: Detailed information about API components and functionality
- **Appendix**: Additional resources and reference materials

Whether you're a graph API user or implementing your own graph backend, this book provides comprehensive documentation
to help you make the most of the Graph API library.
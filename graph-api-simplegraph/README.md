# SimpleGraph

**Introducing SimpleGraph** — a lightweight, efficient, and feature-rich reference implementation of the Graph-API ecosystem!

Looking for a graph implementation that just works? SimpleGraph delivers a complete, optimized solution for in-memory graph storage with powerful indexing capabilities. As the reference implementation for Graph-API, SimpleGraph sets the standard for what a graph implementation should do.

## Features That Delight

### Comprehensive Indexing
* **Hash Index**: Lightning-fast lookups for exact matches — perfect for finding elements by ID or name
* **Range Index**: Efficient range queries — ideal for age ranges, date spans, or alphabetical searches
* **Full Text Index**: Text search capabilities — find elements containing specific words or phrases
* **Label Indexing**: Quick access to all vertices or edges of a specific type

### Smart Implementation

* **Type-Safe API**: Work with confidence using Rust's type system
* **Memory-Efficient Storage**: Carefully designed to minimize memory footprint
* **Stable IDs**: References to graph elements remain valid even as the graph changes
* **Optimized Traversals**: Navigate your graph with minimal overhead

## Performance Highlights

SimpleGraph is built with performance in mind:

* **Hash Index**: O(1) lookups and inserts — as fast as it gets!
* **Range Index**: O(log n) operations — efficient even with millions of elements
* **Full Text Index**: Search through text fields without manual string parsing
* **Stable Storage**: O(1) element access with slot reuse for removed elements

## Perfect For

* Building recommendation systems
* Social network analysis
* Knowledge representation
* Data relationship visualization
* Any application requiring indexed graph operations

## Coming Soon

SimpleGraph is actively evolving with enhancements on the horizon:
* Optimized memory usage for large graphs
* Edge indexing for even more flexible queries
* Performance improvements for specific traversal patterns

Experience the simplicity and power of a well-designed graph implementation with SimpleGraph!

Learn more in the [graph-api book](https://bryncooke.github.io/graph-api/).
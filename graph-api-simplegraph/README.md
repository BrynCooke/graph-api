# SimpleGraph

A lightweight and efficient graph indexing library written in Rust that provides flexible storage and querying
capabilities.

SimpleGraph is the reference implementation of Graph-API and graph implementors should match behaviour.

## Features

### Indexing Support

- **Hash Index**: Perfect for label-based lookups and general key-value associations
- **Range Index**: Supports range queries and range traversal of data
- **Full Text Index**: Supports full text searches for vertices
- **Vertex and Edge Labels**: Search all elements matching a label.

### Key Characteristics

- Type-safe implementations
- Memory-efficient storage

## Storage Structure

Vertices are stores in a stable Vec to provide stable indexing and reuse of empty slots.

## Performance Characteristics

### Hash Index

- Insert: O(1) average case
- Lookup: O(1) average case
- Remove: O(1) average case
- Memory: O(n) where n is the total number of key-value pairs

### Range Index

- Insert: O(log n) where n is the number of unique keys
- Lookup: O(log n)
- Range Query: O(log n + m) where m is the number of elements in range
- Remove: O(log n)
- Memory: O(n) where n is the total number of key-value pairs

### Full text index

- Insert: O(n) where n is the number of tokens in the document
- Lookup: O(n) where n is the number of documents
- Remove: O(1) average case
- Memory: O(n) where n is the total number of documents

## Use Cases

- Graph databases requiring multiple indexing strategies
- Label-based search systems
- Range-based data queries
- Applications requiring both range and hash data access patterns
- Full text search

## Future development

SimpleGraph has is not finished and requires:

* Optimisation for memory and performance
* Edge indexes support.

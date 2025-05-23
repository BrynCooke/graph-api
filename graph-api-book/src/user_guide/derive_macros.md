# Derive Macros

Graph API's derive macros automatically generate code that integrates your custom types with the Graph API framework,
making your graph operations type-safe and ergonomic. This section explains the types that are generated and how to use
them effectively.

## Overview

Graph API provides two primary derive macros:

1. `VertexExt` - For vertex enum types
2. `EdgeExt` - For edge enum types

```rust,noplayground
{{#include model_definition.rs:usage}}
```

## Generated Types for VertexExt

When you apply `#[derive(VertexExt)]` to an enum, several useful types are generated:

### 1. Label Enum (VertexLabel)

A type-safe enum representing the variants of your vertex enum:

```rust,noplayground
// Given this vertex definition
#[derive(VertexExt)]
enum Vertex {
    Person { /* fields */ },
    Project { /* fields */ },
    Comment { /* fields */ },
}

// This is generated
pub enum VertexLabel {
    Person,
    Project,
    Comment,
}
```

**Purpose**: Provides type-safe representations of vertex types, used for label-based indexing and filtering.

### 2. Index selectors

Selectors to make querying easier:

```rust,noplayground
impl Vertex {
    // Find all Person vertices
    pub fn person() -> VertexSearch { /* ... */ }
    
    // Find Person vertices by indexed properties
    pub fn person_by_username(username: &str) -> VertexSearch { /* ... */ }
    pub fn person_by_biography(text: &str) -> VertexSearch { /* ... */ }
    pub fn person_by_age_range(range: Range<u8>) -> VertexSearch { /* ... */ }
}
```

**Purpose**: Enables efficient querying of vertices by label and properties.

### 3. Projection Types

For each variant with fields, two projection structs are generated:

```rust,noplayground
// For Person variant
pub struct Person<'a, V> {
    // Immutable references to fields
    name: &'a String,
    username: &'a String,
    biography: &'a String,
    age: &'a u8,
    unique_id: &'a Uuid,
}

pub struct PersonMut<'a, V> {
    // Mutable references to fields
    name: &'a mut String,
    username: &'a mut String,
    biography: &'a mut String,
    age: &'a mut u8,
    unique_id: &'a mut Uuid,
}
```

**Purpose**: Provides type-safe access to variant fields without manually matching on enum variants.

### 4. Filter Extension Traits

Extension methods for the walker builder to filter by vertex type:

```rust,noplayground
// On WalkerBuilder
// For all Person vertices
fn filter_person(self) -> Self;

// For Person vertices with custom criteria
fn filter_by_person<F>(self, filter: F) -> Self
where
    F: Fn(Person<Vertex>, &Context) -> bool;
```

**Purpose**: Enables type-safe filtering of vertices during traversals.

## Generated Types for EdgeExt

When you apply `#[derive(EdgeExt)]` to an enum, similar types are generated:

### 1. Label Enum (EdgeLabel)

```rust,noplayground
// Given this edge definition
#[derive(EdgeExt)]
pub enum Edge {
    Created,
    Follows,
    Liked { timestamp: String },
    Commented { timestamp: String },
}

// This is generated
pub enum EdgeLabel {
    Created,
    Follows,
    Liked,
    Commented,
}
```

**Purpose**: Provides type-safe representations of edge types, used for label-based indexing and filtering.

### 2. Index selectors

Selectors to make querying easier

```rust,noplayground
impl Edge {
    // Find all Created edges
    pub fn created() -> EdgeSearch { /* ... */ }
    
    // Find all Follows edges
    pub fn follows() -> EdgeSearch { /* ... */ }
    
    // Find all Liked edges
    pub fn liked() -> EdgeSearch { /* ... */ }
    
    // Find all Commented edges
    pub fn commented() -> EdgeSearch { /* ... */ }
}
```

**Purpose**: Enables efficient querying of edges by label.

### 3. Projection Types

For each variant with fields, projection structs are generated:

```rust,noplayground
// For Liked variant
pub struct Liked<'a, E> {
    timestamp: &'a String,
}

pub struct LikedMut<'a, E> {
    timestamp: &'a mut String,
}
```

**Purpose**: Provides type-safe access to edge properties.

### 4. Filter Extension Traits

Extension methods for the walker builder to filter by edge type:

```rust,noplayground
// On WalkerBuilder
// For all Liked edges
fn filter_liked(self) -> Self;

// For Liked edges with custom criteria
fn filter_by_liked<F>(self, filter: F) -> Self
where
    F: Fn(Liked<Edge>, &Context) -> bool;
```

**Purpose**: Enables type-safe filtering of edges during traversals.

## Using the Generated Types

### Vertex Querying

```rust,noplayground
// Find all Person vertices
let people = graph
    .walk()
    .vertices(Vertex::person())
    .collect::<Vec<_>>();

// Find Person vertices with a specific username
let user = graph
    .walk()
    .vertices(Vertex::person_by_username("bryn123"))
    .first()
    .unwrap();

// Find Person vertices in an age range
let adults = graph
    .walk()
    .vertices(Vertex::person_by_age_range(18..65))
    .collect::<Vec<_>>();
```

### Edge Traversal

```rust,noplayground
// Find outgoing Created edges from a person
let created_projects = graph
    .walk()
    .vertices_by_id([person_id])
    .edges(Edge::created().outgoing())
    .tail()  // Follow edges to target vertices
    .collect::<Vec<_>>();
```

### Type-Safe Filtering

```rust,noplayground
// Find Person vertices that match specific criteria
let experienced_devs = graph
    .walk()
    .vertices(Vertex::person())
    .filter_by_person(|person, _| {
        person.age() > 25 && person.biography().contains("developer")
    })
    .collect::<Vec<_>>();
```

### Projection for Type Safety

```rust,noplayground
// Working with a vertex reference
if let Some(vertex_ref) = graph.vertex(vertex_id) {
    // Project to Person variant
    if let Some(person) = vertex_ref.project::<Person<_>>() {
        println!("Name: {}, Age: {}", person.name(), person.age());
    }
}
```

## Index Attributes

You can apply these attributes to fields to control indexing behavior:

- `#[index]` - Standard index for exact match queries
- `#[index(range)]` - Range index for range queries
- `#[index(full_text)]` - Full-text index for text search (String fields only)

```rust,noplayground
#[derive(VertexExt)]
enum User {
    Profile {
        #[index]  // Standard index
        username: String,
        
        #[index(range)]  // Range queries possible
        age: u32,
        
        #[index(full_text)]  // Text search possible
        bio: String,
        
        // Not indexed
        email: String,
    }
}
```

## Best Practices

1. **Selective Indexing**: Only add indexes to fields you'll frequently query, as each index increases memory usage.

2. **Choose Appropriate Index Types**:
    - Use `#[index]` for exact match lookups
    - Use `#[index(range)]` for numeric fields that need range queries
    - Use `#[index(full_text)]` for text fields that need substring or keyword search

3. **Use Type-Safe Methods**:
    - Prefer `filter_by_person()` over generic `filter()` with manual pattern matching
    - Use the generated search methods (`person_by_username()`, etc.) for efficient queries

4. **Leverage Projections**:
    - Use the projection types to safely access variant fields without repetitive pattern matching
    - This makes your code more maintainable and less error-prone

5. **Consider Query Performance**:
    - Using an indexed search in the initial `vertices()` step is typically more efficient than scanning and then
      filtering
    - The more you can narrow your search using indexes, the better your graph traversal performance

For more detailed information on the derive macros, see the [API Reference](../reference/derive_macros.md).
# Derive Macros

The Graph API provides derive macros to make working with your graph model types straightforward and type-safe. These
macros generate code that integrates your custom types with the Graph API framework.

## Overview

There are two primary derive macros:

1. `VertexExt` - For vertex enum types
2. `EdgeExt` - For edge enum types

These macros generate:

- Label enums for type-safe queries
- Index enums for efficient property lookups
- Helper methods for traversing and querying the graph
- Projection types for type-safe access to variant fields
- Walker builder filter extensions for type-safe filtering

## VertexExt Derive Macro

### Generated Types

When you apply `#[derive(VertexExt)]` to an enum, the following types are generated:

1. **VertexLabel** enum - Contains variants matching your enum's variants
2. **VertexIndex** enum - Contains variants for each indexed field
3. Projection structs - For accessing fields in a type-safe way

### Example

```rust
use graph_api_derive::VertexExt;
use uuid::Uuid;

#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index]
        name: String,

        #[index(ordered)]
        age: u64,

        #[index(full_text)]
        biography: String,

        unique_id: Uuid, // Not indexed
    },
    Project {
        name: String,
    },
    Tag, // Unit variant
}
```

This generates:

```rust
// Label enum
pub enum VertexLabel {
    Person,
    Project,
    Tag,
}

// Index enum with methods
pub enum VertexIndex {
    PersonName,
    PersonAge,
    PersonBiography,
}

// Projection structs (simplified)
pub struct Person<'a, V> {
    name: &'a String,
    age: &'a u64,
    biography: &'a String,
    unique_id: &'a Uuid,
}

pub struct PersonMut<'a, V, L> {
    name: &'a mut String,
    age: &'a mut u64,
    biography: &'a mut String,
    unique_id: &'a mut Uuid,
}
```

### Generated Methods

The derive macro generates several methods on the `VertexIndex` enum:

#### Label-based Querying

For each enum variant, a method is generated to query by label:

```rust
// Query for all Person vertices
VertexIndex::person() -> VertexSearch<'_, Graph>
```

#### Property-based Querying

For each indexed field, methods are generated for exact matching:

```rust
// Query for Person vertices with a specific name
VertexIndex::person_by_name(value: & str) -> VertexSearch<'_, Graph>

// Query for Person vertices with a specific age
VertexIndex::person_by_age(value: u64) -> VertexSearch<'_, Graph>
```

#### Range-based Querying

For fields with the `#[index(ordered)]` attribute:

```rust
// Query for Person vertices with age in a range
VertexIndex::person_by_age_range(range: Range<u64>) -> VertexSearch<'_, Graph>
```

#### Full-text Querying

For fields with the `#[index(full_text)]` attribute:

```rust
// Query for Person vertices with matching text in biography
VertexIndex::person_by_biography(search: & str) -> VertexSearch<'_, Graph>
```

## EdgeExt Derive Macro

### Generated Types

When you apply `#[derive(EdgeExt)]` to an enum, similar types are generated:

1. **EdgeLabel** enum - Contains variants matching your enum's variants
2. **EdgeIndex** enum - Contains variants for each indexed field
3. Projection structs - For accessing fields in a type-safe way

### Example

```rust
use graph_api_derive::EdgeExt;

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows {
        since: u32,
    },
    Created,
    Rated(Rating),
}
```

This generates:

```rust
// Label enum
pub enum EdgeLabel {
    Knows,
    Created,
    Rated,
}

// Index enum with methods
pub enum EdgeIndex {
    // EdgeIndex variants (if indexed fields exist)
}
```

### Generated Methods

The EdgeIndex enum offers methods for edge traversal:

```rust
// Query for all Knows edges
EdgeIndex::knows() -> EdgeSearch<'_, Graph>

// Specify outgoing direction
EdgeIndex::knows().outgoing()

// Specify incoming direction
EdgeIndex::knows().incoming()

// Limit result count
EdgeIndex::knows().limit(n)
```

## Walker Builder Filter Extensions

The derive macros also generate filter extension methods on the walker builders to simplify filtering based on
vertex/edge types.

### For Unit Variants

For unit variants (without fields), a single filter method is generated:

```rust
// Filter for all instances of the unit variant
fn filter_tag(self) -> /* walker builder */
```

Usage example:

```rust
// Get all Tag vertices
let tags = graph
.walk()
.vertices(VertexSearch::scan())
.filter_tag()
.collect::<Vec<_ > > ();
```

### For Named Fields Variants

For variants with named fields, two filter methods are generated:

```rust
// 1. Filter for all instances of this variant
fn filter_person(self) -> /* walker builder */

// 2. Filter with custom logic using the projected fields
fn filter_by_person<F>(self, filter: F) -> /* walker builder */
where
    F: Fn(Person<Graph::Vertex>, &Context) -> bool
```

Usage example:

```rust
// Get all Person vertices
let all_persons = graph
.walk()
.vertices(VertexSearch::scan())
.filter_person()
.collect::<Vec<_ > > ();

// Get Person vertices with specific criteria
let adults = graph
.walk()
.vertices(VertexSearch::scan())
.filter_by_person( | person, _ | person.age() > = 18)
.collect::<Vec<_ > > ();
```

### For Tuple Variants

For tuple variants, similar filter methods are generated:

```rust
// 1. Filter for all instances of this variant
fn filter_rated(self) -> /* walker builder */

// 2. Filter with custom logic using the tuple fields
fn filter_by_rated<F>(self, filter: F) -> /* walker builder */
where
    F: Fn(&Rating, &Context) -> bool
```

Usage example:

```rust
// Get edges with high ratings
let high_ratings = graph
.walk()
.vertices_by_id([person_id])
.edges(EdgeSearch::scan())
.filter_by_rated( | rating, _ | rating.stars > = 4)
.collect::<Vec<_ > > ();
```

### Benefits of Filter Extensions

These filter extensions provide several advantages:

1. **Type Safety** - The closures receive strongly typed projections
2. **Code Clarity** - Filters are expressive and self-documenting
3. **IDE Support** - Better autocompletion for variant fields
4. **Context Access** - Access to the walker's context object
5. **Pattern Matching** - No need for manual pattern matching

## Using Generated Types

### In Graph Queries

The generated types integrate with the Graph walker pattern:

```rust
// Find all person vertices
let people = graph
.walk()
.vertices(VertexIndex::person())
.collect::<Vec<_ > > ();

// Find people with a specific name
let named_people = graph
.walk()
.vertices(VertexIndex::person_by_name("Alice"))
.collect::<Vec<_ > > ();

// Find people in an age range
let adults = graph
.walk()
.vertices(VertexIndex::person_by_age_range(18..65))
.collect::<Vec<_ > > ();

// Find outgoing 'knows' edges from a vertex
let friends = graph
.walk()
.vertices_by_id([person_id])
.edges(EdgeIndex::knows().outgoing())
.collect::<Vec<_ > > ();
```

### Combined with Filter Extensions

Filter extensions can be combined with other walker steps:

```rust
// Find adults named "Alice" with a complex filter
let result = graph
.walk()
.vertices(VertexIndex::person())
.filter_by_person( | person, _ | {
person.age() > = 18 & & person.name().contains("Alice")
})
.collect::<Vec<_ > > ();

// Find friendship edges created before 2000
let old_friendships = graph
.walk()
.vertices_by_id([person_id])
.edges(EdgeIndex::knows().outgoing())
.filter_by_knows( | knows, _ | knows.since() < 2000)
.collect::<Vec<_ > > ();
```

### Type Constraints

When using these types, your Graph type needs to implement appropriate support:

```rust
fn example<G>(graph: &G)
where
    G: Graph<Vertex=Vertex, Edge=Edge>,
    G::SupportsVertexLabelIndex: Supported,
{
    // Now you can use label-based indexes
    graph.walk().vertices(VertexIndex::person())...
}
```

## Index Attributes

You can use these attributes on struct fields:

- `#[index]` - Basic indexing for efficient lookups
- `#[index(ordered)]` - Enables range queries
- `#[index(full_text)]` - Enables text search (String fields only)

## Best Practices

1. Use the appropriate index type for your query pattern:
    - Use label index for type filtering
    - Use property index for exact matches
    - Use range index for numeric ranges
    - Use full-text for searching text content

2. Apply indexes sparingly:
    - Each index adds memory overhead
    - Only index fields you'll query frequently, it's OK to filter once you are on the graph.

3. Consider the query planner:
    - Using an index in a vertices() step is typically more efficient than filtering the entire graph
    - Combining indices with other walker steps can create efficient traversal patterns

4. Use filter extensions for type-safety:
    - Prefer `filter_by_person()` over `filter()` with manual pattern matching
    - Leverage the projection types for field access
    - Use specific filter methods for clearer, more maintainable code
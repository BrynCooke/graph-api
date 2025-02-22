# Index Support in Graph API

The Graph API provides a powerful indexing system through static definitions. This document explains how indexing works and how to use it effectively.

## Overview

When you use `#[derive(VertexExt)]` or `#[derive(EdgeExt)]` on an enum, the `Element` trait automatically implemented. This trait is fundamental as it defines:
- Labels that identify different types of elements
- Indexes for fast element access

## Core Concepts

### Elements

Elements are the basic building blocks of the graph, represented by vertices and edges. Each element has:
- A label that identifies its type
- Optional indexed values for fast lookups

### Labels

Labels identify the type of an element (e.g., a "Person" vertex or a "Knows" edge). They provide:
- Runtime introspection capabilities
- Index definitions
- Unique identification through ordinals
- Human-readable names

### Indexes

Indexes enable fast access to elements based on their properties. The system supports three index types:

1. **Default Index**
    - Standard lookup by value
    - Suitable for exact matches

2. **Ordered Index**
    - Supports range searches
    - Useful for numeric or date-based queries

3. **Full-Text Index**
    - Enables text search capabilities
    - Ideal for searching through longer text content

## Implementation Details

### The Element Trait

```rust
pub trait Element: Debug {
    type Label: Eq + Copy + Hash + Debug + Label + 'static;
    
    fn label(&self) -> Self::Label;
    
    fn value(&self, _index: &Self::Index) -> Option<Value> {
        None
    }
}
```

This trait provides:
- Label type definition
- Label access
- Index value retrieval

### The Label Trait

```rust
pub trait Label
where
    Self: Sized + Copy + Eq + Hash + Debug,
{
    type Index: Eq + Copy + Hash + Debug + Index + 'static;
    
    fn variants() -> &'static [Self];
    fn indexes(&self) -> &'static [Self::Index];
    fn ordinal(&self) -> usize;
    fn name(&self) -> &'static str;
}
```

The Label trait enables:
- Index type definition
- Label variant enumeration
- Index association
- Unique identification
- Name resolution

## Usage Examples

### Searching the Graph

You can search the graph in two ways:

1. Using explicit search criteria:
```rust
graph.walk()
    .vertices(VertexSearch::index(VertexIndex::PersonName, "Bryn"))
    .collect::<Vec<_>>();
```

2. Using generated shortcut methods:
```rust
graph.walk()
    .vertices(VertexIndex::person_by_name("Bryn"))
    .collect::<Vec<_>>();
```

## Handling Mutations

When an element is mutated it may affect indexes that reference it. To support this the `VertexReferenceMut` and
`EdgeReferenceMut`
contain an associated type `MutationListener`.

This will be called whenever the setter method is called on a mutable projection of an element.

```rust
/// Trait to allow graphs to react to mutation of elements.
/// When an indexed element is updated the mutation listener is called with the index and the before and after values.
pub trait MutationListener<'reference, Element>
where
    Element: crate::Element,
{
    /// Called when a setter is called on a projection of an indexed `Element`.
    fn update(&mut self, index: Element::Index, before: Value, after: Value);
}
```

`MutationListener`s are created at the point where `project_mut` is called. For example here is the `SimpleGraph`
implementation:

```rust
impl<'graph, Graph> graph_api_lib::VertexReferenceMut<'graph, Graph>
for VertexReferenceMut<'graph, Graph>
where
    Graph: graph_api_lib::Graph<VertexId=VertexId> + 'graph,
{
    type MutationListener<'reference> = VertexMutationListener<'reference, Graph::Vertex>;

    fn weight_mut(&mut self) -> &mut Graph::Vertex {
        self.weight
    }

    fn project_mut<
        'reference,
        T: graph_api_lib::ProjectMut<
            'reference,
            <Graph as graph_api_lib::Graph>::Vertex,
            Self::MutationListener<'reference>,
        >,
    >(
        &'reference mut self,
    ) -> Option<T> {
        graph_api_lib::ProjectMut::project_mut(
            self.weight,
            VertexMutationListener {
                phantom_data: Default::default(),
                indexes: self.indexes,
                id: self.id.vertex(),
            },
        )
    }
}
```

`project_mut` creates a mutation listener with all the information that is required to update relevant indexes when
needed.


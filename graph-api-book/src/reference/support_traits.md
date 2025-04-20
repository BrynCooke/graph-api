# Support Traits

The Graph API uses a trait-based approach to indicate which optional features a graph implementation supports. This
allows for a more flexible and extensible API, where new capabilities can be added over time without breaking existing
implementations.

## Core Trait

The core `Graph` trait provides the essential functionality that all graph implementations must provide:

```rust
pub trait Graph: Sized + Debug {
    type Vertex: Debug + Element;
    type Edge: Debug + Element;
    type VertexId: Debug + Eq + PartialEq + Copy + Clone + Hash + Into<ElementId<Self>> + 'static;
    type EdgeId: Debug + Eq + PartialEq + Copy + Clone + Hash + Into<ElementId<Self>> + 'static;

    // Reference types
    type VertexReference<'graph>: VertexReference<'graph, Self>
    where
        Self: 'graph;
    type VertexReferenceMut<'graph>: VertexReferenceMut<'graph, Self>
    where
        Self: 'graph;
    type EdgeReference<'graph>: EdgeReference<'graph, Self>
    where
        Self: 'graph;
    type EdgeReferenceMut<'graph>: EdgeReferenceMut<'graph, Self>
    where
        Self: 'graph;

    // Iterator types
    type EdgeIter<'search, 'graph>: Iterator<Item=Self::EdgeReference<'graph>>
    where
        Self: 'graph;
    type VertexIter<'search, 'graph>: Iterator<Item=Self::VertexReference<'graph>>
    where
        Self: 'graph;

    // Core methods
    fn add_vertex(&mut self, vertex: Self::Vertex) -> Self::VertexId;
    fn add_edge(&mut self, from: Self::VertexId, to: Self::VertexId, edge: Self::Edge) -> Self::EdgeId;
    fn vertex(&self, id: Self::VertexId) -> Option<Self::VertexReference<'_>>;
    fn vertex_mut(&mut self, id: Self::VertexId) -> Option<Self::VertexReferenceMut<'_>>;
    fn vertices<'search>(&self, vertex_search: &VertexSearch<'search, Self>) -> Self::VertexIter<'search, '_>;
    fn edge(&self, id: Self::EdgeId) -> Option<Self::EdgeReference<'_>>;
    fn edge_mut(&mut self, id: Self::EdgeId) -> Option<Self::EdgeReferenceMut<'_>>;
    fn edges<'search>(&self, id: Self::VertexId, search: &EdgeSearch<'search, Self>) -> Self::EdgeIter<'search, '_>;

    // Default implementations
    fn dbg<T: Into<ElementId<Self>>>(&self, id: T) -> String { ... }
    fn walk(&self) -> StartWalkerBuilder<ImmutableMarker, Self> { ... }
    fn walk_mut(&mut self) -> StartWalkerBuilder<MutableMarker, Self> { ... }

    // Default implementation that panics if SupportsClear is not implemented
    fn clear(&mut self) {
        panic!("This graph implementation does not support clearing. Implement the SupportsClear trait for this graph type to add clearing support.")
    }
}
```

## Support Traits

These traits enable optional features for graph implementations. Each support trait extends the `Graph` trait, so
implementations must first implement `Graph` before they can implement any support traits.

### Vertex Index Support

```rust
/// Supports indexing of vertices by label
pub trait SupportsVertexLabelIndex: Graph {}

/// Supports indexing of vertices by field using a hash index
pub trait SupportsVertexHashIndex: Graph {}

/// Supports indexing of vertices by field with range queries
pub trait SupportsVertexRangeIndex: Graph {}

/// Supports indexing of vertices by field using a full text index
pub trait SupportsVertexFullTextIndex: Graph {}
```

### Edge Index Support

```rust
/// Supports indexing of edges by label
pub trait SupportsEdgeLabelIndex: Graph {}

/// Supports indexing of edges by field using a hash index
pub trait SupportsEdgeHashIndex: Graph {}

/// Supports indexing of edges by field with range queries
pub trait SupportsEdgeRangeIndex: Graph {}

/// Supports indexing of edges by adjacent vertex label
pub trait SupportsEdgeAdjacentLabelIndex: Graph {}
```

### Mutation Support

```rust
/// Supports clearing all vertices and edges
pub trait SupportsClear: Graph {
    /// Clears the graph, removing all vertices and edges
    fn clear(&mut self);
}

/// Supports removal of individual vertices and edges
pub trait SupportsElementRemoval: Graph {
    /// Removes a vertex from the graph and returns the vertex.
    fn remove_vertex(&mut self, id: Self::VertexId) -> Option<Self::Vertex>;

    /// Removes an edge from the graph and returns the edge.
    fn remove_edge(&mut self, id: Self::EdgeId) -> Option<Self::Edge>;
}
```

## Using Support Traits

When you implement a graph, first implement the core `Graph` trait, and then implement any support traits for the
features you want to provide:

```rust
// Core implementation
impl<V, E> Graph for MyGraph<V, E>
where
    V: Element,
    E: Element,
{
    // Implement required methods and types...
}

// Add support for vertex label indexing
impl<V, E> SupportsVertexLabelIndex for MyGraph<V, E>
where
    V: Element,
    E: Element,
{
    // No additional methods required
}

// Add support for clearing
impl<V, E> SupportsClear for MyGraph<V, E>
where
    V: Element,
    E: Element,
{
    fn clear(&mut self) {
        // Implementation to clear all vertices and edges
        self.vertices.clear();
        self.edges.clear();
        // Clear any indexes...
    }
}
```

## Using Supported Features

When writing code that uses supported features, use trait bounds to ensure the graph implementation supports the
required features:

```rust
// Function that uses vertex label indexing
fn get_people<G>(graph: &G) -> Vec<G::VertexId>
where
    G: Graph + SupportsVertexLabelIndex,
{
    // Can safely use label indexing here
    graph.walk()
        .vertices(Vertex::person())
        .map(|v| v.id())
        .collect()
}

// Function that uses range indexing
fn get_adults<G>(graph: &G) -> Vec<G::VertexId>
where
    G: Graph + SupportsVertexRangeIndex,
{
    // Can safely use range indexing here
    graph.walk()
        .vertices(Vertex::person_by_age_range(18..))
        .map(|v| v.id())
        .collect()
}

// Function that removes elements
fn remove_vertex_and_connected_edges<G>(graph: &mut G, id: G::VertexId)
where
    G: Graph + SupportsElementRemoval,
{
    // Can safely use element removal here
    graph.remove_vertex(id);
}
```

## Adding New Support Traits

The trait-based approach allows for adding new support traits over time without breaking existing code. To add a new
support trait:

1. Define the new trait extending the `Graph` trait
2. Add any required methods
3. Implement the trait for graph implementations that support the feature

```rust
// Example: Adding support for spatial indexing
pub trait SupportsSpatialIndex: Graph {
    // Optional specialized methods for spatial indexing
    fn nearest_neighbors(&self, point: Point, k: usize) -> Vec<Self::VertexId>;
}

// Implement for a graph that supports spatial indexing
impl<V, E> SupportsSpatialIndex for SpatialGraph<V, E>
where
    V: Element,
    E: Element,
{
    fn nearest_neighbors(&self, point: Point, k: usize) -> Vec<Self::VertexId> {
        // Implementation...
    }
}
```
# Implementation Guide for Graph Backends

This guide will walk you through the process of implementing a new graph backend for the Graph API. While the Graph API provides several ready-to-use implementations (like `SimpleGraph` and `PetGraph`), you might want to build your own implementation to support specific requirements or optimize for particular use cases.

## Overview

Implementing a graph backend involves satisfying the requirements of the `Graph` trait and its associated types. This guide will walk you through:

1. Understanding the core components needed for a graph implementation
2. Creating the fundamental data structures
3. Implementing the required traits
4. Building support for indexing (optional but recommended)
5. Testing your implementation

## Core Components

A Graph API implementation requires several components:

1. **Graph Structure**: The primary data structure that stores vertices and edges
2. **Vertex and Edge IDs**: Unique identifiers for graph elements
3. **References**: Structures that refer to vertices and edges
4. **Iterators**: Types for traversing vertices and edges
5. **Index Support**: Optional components for efficient element lookup by property values

## Step 1: Define IDs and Basic Structures

Start by defining your graph ID types and core data structure:

```rust
// Example vertex and edge IDs
pub struct MyVertexId {
    label: u16,    // Identifies the variant of the vertex enum
    index: u32,    // Unique index within that label
}

pub struct MyEdgeId {
    label: u16,    // Identifies the variant of the edge enum
    index: u32,    // Unique index within that label
    from: MyVertexId,
    to: MyVertexId,
}

// Import the necessary traits
use graph_api_lib::{
    Graph, Element, EdgeSearch, VertexSearch, 
    SupportsVertexLabelIndex, SupportsEdgeLabelIndex, SupportsVertexHashIndex,
    SupportsVertexRangeIndex, SupportsEdgeRangeIndex, SupportsVertexFullTextIndex,
    SupportsEdgeAdjacentLabelIndex, SupportsClear
};

// Main graph structure
pub struct MyGraph<Vertex, Edge>
where
    Vertex: Element,  // From graph_api_lib
    Edge: Element,    // From graph_api_lib
{
    // Your internal storage goes here
    // Example:
    vertices: Vec<YourVertexStorage<Vertex>>,
    edges: Vec<YourEdgeStorage<Edge>>, 
    // More fields as needed (indexes, etc.)
}
```

## Step 2: Define Reference Types

Create reference types that will be returned when accessing vertices and edges:

```rust
pub struct MyVertexReference<'graph, Graph>
where
    Graph: graph_api_lib::Graph,
{
    id: Graph::VertexId,
    weight: &'graph Graph::Vertex,
    // Additional fields if needed
}

pub struct MyVertexReferenceMut<'graph, Graph>
where
    Graph: graph_api_lib::Graph,
{
    id: Graph::VertexId,
    weight: &'graph mut Graph::Vertex,
    // Additional fields (like a reference to indexes)
    indexes: &'graph mut YourIndexStorage,
}

// Similar for EdgeReference and EdgeReferenceMut
```

## Step 3: Implement the Required Traits

Now implement the required traits for your reference types:

```rust
impl<'graph, Graph> graph_api_lib::VertexReference<'graph, Graph> for MyVertexReference<'graph, Graph>
where
    Graph: graph_api_lib::Graph,
{
    fn id(&self) -> Graph::VertexId {
        self.id
    }

    fn weight(&self) -> &Graph::Vertex {
        self.weight
    }

    fn project<
        'reference,
        T: graph_api_lib::Project<'reference, <Graph as graph_api_lib::Graph>::Vertex>,
    >(
        &'reference self,
    ) -> Option<T> {
        graph_api_lib::Project::project(self.weight)
    }
}

// Implement VertexReferenceMut, EdgeReference, EdgeReferenceMut similarly
```

## Step 4: Define Iterator Types

Create iterator types for walking through vertices and edges:

```rust
pub struct MyVertexIter<'search, 'graph, Graph>
where
    Graph: graph_api_lib::Graph + 'graph,
{
    // Internal state needed for iteration
    // Example:
    vertices: &'graph [YourVertexStorage<Graph::Vertex>],
    current_label: usize,
    current_index: usize,
    count: usize,
    limit: usize,
}

// Implement Iterator for MyVertexIter

impl<'graph, Graph> Iterator for MyVertexIter<'_, 'graph, Graph>
where
    Graph: graph_api_lib::Graph<VertexId = MyVertexId> + 'graph,
{
    type Item = MyVertexReference<'graph, Graph>;

    fn next(&mut self) -> Option<Self::Item> {
        // Your iterator implementation
        // Return the next vertex reference, or None if done
    }
}

// Similarly for MyEdgeIter
```

## Step 5: Implement the Graph Trait

Now implement the `Graph` trait for your graph structure:

```rust
// First implement the core Graph trait
impl<Vertex, Edge> Graph for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    // Define the core types
    type Vertex = Vertex;
    type Edge = Edge;
    type VertexId = MyVertexId;
    type EdgeId = MyEdgeId;
    
    // Reference types
    type VertexReference<'graph> = MyVertexReference<'graph, Self> where Self: 'graph;
    type VertexReferenceMut<'graph> = MyVertexReferenceMut<'graph, Self> where Self: 'graph;
    type EdgeReference<'graph> = MyEdgeReference<'graph, Self> where Self: 'graph;
    type EdgeReferenceMut<'graph> = MyEdgeReferenceMut<'graph, Self> where Self: 'graph;
    
    // Iterator types
    type EdgeIter<'search, 'graph> = MyEdgeIter<'search, 'graph, Self> where Self: 'graph;
    type VertexIter<'search, 'graph> = MyVertexIter<'search, 'graph, Self> where Self: 'graph;

    // Implement the core graph operations
    fn add_vertex(&mut self, vertex: Self::Vertex) -> Self::VertexId {
        // Implementation details
    }

    fn add_edge(
        &mut self,
        from: Self::VertexId,
        to: Self::VertexId,
        edge: Self::Edge,
    ) -> Self::EdgeId {
        // Implementation details
    }

    fn remove_vertex(&mut self, id: Self::VertexId) -> Option<Self::Vertex> {
        // Implementation details
    }

    fn remove_edge(&mut self, id: Self::EdgeId) -> Option<Self::Edge> {
        // Implementation details
    }

    fn vertex(&self, id: Self::VertexId) -> Option<Self::VertexReference<'_>> {
        // Implementation details
    }

    fn vertex_mut(&mut self, id: Self::VertexId) -> Option<Self::VertexReferenceMut<'_>> {
        // Implementation details
    }

    fn vertices<'search>(
        &self,
        vertex_search: &VertexSearch<'search, Self>,
    ) -> Self::VertexIter<'search, '_> {
        // Implementation details
    }

    fn edge(&self, id: Self::EdgeId) -> Option<Self::EdgeReference<'_>> {
        // Implementation details
    }

    fn edge_mut(&mut self, id: Self::EdgeId) -> Option<Self::EdgeReferenceMut<'_>> {
        // Implementation details
    }

    fn edges<'search>(
        &self,
        id: Self::VertexId,
        search: &EdgeSearch<'search, Self>,
    ) -> Self::EdgeIter<'search, '_> {
        // Implementation details
    }

    // No clear method here - it's moved to the SupportsClear trait
}

// Then implement support traits for the features you want to provide
impl<Vertex, Edge> SupportsVertexLabelIndex for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    // Any trait-specific methods would go here
}

impl<Vertex, Edge> SupportsEdgeLabelIndex for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    // Any trait-specific methods would go here
}

impl<Vertex, Edge> SupportsVertexHashIndex for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    // Any trait-specific methods would go here
}

// Add more support trait implementations as needed

// Implement SupportsClear if you want to support clearing the graph
impl<Vertex, Edge> SupportsClear for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    fn clear(&mut self) {
        // Clear all graph data
    }
}
```

## Step 6: Implement Indexing (Optional)

If your graph supports indexes, you'll need to implement the index handling. This typically involves:

1. Creating a mutation listener for updating indexes on vertex/edge changes
2. Creating storage for different index types (hash, range, full-text)
3. Implementing index update logic when vertices or edges are modified

```rust
// Example MutationListener for vertex modifications
pub struct MyVertexMutationListener<'reference, Element> {
    // References to indexes and ID information
    indexes: &'reference mut YourIndexStorage,
    id: YourInternalId,
}

impl<'reference, Element> graph_api_lib::MutationListener<'reference, Element>
    for MyVertexMutationListener<'reference, Element>
where
    Element: graph_api_lib::Element,
{
    fn update(&mut self, index: <Element::Label as Label>::Index, before: Value, after: Value) {
        // Update the indexes with the changed value
        // Remove old index entry
        // Add new index entry
    }
}
```

## Step 7: Advanced Features

Consider implementing additional features:

1. **Custom Traversal Optimizations**: For specific types of queries
2. **Specialized Index Types**: For domain-specific data types
3. **Persistence Layer**: If your graph needs to be saved/loaded
4. **Concurrency Support**: For multi-threaded access

## Step 8: Testing

Test your implementation using the Graph API's test suite:

```rust
#[cfg(test)]
mod test {
    use crate::MyGraph;
    use graph_api_test::test_suite;

    test_suite!(MyGraph::new());
}
```

## Best Practices

1. **Optimized ID Types**: Use small, efficient ID types (consider using newtype patterns)
2. **Memory Efficiency**: Consider spatial locality and data structure layout
3. **Index Handling**: Carefully manage indexes to avoid inconsistencies
4. **Error Handling**: Provide meaningful errors for invalid operations
5. **Documentation**: Document the specific characteristics of your implementation

## Example: Implementing a Simple Adjacency List Graph

Here's a sketch of how you might implement a simple adjacency list graph:

```rust
pub struct AdjListGraph<V, E> {
    vertices: Vec<Option<V>>,
    edges: Vec<Option<EdgeData<E>>>,
    adjacency: Vec<Vec<EdgeIndex>>, // Outgoing edges for each vertex
}

struct EdgeData<E> {
    edge: E,
    from: VertexIndex,
    to: VertexIndex,
}

// Implement the necessary traits...
```

## Conclusion

Implementing a graph backend requires careful attention to detail, but the Graph API's trait system provides a clear structure to follow. By implementing the core traits and considering performance implications, you can create a specialized graph backend that perfectly fits your use case.

Remember to check the source code of existing implementations like `SimpleGraph` for more detailed examples of how to handle complex scenarios like indexing and traversal.
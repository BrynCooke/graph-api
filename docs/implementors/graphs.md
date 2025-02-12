# Implementing your first Graph

Adding Graph API support is as simple as providing implementations the following traits:

* `Graph`: The main interface to your graph
* `VertexReference`: A reference to a vertex
* `VertexReferenceMut`: A mutable reference to a vertex
    * `MutationListener`: A listener that reacts to vertex mutations
* `EdgeReference`: A reference to an edge
* `EdgeReferenceMut`: A mutable reference to an edge
    * `MutationListener`: A listener that reacts to edge mutations
* `VertexIter`: An iterator over vertices
* `EdgeIter`: An iterator over edges

## Step by step

Start by implementing your graph structures:

```rust
struct MyGraph {}
struct VertexReference<'graph, Graph> {}
struct VertexReferenceMut<'graph, Graph> {}
struct EdgeReference<'graph, Graph> {}
struct EdgeReferenceMut<'graph, Graph> {}
struct VertexIter<'graph, Graph> {}
struct EdgeIter<'graph, Graph> {}
struct VertexId {}
struct EdgeId {}
```

We are omitting `MutationListener` initially as this is only required for graphs that support indexing.

Now implement Graph:

```rust

impl<Vertex, Edge> Graph for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    // We're going to keep things simple and not support indexing initially
    type SupportsVertexLabelIndex = Unsupported;
    type SupportsEdgeLabelIndex = Unsupported;
    type SupportsVertexIndex = Unsupported;
    type SupportsEdgeIndex = Unsupported;
    type SupportsVertexOrderedIndex = Unsupported;
    type SupportsEdgeOrderedIndex = Unsupported;
    type SupportsVertexFullTextIndex = Unsupported;
    type SupportsClear = Unsupported;
    type Vertex = Vertex;
    type Edge = Edge;
    type VertexId = VertexId;
    type EdgeId = EdgeId;
    type VertexReference<'graph> = VertexReference<'graph, Self>
    where
        Self: 'graph;
    type VertexReferenceMut<'graph> = VertexReferenceMut<'graph, Self>
    where
        Self: 'graph;
    type EdgeReference<'graph> = EdgeReference<'graph, Self>
    where
        Self: 'graph;
    type EdgeReferenceMut<'graph> = EdgeReferenceMut<'graph, Self>
    where
        Self: 'graph;
    type EdgeIter<'graph> = EdgeIter<'graph, Self>
    where
        Self: 'graph;
    type VertexIter<'graph> = VertexIter<'graph, Self>
    where
        Self: 'graph;
    fn add_vertex(&mut self, vertex: Self::Vertex) -> Self::VertexId {
        todo!()
    }

    fn add_edge(
        &mut self,
        from: Self::VertexId,
        to: Self::VertexId,
        edge: Self::Edge,
    ) -> Self::EdgeId {
        todo!()
    }

    fn remove_vertex(&mut self, id: Self::VertexId) -> Option<Self::Vertex> {
        todo!()
    }

    fn remove_edge(&mut self, edge: Self::EdgeId) -> Option<Self::Edge> {
        todo!()
    }

    fn vertex(&self, id: Self::VertexId) -> Option<Self::VertexReference<'_>> {
        todo!()
    }

    fn vertex_mut(&mut self, id: Self::VertexId) -> Option<Self::VertexReferenceMut<'_>> {
        todo!()
    }

    fn vertices<'a>(
        &'a self,
        vertex_search: &graph_api_lib::VertexSearch<Self>,
    ) -> Self::VertexIter<'a> {
        todo!()
    }

    fn edge(&self, id: Self::EdgeId) -> Option<Self::EdgeReference<'_>> {
        todo!()
    }

    fn edge_mut(&mut self, edge: Self::EdgeId) -> Option<Self::EdgeReferenceMut<'_>> {
        todo!()
    }

    fn edges(&self, vertex: Self::VertexId, search: &EdgeSearch<Self>) -> Self::EdgeIter<'_> {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }
}
```

Go ahead and do the same for:

* VertexReference
* VertexReferenceMut
* EdgeReference
* EdgeReferenceMut
* VertexIter
* EdgeIter

You've now created a non-functional graph!

The best way to make your graph functional is to use the graph-api-test crate to add tests, and then get
working on your implementation.

See [testing](./testing.md) for more details
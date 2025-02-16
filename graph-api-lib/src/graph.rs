use crate::walker::builder::{ImmutableMarker, MutableMarker, StartWalkerBuilder};
use crate::VertexSearch;
use crate::{walker, EdgeSearch, Element, Value};
use std::fmt::Debug;
use std::hash::Hash;

/// Marker for feature support
pub trait Support {}

/// Marker for feature support
pub struct Supported {}

impl Support for Supported {}

/// Marker for feature support
pub struct Unsupported {}

impl Support for Unsupported {}

/// The direction of an edge in a graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord)]
#[repr(u8)]
pub enum Direction {
    /// Outgoing edges.
    Outgoing,

    /// Incoming edges.
    Incoming,

    /// Both incoming and outgoing edges.
    #[default]
    All,
}

impl Direction {
    pub fn reverse(&self) -> Self {
        match self {
            Direction::Outgoing => Direction::Incoming,
            Direction::Incoming => Direction::Outgoing,
            Direction::All => Direction::All,
        }
    }
}

/// An identifier for a vertex or an edge in a graph.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Id<VertexId, EdgeId>
where
    VertexId: Eq + Copy + Clone,
    EdgeId: Eq + Copy + Clone,
{
    /// A vertex identifier.
    Vertex(VertexId),

    /// An edge identifier.
    Edge(EdgeId),
}

/// Graphs that implement this trait can be used with the walker API.
pub trait Graph: Sized + Debug {
    /// Supports indexing of vertices by label.
    type SupportsVertexLabelIndex: Support;
    /// Supports indexing of edges by label.
    type SupportsEdgeLabelIndex: Support;
    /// Supports indexing of vertices by field.
    type SupportsVertexIndex: Support;
    /// Supports indexing of edges by field.
    type SupportsEdgeIndex: Support;
    /// Supports indexing of vertices by field with range support.
    type SupportsVertexOrderedIndex: Support;
    /// Supports indexing of edges by field with range support.
    type SupportsEdgeOrderedIndex: Support;
    /// Supports indexing of vertices by field using an inverted full text index.
    type SupportsVertexFullTextIndex: Support;
    /// Supports indexing of edges by adjacent vertex label.
    type SupportsEdgeAdjacentLabelIndex: Support;
    /// Supports clearing of all vertices and edges
    type SupportsClear: Support;

    /// The type of the vertices in the graph. This is usually an enum.
    type Vertex: Debug + Element;

    /// The type of the edges in the graph. This is usually an enum.
    type Edge: Debug + Element;

    /// The `VertexId` type of the graph.
    type VertexId: Debug
        + Eq
        + PartialEq
        + Copy
        + Clone
        + Hash
        + Into<Id<Self::VertexId, Self::EdgeId>>
        + 'static;

    /// The `EdgeId` type of the graph.
    type EdgeId: Debug
        + Eq
        + PartialEq
        + Copy
        + Clone
        + Hash
        + Into<Id<Self::VertexId, Self::EdgeId>>
        + 'static;

    /// A reference to a vertex.
    type VertexReference<'graph>: VertexReference<'graph, Self>
    where
        Self: 'graph;

    /// A mut reference to a vertex.
    type VertexReferenceMut<'graph>: VertexReferenceMut<'graph, Self>
    where
        Self: 'graph;

    /// An edge reference is used during walking over edges. It includes the id, tail, head and weight of the edge.
    type EdgeReference<'graph>: EdgeReference<'graph, Self>
    where
        Self: 'graph;

    /// A mut edge reference to modify the edge.
    type EdgeReferenceMut<'graph>: EdgeReferenceMut<'graph, Self>
    where
        Self: 'graph;

    /// An iterator over the edge references.
    type EdgeIter<'search, 'graph>: Iterator<Item = Self::EdgeReference<'graph>>
    where
        Self: 'graph;

    /// An iterator over the vertex references.
    type VertexIter<'search, 'graph>: Iterator<Item = Self::VertexReference<'graph>>
    where
        Self: 'graph;

    /// Adds a vertex to the graph and returns its identifier.
    fn add_vertex(&mut self, vertex: Self::Vertex) -> Self::VertexId;

    /// Adds an edge to the graph and returns its identifier.
    fn add_edge(
        &mut self,
        from: Self::VertexId,
        to: Self::VertexId,
        edge: Self::Edge,
    ) -> Self::EdgeId;

    /// Removes a vertex from the graph and returns the vertex.
    fn remove_vertex(&mut self, id: Self::VertexId) -> Option<Self::Vertex>;

    /// Removes an edge from the graph and returns the edge.
    fn remove_edge(&mut self, id: Self::EdgeId) -> Option<Self::Edge>;

    /// Gets the vertex with the specified identifier.
    fn vertex(&self, id: Self::VertexId) -> Option<Self::VertexReference<'_>>;

    /// Returns the vertex with the specified identifier.
    fn vertex_mut(&mut self, id: Self::VertexId) -> Option<Self::VertexReferenceMut<'_>>;

    /// Iterate over vertex identifiers.
    /// Graphs should try to narrow down the returned vertices using the search criteria, but overfetch will be filtered.
    fn vertices<'search>(
        &self,
        vertex_search: &VertexSearch<'search, Self>,
    ) -> Self::VertexIter<'search, '_>;

    /// Gets the edge with the specified identifier.
    fn edge(&self, id: Self::EdgeId) -> Option<Self::EdgeReference<'_>>;

    /// Gets the edge with the specified identifier.
    fn edge_mut(&mut self, id: Self::EdgeId) -> Option<Self::EdgeReferenceMut<'_>>;

    /// Returns an iterator over the edges of a vertex.
    /// Graphs should try to narrow down the returned edges using the search criteria, but overfetch will be filtered.
    fn edges<'search>(
        &self,
        id: Self::VertexId,
        search: &EdgeSearch<'search, Self>,
    ) -> Self::EdgeIter<'search, '_>;

    /// Clears the graph. This may not be supported by all graph implementations.
    fn clear(&mut self)
    where
        Self: Graph<SupportsClear = Supported>;

    /// Returns a string representation of an element in the graph.
    fn dbg<T: Into<Id<Self::VertexId, Self::EdgeId>>>(&self, id: T) -> String {
        match id.into() {
            Id::Vertex(vertex_id) => self
                .vertex(vertex_id)
                .map_or_else(|| "<missing>".to_string(), |vertex| format!("{:?}", vertex)),
            Id::Edge(edge_id) => self
                .edge(edge_id)
                .map_or_else(|| "<missing>".to_string(), |edge| format!("{:?}", edge)),
        }
    }

    /// Returns an immutable walker builder for the graph.
    fn walk(&self) -> StartWalkerBuilder<ImmutableMarker, Self>
    where
        Self: Sized,
    {
        walker::builder::new_start(self)
    }

    /// Returns a mutable walker builder for the graph.
    fn walk_mut(&mut self) -> StartWalkerBuilder<MutableMarker, Self>
    where
        Self: Sized,
    {
        walker::builder::new_start_mut(self)
    }
}

pub trait VertexReference<'graph, Graph>: Debug
where
    Graph: crate::Graph,
{
    fn id(&self) -> Graph::VertexId;

    fn weight(&self) -> &Graph::Vertex;

    fn project<'reference, T: Project<'reference, <Graph as crate::Graph>::Vertex>>(
        &'reference self,
    ) -> Option<T>;
}

pub trait VertexReferenceMut<'graph, Graph>: VertexReference<'graph, Graph>
where
    Graph: crate::graph::Graph + 'graph,
{
    type MutationListener<'reference>: MutationListener<'reference, Graph::Vertex>;
    /// Get the raw mutable vertex weight.
    /// WARNING! It is advised to use the generated projections to get a typed reference to the vertex and use the set_ methods instead.
    /// It is only safe to use this if you are mutating non-indexed fields.
    /// Incorrect usage of this method will result in graph indexes being corrupted.
    fn weight_mut(&mut self) -> &mut Graph::Vertex;
    fn project_mut<
        'reference,
        T: ProjectMut<'reference, <Graph as crate::Graph>::Vertex, Self::MutationListener<'reference>>,
    >(
        &'reference mut self,
    ) -> Option<T>;
}

/// A reference to an edge in a graph.
pub trait EdgeReference<'graph, Graph>: Debug
where
    Graph: crate::graph::Graph,
{
    /// Returns the identifier of the edge.
    fn id(&self) -> Graph::EdgeId;

    /// Returns the identifier of the tail vertex of the edge.
    fn tail(&self) -> Graph::VertexId;

    /// Returns the identifier of the head vertex of the edge.
    fn head(&self) -> Graph::VertexId;

    /// Returns a reference to the weight of the edge.
    fn weight(&self) -> &Graph::Edge;

    fn project<'reference, T: Project<'reference, <Graph as crate::Graph>::Edge>>(
        &'reference self,
    ) -> Option<T>;
}

/// A mutable reference to an edge in a graph.
/// This trait extends the `EdgeReference` trait and provides a method to
/// obtain a mutable reference to the weight of the edge.
pub trait EdgeReferenceMut<'graph, Graph>: EdgeReference<'graph, Graph>
where
    Graph: crate::Graph,
{
    type MutationListener<'reference>: MutationListener<'reference, Graph::Edge>;

    fn weight_mut(&mut self) -> &mut Graph::Edge;

    fn project_mut<
        'reference,
        T: ProjectMut<'reference, <Graph as crate::Graph>::Edge, Self::MutationListener<'reference>>,
    >(
        &'reference mut self,
    ) -> Option<T>;
}

pub trait Project<'reference, Weight>
where
    Weight: Element,
    Self: Sized,
{
    fn project(weight: &'reference Weight) -> Option<Self>;
}

pub trait ProjectMut<'reference, Weight, MutationListener>
where
    Weight: Element,
    MutationListener: crate::MutationListener<'reference, Weight>,
    Self: Sized,
{
    fn project_mut(
        weight: &'reference mut Weight,
        mutation_listener: MutationListener,
    ) -> Option<Self>;
}

/// Trait to allow graphs to react to mutation of elements.
/// When an indexed element is updated the mutation listener is called with the index and the before and after values.
pub trait MutationListener<'reference, Element>
where
    Element: crate::Element,
{
    /// Called when a setter is called on a projection of an indexed `Element`.
    fn update(&mut self, index: Element::Index, before: Value, after: Value);
}

impl<Element> MutationListener<'_, Element> for ()
where
    Element: crate::Element,
{
    fn update(&mut self, _index: <Element>::Index, _before: Value, _after: Value) {}
}

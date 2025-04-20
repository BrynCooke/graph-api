use crate::element::Element;
use crate::walker::builder::{ImmutableMarker, MutableMarker, StartWalkerBuilder};
use crate::{EdgeSearch, Value, walker};
use crate::{Label, VertexSearch};
use derivative::Derivative;
use std::fmt::Debug;
use std::hash::Hash;

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
    /// Returns the reverse of this direction.
    ///
    /// - `Outgoing` becomes `Incoming`
    /// - `Incoming` becomes `Outgoing`
    /// - `All` remains `All`
    ///
    /// # Returns
    /// The reversed direction.
    pub fn reverse(&self) -> Self {
        match self {
            Direction::Outgoing => Direction::Incoming,
            Direction::Incoming => Direction::Outgoing,
            Direction::All => Direction::All,
        }
    }
}

/// An identifier for a vertex or an edge in a graph.
#[derive(Debug, Derivative)]
#[derivative(
    Copy(bound = ""),
    Clone(bound = ""),
    Eq(bound = ""),
    PartialEq(bound = ""),
    Hash(bound = "")
)]
pub enum ElementId<Graph>
where
    Graph: crate::Graph,
{
    /// A vertex identifier.
    Vertex(Graph::VertexId),

    /// An edge identifier.
    Edge(Graph::EdgeId),
}

/// Graphs that implement this trait can be used with the walker API.
pub trait Graph: Sized + Debug {
    /// The type of the vertices in the graph. This is usually an enum.
    type Vertex: Debug + Element;

    /// The type of the edges in the graph. This is usually an enum.
    type Edge: Debug + Element;

    /// The `VertexId` type of the graph.
    type VertexId: Debug + Eq + PartialEq + Copy + Clone + Hash + Into<ElementId<Self>> + 'static;

    /// The `EdgeId` type of the graph.
    type EdgeId: Debug + Eq + PartialEq + Copy + Clone + Hash + Into<ElementId<Self>> + 'static;

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

    /// Clears the graph. Default implementation returns an error.
    /// Implement the `SupportsClear` trait to provide this functionality.
    fn clear(&mut self) {
        panic!("This graph implementation does not support clearing")
    }

    /// Returns a string representation of an element in the graph.
    fn dbg<T: Into<ElementId<Self>>>(&self, id: T) -> String {
        match id.into() {
            ElementId::Vertex(vertex_id) => self
                .vertex(vertex_id)
                .map_or_else(|| "<missing>".to_string(), |vertex| format!("{:?}", vertex)),
            ElementId::Edge(edge_id) => self
                .edge(edge_id)
                .map_or_else(|| "<missing>".to_string(), |edge| format!("{:?}", edge)),
        }
    }

    /// Returns an immutable walker builder for the graph.
    fn walk(&self) -> StartWalkerBuilder<ImmutableMarker, Self, ()>
    where
        Self: Sized,
    {
        walker::builder::new_start(self)
    }

    /// Returns a mutable walker builder for the graph.
    fn walk_mut(&mut self) -> StartWalkerBuilder<MutableMarker, Self, ()>
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
    /// Return the ID of the vertex
    fn id(&self) -> Graph::VertexId;

    /// Return the weight of the vertex
    fn weight(&self) -> &Graph::Vertex;

    /// Project the element to a mutation safe struct representing a single labelled vertex.
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

    /// Project the element to a mutation safe struct representing a single labelled vertex. Modifications to the projection will be reflected in graph indexes.
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

    /// Project the element to a struct representing a single labelled edge.
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

    /// Get the raw mutable vertex weight.
    /// WARNING! It is advised to use the generated projections to get a typed reference to the vertex and use the set_ methods instead.
    /// It is only safe to use this if you are mutating non-indexed fields.
    /// Incorrect usage of this method will result in graph indexes being corrupted.
    fn weight_mut(&mut self) -> &mut Graph::Edge;

    /// Project the element to a mutation safe struct representing a single labelled edge. Modifications to the projection will be reflected in graph indexes.
    fn project_mut<
        'reference,
        T: ProjectMut<'reference, <Graph as crate::Graph>::Edge, Self::MutationListener<'reference>>,
    >(
        &'reference mut self,
    ) -> Option<T>;
}

/// Enables projecting a graph element weight to a specific type.
///
/// This trait provides a mechanism to safely convert a generic graph element
/// (vertex or edge) to a specific, strongly-typed representation.
///
/// # Type Parameters
/// - `'reference`: The lifetime of the reference to the weight
/// - `Weight`: The type of the element weight being projected
pub trait Project<'reference, Weight>
where
    Weight: Element,
    Self: Sized,
{
    /// Attempts to convert a weight to this specific type.
    ///
    /// # Parameters
    /// - `weight`: The element weight to project
    ///
    /// # Returns
    /// `Some(Self)` if the weight can be projected to this type, `None` otherwise.
    fn project(weight: &'reference Weight) -> Option<Self>;
}

/// Enables projecting a graph element weight to a mutable specific type.
///
/// This trait provides a mechanism to safely convert a generic graph element
/// (vertex or edge) to a specific, strongly-typed mutable representation.
/// Changes to the projected type will be tracked using the mutation listener.
///
/// # Type Parameters
/// - `'reference`: The lifetime of the mutable reference to the weight
/// - `Weight`: The type of the element weight being projected
/// - `MutationListener`: A type that listens for mutations to track index updates
pub trait ProjectMut<'reference, Weight, MutationListener>
where
    Weight: Element,
    MutationListener: crate::MutationListener<'reference, Weight>,
    Self: Sized,
{
    /// Attempts to convert a weight to this specific mutable type.
    ///
    /// # Parameters
    /// - `weight`: The element weight to project
    /// - `mutation_listener`: The listener that will track mutations to the weight
    ///
    /// # Returns
    /// `Some(Self)` if the weight can be projected to this type, `None` otherwise.
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
    fn update(&mut self, index: <Element::Label as Label>::Index, before: Value, after: Value);
}

impl<Element> MutationListener<'_, Element> for ()
where
    Element: crate::Element,
{
    fn update(&mut self, _index: <Element::Label as Label>::Index, _before: Value, _after: Value) {}
}

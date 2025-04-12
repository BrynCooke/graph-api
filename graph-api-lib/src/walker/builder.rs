use crate::walker::steps::Empty;
use crate::walker::{EdgeWalker, VertexWalker};
use std::marker::PhantomData;

/// A marker trait for types that can be mutated.
///
/// This trait is used to differentiate between mutable and immutable graph walkers.
pub trait Mutable {}

/// A marker type that indicates mutability.
///
/// Used as a type parameter in walker builders to enable mutable graph operations.
pub struct MutableMarker;

impl Mutable for MutableMarker {}

/// A marker type that indicates immutability.
///
/// Used as a type parameter in walker builders to restrict operations to immutable ones.
pub struct ImmutableMarker;

/// Manages access to a graph during traversal operations.
///
/// This enum represents the three states of graph access:
/// - Immutable: A shared reference to the graph
/// - Mutable: An exclusive reference to the graph
/// - Taken: The reference has been consumed
#[derive(Default)]
pub enum GraphAccess<'graph, Graph> {
    /// A shared reference to a graph
    Immutable(&'graph Graph),
    /// An exclusive reference to a graph
    Mutable(&'graph mut Graph),
    /// The reference has been taken and can no longer be used
    #[default]
    Taken,
}

impl<'graph, Graph> GraphAccess<'graph, Graph> {
    /// Takes the graph reference, leaving the enum in the `Taken` state.
    ///
    /// # Returns
    /// A shared reference to the graph.
    ///
    /// # Panics
    /// Panics if the graph reference has already been taken.
    pub fn take(&mut self) -> &'graph Graph {
        match std::mem::take(self) {
            GraphAccess::Immutable(graph) => graph,
            GraphAccess::Mutable(graph) => graph,
            GraphAccess::Taken => panic!("graph already taken"),
        }
    }

    /// Takes the graph reference for mutation, leaving the enum in the `Taken` state.
    ///
    /// # Returns
    /// An exclusive reference to the graph.
    ///
    /// # Panics
    /// Panics if the graph reference has already been taken or if it's an immutable reference.
    pub fn take_mut(&mut self) -> &'graph mut Graph {
        match std::mem::take(self) {
            GraphAccess::Immutable(_) => {
                panic!("graph should not have been accessed mutably")
            }
            GraphAccess::Mutable(graph) => graph,
            GraphAccess::Taken => panic!("graph already taken"),
        }
    }
}

/// A generic builder for graph walkers.
///
/// This builder is used to construct and compose graph traversal operations.
///
/// # Type Parameters
/// - `'graph`: The lifetime of the graph reference
/// - `Mutability`: A marker type indicating whether graph mutations are allowed
/// - `Graph`: The graph type being traversed
/// - `Walker`: The walker implementation that will perform the traversal
pub struct WalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Walker: crate::walker::Walker<'graph>,
{
    pub(crate) _phantom: PhantomData<&'graph (Mutability, Graph, Walker)>,
    pub(crate) walker: Walker,
}

impl<'graph, Mutability, Graph, Walker> WalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Walker: crate::walker::Walker<'graph>,
{
    /// Returns a mutable reference to the underlying walker.
    ///
    /// This method provides access to the walker being built, allowing for direct manipulation.
    ///
    /// # Returns
    /// A mutable reference to the walker.
    pub fn walker(&mut self) -> &mut Walker {
        &mut self.walker
    }
}

impl<'graph, Mutability, Graph, Walker> From<VertexWalkerBuilder<'graph, Mutability, Graph, Walker>>
    for WalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    fn from(value: VertexWalkerBuilder<'graph, Mutability, Graph, Walker>) -> Self {
        WalkerBuilder {
            _phantom: Default::default(),
            walker: value.walker,
        }
    }
}

impl<'graph, Mutability, Graph, Walker> From<EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>>
    for WalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    fn from(value: EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>) -> Self {
        WalkerBuilder {
            _phantom: Default::default(),
            walker: value.walker,
        }
    }
}

/// A builder for vertex-focused graph traversals.
///
/// This builder constructs walkers that navigate graphs by moving from vertex to vertex.
///
/// # Type Parameters
/// - `'graph`: The lifetime of the graph reference
/// - `Mutability`: A marker type indicating whether graph mutations are allowed
/// - `Graph`: The graph type being traversed
/// - `Walker`: The vertex walker implementation that will perform the traversal
pub struct VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    _phantom: PhantomData<&'graph Mutability>,
    walker: Walker,
    graph: GraphAccess<'graph, Graph>,
}

pub(crate) fn new<'graph, Mutability, Graph, Start>(
    graph: GraphAccess<'graph, Graph>,
    start: Start,
) -> VertexWalkerBuilder<'graph, Mutability, Graph, Start>
where
    Graph: crate::graph::Graph,
    Start: VertexWalker<'graph, Graph = Graph>,
{
    VertexWalkerBuilder {
        _phantom: Default::default(),
        walker: start,
        graph,
    }
}

#[allow(dead_code)]
pub(crate) fn new_mut<'graph, Graph, Start>(
    graph: &'graph mut Graph,
    start: Start,
) -> VertexWalkerBuilder<'graph, MutableMarker, Graph, Start>
where
    Graph: crate::graph::Graph,
    Start: VertexWalker<'graph, Graph = Graph>,
{
    VertexWalkerBuilder {
        _phantom: Default::default(),
        walker: start,
        graph: GraphAccess::Mutable(graph),
    }
}

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// Returns an immutable reference to the graph being traversed.
    ///
    /// This consumes the graph reference, leaving the builder in a state
    /// where the graph reference has been taken.
    ///
    /// # Returns
    /// An immutable reference to the graph.
    pub fn graph(&mut self) -> &'graph Graph {
        self.graph.take()
    }

    /// Returns a mutable reference to the graph being traversed.
    ///
    /// This consumes the graph reference, leaving the builder in a state
    /// where the graph reference has been taken.
    ///
    /// # Returns
    /// A mutable reference to the graph.
    ///
    /// # Panics
    /// Panics if the graph was not accessed mutably or has already been taken.
    pub fn graph_mut(&mut self) -> &'graph mut Graph {
        self.graph.take_mut()
    }

    /// Consumes the builder and returns the underlying walker.
    ///
    /// This extracts the walker from the builder, allowing it to be used directly.
    ///
    /// # Returns
    /// The walker that was being built.
    pub fn walker(self) -> Walker {
        self.walker
    }

    /// Transforms the current vertex walker into an edge walker.
    ///
    /// This method allows for changing the traversal from vertex-oriented to edge-oriented
    /// by applying a transformation function to the current walker.
    ///
    /// # Type Parameters
    /// - `EdgeWalker`: The target edge walker type
    /// - `WithFn`: A function type that converts the current walker to an edge walker
    ///
    /// # Parameters
    /// - `step`: The transformation function to apply
    ///
    /// # Returns
    /// A new edge walker builder containing the transformed walker
    pub fn with_edge_walker<
        EdgeWalker: crate::walker::EdgeWalker<'graph>,
        WithFn: FnOnce(Walker) -> EdgeWalker,
    >(
        self,
        step: WithFn,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeWalker> {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: step(self.walker),
            graph: self.graph,
        }
    }

    /// Transforms the current vertex walker into another vertex walker.
    ///
    /// This method allows for changing or extending the traversal while remaining
    /// vertex-oriented by applying a transformation function to the current walker.
    ///
    /// # Type Parameters
    /// - `VertexWalker`: The target vertex walker type
    /// - `WithFn`: A function type that converts the current walker to another vertex walker
    ///
    /// # Parameters
    /// - `step`: The transformation function to apply
    ///
    /// # Returns
    /// A new vertex walker builder containing the transformed walker
    pub fn with_vertex_walker<
        VertexWalker: crate::walker::VertexWalker<'graph, Graph = Graph>,
        WithFn: FnOnce(Walker) -> VertexWalker,
    >(
        self,
        step: WithFn,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexWalker> {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: step(self.walker),
            graph: self.graph,
        }
    }
}

/// A builder for edge-focused graph traversals.
///
/// This builder constructs walkers that navigate graphs by moving along edges.
///
/// # Type Parameters
/// - `'graph`: The lifetime of the graph reference
/// - `Mutability`: A marker type indicating whether graph mutations are allowed
/// - `Graph`: The graph type being traversed
/// - `Walker`: The edge walker implementation that will perform the traversal
pub struct EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Walker: EdgeWalker<'graph>,
{
    _phantom: PhantomData<&'graph Mutability>,
    walker: Walker,
    graph: GraphAccess<'graph, Graph>,
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
    <Walker as crate::walker::Walker<'graph>>::Context: Clone + 'static,
{
    /// Returns an immutable reference to the graph being traversed.
    ///
    /// This consumes the graph reference, leaving the builder in a state
    /// where the graph reference has been taken.
    ///
    /// # Returns
    /// An immutable reference to the graph.
    pub fn graph(&mut self) -> &'graph Graph {
        self.graph.take()
    }

    /// Returns a mutable reference to the graph being traversed.
    ///
    /// This consumes the graph reference, leaving the builder in a state
    /// where the graph reference has been taken.
    ///
    /// # Returns
    /// A mutable reference to the graph.
    ///
    /// # Panics
    /// Panics if the graph was not accessed mutably or has already been taken.
    pub fn graph_mut(&mut self) -> &'graph mut Graph {
        self.graph.take_mut()
    }

    /// Consumes the builder and returns the underlying walker.
    ///
    /// This extracts the walker from the builder, allowing it to be used directly.
    ///
    /// # Returns
    /// The walker that was being built.
    pub fn walker(self) -> Walker {
        self.walker
    }

    /// Transforms the current walker into another edge walker.
    ///
    /// This method allows for changing or extending the traversal using
    /// a transformation function applied to the current walker.
    ///
    /// # Type Parameters
    /// - `EdgeWalker`: The target edge walker type
    /// - `WithFn`: A function type that converts the current walker
    ///
    /// # Parameters
    /// - `step`: The transformation function to apply
    ///
    /// # Returns
    /// A new edge walker builder containing the transformed walker
    pub fn with_edge_walker<
        EdgeWalker: crate::walker::EdgeWalker<'graph>,
        WithFn: FnOnce(Walker) -> EdgeWalker,
    >(
        self,
        step: WithFn,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeWalker> {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: step(self.walker),
            graph: self.graph,
        }
    }

    /// Transforms the current walker into a vertex walker.
    ///
    /// This method allows for changing the traversal from edge-oriented to vertex-oriented
    /// by applying a transformation function to the current walker.
    ///
    /// # Type Parameters
    /// - `VertexWalker`: The target vertex walker type
    /// - `WithFn`: A function type that converts the current walker
    ///
    /// # Parameters
    /// - `step`: The transformation function to apply
    ///
    /// # Returns
    /// A new vertex walker builder containing the transformed walker
    pub fn with_vertex_walker<
        VertexWalker: crate::walker::VertexWalker<'graph, Graph = Graph>,
        WithFn: FnOnce(Walker) -> VertexWalker,
    >(
        self,
        step: WithFn,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexWalker> {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: step(self.walker),
            graph: self.graph,
        }
    }
}

pub(crate) fn new_start<Graph>(graph: &Graph) -> StartWalkerBuilder<'_, ImmutableMarker, Graph, ()>
where
    Graph: crate::graph::Graph,
{
    StartWalkerBuilder {
        _phantom: Default::default(),
        graph: GraphAccess::Immutable(graph),
        empty: Empty::default(),
    }
}

pub(crate) fn new_start_mut<Graph>(
    graph: &mut Graph,
) -> StartWalkerBuilder<'_, MutableMarker, Graph, ()>
where
    Graph: crate::graph::Graph,
{
    StartWalkerBuilder {
        _phantom: Default::default(),
        graph: GraphAccess::Mutable(graph),
        empty: Empty::default(),
    }
}

/// The initial builder for starting a graph traversal.
///
/// This builder is the entry point for creating graph traversals. It contains
/// a reference to the graph and an empty context, and provides methods to begin
/// traversing the graph.
///
/// # Type Parameters
/// - `'graph`: The lifetime of the graph reference
/// - `Mutability`: A marker type indicating whether graph mutations are allowed
/// - `Graph`: The graph type being traversed
/// - `Context`: The initial context type for the traversal
pub struct StartWalkerBuilder<'graph, Mutability, Graph, Context> {
    pub(crate) _phantom: PhantomData<&'graph (Mutability, Graph)>,
    pub(crate) graph: GraphAccess<'graph, Graph>,
    pub(crate) empty: Empty<Graph, Context>,
}

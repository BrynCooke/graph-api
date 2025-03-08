use crate::graph::Graph;
use crate::search::vertex::VertexSearch;
use crate::walker::builder::{ImmutableMarker, VertexWalkerBuilder};
use crate::walker::steps::{Detour, EdgeContext, EdgeFilter, EdgeLimit, Edges, End, Endpoints, VertexContext, VertexFilter, VertexIter, VertexLimit, Vertices, Waypoint};
use crate::{EdgeSearch, ElementId};

pub mod builder;
mod iter;
pub mod steps;

/// A trait that defines the basic behavior of a graph walker.
///
/// The `Walker` trait is the foundation for traversing and exploring graphs in the
/// graph-api-lib crate. It defines the core methods that all walkers must implement,
/// such as `next_element` to retrieve the next vertex or edge in the graph, and
/// `ctx` to access the walker's internal context.
///
/// Implementors of this trait are responsible for defining the specific graph
/// representation they work with (via the `Graph` associated type) and the
/// internal state they need to track during the walk (via the `Context`
/// associated type).
pub trait Walker<'graph>
where
    Self: Sized,
{
    /// The graph that the traversal is applied to.
    type Graph: Graph;

    /// The current context of the walker, this allows users to collect data as they traverse
    /// a graph.
    type Context: Clone + 'static;

    /// Return the next element in the traversal.
    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<Self::Graph>>;

    /// Returns the current context of the walker.
    fn ctx(&self) -> &Self::Context;
}

/// A trait that defines the basic behavior of a vertex walker, which is a specialized
/// type of graph walker that focuses on traversing and exploring the vertices in a graph.
///
/// The `VertexWalker` trait provides a set of methods for working with vertices, such as
/// filtering vertices based on a predicate, limiting the number of vertices returned,
/// and collecting the vertices into a specific data structure.
///
/// Implementors of this trait are responsible for defining the specific graph
/// representation they work with (via the `Graph` associated type) and the
/// internal state they need to track during the walk (via the `Context`
/// associated type).
pub trait VertexWalker<'graph>: Walker<'graph> {
    fn vertices_by_id<Iter>(self, vertex_ids: Iter) -> VertexIter<'graph, Self, Iter::IntoIter>
    where
        Iter: IntoIterator<Item = <Self::Graph as Graph>::VertexId>,
    {
        VertexIter::new(self, vertex_ids.into_iter())
    }

    fn vertices<'search>(
        self,
        vertex_search: VertexSearch<'search, Self::Graph>,
    ) -> Vertices<'search, 'graph, Self> {
        Vertices::new(self, vertex_search)
    }

    fn filter<Predicate>(self, predicate: Predicate) -> VertexFilter<'graph, Self, Predicate>
    where
        Predicate: Fn(&<Self::Graph as Graph>::VertexReference<'_>, &Self::Context) -> bool,
    {
        VertexFilter::new(self, predicate)
    }

    fn limit(self, limit: usize) -> VertexLimit<'graph, Self> {
        VertexLimit::new(self, limit)
    }

    fn detour<Path, Terminal, WalkerBuilder>(
        self,
        path: Path,
    ) -> Detour<'graph, Self, Path, Terminal>
    where
        Path: Fn(
            VertexWalkerBuilder<
                'graph,
                ImmutableMarker,
                Self::Graph,
                Waypoint<'graph, Self::Graph, Self::Context>,
            >,
        ) -> WalkerBuilder,
        WalkerBuilder: Into<builder::WalkerBuilder<'graph, ImmutableMarker, Self::Graph, Terminal>>,
        Terminal: Walker<'graph, Graph = Self::Graph>,
        <Self as Walker<'graph>>::Graph: 'graph,
    {
        Detour::new(self, path)
    }

    fn context<Callback, Context>(
        self,
        predicate: Callback,
    ) -> VertexContext<'graph, Self, Callback, Context>
    where
        Callback: Fn(&<Self::Graph as Graph>::VertexReference<'_>, &Self::Context) -> Context,
    {
        VertexContext::new(self, predicate)
    }

    fn edges(self, search: EdgeSearch<Self::Graph>) -> Edges<'_, 'graph, Self> {
        Edges::new(self, search)
    }

    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId>;
}

/// Trait for walking over edges in a graph.
///
/// This trait provides methods for working with edges in a graph, including
/// filtering, limiting, and collecting edges. It also provides methods for
/// accessing the head and tail of an edge.
///
/// Implementors of this trait are expected to be able to walk over the edges
/// of a graph, and to provide access to the edge references and the context
/// associated with the edge walker.
pub trait EdgeWalker<'graph>: Walker<'graph> {
    fn context<Callback, Context>(
        self,
        predicate: Callback,
    ) -> EdgeContext<'graph, Self, Callback, Context>
    where
        Callback: Fn(&<Self::Graph as Graph>::EdgeReference<'_>, &Self::Context) -> Context,
    {
        EdgeContext::new(self, predicate)
    }

    fn filter<Predicate>(self, predicate: Predicate) -> EdgeFilter<'graph, Self, Predicate>
    where
        Predicate: Fn(&<Self::Graph as Graph>::EdgeReference<'_>, &Self::Context) -> bool,
    {
        EdgeFilter::new(self, predicate)
    }

    fn head(self) -> Endpoints<'graph, Self> {
        Endpoints::new(self, End::Head)
    }

    fn tail(self) -> Endpoints<'graph, Self> {
        Endpoints::new(self, End::Tail)
    }

    fn limit(self, limit: usize) -> EdgeLimit<'graph, Self> {
        EdgeLimit::new(self, limit)
    }

    fn next(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<<Self::Graph as Graph>::EdgeId>;
}

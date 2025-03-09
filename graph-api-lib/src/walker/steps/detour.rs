use crate::walker::builder::{ImmutableMarker, VertexWalkerBuilder, WalkerBuilder};
use crate::walker::{VertexWalker, Walker};
use crate::graph::Graph;
use crate::ElementId;
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

// ================ DETOUR IMPLEMENTATION ================

/// A Waypoint represents a temporary position in a graph traversal.
///
/// It acts as a bridge between the main traversal and a detour (sub-traversal),
/// storing the vertex ID and context needed to continue the main traversal
/// after the detour completes.
pub struct Waypoint<'graph, Graph, Context>
where
    Graph: crate::graph::Graph,
    Context: Clone,
{
    _phantom: PhantomData<&'graph (Graph, Context)>,
    // Shared cell containing the next vertex ID to visit
    // Rc is needed here to share state between the Detour and Waypoint
    next: Rc<Cell<Option<Graph::VertexId>>>,
    // Shared cell containing the context from the parent traversal
    // Rc is needed to share context between the Detour and Waypoint
    context: Rc<Cell<Option<Context>>>,
    // The currently active context for this waypoint
    current_context: Option<Context>,
}

impl<'graph, Graph, Context> Walker<'graph> for Waypoint<'graph, Graph, Context>
where
    Graph: crate::graph::Graph,
    Context: 'static + Clone,
{
    type Graph = Graph;
    type Context = Context;

    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<
        ElementId<Self::Graph>,
    > {
        self.next(graph).map(ElementId::Vertex)
    }
    fn ctx(&self) -> &Self::Context {
        self.current_context
            .as_ref()
            .expect("context must be set before access")
    }
}

impl<'graph, Graph, Context> VertexWalker<'graph> for Waypoint<'graph, Graph, Context>
where
    Graph: crate::graph::Graph,
    Context: 'static + Clone,
{
    fn next(
        &mut self,
        _graph: &Self::Graph,
    ) -> Option<<Self::Graph as crate::graph::Graph>::VertexId> {
        // Extract the context and vertex ID from the shared cells
        self.current_context = self.context.take();
        self.next.take()
    }
}

/// Detour creates a sub-traversal for each element in the main traversal.
///
/// It allows exploring connected elements without losing the current position
/// in the main traversal. For each element in the parent traversal, a new
/// sub-traversal is created using the provided path function.
pub struct Detour<'graph, Parent, Path, Terminal>
where
    Parent: VertexWalker<'graph>,
    Terminal: Walker<'graph, Graph = Parent::Graph>,
{
    _phantom_data: PhantomData<&'graph Terminal>,
    // The parent traversal that provides vertices to detour from
    parent: Parent,
    // Function that builds the detour path for each vertex
    path: Path,
    // The current sub-traversal walker (created on demand)
    walker: Option<
        crate::walker::builder::WalkerBuilder<'graph, ImmutableMarker, Parent::Graph, Terminal>,
    >,
    // The current vertex ID from the parent traversal
    next: Option<<Parent::Graph as Graph>::VertexId>,
    // The context from the terminal (detour) traversal
    context: Option<Terminal::Context>,
    // Shared cell for the next vertex ID (shared with Waypoint)
    waypoint_next: Rc<Cell<Option<<Parent::Graph as Graph>::VertexId>>>,
    // Shared cell for the context (shared with Waypoint)
    waypoint_context: Rc<Cell<Option<Parent::Context>>>,
}

impl<'graph, Parent, Path, Terminal> Detour<'graph, Parent, Path, Terminal>
where
    Parent: VertexWalker<'graph>,
    Terminal: Walker<'graph, Graph = Parent::Graph>,
{
    pub(crate) fn new(parent: Parent, path: Path) -> Self {
        Detour {
            _phantom_data: Default::default(),
            parent,
            path,
            walker: None,
            next: None,
            context: None,
            waypoint_next: Default::default(),
            waypoint_context: Default::default(),
        }
    }
}

impl<'graph, Parent, Path, Terminal, WalkerBuilder> Walker<'graph>
    for Detour<'graph, Parent, Path, Terminal>
where
    Parent: VertexWalker<'graph>,
    Path: Fn(
        VertexWalkerBuilder<
            'graph,
            ImmutableMarker,
            Parent::Graph,
            Waypoint<'graph, Parent::Graph, Parent::Context>,
        >,
    ) -> WalkerBuilder,
    WalkerBuilder: Into<
        crate::walker::builder::WalkerBuilder<'graph, ImmutableMarker, Parent::Graph, Terminal>,
    >,
    Terminal: Walker<'graph, Graph = Parent::Graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    type Graph = Parent::Graph;
    type Context = Terminal::Context;

    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }

    fn ctx(&self) -> &Self::Context {
        self.context
            .as_ref()
            .expect("next must be called before trying to get context")
    }
}

impl<'graph, Parent, Path, Terminal, WalkerBuilder> VertexWalker<'graph>
    for Detour<'graph, Parent, Path, Terminal>
where
    Parent: VertexWalker<'graph>,
    Path: Fn(
        VertexWalkerBuilder<
            'graph,
            ImmutableMarker,
            Parent::Graph,
            Waypoint<'graph, Parent::Graph, Parent::Context>,
        >,
    ) -> WalkerBuilder,
    WalkerBuilder: Into<
        crate::walker::builder::WalkerBuilder<'graph, ImmutableMarker, Parent::Graph, Terminal>,
    >,
    Terminal: Walker<'graph, Graph = Parent::Graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        // Initialize the walker on first use
        if self.walker.is_none() {
            // Create a new Waypoint that shares state with this Detour
            // The waypoint allows the detour traversal to access the current vertex and context
            self.walker = Some(
                (self.path)(crate::walker::builder::new(
                    graph,
                    Waypoint {
                        _phantom: Default::default(),
                        next: self.waypoint_next.clone(),
                        context: self.waypoint_context.clone(),
                        current_context: None,
                    },
                ))
                .into(),
            );
        }
        let walker = self.walker.as_mut().expect("walker must be set").walker();

        loop {
            match walker.next_element(graph) {
                None => {
                    // The detour traversal is exhausted, get the next vertex from parent
                    match self.parent.next(graph) {
                        None => {
                            // No more vertices in parent traversal
                            return None;
                        }
                        Some(next) => {
                            // Found a new vertex from parent, set up for next detour
                            self.next = Some(next);
                            // Share the next vertex ID with the waypoint
                            self.waypoint_next.replace(Some(next));
                            // Share the context with the waypoint
                            self.waypoint_context
                                .replace(Some(self.parent.ctx().clone()));
                        }
                    }
                }
                Some(_ctx) => {
                    // The detour found an element, save its context
                    self.context = Some(walker.ctx().clone());
                    // Return the original vertex from parent traversal
                    // (detour only provides context, doesn't change the traversal elements)
                    return self.next;
                }
            }
        }
    }
}

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/detour.md")]
    pub fn detour<Path, Terminal, WalkerBuilderT>(
        self,
        predicate: Path,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, Detour<'graph, Walker, Path, Terminal>>
    where
        Path: Fn(
            VertexWalkerBuilder<
                'graph,
                ImmutableMarker,
                Graph,
                Waypoint<'graph, Graph, Walker::Context>,
            >,
        ) -> WalkerBuilderT,
        WalkerBuilderT: Into<self::WalkerBuilder<'graph, ImmutableMarker, Graph, Terminal>>,
        Terminal: crate::walker::Walker<'graph, Graph = Graph>,
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: Detour::new(self.walker, predicate),
            graph: self.graph,
        }
    }
}
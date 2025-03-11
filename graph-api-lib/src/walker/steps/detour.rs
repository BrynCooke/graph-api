use crate::ElementId;
use crate::graph::Graph;
use crate::walker::builder::{ImmutableMarker, VertexWalkerBuilder, WalkerBuilder};
use crate::walker::{VertexWalker, Walker};
use include_doc::function_body;
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

    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }
    fn ctx(&self) -> &Self::Context {
        self.current_context
            .as_ref()
            .expect("context must be set before access")
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.current_context
            .as_mut()
            .expect("context cannot be retrieved before call to next")
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

    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }

    fn ctx(&self) -> &Self::Context {
        self.context
            .as_ref()
            .expect("next must be called before trying to get context")
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.context
            .as_mut()
            .expect("context cannot be retrieved before call to next")
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
    /// # Detour Step
    ///
    /// The `detour` step allows you to create a sub-traversal for each element in the current traversal.
    /// It's like a temporary branch in the traversal that returns to the main traversal when complete.
    /// This is powerful for exploring connected elements without losing your current position.
    ///
    /// ## Visual Diagram
    ///
    /// Before detour step (traversal position on Person A):
    /// ```text
    ///   [Person A]* --- knows ---> [Person B] --- created ---> [Project 1]
    ///                                             
    ///                                             created
    ///                                             
    ///                                             ↓
    ///                                          
    ///                                          [Project 2]
    /// ```
    ///
    /// During detour execution (for each element, a sub-traversal is performed):
    /// ```text
    ///   Main traversal:
    ///   [Person A]* --- knows ---> [Person B]
    ///   
    ///   Sub-traversal from Person A:
    ///   [Person A] --- knows ---> [Person B]
    ///                                            
    ///    created
    ///                                            
    ///      ↓
    ///                                          
    ///   [Project 2]*
    /// ```
    ///
    /// After detour step (traversal position returns to original elements):
    /// ```text
    ///   [Person A]* --- knows ---> [Person B]
    ///                                                                                  
    ///    created
    ///                                            
    ///      ↓
    ///                                          
    ///   [Project 2]
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `traversal_fn`: A function that takes a reference to the current element and returns a new traversal.
    ///   The results of this traversal are collected in the context.
    ///
    /// ## Return Value
    ///
    /// A walker with the same elements as before, but with the results of the sub-traversals stored in the context.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/detour.rs", example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The detour doesn't change the main traversal elements - it only adds context data
    /// - Detours can be nested for complex traversals
    /// - The detour function can return any walker, allowing for flexible sub-traversals
    /// - Use `push_context` inside detours to store data from the sub-traversal
    /// - Detours are executed eagerly for each element in the traversal
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

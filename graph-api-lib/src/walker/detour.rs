use crate::graph::Graph;
use crate::walker::builder::{ImmutableMarker, VertexWalkerBuilder};
use crate::walker::{ VertexWalker, Walker};
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;
use crate::ElementId;

pub struct Waypoint<'graph, Graph, Context>
where
    Graph: crate::graph::Graph,
    Context: Clone,
{
    _phantom: PhantomData<&'graph (Graph, Context)>,
    next: Rc<Cell<Option<Graph::VertexId>>>,
    context: Rc<Cell<Option<Context>>>,
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
        ElementId<
            <Self::Graph as crate::graph::Graph>::VertexId,
            <Self::Graph as crate::graph::Graph>::EdgeId,
        >,
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
        self.current_context = self.context.take();
        self.next.take()
    }
}

pub struct Detour<'graph, Parent, Path, Terminal>
where
    Parent: VertexWalker<'graph>,
    Terminal: Walker<'graph, Graph = Parent::Graph>,
{
    _phantom_data: PhantomData<&'graph Terminal>,
    parent: Parent,
    path: Path,
    walker: Option<
        crate::walker::builder::WalkerBuilder<'graph, ImmutableMarker, Parent::Graph, Terminal>,
    >,
    next: Option<<Parent::Graph as Graph>::VertexId>,
    context: Option<Terminal::Context>,
    waypoint_next: Rc<Cell<Option<<Parent::Graph as Graph>::VertexId>>>,
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
    ) -> Option<ElementId<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
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
        if self.walker.is_none() {
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
                    // The detour needs a new waypoint
                    match self.parent.next(graph) {
                        None => {
                            // Nothing left
                            return None;
                        }
                        Some(next) => {
                            // Set the next waypoint
                            self.next = Some(next);
                            self.waypoint_next.replace(Some(next));
                            self.waypoint_context
                                .replace(Some(self.parent.ctx().clone()));
                        }
                    }
                }
                Some(_ctx) => {
                    self.context = Some(walker.ctx().clone());
                    return self.next;
                }
            }
        }
    }
}

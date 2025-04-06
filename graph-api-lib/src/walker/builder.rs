use crate::walker::steps::Empty;
use crate::walker::{EdgeWalker, VertexWalker};
use std::marker::PhantomData;

pub trait Mutable {}

pub struct MutableMarker;

impl Mutable for MutableMarker {}
pub struct ImmutableMarker;

#[derive(Default)]
pub enum GraphAccess<'graph, Graph> {
    Immutable(&'graph Graph),
    Mutable(&'graph mut Graph),
    #[default]
    Taken,
}

impl<'graph, Graph> GraphAccess<'graph, Graph> {
    pub fn take(&mut self) -> &'graph Graph {
        match std::mem::take(self) {
            GraphAccess::Immutable(graph) => graph,
            GraphAccess::Mutable(graph) => graph,
            GraphAccess::Taken => panic!("graph already taken"),
        }
    }

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
    pub(crate) fn walker(&mut self) -> &mut Walker {
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

pub struct VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    pub(crate) _phantom: PhantomData<&'graph Mutability>,
    pub(crate) walker: Walker,
    pub(crate) graph: GraphAccess<'graph, Graph>,
}

pub(crate) fn new<'graph, Graph, Start>(
    graph: &'graph Graph,
    start: Start,
) -> VertexWalkerBuilder<'graph, ImmutableMarker, Graph, Start>
where
    Graph: crate::graph::Graph,
    Start: VertexWalker<'graph, Graph = Graph>,
{
    VertexWalkerBuilder {
        _phantom: Default::default(),
        walker: start,
        graph: GraphAccess::Immutable(graph),
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
    // All step methods were moved to respective step files
}

pub struct EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Walker: EdgeWalker<'graph>,
{
    pub(crate) _phantom: PhantomData<&'graph Mutability>,
    pub(crate) walker: Walker,
    pub(crate) graph: GraphAccess<'graph, Graph>,
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
    <Walker as crate::walker::Walker<'graph>>::Context: Clone + 'static,
{
    // All step methods were moved to respective step files
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

pub struct StartWalkerBuilder<'graph, Mutability, Graph, Context> {
    pub(crate) _phantom: PhantomData<&'graph (Mutability, Graph)>,
    pub(crate) graph: GraphAccess<'graph, Graph>,
    pub(crate) empty: Empty<Graph, Context>,
}

use crate::graph::VertexReference;
use crate::search::vertex::VertexSearch;
use crate::walker::context::{
    ContextRef, DefaultEdgeContext, DefaultVertexContext, EdgeContext, VertexContext,
};
use crate::walker::detour::{Detour, Waypoint};
use crate::walker::edges::Edges;
use crate::walker::empty::Empty;
use crate::walker::endpoints::Endpoints;
use crate::walker::filter::{EdgeFilter, VertexFilter};
use crate::walker::limit::{EdgeLimit, VertexLimit};
use crate::walker::probe::{EdgeProbe, VertexProbe};
use crate::walker::vertex_iter::VertexIter;
use crate::walker::vertices::Vertices;
use crate::walker::{EdgeWalker, VertexWalker};
use crate::{EdgeReference, EdgeSearch};
use std::marker::PhantomData;
use crate::walker::iter::{EdgeIterImpl, EdgeReferenceIterImpl, VertexIterImpl, VertexReferenceIterImpl};

pub trait Mutable {}

pub struct MutableMarker;

impl Mutable for MutableMarker {}
pub struct ImmutableMarker;

#[derive(Default)]
enum GraphAccess<'graph, Graph> {
    Immutable(&'graph Graph),
    Mutable(&'graph mut Graph),
    #[default]
    Taken,
}

impl<'graph, Graph> GraphAccess<'graph, Graph> {
    fn take(&mut self) -> &'graph Graph {
        match std::mem::take(self) {
            GraphAccess::Immutable(graph) => graph,
            GraphAccess::Mutable(graph) => graph,
            GraphAccess::Taken => panic!("graph already taken"),
        }
    }

    fn take_mut(&mut self) -> &'graph mut Graph {
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
    _phantom: PhantomData<&'graph (Mutability, Graph, Walker)>,
    walker: Walker,
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
    _phantom: PhantomData<&'graph Mutability>,
    pub(crate) walker: Walker,
    graph: GraphAccess<'graph, Graph>,
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
    #[doc = include_str!("../../../docs/users/steps/vertices_by_id.md")]
    pub fn vertices_by_id<Iter>(
        self,
        vertex_ids: Iter,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexIter<'graph, Walker, Iter::IntoIter>>
    where
        Iter: IntoIterator<Item = Graph::VertexId>,
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.vertices_by_id(vertex_ids),
            graph: self.graph,
        }
    }

    #[doc = include_str!("../../../docs/users/steps/vertices.md")]
    pub fn vertices<'a, T: Into<VertexSearch<'a, Graph>>>(
        self,
        vertex_search: T,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, Vertices<'a, 'graph, Walker>> {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.vertices(vertex_search.into()),
            graph: self.graph,
        }
    }

    #[doc = include_str!("../../../docs/users/steps/context.md")]
    pub fn push_context<Callback, Context>(
        self,
        callback: Callback,
    ) -> VertexWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        VertexContext<
            'graph,
            Walker,
            impl Fn(
                &Graph::VertexReference<'_>,
                &Walker::Context,
            ) -> ContextRef<Context, Walker::Context>,
            ContextRef<Context, Walker::Context>,
        >,
    >
    where
        Callback: Fn(&Graph::VertexReference<'_>, &Walker::Context) -> Context + 'graph,
        Context: Clone + 'static,
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.context(move |vertex, context| {
                ContextRef::new(callback(vertex, context), context.clone())
            }),
            graph: self.graph,
        }
    }

    #[doc = include_str!("../../../docs/users/steps/default_context.md")]
    pub fn push_default_context(
        self,
    ) -> VertexWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        VertexContext<
            'graph,
            Walker,
            impl Fn(
                &Graph::VertexReference<'_>,
                &Walker::Context,
            )
                -> ContextRef<DefaultVertexContext<Graph::VertexId, Graph::Vertex>, Walker::Context>,
            ContextRef<DefaultVertexContext<Graph::VertexId, Graph::Vertex>, Walker::Context>,
        >,
    >
    where
        Graph::Vertex: Clone + 'static,
    {
        self.push_context(|vertex, _context| DefaultVertexContext {
            vertex_id: vertex.id(),
            vertex: vertex.weight().clone(),
        })
    }

    #[doc = include_str!("../../../docs/users/steps/filter.md")]
    pub fn filter<Predicate>(
        self,
        predicate: Predicate,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexFilter<'graph, Walker, Predicate>>
    where
        Predicate: Fn(&Graph::VertexReference<'_>, &Walker::Context) -> bool,
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),

            walker: self.walker.filter(predicate),
            graph: self.graph,
        }
    }

    #[doc = include_str!("../../../docs/users/steps/detour.md")]
    pub fn detour<Path, Terminal, WalkerBuilder>(
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
        ) -> WalkerBuilder,
        WalkerBuilder: Into<self::WalkerBuilder<'graph, ImmutableMarker, Graph, Terminal>>,
        Terminal: crate::walker::Walker<'graph, Graph = Graph>,
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.detour(predicate),
            graph: self.graph,
        }
    }

    #[doc = include_str!("../../../docs/users/steps/edges.md")]
    pub fn edges<'a, T: Into<EdgeSearch<'a, Graph>>>(
        self,
        search: T,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, Edges<'a, 'graph, Walker>> {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.edges(search.into()),
            graph: self.graph,
        }
    }

    #[doc = include_str!("../../../docs/users/steps/collect.md")]
    pub fn collect<T: FromIterator<Graph::VertexId>>(self) -> T
    where
        Walker: VertexWalker<'graph>,
    {
        self.into_iter().collect()
    }

    #[doc = include_str!("../../../docs/users/steps/mutate.md")]
    pub fn mutate<Callback>(mut self, callback: Callback) -> usize
    where
        Callback: Fn(&mut Walker::Graph, Graph::VertexId, &Walker::Context),
        Mutability: Mutable,
        'graph: 'graph,
    {
        let graph = self.graph.take_mut();
        let graph_copy: &Graph = unsafe { std::mem::transmute(&*graph) };
        let mut walker = self.walker;

        let mut contexts = Vec::new();
        while let Some(vertex_id) = walker.next(graph_copy) {
            let ctx = walker.ctx().clone();
            contexts.push((vertex_id, ctx));
        }

        let mut count = 0;
        for (vertex_id, ctx) in contexts {
            callback(graph, vertex_id, &ctx);
            count += 1;
        }
        count
    }

    //#[doc = include_str!("../../../docs/users/steps/count.md")]
    pub fn count(mut self) -> usize
    where
        'graph: 'graph,
    {
        let mut count = 0;
        let graph = self.graph.take();
        let mut walker = self.walker;
        while let Some(_vertex_id) = walker.next(graph) {
            count += 1;
        }
        count
    }

    #[doc = include_str!("../../../docs/users/steps/limit.md")]
    pub fn limit(
        self,
        limit: usize,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexLimit<'graph, Walker>> {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.limit(limit),
            graph: self.graph,
        }
    }

    #[doc = include_str!("../../../docs/users/steps/first.md")]
    pub fn first(mut self) -> Option<Graph::VertexId>
    where
        'graph: 'graph,
    {
        let graph = self.graph.take();
        let mut walker = self.walker;
        walker.next(graph)
    }

    #[doc = include_str!("../../../docs/users/steps/dbg.md")]
    pub fn dbg(
        self,
        tag: &'static str
    ) -> VertexWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        VertexProbe<'graph, Walker, impl FnMut(&Graph::VertexReference<'_>)>,
    > {
        let callback = move |vertex: &Graph::VertexReference<'_>| {
            println!("{}: {:?}", tag, vertex);
        };
        self.probe(callback)
    }

    #[doc = include_str!("../../../docs/users/steps/probe.md")]
    pub fn probe<Callback>(
        self,
        callback: Callback,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexProbe<'graph, Walker, Callback>>
    where
        Callback: FnMut(&Graph::VertexReference<'_>),
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: VertexProbe::new(self.walker, callback),
            graph: self.graph,
        }
    }

    #[doc = include_str!("../../../docs/users/steps/map.md")]
    pub fn map<R, M: FnMut(Graph::VertexReference<'graph>, Walker::Context) -> R>(mut self, mut mapping: M) -> impl Iterator<Item=R> + 'graph
    where
        M: 'graph,
        Walker: 'graph,
    {
        VertexReferenceIterImpl::new(self.graph.take(), self.walker).map(move |(reference, ctx)| mapping(reference, ctx))
    }
    
    #[doc = include_str!("../../../docs/users/steps/fold.md")]
    pub fn fold<Acc, F>(mut self, init: Acc, mut f: F) -> Acc 
    where
        F: FnMut(Acc, Graph::VertexReference<'graph>, &Walker::Context) -> Acc,
        'graph: 'graph,
    {
        let graph = self.graph.take();
        let mut walker = self.walker;
        let mut acc = init;
        
        while let Some(vertex_id) = walker.next(graph) {
            let vertex = graph.vertex(vertex_id).expect("vertex ID must resolve to vertex");
            acc = f(acc, vertex, walker.ctx());
        }
        
        acc
    }
}

#[doc = include_str!("../../../docs/users/steps/iter.md")]
impl<'graph, Mutability, Graph, Walker> IntoIterator
    for VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    type Item = Graph::VertexId;
    type IntoIter = VertexIterImpl<'graph, Graph, Walker>;

    fn into_iter(mut self) -> Self::IntoIter {
        VertexIterImpl::new(self.graph.take(), self.walker)
    }
}

impl<'graph, Mutability, Graph, Walker> IntoIterator
    for EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    type Item = Graph::EdgeId;
    type IntoIter = EdgeIterImpl<'graph, Graph, Walker>;

    fn into_iter(mut self) -> Self::IntoIter {
        EdgeIterImpl::new(self.graph.take(), self.walker)
    }
}

pub struct EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Walker: EdgeWalker<'graph>,
{
    _phantom: PhantomData<&'graph Mutability>,
    pub(crate) walker: Walker,
    graph: GraphAccess<'graph, Graph>,
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
    <Walker as crate::walker::Walker<'graph>>::Context: Clone + 'static,
{
    pub fn push_context<Callback, Context>(
        self,
        callback: Callback,
    ) -> EdgeWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        EdgeContext<
            'graph,
            Walker,
            impl Fn(&Graph::EdgeReference<'_>, &Walker::Context) -> ContextRef<Context, Walker::Context>,
            ContextRef<Context, Walker::Context>,
        >,
    >
    where
        Callback: Fn(&Graph::EdgeReference<'_>, &Walker::Context) -> Context,
        Context: Clone + 'static,
    {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.context(move |edge, context| {
                ContextRef::new(callback(edge, context), context.clone())
            }),
            graph: self.graph,
        }
    }

    //#[doc = include_str!("../../../docs/users/steps/default_context.md")]
    pub fn push_default_context(
        self,
    ) -> EdgeWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        EdgeContext<
            'graph,
            Walker,
            impl Fn(
                &Graph::EdgeReference<'_>,
                &Walker::Context,
            )
                -> ContextRef<DefaultEdgeContext<Graph::EdgeId, Graph::Edge>, Walker::Context>,
            ContextRef<DefaultEdgeContext<Graph::EdgeId, Graph::Edge>, Walker::Context>,
        >,
    >
    where
        Graph::Edge: Clone + 'static,
    {
        self.push_context(|edge, _context| DefaultEdgeContext {
            edge_id: edge.id(),
            edge: edge.weight().clone(),
        })
    }

    //#[doc = include_str!("../../../docs/users/steps/filter.md")]
    pub fn filter<Predicate>(
        self,
        predicate: Predicate,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeFilter<'graph, Walker, Predicate>>
    where
        Predicate: Fn(&Graph::EdgeReference<'_>, &Walker::Context) -> bool,
    {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.filter(predicate),
            graph: self.graph,
        }
    }

    #[doc = include_str!("../../../docs/users/steps/head.md")]
    pub fn head(self) -> VertexWalkerBuilder<'graph, Mutability, Graph, Endpoints<'graph, Walker>> {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.head(),
            graph: self.graph,
        }
    }

    #[doc = include_str!("../../../docs/users/steps/tail.md")]
    pub fn tail(self) -> VertexWalkerBuilder<'graph, Mutability, Graph, Endpoints<'graph, Walker>> {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.tail(),
            graph: self.graph,
        }
    }

    //#[doc = include_str!("../../../docs/users/steps/collect.md")]
    pub fn collect<T: FromIterator<Graph::EdgeId>>(self) -> T
    where
        Walker: EdgeWalker<'graph>,
    {
        self.into_iter().collect()
    }

    #[doc = include_str!("../../../docs/users/steps/mutate.md")]
    pub fn mutate<Callback>(mut self, callback: Callback) -> usize
    where
        Callback: Fn(&mut Walker::Graph, Graph::EdgeId, &Walker::Context),
        Mutability: Mutable,
        'graph: 'graph,
    {
        let graph = self.graph.take_mut();
        let graph_copy: &Graph = unsafe { std::mem::transmute(&*graph) };
        let mut walker = self.walker;

        let mut contexts = Vec::new();
        while let Some(edge) = walker.next(graph_copy) {
            let ctx = walker.ctx().clone();
            contexts.push((edge, ctx));
        }

        let mut count = 0;
        for (edge_id, ctx) in contexts {
            callback(graph, edge_id, &ctx);
            count += 1;
        }
        count
    }

    //#[doc = include_str!("../../../docs/users/steps/count.md")]
    pub fn count(mut self) -> usize
    where
        'graph: 'graph,
    {
        let mut count = 0;
        let graph = self.graph.take();
        let mut walker = self.walker;
        while let Some(_vertex_id) = walker.next(graph) {
            count += 1;
        }
        count
    }

    //#[doc = include_str!("../../../docs/users/steps/limit.md")]
    pub fn limit(
        self,
        limit: usize,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeLimit<'graph, Walker>> {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.limit(limit),
            graph: self.graph,
        }
    }

    #[doc = include_str!("../../../docs/users/steps/first.md")]
    pub fn first(mut self) -> Option<Graph::EdgeId> {
        let graph = self.graph.take();
        let mut walker = self.walker;
        walker.next(graph)
    }

    #[doc = include_str!("../../../docs/users/steps/dbg.md")]
    pub fn dbg(
        self,
        tag: &'static str
    ) -> EdgeWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        EdgeProbe<'graph, Walker, impl FnMut(&Graph::EdgeReference<'_>)>,
    > {
        let callback = move |edge: &Graph::EdgeReference<'_>| {
            println!("{}: {:?}", tag, edge);
        };
        self.probe(callback)
    }

    #[doc = include_str!("../../../docs/users/steps/probe.md")]
    pub fn probe<Callback>(
        self,
        callback: Callback,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeProbe<'graph, Walker, Callback>>
    where
        Callback: FnMut(&Graph::EdgeReference<'_>),
    {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: EdgeProbe::new(self.walker, callback),
            graph: self.graph,
        }
    }

    #[doc = include_str!("../../../docs/users/steps/map.md")]
    pub fn map<R, M: FnMut(Graph::EdgeReference<'graph>, Walker::Context) -> R>(mut self, mut mapping: M) -> impl Iterator<Item=R> + 'graph
    where
        M: 'graph,
        Walker: 'graph,
    {
        EdgeReferenceIterImpl::new(self.graph.take(), self.walker).map(move |(reference, ctx)| mapping(reference, ctx))
    }
    
    #[doc = include_str!("../../../docs/users/steps/fold.md")]
    pub fn fold<Acc, F>(mut self, init: Acc, mut f: F) -> Acc 
    where
        F: FnMut(Acc, Graph::EdgeReference<'graph>, &Walker::Context) -> Acc,
        'graph: 'graph,
    {
        let graph = self.graph.take();
        let mut walker = self.walker;
        let mut acc = init;
        
        while let Some(edge_id) = walker.next(graph) {
            let edge = graph.edge(edge_id).expect("edge ID must resolve to edge");
            acc = f(acc, edge, walker.ctx());
        }
        
        acc
    }
}


pub(crate) fn new_start<Graph>(graph: &Graph) -> StartWalkerBuilder<'_, ImmutableMarker, Graph>
where
    Graph: crate::graph::Graph,
{
    StartWalkerBuilder {
        _phantom: Default::default(),
        graph: GraphAccess::Immutable(graph),
    }
}

pub(crate) fn new_start_mut<Graph>(
    graph: &mut Graph,
) -> StartWalkerBuilder<'_, MutableMarker, Graph>
where
    Graph: crate::graph::Graph,
{
    StartWalkerBuilder {
        _phantom: Default::default(),
        graph: GraphAccess::Mutable(graph),
    }
}

pub struct StartWalkerBuilder<'graph, Mutability, Graph> {
    _phantom: PhantomData<&'graph (Mutability, Graph)>,
    graph: GraphAccess<'graph, Graph>,
}

impl<'graph, Graph, Mutability> StartWalkerBuilder<'graph, Mutability, Graph>
where
    Graph: crate::graph::Graph,
{
    pub fn vertices<'search>(
        self,
        vertex_search: VertexSearch<'search, Graph>,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, Vertices<'search, 'graph, Empty<Graph>>>
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: Empty::default().vertices(vertex_search),
            graph: self.graph,
        }
    }

    pub fn vertices_by_id<Iter>(
        self,
        vertex_ids: Iter,
    ) -> VertexWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        VertexIter<'graph, Empty<Graph>, Iter::IntoIter>,
    >
    where
        Iter: IntoIterator<Item = Graph::VertexId>,
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: Empty::default().vertices_by_id(vertex_ids),
            graph: self.graph,
        }
    }
}

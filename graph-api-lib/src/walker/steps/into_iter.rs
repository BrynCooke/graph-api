use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};

#[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/into_iter.md")]
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


pub struct VertexReferenceIterImpl<'graph, Graph, Walker> {
    graph: &'graph Graph,
    walker: Walker,
}

impl<'graph, Graph, Walker> VertexReferenceIterImpl<'graph, Graph, Walker> {
    pub(crate) fn new(graph: &'graph Graph, walker: Walker) -> Self
    where
        Graph: crate::graph::Graph,
        Walker: VertexWalker<'graph, Graph=Graph>,
    {
        Self {
            graph,
            walker
        }
    }
}

impl<'graph, Graph, Walker> Iterator for VertexReferenceIterImpl<'graph, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    type Item = (Graph::VertexReference<'graph>, Walker::Context);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.walker.next(self.graph) {
            Some((
                self.graph
                    .vertex(next)
                    .expect("vertex ID must resolve to vertex"),
                self.walker.ctx().clone(),
            ))
        } else {
            None
        }
    }
}

pub struct EdgeReferenceIterImpl<'graph, Graph, Walker> {
    graph: &'graph Graph,
    walker: Walker,
}

impl<'graph, Graph, Walker> EdgeReferenceIterImpl<'graph, Graph, Walker> {
    pub(crate) fn new(graph: &'graph Graph, walker: Walker) -> Self
    where
        Graph: crate::graph::Graph,
        Walker: EdgeWalker<'graph, Graph=Graph>,
    {
        Self {
            graph,
            walker,
        }
    }
}

impl<'graph, Graph, Walker> Iterator for EdgeReferenceIterImpl<'graph, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    type Item = (Graph::EdgeReference<'graph>, Walker::Context);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.walker.next(self.graph) {
            let edge = self.graph.edge(next).expect("edge ID must resolve to an edge");
            Some((edge, self.walker.ctx().clone()))
        } else {
            None
        }
    }
}


pub struct VertexIterImpl<'graph, Graph, Walker> {
    graph: &'graph Graph,
    walker: Walker,
}

impl<'graph, Graph, Walker> VertexIterImpl<'graph, Graph, Walker> {
    pub(crate) fn new(graph: &'graph Graph, walker: Walker) -> Self
    where
        Graph: crate::graph::Graph,
        Walker: VertexWalker<'graph, Graph=Graph>,
    {
        Self {
            graph,
            walker,
        }
    }
}

impl<'graph, Graph, Walker> Iterator for VertexIterImpl<'graph, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph= Graph>,
{
    type Item = Graph::VertexId;

    fn next(&mut self) -> Option<Self::Item> {
        self.walker.next(self.graph)
    }
}

pub struct EdgeIterImpl<'graph, Graph, Walker> {
    graph: &'graph Graph,
    walker: Walker,
}

impl<'graph, Graph, Walker> EdgeIterImpl<'graph, Graph, Walker> {
    pub(crate) fn new(graph: &'graph Graph, walker: Walker) -> Self
    where
        Graph: crate::graph::Graph,
        Walker: EdgeWalker<'graph, Graph=Graph>,
    {
        Self {
            graph,
            walker
        }
    }
}

impl<'graph, Graph, Walker> Iterator for EdgeIterImpl<'graph, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph= Graph>,
{
    type Item = Graph::EdgeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.walker.next(self.graph)
    }
}


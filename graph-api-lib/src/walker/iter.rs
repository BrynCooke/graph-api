use crate::{EdgeWalker, VertexWalker};

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
        if let Some(next) = self.walker.next(self.graph) {
            Some(next)
        } else {
            None
        }
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
        if let Some(next) = self.walker.next(self.graph) {
            Some(next)
        } else {
            None
        }
    }
}


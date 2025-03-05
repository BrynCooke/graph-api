use crate::ElementId;
use crate::walker::{VertexWalker, Walker};

pub struct Empty<Graph> {
    _phantom: std::marker::PhantomData<Graph>,
}

impl<Graph> Default for Empty<Graph>
where
    Graph: crate::graph::Graph,
{
    fn default() -> Self {
        Empty {
            _phantom: Default::default(),
        }
    }
}

impl<'graph, Graph> Walker<'graph> for Empty<Graph>
where
    Graph: crate::graph::Graph,
{
    type Graph = Graph;
    type Context = ();
    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<
        ElementId<Self::Graph>,
    > {
        self.next(graph).map(ElementId::Vertex)
    }
    fn ctx(&self) -> &Self::Context {
        &()
    }
}

impl<Graph> VertexWalker<'_> for Empty<Graph>
where
    Graph: crate::graph::Graph,
{
    fn next(
        &mut self,
        _graph: &Self::Graph,
    ) -> Option<<Self::Graph as crate::graph::Graph>::VertexId> {
        None
    }
}

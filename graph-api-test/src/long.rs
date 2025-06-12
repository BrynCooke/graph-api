use crate::{Edge, Vertex, assert_elements_eq, populate_graph};
use graph_api_lib::{EdgeSearch, Graph, VertexSearch};

pub fn long_traversal<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(EdgeSearch::scan())
        .head()
        .edges(EdgeSearch::scan())
        .head()
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.graph_api, refs.julia]);
}

use crate::{assert_elements_eq, populate_graph, Edge, Vertex, VertexLabel};
use graph_api_lib::{Graph, VertexSearch};

pub fn test_head<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .out_edges(None)
        .head()
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.graph_api, refs.julia]);
}

pub fn test_tail<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .out_edges(None)
        .tail()
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}

pub fn test_limit<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices(VertexSearch::new().limit(1))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}

pub fn test_filtered<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices(VertexSearch::new().labelled(VertexLabel::Person))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn, refs.julia]);
}

pub fn test_filtered_limit<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices(VertexSearch::new().labelled(VertexLabel::Person).limit(1))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}

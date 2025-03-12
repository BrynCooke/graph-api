use crate::{Edge, Vertex, assert_elements_eq, populate_graph};
use graph_api_lib::{EdgeReference, EdgeSearch, Graph, VertexReference};

pub fn test_vertices_filter<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn, refs.julia])
        .filter(|person, _| match person.weight() {
            Vertex::Person { name, .. } => name == "Bryn",
            _ => false,
        })
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}

pub fn test_edges_filter<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .filter(|edge, _| match edge.weight() {
            Edge::Knows { since } => *since >= 1999,
            _ => false,
        })
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn_knows_julia]);
}

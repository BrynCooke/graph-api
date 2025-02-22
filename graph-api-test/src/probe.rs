use crate::{populate_graph, Edge, Vertex};
use graph_api_lib::{EdgeSearch, Graph, VertexSearch};

pub fn test_vertices_probe<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    populate_graph(graph);

    // Test probing vertices
    let mut count = 0;
    let result = graph
        .walk()
        .vertices(VertexSearch::scan())
        .probe(|_| {
            count += 1;
        })
        .collect::<Vec<_>>();

    assert_eq!(count, 4); // Should match number of vertices
    assert_eq!(result.len(), 4);
}

pub fn test_edges_probe<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Test probing edges
    let mut edge_count = 0;
    let result = graph
        .walk()
        .vertices_by_id([refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .probe(|_| {
            edge_count += 1;
        })
        .collect::<Vec<_>>();

    assert_eq!(edge_count, 2); // Bryn has 2 outgoing edges
    assert_eq!(result.len(), 2);
}

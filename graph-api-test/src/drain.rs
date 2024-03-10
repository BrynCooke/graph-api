use crate::{populate_graph, Edge, Vertex};
use graph_api_lib::{EdgeReference, Graph, VertexReference};

pub fn test_vertices_drain<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Create a probe to verify the walker is actually traversed
    let mut visited = Vec::new();

    // Start from Bryn, traverse outgoing edges, and drain
    graph
        .walk()
        .vertices_by_id([refs.bryn])
        .probe(|v| visited.push(v.id()))
        .count();

    // Verify that Bryn was visited during drain
    assert_eq!(visited.len(), 1);
    assert_eq!(visited[0], refs.bryn);
}

pub fn test_edges_drain<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    let mut visited = Vec::new();

    // Start from Bryn, get outgoing edges, and drain
    graph
        .walk()
        .vertices_by_id([refs.bryn])
        .out_edges(None)
        .probe(|e| visited.push(e.id()))
        .drain();

    // Verify that Bryn's outgoing edges were visited
    assert_eq!(visited.len(), 2);
    assert!(visited.contains(&refs.bryn_knows_julia));
    assert!(visited.contains(&refs.bryn_created_graph_api));
}

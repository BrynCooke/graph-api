use crate::{assert_elements_eq, populate_graph, Edge, EdgeExt, Vertex};
use graph_api_lib::{EdgeReference, EdgeSearch, Graph};

pub fn test_mutation<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    assert_eq!(
        graph
            .walk_mut()
            .vertices_by_id(vec![refs.julia])
            .mutate(|graph, vertex_id, _context| {
                graph.add_edge(vertex_id, refs.graph_api, Edge::Created);
            }),
        1
    );

    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.julia])
        .edges(EdgeSearch::scan().outgoing())
        .head()
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.graph_api, refs.bryn]);
}

pub fn test_edge_mutation<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    
    // Use a simple approach - start with one vertex, find all outgoing edges
    // We know bryn has two edges: knows julia and created graph_api
    let initial_edge_count = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .count();
    
    // Should be 2 edges (bryn knows julia, bryn created graph_api)
    assert_eq!(initial_edge_count, 2, "Expected 2 initial edges from bryn");
    
    // Now use the mutate fn to add a new edge for each outgoing edge from bryn
    let mutations = graph
        .walk_mut()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .mutate(|graph, edge_id, _context| {
            let tail = graph.edge(edge_id).unwrap().tail();

            // Add a Language edge from this edge's source to the rust vertex
            graph.add_edge(tail, refs.rust, Edge::Language(crate::Language {
                name: "EdgeMutationTest".to_string()
            }));
        });
    
    // Should have mutated 2 edges
    assert_eq!(mutations, 2, "Expected to process 2 edges");
    
    // Count bryn's edges again
    let final_edge_count = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .count();
    
    // Should have 4 edges now - the original 2 plus the 2 new ones
    assert_eq!(final_edge_count, 4, "Expected 4 final edges from bryn");
    
    // Verify new edges connect bryn to rust
    let bryn_language_edges = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .filter_by_language(|_| true)
        .count();
    
    assert_eq!(bryn_language_edges, 2, "Expected to find 2 language edges");
}


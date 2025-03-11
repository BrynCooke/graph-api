use crate::{populate_graph, Edge, Vertex};
use graph_api_lib::{EdgeReference, EdgeSearch, Graph, VertexReference, VertexSearch};

/// Test vertex fold operations
pub fn test_vertices_fold<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Populate the graph with test data
    let _refs = populate_graph(graph);

    // Calculate the sum of ages using fold
    let total_age = graph
        .walk()
        .vertices(VertexSearch::scan())
        .fold(0, |total, vertex, _| {
            if let Vertex::Person { age, .. } = *vertex.weight() {
                total + age
            } else {
                total
            }
        });

    // Check that the total age is positive - should include Bryn and Julia
    assert_eq!(total_age, 93);
}

/// Test edge fold operations
pub fn test_edges_fold<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Populate the graph with test data
    let _refs = populate_graph(graph);

    // Find the relationship year in the graph
    let knows_year = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(EdgeSearch::scan())
        .fold(None, |result, edge, _| {
            if let Edge::Knows { since } = edge.weight() {
                Some(*since)
            } else {
                result
            }
        });

    // There should be at least one relationship with a year
    assert!(knows_year.is_some());
    // The standard test data uses 1999
    assert_eq!(knows_year.unwrap(), 1999);
}

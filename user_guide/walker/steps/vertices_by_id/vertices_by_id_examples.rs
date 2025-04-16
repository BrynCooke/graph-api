use crate::standard_model::{Edge, Vertex};
use graph_api_lib::{Graph, VertexSearch};
use graph_api_simplegraph::SimpleGraph;

// ANCHOR: all
// Function demonstrating the vertices_by_id step
pub fn vertices_by_id_example() {
    // Create a new graph
    let graph = SimpleGraph::new();

    let bryn_id = graph
        .walk()
        .vertices(VertexSearch::scan())
        .first()
        .expect("id");
    let julia_id = graph
        .walk()
        .vertices(VertexSearch::scan())
        .first()
        .expect("id");

    // ANCHOR: basic_usage
    // Start a traversal with specific vertex IDs
    let specific_vertices = graph
        .walk()
        .vertices_by_id(vec![bryn_id, julia_id])
        .collect::<Vec<_>>();

    println!("Found {} specific vertices", specific_vertices.len());
    // ANCHOR_END: basic_usage

    // ANCHOR: followed_by_steps
    // Start with specific vertices and follow relationships
    let knows_relationships = graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .edges(Edge::follows().outgoing())
        .tail()
        .collect::<Vec<_>>();

    println!("Person knows {} other people", knows_relationships.len());
    // ANCHOR_END: followed_by_steps

    // ANCHOR: dynamic_ids
    // Using dynamically collected IDs
    // First, find all project vertices
    let project_vertices = graph.walk().vertices(Vertex::project()).collect::<Vec<_>>();

    // Use those IDs to start a new traversal
    let projects = graph
        .walk()
        .vertices_by_id(project_vertices)
        .collect::<Vec<_>>();

    println!("Found {} projects using vertices_by_id", projects.len());
    // ANCHOR_END: dynamic_ids
}
// ANCHOR_END: all

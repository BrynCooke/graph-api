use graph_api_lib::{EdgeSearch, Graph};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{EdgeIndex, populate_graph};

/* ANCHOR: all */
// Function demonstrating the edges step
pub fn edges_step_example() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);
    let start_id = refs.bryn;

    // ANCHOR: all_edges
    // Get all edges (both incoming and outgoing) from a vertex
    let all_connected_edges = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .edges(EdgeSearch::scan())
        .collect::<Vec<_>>();

    println!(
        "Found {} total edges connected to vertex",
        all_connected_edges.len()
    );
    // ANCHOR_END: all_edges

    // ANCHOR: directional
    // Get only outgoing edges from a vertex
    let outgoing_edges = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .edges(EdgeSearch::scan().outgoing())
        .collect::<Vec<_>>();

    println!("Found {} outgoing edges", outgoing_edges.len());

    // Get only incoming edges to a vertex
    let incoming_edges = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .edges(EdgeSearch::scan().incoming())
        .collect::<Vec<_>>();

    println!("Found {} incoming edges", incoming_edges.len());
    // ANCHOR_END: directional

    // ANCHOR: label_filter
    // Get only edges with a specific label
    // Using the label index is more efficient
    let created_edges = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .edges(EdgeIndex::created())
        .collect::<Vec<_>>();

    println!("Found {} edges with label 'Created'", created_edges.len());
    // ANCHOR_END: label_filter

    // ANCHOR: combined_filter
    // Combine direction and label filtering
    let outgoing_knows_edges = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .edges(EdgeIndex::knows().outgoing())
        .collect::<Vec<_>>();

    println!(
        "Found {} outgoing 'Knows' edges",
        outgoing_knows_edges.len()
    );
    // ANCHOR_END: combined_filter
}
/* ANCHOR_END: all */

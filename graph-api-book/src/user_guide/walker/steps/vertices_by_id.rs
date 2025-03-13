use graph_api_lib::{EdgeSearch, Graph};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Vertex, populate_graph};

pub fn vertices_by_id_example() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);
    
    // Get a list of known IDs to start with
    let known_ids = vec![refs.bryn, refs.alice];

    // Start traversal from specific vertex IDs
    // This is very efficient when you already know the IDs you need
    let specific_vertices = graph
        .walk()
        .vertices_by_id(known_ids.clone())
        .collect::<Vec<_>>();

    println!("Retrieved {} vertices by ID", specific_vertices.len());

    // You can also use this to check if vertices exist
    // Non-existent IDs are simply skipped
    let mut invalid_ids = known_ids.clone();

    // Create an invalid ID by using a very large number 
    // that's unlikely to be a valid vertex ID
    invalid_ids.push(<SimpleGraph as Graph>::VertexId::new(999999));

    let found_vertices = graph.walk().vertices_by_id(invalid_ids.clone()).collect::<Vec<_>>();

    println!(
        "Found {} valid vertices out of {} IDs",
        found_vertices.len(),
        invalid_ids.len()
    );

    // Useful for following relationships between specific vertices
    let friends_of_friends = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .tail() // Follow to friend vertices
        .edges(EdgeSearch::scan().outgoing())
        .tail() // Follow to friends of friends
        .collect::<Vec<_>>();

    println!("Found {} friends of friends", friends_of_friends.len());
}

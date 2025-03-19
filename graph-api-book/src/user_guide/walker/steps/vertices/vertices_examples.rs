use graph_api_lib::{Graph, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{VertexExt, VertexIndex, populate_graph};

/* ANCHOR: all */
// Function demonstrating various ways to use the vertices step
pub fn vertices_step_example() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    // ANCHOR: scan_all
    // Scan all vertices in the graph
    // This performs a full graph scan, which can be expensive for large graphs
    let all_vertices = graph
        .walk()
        .vertices(VertexSearch::scan())
        .collect::<Vec<_>>();

    println!("Found {} total vertices in the graph", all_vertices.len());
    // ANCHOR_END: scan_all

    // ANCHOR: label_index
    // Use a label-based index for more efficient lookups
    // This narrows the search to only person vertices
    let people = graph
        .walk()
        .vertices(VertexIndex::person())
        .collect::<Vec<_>>();

    println!("Found {} person vertices", people.len());
    // ANCHOR_END: label_index

    // ANCHOR: property_filter
    // Use property-based filtering
    // This finds vertices with a specific property value
    let people_named_bob = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_by_person(|person, _| person.name() == "Bob")
        .collect::<Vec<_>>();

    println!("Found {} people named Bob", people_named_bob.len());
    // ANCHOR_END: property_filter

    // ANCHOR: combined_filter
    // Combine filtering to find young people
    // Filter after retrieval when specialized indexes aren't available
    let young_people = graph
        .walk()
        .vertices(VertexIndex::person())
        .filter_by_person(|person, _| person.age() < 25)
        .collect::<Vec<_>>();

    println!("Found {} people under age 25", young_people.len());
    // ANCHOR_END: combined_filter
}
/* ANCHOR_END: all */
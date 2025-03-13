use graph_api_lib::{Graph, Supported, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Vertex, VertexIndex, populate_graph};

pub fn vertices_step_example() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    // Scan all vertices in the graph
    // This performs a full graph scan, which can be expensive for large graphs
    let all_vertices = graph
        .walk()
        .vertices(VertexSearch::scan())
        .collect::<Vec<_>>();

    println!("Found {} total vertices in the graph", all_vertices.len());

    // Use a label-based index for more efficient lookups
    // This narrows the search to only person vertices
    let people = graph
        .walk()
        .vertices(VertexIndex::person())
        .collect::<Vec<_>>();

    println!("Found {} person vertices", people.len());

    // Use property-based filtering
    // This finds vertices with a specific property value
    let people_named_bob = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter(|vertex, _| {
            if let Vertex::Person(p) = vertex.weight() {
                p.name == "Bob"
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    println!("Found {} people named Bob", people_named_bob.len());

    // Combine filtering to find young people
    // Filter after retrieval when specialized indexes aren't available
    let young_people = graph
        .walk()
        .vertices(VertexIndex::person())
        .filter(|vertex, _| {
            if let Vertex::Person(p) = vertex.weight() {
                p.age == 25
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    println!("Found {} people aged 25", young_people.len());
}

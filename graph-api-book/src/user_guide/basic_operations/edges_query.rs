use graph_api_book::standard_model::{Edge, Vertex, standard_populated_graph};
use graph_api_lib::{EdgeSearch, Graph, VertexSearch};

fn main() {
    // ANCHOR: all

    // Create our standard graph for examples
    let graph = standard_populated_graph();

    // Example showing all edges connected to a vertex
    fn all_edges_example(graph: &impl Graph<Vertex = Vertex, Edge = Edge>) {
        // Find Bryn's vertex to use in examples
        let bryn_id = graph
            .walk()
            .vertices()
            .filter_by_person(|p, _| p.username() == "bryn123")
            .map(|v| v.id())
            .first();

        if let Some(person_id) = bryn_id {
            // ANCHOR: all_edges
            // Get all edges connected to a specific vertex
            let connected_edges = graph
                .walk()
                .vertices_by_id(vec![person_id])
                .edges(EdgeSearch::scan()) // Get all connected edges
                .collect::<Vec<_>>();
            // ANCHOR_END: all_edges

            println!("Found {} connected edges", connected_edges.len());
        }
    }

    // Example showing directional edge queries
    fn directional_edges_example(graph: &impl Graph<Vertex = Vertex, Edge = Edge>) {
        // Find Julia's vertex to use in examples
        let julia_id = graph
            .walk()
            .vertices()
            .filter_by_person(|p, _| p.username() == "julia456")
            .map(|v| v.id())
            .first();

        if let Some(person_id) = julia_id {
            // ANCHOR: directional
            // Get only outgoing edges
            let outgoing_edges = graph
                .walk()
                .vertices_by_id(vec![person_id])
                .edges(EdgeSearch::scan().outgoing()) // Only outgoing edges
                .collect::<Vec<_>>();

            // Get only incoming edges
            let incoming_edges = graph
                .walk()
                .vertices_by_id(vec![person_id])
                .edges(EdgeSearch::scan().incoming()) // Only incoming edges
                .collect::<Vec<_>>();
            // ANCHOR_END: directional

            println!("Outgoing edges: {}", outgoing_edges.len());
            println!("Incoming edges: {}", incoming_edges.len());
        }
    }

    // Example showing label-based edge filtering
    fn label_filter_example(graph: &impl Graph<Vertex = Vertex, Edge = Edge>) {
        // ANCHOR: label_filter
        // Find all "Created" edges in the graph
        let creation_edges = graph
            .walk()
            .edges(EdgeSearch::scan())
            .filter_created() // Type-safe filter using generated helper
            .collect::<Vec<_>>();

        // Find all "Follows" edges in the graph
        let follow_edges = graph
            .walk()
            .edges(EdgeSearch::scan())
            .filter_follows() // Type-safe filter using generated helper
            .collect::<Vec<_>>();

        // Find all edges with timestamp properties
        let timestamped_edges = graph
            .walk()
            .edges(EdgeSearch::scan())
            .filter(|e, _| match e.weight() {
                Edge::Liked { timestamp } => true,
                Edge::Commented { timestamp } => true,
                _ => false,
            })
            .collect::<Vec<_>>();
        // ANCHOR_END: label_filter

        println!("Creation edges: {}", creation_edges.len());
        println!("Follow edges: {}", follow_edges.len());
        println!("Timestamped edges: {}", timestamped_edges.len());
    }

    // ANCHOR_END: all

    // Run examples
    all_edges_example(&graph);
    directional_edges_example(&graph);
    label_filter_example(&graph);
}

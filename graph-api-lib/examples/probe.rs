use graph_api_lib::Graph;
use graph_api_lib::{EdgeSearch, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::populate_graph;

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);
    example(graph, refs.bryn);
}

fn example<G: Graph>(graph: G, bryn_id: G::VertexId) {
    // Simple counting example with vertices
    let mut vertex_count = 0;
    let result = graph
        .walk()
        .vertices(VertexSearch::scan())
        .probe(|_| {
            vertex_count += 1;
            // println!("Visited vertex #{}", vertex_count); // Uncomment to see debug output
        })
        .collect::<Vec<_>>();

    // We should have counted the same number of vertices as were in the result
    assert_eq!(vertex_count, result.len());
    assert!(vertex_count >= 4); // At least bryn, julia, graph_api, rust

    // Simple counting example with edges
    let mut edge_count = 0;
    let result = graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .edges(EdgeSearch::scan())
        .probe(|_| {
            edge_count += 1;
            // println!("Visited edge #{}", edge_count); // Uncomment to see debug output
        })
        .collect::<Vec<_>>();

    // We should have counted the same number of edges as were in the result
    assert_eq!(edge_count, result.len());
    assert!(edge_count > 0); // Bryn should have some edges

    // Multiple probe points in a chain
    let mut vertex_start_count = 0;
    let mut edge_count = 0;
    let mut tail_count = 0;
    let result = graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .probe(|_| { 
            vertex_start_count += 1; 
            // println!("Starting at vertex: {}", vertex_start_count); // Uncomment to see debug output
        })
        .edges(EdgeSearch::scan())
        .probe(|_| { 
            edge_count += 1; 
            // println!("Following edge: {}", edge_count); // Uncomment to see debug output
        })
        .tail()
        .probe(|_| { 
            tail_count += 1; 
            // println!("Arriving at vertex: {}", tail_count); // Uncomment to see debug output
        })
        .collect::<Vec<_>>();

    // The counts should match the number of elements at each stage
    assert_eq!(vertex_start_count, 1); // Started with just bryn
    assert_eq!(edge_count, result.len()); // Same number of edges as final results
    assert_eq!(tail_count, result.len()); // Same number of tail vertices as edges
}
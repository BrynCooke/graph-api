use crate::standard_model::{Edge, Vertex};
use graph_api_lib::EdgeSearch;
use graph_api_lib::Graph;
use graph_api_lib::SupportsEdgeLabelIndex;

// ANCHOR: all

pub fn vertex_count_example<G>(graph: &G, start_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
{
    // ANCHOR: vertex_count
    // Use mutate_context to track each vertex we've seen
    let count = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .push_context(|_, _| 0) // Initialize a counter at 0
        .mutate_context(|_, ctx| {
            // Increment the counter for each vertex
            **ctx += 1;
        })
        .map(|_, ctx| *ctx)
        .next()
        .expect("Expected to process at least one vertex");

    println!("Visited {} vertices", count);
    // ANCHOR_END: vertex_count
}

pub fn edge_count_example<G>(graph: &G, start_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
{
    // ANCHOR: edge_count
    // Use mutate_context to track edges in the traversal
    let edge_count = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .push_context(|_, _| 0) // Initialize counter at 0
        .edges(EdgeSearch::scan().outgoing())
        .mutate_context(|edge, ctx| {
            // Increment for each edge encountered
            **ctx += 1;
            
            // Example of using the edge information while updating context
            println!("Processed edge #{}: {:?}", **ctx, edge.id());
        })
        .map(|_, ctx| *ctx)
        .last() // Get the final count
        .unwrap_or(0);

    println!("Total edges traversed: {}", edge_count);
    // ANCHOR_END: edge_count
}

pub fn accumulator_example<G>(graph: &G, start_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
{
    // ANCHOR: accumulator
    // Use mutate_context to build a collection during traversal
    let person_names = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .push_context(|_, _| Vec::<String>::new()) // Start with empty vector
        .edges(EdgeSearch::scan().outgoing())
        .filter_follows()
        .tail() // Go to target vertices
        .mutate_context(|vertex, ctx| {
            // Extract name from person vertex and add to collection
            if let Vertex::Person { name, .. } = vertex.weight() {
                (**ctx).push(name.clone());
            }
        })
        .map(|_, ctx| ctx.clone())
        .next()
        .unwrap_or_default();

    println!("Connected people: {:?}", person_names);
    // ANCHOR_END: accumulator
}

// ANCHOR_END: all
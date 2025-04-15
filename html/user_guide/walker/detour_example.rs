use crate::standard_model::{Edge, Vertex};
use graph_api_lib::{Graph, SupportsEdgeLabelIndex, SupportsVertexLabelIndex};

// Traversal with detour walker example
pub fn detour_traversal_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexLabelIndex + SupportsEdgeLabelIndex,
{
    // ANCHOR: detour_traversal_example
    // For each person, find the projects they created
    let _results = graph
        .walk()
        .vertices(Vertex::person()) // Start with all Person vertices
        .detour(|person_walker| {
            // For each person, collect the projects they created
            person_walker
                .edges(Edge::created()) // Follow "created" edges
                .tail() // Move to the target (projects)
                .take(1) // Only need one match to qualify
        })
        // Continue with original person vertices that have created projects
        .collect::<Vec<_>>(); // Collect results
    // ANCHOR_END: detour_traversal_example
}

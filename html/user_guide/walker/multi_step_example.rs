use crate::standard_model::{Edge, Vertex};
use graph_api_lib::{Graph, SupportsEdgeLabelIndex, SupportsVertexLabelIndex};

// Multi-step walker traversal example
pub fn multi_step_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexLabelIndex + SupportsEdgeLabelIndex,
{
    // ANCHOR: multi_step_example
    // Find all people who follow someone who created a project
    let _results = graph
        .walk()
        .vertices(Vertex::person()) // Start with all Person vertices
        .edges(Edge::follows()) // Follow "follows" edges
        .tail() // Move to the followed person
        .edges(Edge::created()) // Find "created" edges from these people
        .tail() // Move to the Project vertices
        .collect::<Vec<_>>(); // Collect results
    // ANCHOR_END: multi_step_example
}

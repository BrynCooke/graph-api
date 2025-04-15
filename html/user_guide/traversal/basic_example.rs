use crate::standard_model::{Edge, Vertex, VertexExt};
use graph_api_lib::{Graph, SupportsEdgeLabelIndex, SupportsVertexLabelIndex};

// ANCHOR: all
// Basic traversal example showing a complex path through the graph
pub fn basic_traversal_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexLabelIndex + SupportsEdgeLabelIndex,
{
    // Find all Person vertices who know someone who created a Project
    let _results = graph
        .walk()
        .vertices(Vertex::person()) // Start with Person vertices
        .edges(Edge::follows()) // Follow "follows" edges
        .tail() // Move to the target Person
        .edges(Edge::created()) // Follow "created" edges
        .tail() // Move to the Project
        .filter_project()
        .collect::<Vec<_>>(); // Collect results
}
// ANCHOR_END: all

use crate::standard_model::{Edge, Vertex};
use graph_api_lib::{Graph, SupportsEdgeLabelIndex, SupportsVertexLabelIndex};

// Basic walker traversal example
pub fn basic_walker_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexLabelIndex + SupportsEdgeLabelIndex,
{
    // ANCHOR: basic_walker_example
    // Find all Project vertices created by a Person
    let _results = graph
        .walk()
        .vertices(Vertex::person()) // Start with all Person vertices
        .edges(Edge::created()) // Follow "Created" edges
        .tail() // Move to the target vertices (Projects)
        .collect::<Vec<_>>(); // Collect the Project vertices
    // ANCHOR_END: basic_walker_example
}

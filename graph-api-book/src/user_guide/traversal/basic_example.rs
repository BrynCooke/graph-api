use graph_api_lib::{Graph, Supported};
use graph_api_test::{Edge, EdgeIndex, Vertex, VertexExt, VertexIndex};

/* ANCHOR: all */
// Basic traversal example showing a complex path through the graph
pub fn basic_traversal_example<G>(graph: &G)
where
    G: Graph<
            Vertex = Vertex,
            Edge = Edge,
            SupportsVertexLabelIndex = Supported,
            SupportsEdgeLabelIndex = Supported,
        >,
{
    // Find all Person vertices who know someone who created a Project
    let _results = graph
        .walk()
        .vertices(VertexIndex::person()) // Start with Person vertices
        .edges(EdgeIndex::knows()) // Follow "knows" edges
        .tail() // Move to the target Person
        .edges(EdgeIndex::created()) // Follow "created" edges
        .tail() // Move to the Project
        .filter_project()
        .collect::<Vec<_>>(); // Collect results
}
/* ANCHOR_END: all */

use crate::standard_model::{Edge, Vertex};
use graph_api_lib::{
    EdgeReference, EdgeSearch, Graph, SupportsEdgeLabelIndex, VertexReference, VertexSearch,
};
use std::ops::ControlFlow;

pub fn vertex_context_example<G>(graph: &G, start_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
{
    // ANCHOR: vertex_example
    // Use control_flow to either skip a vertex (None), include it (Some), or stop traversal (Break)
    let _project = graph
        .walk()
        .vertices(VertexSearch::scan())
        .control_flow(|vertex, _| {
            if let Vertex::Project { name } = vertex.weight() {
                // If we find a project with "Graph" in the name, stop traversal
                if name.contains("Graph") {
                    return ControlFlow::Break(Some(vertex));
                }
                // Include other project vertices
                return ControlFlow::Continue(Some(vertex));
            }
            // Skip non-project vertices
            ControlFlow::Continue(None)
        })
        .first();
    // ANCHOR_END: vertex_example

    // ANCHOR: edge_example
    // Use control_flow to skip edges (None), include them (Some), or stop traversal (Break)
    let _early_connection = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .edges(EdgeSearch::scan())
        .control_flow(|edge, _| {
            if let Edge::Follows = edge.weight() {
                // With Follows edge type, always break
                return ControlFlow::Break(Some(edge));
            }
            // Skip non-'follows' edges
            ControlFlow::Continue(None)
        })
        .first();
    // ANCHOR_END: edge_example
}

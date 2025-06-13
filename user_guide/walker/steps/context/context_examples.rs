use crate::standard_model::{Edge, EdgeExt, Person, Vertex};
use graph_api_lib::EdgeReference;
use graph_api_lib::EdgeSearch;
use graph_api_lib::Graph;
use graph_api_lib::SupportsEdgeLabelIndex;
use graph_api_lib::VertexReference;

// ANCHOR: all

pub fn vertex_context_example<G>(graph: &G, bryn_id: G::VertexId, julia_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
{
    // ANCHOR: vertex_context
    // Use push_default_context to make source vertex information available during traversal
    let knows: Vec<_> = graph
        .walk()
        .vertices_by_id(vec![bryn_id, julia_id])
        .push_default_context()
        .edges(EdgeSearch::scan().outgoing())
        .filter_follows()
        .head()
        .map(|target, ctx| {
            if let Vertex::Person { name, .. } = ctx.vertex() {
                format!(
                    "{} follows {}",
                    name,
                    target.project::<Person<_>>().unwrap().name()
                )
            } else {
                "Not a person".to_string()
            }
        })
        .collect::<Vec<_>>();

    // Check the results - should have 2 person descriptions
    assert_eq!(knows.len(), 2);
    println!("Vertex Context Example - Relationships found:");
    for relationship in &knows {
        println!("- {}", relationship);
    }
    // ANCHOR_END: vertex_context
}

pub fn edge_context_example<G>(graph: &G, person_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
{
    // ANCHOR: edge_context
    // Walk the graph starting from the person vertex
    let edge_types = graph
        .walk()
        .vertices_by_id(vec![person_id])
        .edges(EdgeSearch::scan().outgoing())
        .push_context(|edge, _ctx| {
            // Determine edge type based on the edge type
            // Return the edge type as context
            match edge.weight() {
                Edge::Created => "Created",
                Edge::Follows => "Follows",
                Edge::Liked { .. } => "Liked",
                Edge::Commented { .. } => "Commented",
            }
        })
        .map(|_v, c| *c)
        .collect::<Vec<_>>();

    println!("{:?}", edge_types);
    // ANCHOR_END: edge_context
}

pub fn path_tracking_example<G>(graph: &G, start_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
{
    // ANCHOR: path_tracking
    // Track the path while traversing
    let paths = graph
        .walk()
        .vertices_by_id(vec![start_id])
        // Start with an empty path containing just the start vertex name
        .push_context(|v, _| {
            vec![match v.weight() {
                Vertex::Person { name, .. } => name.clone(),
                _ => "Unknown".to_string(),
            }]
        })
        // Follow outgoing follows edges
        .edges(EdgeSearch::scan().outgoing())
        .filter_follows()
        .tail()
        // Add each person to the path
        .push_context(|v, ctx| {
            let mut new_path = (**ctx).clone();
            if let Vertex::Person { name, .. } = v.weight() {
                new_path.push(name.clone());
            }
            new_path
        })
        // Collect all paths
        .map(|_, ctx| ctx.join(" -> "))
        .collect::<Vec<_>>();

    // Print all paths
    println!("All paths from start:");
    for path in paths {
        println!("- {}", path);
    }
    // ANCHOR_END: path_tracking
}

// ANCHOR_END: all

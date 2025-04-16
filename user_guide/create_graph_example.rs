use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::Graph;
use graph_api_simplegraph::SimpleGraph;
use uuid::Uuid;

// ANCHOR: all
// ANCHOR: model
#[derive(Debug, Clone, VertexExt)]
pub enum SocialVertex {
    Person {
        name: String,
        username: String,
        age: u8,
        id: Uuid,
    },
    Post {
        title: String,
        content: String,
        created_at: String,
    },
}

#[derive(Debug, Clone, EdgeExt)]
pub enum SocialEdge {
    Follows,
    Posted,
    Liked,
    Commented { text: String },
}
// ANCHOR_END: model

pub fn create_graph_example() {
    // ANCHOR: create_graph
    // Create a new graph with our vertex and edge types
    let mut graph = SimpleGraph::<SocialVertex, SocialEdge>::new();
    // ANCHOR_END: create_graph

    // ANCHOR: add_vertices
    // Add vertices to the graph
    let bryn_id = graph.add_vertex(SocialVertex::Person {
        name: "Bryn".to_string(),
        username: "bryn123".to_string(),
        age: 28,
        id: Uuid::new_v4(),
    });

    let julia_id = graph.add_vertex(SocialVertex::Person {
        name: "Julia".to_string(),
        username: "julia456".to_string(),
        age: 32,
        id: Uuid::new_v4(),
    });

    let post_id = graph.add_vertex(SocialVertex::Post {
        title: "Graph Databases".to_string(),
        content: "Graph databases are excellent for connected data!".to_string(),
        created_at: "2023-06-15T14:30:00Z".to_string(),
    });
    // ANCHOR_END: add_vertices

    // ANCHOR: add_edges
    // Add edges to connect vertices

    // Bryn follows Julia
    let _follows_edge = graph.add_edge(bryn_id, julia_id, SocialEdge::Follows);

    // Bryn posted the article
    let _posted_edge = graph.add_edge(bryn_id, post_id, SocialEdge::Posted);

    // Julia liked Bryn's post
    let _liked_edge = graph.add_edge(julia_id, post_id, SocialEdge::Liked);

    // Julia commented on Bryn's post
    let _comment_edge = graph.add_edge(
        julia_id,
        post_id,
        SocialEdge::Commented {
            text: "Great overview of graph databases!".to_string(),
        },
    );
    // ANCHOR_END: add_edges
}
// ANCHOR_END: all

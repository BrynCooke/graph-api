use graph_api_simplegraph::SimpleGraph;
use uuid::Uuid;

// Model definitions for a social network
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum SocialEdge {
    Follows,
    Posted,
    Liked,
    Commented {
        text: String,
    },
}

// Function demonstrating how to create a graph and add elements
pub fn create_graph_example() {
    // Create a new graph with our vertex and edge types
    let mut graph = SimpleGraph::<SocialVertex, SocialEdge>::new();
    
    // Add vertices to the graph
    let alice_id = graph.add_vertex(SocialVertex::Person {
        name: "Alice".to_string(),
        username: "alice123".to_string(),
        age: 28,
        id: Uuid::new_v4(),
    });
    
    let bob_id = graph.add_vertex(SocialVertex::Person {
        name: "Bob".to_string(),
        username: "bob456".to_string(),
        age: 32,
        id: Uuid::new_v4(),
    });
    
    let post_id = graph.add_vertex(SocialVertex::Post {
        title: "Graph Databases".to_string(),
        content: "Graph databases are excellent for connected data!".to_string(),
        created_at: "2023-06-15T14:30:00Z".to_string(),
    });
    
    // Add edges to connect vertices
    
    // Alice follows Bob
    let _follows_edge = graph.add_edge(alice_id, bob_id, SocialEdge::Follows);
    
    // Alice posted the article
    let _posted_edge = graph.add_edge(alice_id, post_id, SocialEdge::Posted);
    
    // Bob liked Alice's post
    let _liked_edge = graph.add_edge(bob_id, post_id, SocialEdge::Liked);
    
    // Bob commented on Alice's post
    let _comment_edge = graph.add_edge(
        bob_id, 
        post_id, 
        SocialEdge::Commented {
            text: "Great overview of graph databases!".to_string(),
        }
    );
    
    // Print graph stats
    let vertex_count = graph.vertex_count();
    let edge_count = graph.edge_count();
    println!("Created graph with {} vertices and {} edges", vertex_count, edge_count);
}
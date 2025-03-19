use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::Graph;
use graph_api_simplegraph::SimpleGraph;

// Define vertex types
#[derive(Debug, Clone, VertexExt)]
pub enum SocialVertex {
    // Person vertex with various properties
    Person {
        name: String,
        age: u8,
    },
    // Post vertex with content and timestamp
    Post {
        title: String,
        content: String,
        created_at: String,
    },
    // Simple tag vertex (no properties)
    Tag(String),
}

// Define edge types
#[derive(Debug, Clone, EdgeExt)]
pub enum SocialEdge {
    // Simple edge without properties
    Knows,
    // Edge with properties
    Posted { timestamp: String },
    // Edge using tuple variant
    Tagged(String),
}

// Define a basic property graph model for a social network
pub fn define_basic_model() {
    // Create a graph using our model
    let mut graph = SimpleGraph::<SocialVertex, SocialEdge>::new();
    
    // Add vertices
    let alice = graph.add_vertex(SocialVertex::Person {
        name: "Alice".to_string(),
        age: 30,
    });
    
    let post = graph.add_vertex(SocialVertex::Post {
        title: "Graph Databases".to_string(),
        content: "Property graphs are amazing!".to_string(),
        created_at: "2023-06-15".to_string(),
    });
    
    let tag = graph.add_vertex(SocialVertex::Tag("Database".to_string()));
    
    // Add edges to connect vertices
    graph.add_edge(
        alice, 
        post, 
        SocialEdge::Posted { timestamp: "2023-06-15T10:30:00Z".to_string() }
    );
    
    graph.add_edge(post, tag, SocialEdge::Tagged("Technical".to_string()));
    
    // The graph now represents:
    // Alice --Posted--> Post --Tagged--> Tag
}
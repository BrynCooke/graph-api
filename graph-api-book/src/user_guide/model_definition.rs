use graph_api_derive::{EdgeExt, VertexExt};
use uuid::Uuid;

// Define vertex types for a social media application
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    // Person vertex with various properties
    Person {
        name: String,
        
        #[index] // Standard index for exact lookups
        username: String,
        
        #[index(full_text)] // Full-text index for text search
        biography: String,
        
        #[index(ordered)] // Ordered index for range queries
        age: u8,
        
        unique_id: Uuid, // Not indexed
    },
    
    // Project vertex with minimal properties
    Project {
        name: String,
    },
    
    // Comment vertex
    Comment {
        text: String,
        date: String,
    }
}

// Define edge types that connect vertices
#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    // Simple edges without properties
    Created,
    Follows,
    
    // Edges with properties
    Liked {
        timestamp: String,
    },
    Commented {
        timestamp: String,
    },
}

// Function that demonstrates defining a graph model
pub fn model_definition_example() {
    // Example of creating a vertex instance
    let person = Vertex::Person {
        name: "Alice".to_string(),
        username: "alice123".to_string(),
        biography: "Graph database enthusiast".to_string(),
        age: 28,
        unique_id: Uuid::new_v4(),
    };
    
    // Example of creating an edge instance
    let edge = Edge::Liked {
        timestamp: "2023-06-15T10:30:00Z".to_string(),
    };
    
    println!("Created a person vertex: {}", person.label());
    println!("Created an edge: {}", edge.label());
}
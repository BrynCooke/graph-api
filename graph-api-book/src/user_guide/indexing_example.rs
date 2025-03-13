use graph_api_derive::VertexExt;

// Function demonstrating different index types in vertex models
pub fn indexing_example() {
    #[derive(Debug, Clone, VertexExt)]
    pub enum User {
        // Different types of indexes for different query patterns
        Profile {
            #[index] // Standard index for exact lookups
            username: String,
            
            #[index(ordered)] // Ordered index for range queries
            age: u8,
            
            #[index(full_text)] // Full-text index for text search
            bio: String,
            
            // No index - won't be searchable by this field
            email: String,
        }
    }
    
    // Example of accessing indexed properties
    let profile = User::Profile {
        username: "alice123".to_string(),
        age: 28,
        bio: "Graph database enthusiast from Seattle".to_string(),
        email: "alice@example.com".to_string(),
    };
    
    // Accessing indexed properties
    println!("Username (indexed): {}", profile.username());
    println!("Age (ordered index): {}", profile.age());
    println!("Bio (full-text indexed): {}", profile.bio());
}
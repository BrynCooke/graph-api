use crate::standard_model::{Edge, Vertex, standard_populated_graph};
use graph_api_lib::{Graph, VertexSearch};

/* ANCHOR: all */
// ANCHOR: define_combined_indexes
// Function explaining combined indexes
pub fn define_combined_indexes() {
    // In the standard model, we combine different index types on Person vertices:
    //
    // 1. Person::username - Standard index for exact lookups
    //    - Used for finding users by their unique username
    //
    // 2. Person::biography - Full-text index for text search
    //    - Used for searching content by keywords
    //
    // 3. Person::age - Ordered index for range queries
    //    - Used for finding people in specific age ranges
    //
    // By combining these different index types, we can create powerful
    // queries that address different search patterns efficiently
}
// ANCHOR_END: define_combined_indexes

// ANCHOR: multi_step_traversal
// Example showing multi-step traversals starting with indexed lookups
pub fn multi_step_traversal() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // Find a person by username (using standard index)
    // Then traverse to people they follow (following edges)
    let people_followed_by_alice = graph
        .walk()
        .vertices(VertexIndex::person_by_username("alice123")) // Start with indexed lookup
        .edges(EdgeIndex::follows().outgoing()) // Follow "Follows" edges outward
        .vertices() // Get the vertices on the other end
        .filter(|vertex, _| {
            // Verify they're Person vertices
            matches!(vertex.weight(), Vertex::Person { .. })
        })
        .collect::<Vec<_>>();

    println!("Alice follows {} people", people_followed_by_alice.len());

    // Start with people in a specific age range (using ordered index)
    // Find the projects they created (using edge label index)
    let projects_by_people_in_30s = graph
        .walk()
        .vertices(VertexIndex::person_by_age_range(30..40)) // Indexed age range lookup
        .edges(EdgeIndex::created().outgoing()) // Follow "Created" edges outward
        .vertices() // Get the vertices on the other end
        .filter(|vertex, _| {
            // Verify they're Project vertices
            matches!(vertex.weight(), Vertex::Project { .. })
        })
        .collect::<Vec<_>>();

    println!(
        "People in their 30s created {} projects",
        projects_by_people_in_30s.len()
    );
}
// ANCHOR_END: multi_step_traversal

// ANCHOR: compound_queries
// Example of compound queries combining multiple indexed lookups
pub fn compound_queries() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // Find developers (full-text index) who are in their 30s (ordered index)
    let developers_in_30s = graph
        .walk()
        .vertices(VertexIndex::person_by_biography_containing("developer")) // Full-text index
        .filter(|vertex, _| {
            // Additional filter using age
            if let Vertex::Person { age, .. } = vertex.weight() {
                (30..40).contains(age)
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    println!("Found {} developers in their 30s", developers_in_30s.len());

    // Find projects created by people under 30 who are interested in graphs
    let projects_by_young_graph_enthusiasts = graph
        .walk()
        // Start with people interested in graphs (full-text index)
        .vertices(VertexIndex::person_by_biography_containing("graph"))
        // Filter for those under 30 (could use ordered index directly)
        .filter(|vertex, _| {
            if let Vertex::Person { age, .. } = vertex.weight() {
                age < &30
            } else {
                false
            }
        })
        // Follow Created edges
        .edges(EdgeIndex::created().outgoing())
        // Get the projects
        .vertices()
        .collect::<Vec<_>>();

    println!(
        "Found {} projects created by young graph enthusiasts",
        projects_by_young_graph_enthusiasts.len()
    );
}
// ANCHOR_END: compound_queries

// ANCHOR: hierarchical_navigation
// Example of navigating hierarchical data structures with indexes
pub fn hierarchical_navigation() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // Find the followers (people who follow) of people who created projects
    let followers_of_creators = graph
        .walk()
        // Start with all projects
        .vertices(VertexIndex::project())
        // Find incoming "Created" edges
        .edges(EdgeIndex::created().incoming())
        // Get the creators
        .vertices()
        // Find incoming "Follows" edges (people who follow these creators)
        .edges(EdgeIndex::follows().incoming())
        // Get the followers
        .vertices()
        .collect::<Vec<_>>();

    println!(
        "Found {} followers of project creators",
        followers_of_creators.len()
    );

    // Find projects liked by followers of Bob
    let projects_liked_by_bobs_followers = graph
        .walk()
        // Start with Bob
        .vertices(VertexIndex::person_by_username("bob456"))
        // Find incoming "Follows" edges (people who follow Bob)
        .edges(EdgeIndex::follows().incoming())
        // Get Bob's followers
        .vertices()
        // Find outgoing "Liked" edges
        .edges(EdgeIndex::liked().outgoing())
        // Get the projects they liked
        .vertices()
        .filter(|vertex, _| matches!(vertex.weight(), Vertex::Project { .. }))
        .collect::<Vec<_>>();

    println!(
        "Found {} projects liked by Bob's followers",
        projects_liked_by_bobs_followers.len()
    );
}
// ANCHOR_END: hierarchical_navigation
/* ANCHOR_END: all */

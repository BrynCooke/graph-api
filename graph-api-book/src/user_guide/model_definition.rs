use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::{Graph, VertexReference, VertexSearch};
use graph_api_simplegraph::SimpleGraph;

// ANCHOR: model
// Define vertex types for a social media application
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    // Person vertex with properties
    Person {
        name: String,

        #[index(hash)] // Hash index for exact lookups
        username: String,

        #[index(full_text)] // Full-text index for text search
        biography: String,

        #[index(range)] // Range index for queries like age > 30
        age: u8,
    },

    // Project vertex with properties
    Project {
        name: String,
    },

    // Comment vertex with properties
    Comment {
        text: String,
        date: String,
    },
}

// Define edge types between vertices
#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    // Simple edges without properties
    Follows,
    Created,

    // Edges with properties
    Liked { timestamp: String },
    Commented { timestamp: String },
}
// ANCHOR_END: model

// Example of how to use the model definition
fn usage_example() {
    // ANCHOR: usage
    // Create a new graph with our model
    let mut graph = SimpleGraph::<Vertex, Edge>::new();

    // Add vertices of different types
    let alice = graph.add_vertex(Vertex::Person {
        name: "Alice".to_string(),
        username: "alice_dev".to_string(),
        biography: "Software engineer and graph enthusiast".to_string(),
        age: 29,
    });

    let project = graph.add_vertex(Vertex::Project {
        name: "GraphDB".to_string(),
    });

    // Add edges connecting vertices
    graph.add_edge(alice, project, Edge::Created);

    // Use type-safe projections for property access
    let person_info = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person() // Generated helper method
        .map(|v, _| {
            // Use type-safe projection
            let person = v.project::<Person<_>>().unwrap();

            // Return formatted string with person information
            format!(
                "Person: {} (@{})\n  Bio: {}\n  Age: {}",
                person.name(),
                person.username(),
                person.biography(),
                person.age()
            )
        })
        .collect::<Vec<_>>();

    // Print the collected information
    for info in person_info {
        println!("{}", info);
    }
    // ANCHOR_END: usage
}

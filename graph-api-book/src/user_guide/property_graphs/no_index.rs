use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::{Graph, Supported, VertexReference, VertexSearch};

// Note: No #[index] attributes on any fields
#[derive(Debug, Clone, VertexExt)]
pub enum User {
    Person {
        name: String,
        age: u8,
        email: String,
    },
    Organization {
        name: String,
        founded: u16,
    },
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Relation {
    WorksFor,
    Follows,
    Contacts { frequency: String },
}

// Example showing how to scan the entire graph without indexes
pub fn scan_example<G>(mut graph: G)
where
    G: Graph<Vertex = User, Edge = Relation, SupportsEdgeLabelIndex = Supported>,
{
    // Add several users
    graph.add_vertex(User::Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    });

    graph.add_vertex(User::Person {
        name: "Bob".to_string(),
        age: 42,
        email: "bob@example.com".to_string(),
    });

    graph.add_vertex(User::Person {
        name: "Charlie".to_string(),
        age: 25,
        email: "charlie@example.com".to_string(),
    });

    graph.add_vertex(User::Organization {
        name: "Acme Inc.".to_string(),
        founded: 1985,
    });

    // INEFFICIENT: Find all people named "Alice" by scanning
    let _alice_vertices = graph
        .walk()
        .vertices(VertexSearch::scan()) // Must scan ALL vertices
        .filter(|vertex, _| {
            // Manual pattern matching and filtering
            if let User::Person { name, .. } = vertex.weight() {
                name == "Alice"
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    // INEFFICIENT: Find all people over 40 by scanning
    let _older_people = graph
        .walk()
        .vertices(VertexSearch::scan()) // Must scan ALL vertices
        .filter(|vertex, _| {
            // Manual pattern matching and filtering
            if let User::Person { age, .. } = vertex.weight() {
                age > &40
            } else {
                false
            }
        })
        .collect::<Vec<_>>();
}

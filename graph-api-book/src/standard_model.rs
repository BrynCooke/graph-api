use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::Graph;
use graph_api_simplegraph::SimpleGraph;
use uuid::Uuid;
/* ANCHOR: all */

// ANCHOR: model_definition
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
    },
}

// Define edge types that connect vertices
#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    // Simple edges without properties
    Created,
    Follows,

    // Edges with properties
    Liked { timestamp: String },
    Commented { timestamp: String },
}
// ANCHOR_END: model_definition

/// This standard graph is used in all the examples in this book
///
/// ```svgbob
///                                                                   .───────────────────.
///                                                                   │      "Person"     │
///                                            .──────Follows────────►│    "name: Eve"    │
///                                            │                      │    "age: 31"      │
///                                            │                      '─────────┬─────────'
///                                            │                                │
///                                            │                             "Follows"
///                                            │                                │
///                                            │                                │
///                                            ▼                                ▼
///                                 .────────────────────.           .───────────────────.
///   .──────────────────.          │      "Person"      │           │      "Person"     │
///   │    "Project"     │◄─Created─┤     "name: Alice"  │◄──Follows─┤     "name: Bob"   │
///   │  "name: GraphApi"│          │     "age: 28"      │           │     "age: 34"     │
///   '──────────────────'          '──────────┬─────────'           '─────────┬─────────'
///                                            │                               │
///                                            │                               │
///                                          "Liked"                        "Created"
///                                        "Commented"                         │
///                                            │                               │
///                                            ▼                               │
///                                    .────────────────.                      │
///                                    │    "Project"   │◄─────────────────────'
///                                    │  "name: Alpaca"│
///                                    '────────────────'
/// ```
pub fn standard_populated_graph() -> SimpleGraph<Vertex, Edge> {
    // ANCHOR: setup
    let mut graph = SimpleGraph::new();

    // Create vertices
    let alice = graph.add_vertex(Vertex::Person {
        name: "Alice".to_string(),
        username: "alice123".to_string(),
        biography: "Graph enthusiast".to_string(),
        age: 28,
        unique_id: Uuid::new_v4(),
    });

    let bob = graph.add_vertex(Vertex::Person {
        name: "Bob".to_string(),
        username: "bob456".to_string(),
        biography: "Software developer".to_string(),
        age: 34,
        unique_id: Uuid::new_v4(),
    });

    let eve = graph.add_vertex(Vertex::Person {
        name: "Eve".to_string(),
        username: "eve789".to_string(),
        biography: "Network specialist".to_string(),
        age: 31,
        unique_id: Uuid::new_v4(),
    });

    let graph_api = graph.add_vertex(Vertex::Project {
        name: "GraphApi".to_string(),
    });

    let alpaca = graph.add_vertex(Vertex::Project {
        name: "Alpaca".to_string(),
    });

    // Create edges
    graph.add_edge(alice, graph_api, Edge::Created);
    graph.add_edge(bob, alpaca, Edge::Created);
    graph.add_edge(bob, alice, Edge::Follows);
    graph.add_edge(eve, bob, Edge::Follows);
    graph.add_edge(alice, eve, Edge::Follows);
    graph.add_edge(
        alice,
        alpaca,
        Edge::Liked {
            timestamp: "2023-01-01".to_string(),
        },
    );
    graph.add_edge(
        alice,
        alpaca,
        Edge::Commented {
            timestamp: "2023-01-02".to_string(),
        },
    );

    graph
    // ANCHOR_END: setup
}

/* ANCHOR_END: all */

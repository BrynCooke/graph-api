use graph_api_derive::{VertexExt, EdgeExt};
use graph_api_simplegraph::SimpleGraph;
use graph_api_lib::{Graph, VertexSearch};

/* ANCHOR: all */
// Example function showing basic graph creation and usage
pub fn first_graph_example() {
    // ANCHOR: model
    // Define vertex types
    #[derive(Debug, Clone, VertexExt)]
    pub enum Vertex {
        Person {
            #[index]
            name: String,
            age: u64,
        },
        Project {
            name: String,
        },
    }

    // Define edge types
    #[derive(Debug, Clone, EdgeExt)]
    pub enum Edge {
        Knows { since: i32 },
        Created,
    }
    // ANCHOR_END: model

    // ANCHOR: create_graph
    // Create a graph
    let mut graph = SimpleGraph::new();

    // Add vertices
    let alice = graph.add_vertex(Vertex::Person {
        name: "Alice".to_string(),
        age: 30,
    });

    let bob = graph.add_vertex(Vertex::Person {
        name: "Bob".to_string(),
        age: 28,
    });

    let project = graph.add_vertex(Vertex::Project {
        name: "Graph API".to_string(),
    });

    // Add edges
    graph.add_edge(alice, bob, Edge::Knows { since: 2020 });
    graph.add_edge(alice, project, Edge::Created);
    // ANCHOR_END: create_graph

    // ANCHOR: query
    // Find all vertices
    let vertex_count = graph.walk()
        .vertices(VertexSearch::scan())
        .count::<Vec<_>>();

    println!("Found {} vertices", vertex_count);
    // ANCHOR_END: query
}
/* ANCHOR_END: all */
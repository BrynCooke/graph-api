use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::{Graph, VertexReference, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
// ANCHOR: model
// Define our vertex types using derive macro
#[derive(Debug, Clone, VertexExt)]
enum Vertex {
    // A person vertex with name and age properties
    Person {
        name: String,
        #[index(hash)]
        username: String,
        age: u8,
    },
    // A project vertex with just a name
    Project {
        name: String,
    },
}

// Define our edge types using derive macro
#[derive(Debug, Clone, EdgeExt)]
enum Edge {
    // Simple relationship types
    Knows,
    Created,
    WorksOn,
}
// ANCHOR_END: model
fn main() {
    // ANCHOR: all

    // Function that demonstrates graph creation
    fn create_simple_graph() -> SimpleGraph<Vertex, Edge> {
        // ANCHOR: create_graph
        // Create a new empty graph
        let mut graph = SimpleGraph::<Vertex, Edge>::new();

        // Add vertices
        let alice = graph.add_vertex(Vertex::Person {
            name: "Alice".to_string(),
            username: "alice123".to_string(),
            age: 30,
        });

        let bob = graph.add_vertex(Vertex::Person {
            name: "Bob".to_string(),
            username: "bob456".to_string(),
            age: 28,
        });

        let project = graph.add_vertex(Vertex::Project {
            name: "Graph API".to_string(),
        });

        // Connect vertices with edges
        graph.add_edge(alice, bob, Edge::Knows);
        graph.add_edge(alice, project, Edge::Created);
        graph.add_edge(bob, project, Edge::WorksOn);
        // ANCHOR_END: create_graph

        graph
    }

    // Function that demonstrates graph queries
    fn query_graph(graph: &SimpleGraph<Vertex, Edge>) {
        // ANCHOR: query
        // Find all people who created a project
        let creators = graph
            .walk()
            .vertices(VertexSearch::scan())
            .filter_person() // Type-safe filter using generated helper
            .edges(Edge::created()) // Using generated search function
            .head()
            .filter_project() // Type-safe filter using generated helper
            .map(|v, _| {
                // Use projection for type-safe property access
                v.project::<Project<_>>().unwrap().name().to_string()
            })
            .collect::<Vec<_>>();

        // Find all people who know someone
        let people_with_friends = graph
            .walk()
            .vertices(VertexSearch::scan())
            .filter_person() // Type-safe filter using generated helper
            .edges(Edge::knows()) // Using generated search function
            .tail()
            .filter_person() // Type-safe filter using generated helper
            .map(|v, _| {
                // Use projection for type-safe property access
                v.project::<Person<_>>().unwrap().name().to_string()
            })
            .collect::<Vec<_>>();
        // ANCHOR_END: query

        println!("Created projects: {:?}", creators);
        println!("People with friends: {:?}", people_with_friends);
    }

    // ANCHOR_END: all

    // Create and query the graph
    let graph = create_simple_graph();
    query_graph(&graph);
}

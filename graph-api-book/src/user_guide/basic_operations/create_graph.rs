use graph_api_book::standard_model::{Vertex, Edge, standard_populated_graph};
use graph_api_simplegraph::SimpleGraph;

fn main() {
    // ANCHOR: all
    
    // Example showing how to create a new graph
    fn create_graph_example() {
        // ANCHOR: create_graph
        // Import the standard model and create a new graph
        let mut graph = SimpleGraph::<Vertex, Edge>::new();
        // ANCHOR_END: create_graph
    }
    
    // Example showing how to add vertices
    fn add_vertices_example() {
        let mut graph = SimpleGraph::<Vertex, Edge>::new();
        
        // ANCHOR: add_vertices
        // Add a person vertex
        let bryn = graph.add_vertex(Vertex::Person {
            name: "Bryn".to_string(),
            username: "bryn123".to_string(),
            biography: "Graph enthusiast".to_string(),
            age: 28,
        });
        
        // Add a project vertex
        let project = graph.add_vertex(Vertex::Project {
            name: "GraphAPI".to_string(),
        });
        
        // Add a comment vertex
        let comment = graph.add_vertex(Vertex::Comment {
            text: "Great project!".to_string(),
            date: "2023-05-15".to_string(),
        });
        // ANCHOR_END: add_vertices
    }
    
    // Example showing how to add edges
    fn add_edges_example() {
        let mut graph = SimpleGraph::<Vertex, Edge>::new();
        
        // Create some vertices first
        let bryn = graph.add_vertex(Vertex::Person {
            name: "Bryn".to_string(),
            username: "bryn123".to_string(),
            biography: "Graph enthusiast".to_string(),
            age: 28,
        });
        
        let julia = graph.add_vertex(Vertex::Person {
            name: "Julia".to_string(),
            username: "julia456".to_string(),
            biography: "Software developer".to_string(),
            age: 34,
        });
        
        let project = graph.add_vertex(Vertex::Project {
            name: "GraphAPI".to_string(),
        });
        
        // ANCHOR: add_edges
        // Add simple edges
        graph.add_edge(bryn, project, Edge::Created);
        graph.add_edge(julia, bryn, Edge::Follows);
        
        // Add edges with properties
        graph.add_edge(
            bryn, 
            project,
            Edge::Liked { 
                timestamp: "2023-01-15".to_string() 
            }
        );
        // ANCHOR_END: add_edges
    }
    
    // ANCHOR_END: all
    
    // Run examples
    create_graph_example();
    add_vertices_example();
    add_edges_example();
}
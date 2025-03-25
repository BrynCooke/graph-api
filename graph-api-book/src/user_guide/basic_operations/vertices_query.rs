use graph_api_lib::{Graph, VertexSearch};
use graph_api_book::standard_model::{Vertex, Edge, standard_populated_graph};

fn main() {
    // ANCHOR: all
    
    // Create our standard graph for examples
    let graph = standard_populated_graph();
    
    // Example showing a full vertex scan
    fn scan_all_example(graph: &impl Graph<Vertex = Vertex, Edge = Edge>) {
        // ANCHOR: scan_all
        // Get all vertices in the graph
        let all_vertices = graph.walk()
            .vertices() // Start with all vertices
            .collect::<Vec<_>>();
        
        // Count vertices by type using type-safe filters
        let person_count = graph.walk()
            .vertices()
            .filter_person() // Use generated helper method
            .count();
        
        let project_count = graph.walk()
            .vertices()
            .filter_project() // Use generated helper method
            .count();
        // ANCHOR_END: scan_all
        
        println!("Total vertices: {}", all_vertices.len());
        println!("Person count: {}", person_count);
        println!("Project count: {}", project_count);
    }
    
    // Example showing label-based lookup
    fn label_index_example(graph: &impl Graph<Vertex = Vertex, Edge = Edge>) {
        // ANCHOR: label_index
        // Find all Person vertices using label index
        let all_people = graph.walk()
            .vertices()
            .filter_person() // Use generated helper for label filtering
            .collect::<Vec<_>>();
        
        // Find all Project vertices using label index
        let all_projects = graph.walk()
            .vertices()
            .filter_project() // Use generated helper for label filtering
            .collect::<Vec<_>>();
        // ANCHOR_END: label_index
        
        println!("Found {} people", all_people.len());
        println!("Found {} projects", all_projects.len());
    }
    
    // Example showing property-based filtering
    fn property_filter_example(graph: &impl Graph<Vertex = Vertex, Edge = Edge>) {
        // ANCHOR: property_filter
        // Find people over 30 years old
        let older_people = graph.walk()
            .vertices()
            .filter_by_person(|person, _| {
                // Type-safe access to Person properties
                person.age() > 30
            })
            .map(|v, _| {
                // Use type-safe projection and accessor methods
                let person = v.project::<Person<_>>().unwrap();
                format!("{} ({})", person.name(), person.age())
            })
            .collect::<Vec<_>>();
        
        // Find people with "programmer" in their biography
        let programmers = graph.walk()
            .vertices()
            .filter_by_person(|person, _| {
                // Type-safe access to Person properties
                person.biography().contains("programmer")
            })
            .map(|v, _| {
                // Use type-safe projection and accessor methods
                v.project::<Person<_>>().unwrap().name().to_string()
            })
            .collect::<Vec<_>>();
        // ANCHOR_END: property_filter
        
        println!("People over 30: {:?}", older_people);
        println!("Programmers: {:?}", programmers);
    }
    
    // ANCHOR_END: all
    
    // Run examples
    scan_all_example(&graph);
    label_index_example(&graph);
    property_filter_example(&graph);
}
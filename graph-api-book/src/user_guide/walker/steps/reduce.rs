use graph_api_lib::Graph;
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, Vertex, VertexIndex, populate_graph};

pub fn reduce_example() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    // Find the person with the highest age using reduce
    let oldest_person = graph
        .walk()
        .vertices(VertexIndex::person())
        .reduce(|person1, person2, _, _| {
            let age1 = match person1.weight() {
                Vertex::Person { age, .. } => *age,
                _ => 0,
            };
            
            let age2 = match person2.weight() {
                Vertex::Person { age, .. } => *age,
                _ => 0,
            };
            
            if age1 > age2 {
                person1
            } else {
                person2
            }
        });

    // Output the result
    if let Some(person) = oldest_person {
        if let Vertex::Person { name, age, .. } = person.weight() {
            println!("Oldest person is {} at age {}", name, age);
        }
    } else {
        println!("No people found in the graph");
    }
}
use crate::standard_model::{Edge, Person, PersonMut, Vertex, standard_populated_graph};
use graph_api_lib::{Graph, VertexReference, VertexReferenceMut};

// ANCHOR: all
// Function demonstrating how to use the mutate step for graph modifications
pub fn mutate_example() {
    // Use the standard graph defined in standard_model.rs
    let mut graph = standard_populated_graph();

    // ANCHOR: update_vertex
    // Example 1: Update all person vertices to increment their age
    println!("Incrementing age of all people:");

    // Print original ages
    graph
        .walk()
        .vertices(Vertex::person())
        .probe(|vertex, _| {
            if let Some(person) = vertex.project::<Person<_>>() {
                println!("  Before: {} is {} years old", person.name(), person.age());
            }
        })
        .count();

    // Perform the mutation - increment everyone's age by 1
    let updated_count = graph
        .walk_mut() // Must use walk_mut() for mutations
        .vertices(Vertex::person())
        .mutate(|graph, vertex_id, _| {
            // Note: updating requires cloning and replacing due to Rust's ownership model
            if let Some(mut vertex) = graph.vertex_mut(vertex_id) {
                if let Some(mut person) = vertex.project_mut::<PersonMut<_, _>>() {
                    // Increment the person age
                    person.set_age(person.age() + 1);
                }
            }
        });

    println!("  Updated {} person vertices", updated_count);
    // ANCHOR_END: update_vertex

    // ANCHOR: add_edges
    // Example 2: Create a new project and add edges from all people to it
    println!("\nAdding 'Created' edges from all people to a new project:");

    // Add a new project vertex
    let new_project_id = graph.add_vertex(Vertex::Project {
        name: "GraphDB".to_string(),
    });

    println!("  Created new project: GraphDB");

    // Add 'Created' edges from all people to the new project
    let edge_count = graph
        .walk_mut()
        .vertices(Vertex::person())
        .mutate(|graph, person_id, _| {
            // Create an edge from the person to the new project
            graph.add_edge(person_id, new_project_id, Edge::Created);

            // Get person name for logging
            if let Some(vertex) = graph.vertex(person_id) {
                if let Some(person) = vertex.project::<Person<_>>() {
                    println!("  Added edge: {} -> GraphDB", person.name());
                }
            }
        });

    println!("  Added {} 'Created' edges to the graph", edge_count);
    // ANCHOR_END: add_edges
}
// ANCHOR_END: all

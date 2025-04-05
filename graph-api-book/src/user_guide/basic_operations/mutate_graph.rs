use graph_api_book::standard_model::{Edge, Vertex, standard_populated_graph};
use graph_api_lib::{EdgeSearch, Graph, VertexSearch};

fn main() {
    // ANCHOR: all

    // Create a mutable copy of our standard graph
    let mut graph = standard_populated_graph();

    // Example showing how to update vertex properties
    fn update_vertices_example(graph: &mut impl Graph<Vertex = Vertex, Edge = Edge>) {
        // ANCHOR: update_vertices
        // Update all Person vertices to increment their age
        let updated_count = graph
            .walk_mut()
            .vertices()
            .filter_person() // Type-safe filter using generated helper
            .mutate(|v| {
                // Type-safe mutation using projection
                if let Some(mut person) = v.project_mut::<Person<_>>() {
                    // Update the age property
                    let current_age = person.age();
                    person.set_age(current_age + 1);
                    true // Indicate that we modified the vertex
                } else {
                    false // No modification
                }
            });

        // Add a property to a specific person
        graph
            .walk_mut()
            .vertices()
            .filter_by_person(|p, _| p.username() == "bryn123")
            .mutate(|v| {
                // Type-safe mutation using projection
                if let Some(mut person) = v.project_mut::<Person<_>>() {
                    // Update the biography property
                    person.set_biography("Updated biography: Graph API expert");
                    true // Indicate that we modified the vertex
                } else {
                    false // No modification
                }
            });
        // ANCHOR_END: update_vertices
    }

    // Example showing how to verify changes
    fn verify_changes_example(graph: &impl Graph<Vertex = Vertex, Edge = Edge>) {
        // ANCHOR: verify_changes
        // Check that Bryn's biography was updated
        let updated_bio = graph
            .walk()
            .vertices()
            .filter_by_person(|p, _| p.username() == "bryn123")
            .map(|v, _| {
                // Type-safe access using projection
                v.project::<Person<_>>().unwrap().biography().to_string()
            })
            .first();

        // Verify all people are now older
        let all_ages = graph
            .walk()
            .vertices()
            .filter_person() // Type-safe filter using generated helper
            .map(|v, _| {
                // Type-safe access using projection
                let person = v.project::<Person<_>>().unwrap();
                (person.name().to_string(), person.age())
            })
            .collect::<Vec<_>>();
        // ANCHOR_END: verify_changes

        if let Some(bio) = updated_bio {
            println!("Updated biography: {}", bio);
        }

        println!("Updated ages:");
        for (name, age) in all_ages {
            println!("  {} is now {}", name, age);
        }
    }

    // ANCHOR_END: all

    // Run examples
    update_vertices_example(&mut graph);
    verify_changes_example(&graph);
}

use crate::standard_model::{EdgeIndex, VertexExt, VertexIndex, standard_populated_graph};
use graph_api_lib::{Graph, VertexReference};

// ANCHOR: all
// Function demonstrating how to use the detour step
pub fn detour_example() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: basic_detour
    // Basic detour example: Find projects created by people who follow someone over 30
    println!("Projects created by people who follow someone over 30:");

    let projects = graph
        .walk()
        .vertices(VertexIndex::person()) // Start with all people
        .detour(|sub_walker| {
            // The detour checks if this person follows someone over 30
            sub_walker
                .edges(EdgeIndex::follows()) // Follow "follows" edges
                .head() // Move to the target person
                .filter_by_person(|person, _| person.age() > 30) // Check if they're over 30
                .take(1) // We only need one match to qualify
        })
        // Back to original person vertices that passed the detour check
        .edges(EdgeIndex::created()) // Find what they created
        .head() // Move to the created project
        .filter_project() // Keep only project vertices
        .collect::<Vec<_>>();

    for project in projects {
        println!("{:?}", project);
    }
    // ANCHOR_END: basic_detour

    // ANCHOR: complex_detour
    // Complex detour example: Find people who created projects that were liked by others
    println!("\nPeople who created projects that were liked by others:");

    let creators = graph
        .walk()
        .vertices(VertexIndex::person()) // Start with all people
        .push_default_context()
        .detour(|creator_walker| {
            // Detour to check if this person created something that was liked
            creator_walker
                .edges(EdgeIndex::created()) // Find what they created
                .head() // Move to the created project
                .detour(|project_walker| {
                    // Nested detour to check if the project was liked
                    project_walker
                        .edges(EdgeIndex::liked()) // Find "liked" edges pointing to this project
                        .tail() // Move to the person who liked it
                        .filter(|liker, ctx| {
                            // Check that the liker is different from the creator
                            *ctx.vertex_id() != liker.id()
                        })
                })
        })
        .collect::<Vec<_>>();

    for creator in creators {
        println!("{:?} created something that was liked", creator);
    }
    // ANCHOR_END: complex_detour
}
// ANCHOR_END: all

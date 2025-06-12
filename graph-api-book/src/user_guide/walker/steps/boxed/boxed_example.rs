use crate::standard_model::{Person, Project, Vertex, VertexExt, standard_populated_graph};
use graph_api_lib::{Graph, VertexReference};

// ANCHOR: all
// Function demonstrating how to use the boxed step for type erasure
pub fn boxed_examples() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: basic_boxing
    // Without boxing - creates complex nested types like:
    // Endpoints<Edges<Endpoints<Edges<Vertices<Empty>>>>>
    let _complex_type = graph
        .walk()
        .vertices(Vertex::person()) // Start with Person vertices
        .edges_out() // Get outgoing edges
        .head() // Move to target vertices
        .edges_out() // Get their outgoing edges
        .head(); // Move to final target vertices
    // This creates deeply nested generic types that slow compilation

    // With strategic boxing - simplified types
    let people_connections: Vec<_> = graph
        .walk()
        .vertices(Vertex::person()) // Start with Person vertices
        .edges_out() // Get outgoing edges
        .boxed() // ← Breaks type complexity chain
        .head() // Move to target vertices
        .edges_out() // Get their outgoing edges
        .boxed() // ← Further reduces complexity
        .head() // Move to final target vertices
        .collect();

    println!("Found {} connected vertices through people", people_connections.len());
    // ANCHOR_END: basic_boxing

    // ANCHOR: complex_traversal
    // Complex multi-hop traversal with strategic boxing
    let projects_via_people: Vec<_> = graph
        .walk()
        .vertices(Vertex::person()) // Start with Person vertices
        .filter_by_person(|person, _| person.age() > 25) // Filter by age
        .take(5) // Limit to first 5 people
        .boxed() // ← Box after initial filtering
        .edges_out() // Get their outgoing relationships
        .head() // Move to connected vertices
        .filter_project() // Keep only Project vertices
        .boxed() // ← Box after project filtering
        .collect();

    println!("Found {} projects created by people over 25", projects_via_people.len());
    // ANCHOR_END: complex_traversal

    // ANCHOR: builder_pattern
    // Builder pattern enabled by boxing
    struct PersonTraversalBuilder<'graph, G: Graph> {
        walker: Option<Box<dyn Iterator<Item = G::VertexId> + 'graph>>,
    }

    impl<'graph, G: Graph> PersonTraversalBuilder<'graph, G> {
        fn add_young_people(&mut self, graph: &'graph G) {
            let walker = graph
                .walk()
                .vertices(Vertex::person()) // Type-safe Person lookup
                .filter_by_person(|person, _| person.age() < 30) // Filter young people
                .boxed() // ← Boxing enables storage in the builder
                .into_iter();
            self.walker = Some(Box::new(walker));
        }
    }

    let mut builder = PersonTraversalBuilder { walker: None };
    builder.add_young_people(&graph);
    // ANCHOR_END: builder_pattern

    // ANCHOR: walker_collection
    // Store different walker types in a collection using boxing
    let vertex_walkers: Vec<Box<dyn Iterator<Item = _>>> = vec![
        // First walker: all people
        Box::new(
            graph
                .walk()
                .vertices(Vertex::person()) // Type-safe Person lookup
                .boxed() // ← Boxing makes storage possible
                .into_iter()
        ),
        // Second walker: young people only
        Box::new(
            graph
                .walk()
                .vertices(Vertex::person())
                .filter_by_person(|person, _| person.age() < 30)
                .boxed() // ← Boxing makes storage possible
                .into_iter()
        ),
    ];

    println!("Created {} different walker configurations", vertex_walkers.len());
    // ANCHOR_END: walker_collection

    // ANCHOR: logical_boundaries
    // Box at logical traversal boundaries for better organization
    let connection_chain: Vec<_> = graph
        .walk()
        // Phase 1: Find starting people
        .vertices(Vertex::person())
        .filter_by_person(|person, _| person.age() > 20)
        .take(3)
        .boxed() // ← End of phase 1: people selection
        
        // Phase 2: Follow their connections
        .edges_out()
        .head()
        .boxed() // ← End of phase 2: first hop
        
        // Phase 3: Follow second-degree connections
        .edges_out()
        .head()
        .collect();

    println!("Found {} vertices in 2-hop traversal", connection_chain.len());
    // ANCHOR_END: logical_boundaries

    // ANCHOR: hybrid_approach
    // Mix boxed and unboxed based on complexity
    let hybrid_result: Vec<_> = graph
        .walk()
        .vertices(Vertex::person())
        .take(2) // Simple operation - no boxing needed
        .edges_out() // Simple edge traversal
        .head() // Simple movement to head
        .edges_out() // This starts getting complex...
        .boxed() // ← Box only when complexity grows
        .head()
        .edges_out()
        .head()
        .collect();

    println!("Hybrid approach found {} vertices", hybrid_result.len());
    // ANCHOR_END: hybrid_approach
}
// ANCHOR_END: all

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxed_examples() {
        boxed_examples();
    }
}
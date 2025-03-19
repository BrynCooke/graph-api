use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::{Graph, VertexReference, VertexSearch};
use graph_api_simplegraph::SimpleGraph;

// Model with standard indexes
#[derive(Debug, Clone, VertexExt)]
pub enum User {
    Person {
        #[index] // Standard index for exact lookups
        username: String,

        #[index] // Another standard index
        email: String,

        // Not indexed
        name: String,
        age: u8,
    },
    Product {
        #[index] // Index for product code
        product_code: String,

        name: String,
        price: f64,
    },
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Relation {
    Purchased,
    Reviewed { rating: u8 },
}

// Function explaining standard indexes
pub fn define_standard_index() {
    // Standard indexes provide efficient lookups based on exact property values
    // They are defined using the #[index] attribute

    // In the User enum above:
    // - Person::username has a standard index for exact username lookups
    // - Person::email has a standard index for exact email lookups
    // - Product::product_code has a standard index for exact product code lookups

    // Properties without the #[index] attribute (name, age, price)
    // can only be searched via full scan
}

// Example of querying with standard indexes
pub fn standard_index_queries() {
    // Create a graph with some data
    let mut graph = SimpleGraph::<User, Relation>::new();

    // Add vertices
    graph.add_vertex(User::Person {
        username: "alice123".to_string(),
        email: "alice@example.com".to_string(),
        name: "Alice Smith".to_string(),
        age: 30,
    });

    graph.add_vertex(User::Person {
        username: "bob456".to_string(),
        email: "bob@example.com".to_string(),
        name: "Robert Jones".to_string(),
        age: 42,
    });

    let _laptop_id = graph.add_vertex(User::Product {
        product_code: "LAPTOP-X1".to_string(),
        name: "High-Performance Laptop".to_string(),
        price: 1299.99,
    });

    graph.add_vertex(User::Product {
        product_code: "PHONE-P5".to_string(),
        name: "Smartphone Pro".to_string(),
        price: 899.99,
    });

    // The VertexIndex enum is automatically generated from the User enum
    // by the VertexExt derive macro. It provides methods for each indexed field.

    // EFFICIENT: Find a person by username (using standard index)
    // This uses the auto-generated index methods from the VertexExt derive macro
    let _alice = graph
        .walk()
        // Use the index
        .vertices(UserIndex::person_by_username("alice123"))
        .first();

    // Also efficient: Find a person by email
    let _bob = graph
        .walk()
        // Use the index
        .vertices(UserIndex::person_by_email("bob@example.com"))
        .first();

    // Find a product by its product code
    let _laptop = graph
        .walk()
        // use the index
        .vertices(UserIndex::product_by_product_code("LAPTOP-X1"))
        .first();

    // Note: No direct index for looking up by name or age
    // This would require a full scan with filtering
    let _people_named_robert = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter(|vertex, _| {
            // We need to manually check the type and fields
            // because there's no index for the name property
            if let User::Person { name, .. } = vertex.weight() {
                name.contains("Robert")
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    // While the above scan works, it's much less efficient than using indexes
    // It will examine every vertex in the graph, rather than just
    // going directly to the relevant vertices
}

use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::{EdgeSearch, Graph, VertexReference, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
// ANCHOR: define_standard_index
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        name: String,

        #[index(hash)] // Hash index for username lookups
        username: String,

        age: u8,
    },

    Product {
        name: String,

        #[index(hash)] // Hash index for SKU lookups
        sku: String,

        price: f32,
    },
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Purchased,
    Viewed,
}
// ANCHOR_END: define_standard_index

fn main() {
    // ANCHOR: all

    // Create a graph and populate it with data
    let mut graph = SimpleGraph::<Vertex, Edge>::new();

    // Add some person vertices
    let alice = graph.add_vertex(Vertex::Person {
        name: "Alice".to_string(),
        username: "alice123".to_string(),
        age: 28,
    });

    let bob = graph.add_vertex(Vertex::Person {
        name: "Bob".to_string(),
        username: "bob456".to_string(),
        age: 34,
    });

    // Add some product vertices
    let product1 = graph.add_vertex(Vertex::Product {
        name: "Laptop".to_string(),
        sku: "LAP-1001".to_string(),
        price: 999.99,
    });

    let product2 = graph.add_vertex(Vertex::Product {
        name: "Headphones".to_string(),
        sku: "AUD-2002".to_string(),
        price: 79.99,
    });

    // Add some edges
    graph.add_edge(alice, product1, Edge::Purchased);
    graph.add_edge(alice, product2, Edge::Viewed);
    graph.add_edge(bob, product2, Edge::Purchased);

    // ANCHOR: standard_index_queries
    // Find a person by username (using hash index)
    let _alice_results = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_by_person(|p, _| p.username() == "alice123")
        .collect::<Vec<_>>();

    // Find a product by SKU (using hash index)
    let headphones = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_by_product(|p, _| p.sku() == "AUD-2002")
        .first();

    // Find who purchased a specific product (combining index lookup with traversal)
    // For real code, we would get the ID from the vertex
    // For this documentation example, we'll simplify
    if headphones.is_some() {
        // Get product by SKU instead of by ID
        let purchasers = graph
            .walk()
            .vertices(VertexSearch::scan())
            .filter_by_product(|p, _| p.sku() == "AUD-2002")
            .edges(EdgeSearch::scan().incoming())
            .filter_purchased() // Type-safe filter using the generated helper
            .head()
            .filter_person() // Type-safe filter using the generated helper
            .map(|v, _| {
                // Type-safe property access using projection
                v.project::<Person<_>>().unwrap().name().to_string()
            })
            .collect::<Vec<_>>();

        println!("People who purchased headphones: {:?}", purchasers);
    }
    // ANCHOR_END: standard_index_queries

    // ANCHOR_END: all
}

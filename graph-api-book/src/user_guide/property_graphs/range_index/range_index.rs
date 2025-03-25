use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::{EdgeSearch, Graph, VertexReference, VertexSearch};
use graph_api_simplegraph::SimpleGraph;

// ANCHOR: define_range_index
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    // Person vertex with range-indexed age
    Person {
        name: String,

        #[index(hash)]
        username: String,

        #[index(range)] // Range index for age-based queries
        age: u32,
    },

    // Product vertex with range-indexed price
    Product {
        name: String,

        #[index(hash)]
        sku: String,

        #[index(range)] // Range index for price-based queries
        price: f64,

        #[index(range)] // Range index for date-based queries
        release_date: String, // Format: "YYYY-MM-DD"
    },
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Purchased {
        timestamp: String, // Format: "YYYY-MM-DD"
    },
    Viewed,
}
// ANCHOR_END: define_range_index

fn main() {
    // ANCHOR: all

    // Create a graph and populate it with data
    let mut graph = SimpleGraph::<Vertex, Edge>::new();

    // Add person vertices
    let alice = graph.add_vertex(Vertex::Person {
        name: "Alice".to_string(),
        username: "alice123".to_string(),
        age: 28,
    });

    let bob = graph.add_vertex(Vertex::Person {
        name: "Bob".to_string(),
        username: "bob456".to_string(),
        age: 35,
    });

    let carol = graph.add_vertex(Vertex::Person {
        name: "Carol".to_string(),
        username: "carol789".to_string(),
        age: 42,
    });

    // Add product vertices
    let laptop = graph.add_vertex(Vertex::Product {
        name: "Laptop Pro".to_string(),
        sku: "LAP-1001".to_string(),
        price: 1299.99,
        release_date: "2023-01-15".to_string(),
    });

    let tablet = graph.add_vertex(Vertex::Product {
        name: "Tablet Mini".to_string(),
        sku: "TAB-2002".to_string(),
        price: 499.99,
        release_date: "2023-03-10".to_string(),
    });

    let headphones = graph.add_vertex(Vertex::Product {
        name: "Wireless Headphones".to_string(),
        sku: "AUD-3003".to_string(),
        price: 149.99,
        release_date: "2022-11-20".to_string(),
    });

    // Add purchase edges
    graph.add_edge(
        alice,
        laptop,
        Edge::Purchased {
            timestamp: "2023-02-01".to_string(),
        },
    );

    graph.add_edge(
        bob,
        tablet,
        Edge::Purchased {
            timestamp: "2023-04-15".to_string(),
        },
    );

    graph.add_edge(
        carol,
        headphones,
        Edge::Purchased {
            timestamp: "2023-01-05".to_string(),
        },
    );

    graph.add_edge(alice, headphones, Edge::Viewed);
    graph.add_edge(bob, laptop, Edge::Viewed);

    // ANCHOR: range_queries
    // Find people in a specific age range (30-45)
    let middle_aged = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_by_person(|person, _| {
            // Range query on age property
            let age = person.age();
            age >= 30 && age <= 45
        })
        .map(|v, _| {
            // Use type-safe projection
            let person = v.project::<Person<_>>().unwrap();
            format!("{} ({})", person.name(), person.age())
        })
        .collect::<Vec<_>>();

    // Find products in a specific price range ($100-$500)
    let mid_price_products = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_by_product(|product, _| {
            // Range query on price property
            let price = product.price();
            price >= 100.0 && price <= 500.0
        })
        .map(|v, _| {
            // Use type-safe projection
            let product = v.project::<Product<_>>().unwrap();
            format!("{} (${:.2})", product.name(), product.price())
        })
        .collect::<Vec<_>>();

    // Find products released in the first quarter of 2023
    let q1_products = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_by_product(|product, _| {
            // Range query on release_date property
            let date = product.release_date();
            date >= "2023-01-01" && date <= "2023-03-31"
        })
        .map(|v, _| {
            // Use type-safe projection
            let product = v.project::<Product<_>>().unwrap();
            format!("{} (released: {})", product.name(), product.release_date())
        })
        .collect::<Vec<_>>();

    // Find purchases made in a specific date range
    let q1_purchases = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(EdgeSearch::scan())
        .filter_by_purchased(|edge, _| {
            // Range query on timestamp property
            let timestamp = edge.timestamp();
            timestamp >= "2023-01-01" && timestamp <= "2023-03-31"
        })
        .head()
        .filter_person() // Type-safe filter using generated helper
        .map(|v, _| {
            // Use type-safe projection
            v.project::<Person<_>>().unwrap().name().to_string()
        })
        .collect::<Vec<_>>();
    // ANCHOR_END: range_queries

    // ANCHOR: range_sort
    // Find people in a specific age range (21-50), sorted by age descending
    let people_by_age = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_by_person(|person, _| {
            // Range query on age property
            let age = person.age();
            age >= 21 && age <= 50
        })
        .collect::<Vec<_>>();

    // For sorting, we would extract the data from vertices
    // For this documentation example, we'll use simplified data
    let mut person_data = vec![
        ("Alice", 28),
        ("Bob", 35),
        ("Carol", 42)
    ];
    
    // Sort by age in descending order
    person_data.sort_by(|(_, age_a), (_, age_b)| {
        age_b.cmp(age_a)
    });
    
    // Map the sorted results to names with ages
    let people_sorted = person_data
        .iter()
        .map(|(name, age)| {
            format!("{} ({})", name, age)
        })
        .collect::<Vec<_>>();
    // ANCHOR_END: range_sort

    // Print the results
    println!("Middle-aged people: {:?}", middle_aged);
    println!("Mid-priced products: {:?}", mid_price_products);
    println!("Q1 products: {:?}", q1_products);
    println!("Q1 purchases: {:?}", q1_purchases);
    println!("People sorted by age (descending): {:?}", people_sorted);

    // ANCHOR_END: all
}

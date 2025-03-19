use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::Graph;
use graph_api_simplegraph::SimpleGraph;
use std::ops::Range;

// Define a model with ordered indexes
pub fn define_ordered_index() {
    #[derive(Debug, Clone, VertexExt)]
    pub enum Content {
        User {
            username: String,

            #[index(ordered)] // Ordered index for age ranges
            age: u8,

            #[index(ordered)] // Ordered index for signup date
            signup_date: u64, // Unix timestamp
        },
        Product {
            name: String,

            #[index(ordered)] // Ordered index for price ranges
            price: f64,

            #[index(ordered)] // Ordered index for review score
            average_rating: f32,
        },
    }
}

// Example of querying with ordered indexes
pub fn ordered_index_queries() {
    // Define our model with ordered indexes
    #[derive(Debug, Clone, VertexExt)]
    pub enum Content {
        User {
            username: String,

            #[index(ordered)]
            age: u8,

            #[index(ordered)]
            signup_date: u64, // Unix timestamp
        },
        Product {
            name: String,

            #[index(ordered)]
            price: f64,

            #[index(ordered)]
            average_rating: f32,
        },
    }

    #[derive(Debug, Clone, EdgeExt)]
    pub enum Interaction {
        Purchased,
        Reviewed,
    }

    // Create a graph with some data
    let mut graph = SimpleGraph::<Content, Interaction>::new();

    // Add some users with different ages
    graph.add_vertex(Content::User {
        username: "user1".to_string(),
        age: 22,
        signup_date: 1609459200, // 2021-01-01
    });

    graph.add_vertex(Content::User {
        username: "user2".to_string(),
        age: 35,
        signup_date: 1625097600, // 2021-07-01
    });

    graph.add_vertex(Content::User {
        username: "user3".to_string(),
        age: 42,
        signup_date: 1640995200, // 2022-01-01
    });

    graph.add_vertex(Content::User {
        username: "user4".to_string(),
        age: 19,
        signup_date: 1656633600, // 2022-07-01
    });

    // Add some products with different prices
    graph.add_vertex(Content::Product {
        name: "Budget Phone".to_string(),
        price: 299.99,
        average_rating: 3.5,
    });

    graph.add_vertex(Content::Product {
        name: "Mid-range Phone".to_string(),
        price: 599.99,
        average_rating: 4.0,
    });

    graph.add_vertex(Content::Product {
        name: "Premium Phone".to_string(),
        price: 999.99,
        average_rating: 4.8,
    });

    // Example 1: Find young adults (18-25)
    let young_adults = graph
        .walk()
        .vertices(VertexIndex::user_by_age_range(18..26))
        .collect::<Vec<_>>();

    // Example 2: Find users who signed up in the first half of 2022
    let q1_q2_2022_signups = graph
        .walk()
        .vertices(VertexIndex::user_by_signup_date_range(
            1640995200..1656633600,
        )) // Jan 1 to Jun 30, 2022
        .collect::<Vec<_>>();

    // Example 3: Find mid-range products ($500-$800)
    let mid_range_products = graph
        .walk()
        .vertices(VertexIndex::product_by_price_range(500.0..800.0))
        .collect::<Vec<_>>();

    // Example 4: Find highly-rated products (4.5+ stars)
    let highly_rated = graph
        .walk()
        .vertices(VertexIndex::product_by_average_rating_range(4.5..5.1))
        .collect::<Vec<_>>();
}

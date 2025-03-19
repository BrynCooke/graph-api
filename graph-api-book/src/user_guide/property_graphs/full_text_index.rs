use graph_api_derive::VertexExt;
use graph_api_lib::{Graph, Supported};

// Define a model with full-text indexes
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Article {
        title: String,

        #[index(full_text)] // Full-text index for content searching
        content: String,

        author: String,

        #[index(full_text)] // Full-text index for tags
        tags: String,
    },
    User {
        username: String,

        #[index(full_text)] // Full-text index for bio
        biography: String,

        #[index(full_text)] // Full-text index for skills
        skills: String,
    },
}

fn define_full_text_index() {}

// Example demonstrating full-text search capabilities
pub fn full_text_queries<G>(graph: G)
where
    G: Graph<Vertex = Vertex, Edge = (), SupportsVertexFullTextIndex = Supported>,
{
    // Example 1: Find articles about "rust"
    let _rust_articles = graph
        .walk()
        .vertices(VertexIndex::article_by_content("rust"))
        .collect::<Vec<_>>();

    // Example 2: Find articles tagged with "graph"
    let _graph_articles = graph
        .walk()
        .vertices(VertexIndex::article_by_tags("graph"))
        .collect::<Vec<_>>();

    // Example 3: Find users with "graph databases" mentioned in bio
    let _graph_db_users = graph
        .walk()
        .vertices(VertexIndex::user_by_biography("graph databases"))
        .collect::<Vec<_>>();

    // Example 4: Find users skilled in "systems programming"
    let _systems_programmers = graph
        .walk()
        .vertices(VertexIndex::user_by_skills("systems programming"))
        .collect::<Vec<_>>();
}

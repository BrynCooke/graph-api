use graph_api_derive::{EdgeExt, VertexExt};
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index]
        name: String,
        age: u64,
    },
    Project(Project),
    Rust,
}

#[derive(Debug, Clone)]
pub struct Project {
    name: String,
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows { since: i32 },
    Created,
    Language(Language),
}
#[derive(Debug, Clone)]
pub struct Language {
    name: String,
}
fn main() {}

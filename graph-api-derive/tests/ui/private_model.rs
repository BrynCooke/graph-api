use graph_api_derive::{EdgeExt, VertexExt};
#[derive(Debug, Clone, VertexExt)]
enum Vertex {
    Person {
        #[index(hash)]
        name: String,
        age: u64,
    },
    Project(Project),
    Rust,
}

#[derive(Debug, Clone)]
struct Project {
    name: String,
}

#[derive(Debug, Clone, EdgeExt)]
enum Edge {
    Knows { since: i32 },
    Created,
    Language(Language),
}
#[derive(Debug, Clone)]
struct Language {
    name: String,
}
fn main() {}

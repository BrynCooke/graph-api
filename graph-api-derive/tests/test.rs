use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::{Element, Index, Label};
use uuid::Uuid;

#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        non_indexed: usize,
        #[index(hash)]
        name: String,
        #[index(range)]
        age: u64,
        #[index(hash)]
        unique_id: Uuid,
        #[index(range)]
        username: String,
        #[index(full_text)]
        biography: String,
    },
    Project(Project),
    Rust,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows { since: i32 },
    Created,
    Language(Language),
}
#[derive(Debug, Clone)]
pub struct Language {
    pub name: String,
}

#[test]
fn test_label() {
    let label = Vertex::Person {
        non_indexed: 0,
        name: "".to_string(),
        age: 0,
        unique_id: Default::default(),
        username: "".to_string(),
        biography: "".to_string(),
    }
    .label();
    assert_eq!(label.name(), "Person");
    assert_eq!(label.indexes().len(), 5);
    assert_eq!(label.indexes()[0].ty(), std::any::TypeId::of::<String>());
}

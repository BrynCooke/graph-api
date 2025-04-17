use graph_api_lib::VertexReference;
use graph_api_lib::VertexSearch;
mod module {
    use graph_api_derive::{EdgeExt, VertexExt};

    #[derive(Debug, Clone, VertexExt)]
    pub(super) enum Vertex {
        Person {
            #[index(hash)]
            name: String,
            age: u64,
        },
        Project(Project),
        Rust,
    }

    #[derive(Debug, Clone)]
    pub(super) struct Project {
        name: String,
    }

    #[derive(Debug, Clone, EdgeExt)]
    pub(super) enum Edge {
        Knows { since: i32 },
        Created,
        Language(Language),
    }
    #[derive(Debug, Clone)]
    pub(super) struct Language {
        name: String,
    }
}

fn main() {}

fn test_projection(g: impl graph_api_lib::Graph<Vertex = module::Vertex, Edge = module::Edge>) {
    g.walk()
        .vertices(VertexSearch::scan())
        .filter(|p, _| p.project::<module::Person<_>>().unwrap().name() == "bob");
}

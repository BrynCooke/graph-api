use graph_api_lib::{EdgeSearch, Graph, VertexSearch};
use graph_api_test::{Edge, Person, Project, Vertex};

// Traversal with detour example
pub fn detour_traversal_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // For each person, find all their projects and collect as a list of (person, projects)
    let person_projects = graph
        .walk()
        .vertices(VertexSearch::scan().with_label(Person::label()))
        .push_context(|v, _| v.project::<Person<_>>().unwrap().name().to_string())
        .detour(|v| {
            // For each person, explore their projects
            v.edges(EdgeSearch::scan().with_label(Edge::created_label()))
                .tail()
                .map(|v, ctx| {
                    // Return person name and project name
                    (
                        ctx.to_string(),
                        v.project::<Project<_>>().unwrap().name().to_string(),
                    )
                })
        })
        .collect::<Vec<(String, String)>>();
}

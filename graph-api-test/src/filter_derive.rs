use crate::{assert_elements_eq, populate_graph, Edge, Vertex};
use crate::{EdgeExt, VertexExt};
use graph_api_lib::{EdgeSearch, Graph};

pub fn test_vertices_filter<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn, refs.julia])
        .filter_by_person(|person, _| person.name() == "Bryn")
        .collect::<Vec<T::VertexId>>();
    assert_elements_eq!(graph, collected, [refs.bryn]);

    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn, refs.graph_api])
        .filter_by_project(|project, _| project.name == "GraphApi")
        .collect::<Vec<T::VertexId>>();
    assert_elements_eq!(graph, collected, [refs.graph_api]);

    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn, refs.rust])
        .all_rust()
        .collect::<Vec<T::VertexId>>();
    assert_elements_eq!(graph, collected, [refs.rust]);
}

pub fn test_edges_filter<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .filter_by_knows(|knows, _| knows.since() >= 1999)
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, [refs.bryn_knows_julia]);

    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .all_created()
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, [refs.bryn_created_graph_api]);

    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn, refs.graph_api])
        .edges(EdgeSearch::scan().outgoing())
        .filter_by_language(|language, _| language.name == "Rust")
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, [refs.graph_api_language_rust]);
}

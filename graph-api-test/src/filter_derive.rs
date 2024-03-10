use crate::{assert_elements_eq, populate_graph, Edge, Vertex};
use crate::{EdgeExt, VertexExt};
use graph_api_lib::Graph;

pub fn test_vertices_filter<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn, refs.julia])
        .filter_by_person(|person| person.name() == "Bryn")
        .collect::<Vec<T::VertexId>>();
    assert_elements_eq!(graph, collected, [refs.bryn]);

    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn, refs.graph_api])
        .filter_by_project(|project| project.name == "GraphApi")
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
        .out_edges(None)
        .filter_by_knows(|knows| knows.since() >= 1999)
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, [refs.bryn_knows_julia]);

    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn])
        .out_edges(None)
        .all_created()
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, [refs.bryn_created_graph_api]);

    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn, refs.graph_api])
        .out_edges(None)
        .filter_by_language(|language| language.name == "Rust")
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, [refs.graph_api_language_rust]);
}

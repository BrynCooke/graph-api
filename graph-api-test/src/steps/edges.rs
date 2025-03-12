use crate::{Edge, EdgeLabel, Vertex, assert_elements_eq, assert_elements_one_of, populate_graph};
use graph_api_lib::{EdgeSearch, Graph};

pub fn test_out_edges<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .collect::<Vec<_>>();
    assert_elements_eq!(
        graph,
        collected,
        vec![refs.bryn_knows_julia, refs.bryn_created_graph_api]
    );
}

pub fn test_out_edges_limit<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing().limit(1))
        .collect::<Vec<_>>();
    assert_elements_one_of!(
        graph,
        collected,
        vec![refs.bryn_knows_julia, refs.bryn_created_graph_api]
    );
}

pub fn test_in_edges<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().incoming())
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.julia_knows_bryn]);
}

pub fn test_in_edges_limit<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().incoming().limit(1))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.julia_knows_bryn]);
}

pub fn test_all_edges<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan())
        .collect::<Vec<_>>();
    assert_elements_eq!(
        graph,
        collected,
        vec![
            refs.bryn_knows_julia,
            refs.bryn_created_graph_api,
            refs.julia_knows_bryn
        ]
    );
}

pub fn test_all_edges_limit<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().limit(1))
        .collect::<Vec<_>>();
    assert_elements_one_of!(
        graph,
        collected,
        vec![refs.bryn_knows_julia, refs.bryn_created_graph_api]
    );
}

pub fn test_out_edges_filtered<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::label(EdgeLabel::Knows).outgoing())
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn_knows_julia]);
}

pub fn test_out_edges_filtered_limit<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::label(EdgeLabel::Knows).outgoing().limit(1))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn_knows_julia]);
}

pub fn test_in_edges_filtered<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::label(EdgeLabel::Knows).incoming())
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.julia_knows_bryn]);
}

pub fn test_in_edges_filtered_limit<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::label(EdgeLabel::Knows).incoming().limit(1))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.julia_knows_bryn]);
}

pub fn test_all_edges_filtered<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::label(EdgeLabel::Knows))
        .collect::<Vec<_>>();
    assert_elements_eq!(
        graph,
        collected,
        vec![refs.bryn_knows_julia, refs.julia_knows_bryn]
    );
}

pub fn test_all_edges_filtered_limit<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::label(EdgeLabel::Knows).limit(1))
        .collect::<Vec<_>>();
    assert_elements_one_of!(
        graph,
        collected,
        vec![refs.bryn_knows_julia, refs.julia_knows_bryn]
    );
}

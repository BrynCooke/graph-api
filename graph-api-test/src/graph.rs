use crate::{populate_graph, Edge, Knows, KnowsMut, Person, PersonMut, Vertex};
use uuid::Uuid;
use graph_api_lib::{EdgeReference, EdgeReferenceMut, VertexReference, VertexReferenceMut};

pub fn test_add_vertex<Graph>(graph: &mut Graph)
where
    Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>,
{
    let vertex = graph.add_vertex(Vertex::Person {
        name: "Julia".to_string(),
        age: Default::default(),
        unique_id: Uuid::new_v4(),
        username: "".to_string(),
        biography: "".to_string(),
    });
    assert!(graph.vertex(vertex).is_some());
}

pub fn test_remove_vertex<Graph>(graph: &mut Graph)
where
    Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>,
{
    let vertex = graph.add_vertex(Vertex::Person {
        name: "Bryn".to_string(),
        age: Default::default(),
        unique_id: Uuid::new_v4(),
        username: "".to_string(),
        biography: "".to_string(),
    });
    graph.remove_vertex(vertex);
    assert!(graph.vertex(vertex).is_none());
}

pub fn test_add_edge<Graph>(graph: &mut Graph)
where
    Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>,
{
    let v1 = graph.add_vertex(Vertex::Person {
        name: "Julia".to_string(),
        age: Default::default(),
        unique_id: Uuid::new_v4(),
        username: "".to_string(),
        biography: "".to_string(),
    });
    let v2 = graph.add_vertex(Vertex::Person {
        name: "Bryn".to_string(),
        age: Default::default(),
        unique_id: Uuid::new_v4(),
        username: "".to_string(),
        biography: "".to_string(),
    });
    let edge = graph.add_edge(v1, v2, Edge::Knows { since: 2020 });
    assert!(graph.edge(edge).is_some());
}

pub fn test_remove_edge<Graph>(graph: &mut Graph)
where
    Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>,
{
    let v1 = graph.add_vertex(Vertex::Person {
        name: "Julia".to_string(),
        age: Default::default(),
        unique_id: Uuid::new_v4(),
        username: "".to_string(),
        biography: "".to_string(),
    });
    let v2 = graph.add_vertex(Vertex::Person {
        name: "Bryn".to_string(),
        age: Default::default(),
        unique_id: Uuid::new_v4(),
        username: "".to_string(),
        biography: "".to_string(),
    });
    let edge = graph.add_edge(v1, v2, Edge::Knows { since: 2020 });
    graph.remove_edge(edge);
    assert!(graph.edge(edge).is_none());
}

pub fn test_remove_vertex_with_edges<Graph>(graph: &mut Graph)
where
    Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>,
{
    // Create vertices
    let v1 = graph.add_vertex(Vertex::Person {
        name: "Julia".to_string(),
        age: Default::default(),
        unique_id: Uuid::new_v4(),
        username: "".to_string(),
        biography: "".to_string(),
    });
    let v2 = graph.add_vertex(Vertex::Person {
        name: "Bryn".to_string(),
        age: Default::default(),
        unique_id: Uuid::new_v4(),
        username: "".to_string(),
        biography: "".to_string(),
    });
    let v3 = graph.add_vertex(Vertex::Person {
        name: "Pixel".to_string(),
        age: Default::default(),
        unique_id: Uuid::new_v4(),
        username: "".to_string(),
        biography: "".to_string(),
    });

    // Create edges in different configurations
    let e1 = graph.add_edge(v1, v2, Edge::Knows { since: 2020 });
    let e2 = graph.add_edge(v2, v3, Edge::Knows { since: 2021 });
    let e3 = graph.add_edge(v3, v1, Edge::Knows { since: 2022 });

    // Remove middle vertex
    graph.remove_vertex(v2);

    // Verify v2 and its connected edges are gone
    assert!(graph.vertex(v2).is_none());
    assert!(graph.edge(e1).is_none());
    assert!(graph.edge(e2).is_none());

    // Verify remaining vertices and edge still exist
    assert!(graph.vertex(v1).is_some());
    assert!(graph.vertex(v3).is_some());
    assert!(graph.edge(e3).is_some());
}

pub fn test_mutate_vertex<Graph>(graph: &mut Graph)
where
    Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    {
        let mut person = graph.vertex_mut(refs.bryn).expect("expected vertex");
        let mut bryn = person.project_mut::<PersonMut<_, _>>().expect("expected person");
        let age = bryn.age();
        bryn.set_age(age + 1);
    }
    let person = graph.vertex(refs.bryn).expect("expected vertex");
    let bryn = person.project::<Person<_>>().expect("expected person");
    assert_eq!(bryn.age(), 46);
}


pub fn test_mutate_edge<Graph>(graph: &mut Graph)
where
    Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    {
        let mut edge = graph.edge_mut(refs.bryn_knows_julia).expect("expected edge");
        let mut knows = edge.project_mut::<KnowsMut<_, _>>().expect("expected knows");
        let since = knows.since();
        knows.set_since(since + 1);
    }
    let edge = graph.edge(refs.bryn_knows_julia).expect("expected edge");
    let knows = edge.project::<Knows<_>>().expect("expected person");
    assert_eq!(knows.since(), 2000);
}
use graph_api_lib::{EdgeReference, EdgeSearch, Graph, VertexReference, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, Vertex, VertexExt, populate_graph};

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    vertex_example(&graph);
    edge_example(&graph);
}

fn vertex_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Get the first project vertex in the graph
    let first_project = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter(|v, _| matches!(v.weight(), Vertex::Project(_)))
        .first();

    // If found, print project information
    if let Some(project_id) = first_project {
        if let Some(vertex) = graph.vertex(project_id) {
            if let Vertex::Project(project) = vertex.weight() {
                println!("Found first project: {}", project.name);
            }
        }
    } else {
        println!("No projects found");
    }
}

fn edge_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Get the first "Created" edge in the graph
    let first_created = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person() // Start with person vertices
        .edges(EdgeSearch::scan())
        .filter(|e, _| matches!(e.weight(), Edge::Created))
        .first();

    // If found, print relationship information
    if let Some(edge_id) = first_created {
        if let Some(edge) = graph.edge(edge_id) {
            let source = graph.vertex(edge.tail()).unwrap();
            let target = graph.vertex(edge.head()).unwrap();

            println!(
                "Found first 'Created' relationship: {:?} -> {:?}",
                source.id(),
                target.id()
            );
        }
    } else {
        println!("No 'Created' relationships found");
    }
}

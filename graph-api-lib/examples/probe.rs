use graph_api_lib::{EdgeReference, EdgeSearch, Graph, VertexReference, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{populate_graph, Edge, Vertex};

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);

    vertex_example(&graph);
    edge_example(&graph, refs.bryn);
}

fn vertex_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Use probe to inspect vertices during traversal
    let mut project_count = 0;

    graph
        .walk()
        .vertices(VertexSearch::scan())
        .probe(|vertex| {
            // Inspect each vertex and count projects
            if let Vertex::Project(project) = vertex.weight() {
                project_count += 1;
                println!("Found project: {}", project.name);
            }
        })
        .count();

    println!("Total projects found: {}", project_count);
}

fn edge_example<G>(graph: &G, bryn_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Use probe to analyze edges during traversal
    graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .edges(EdgeSearch::scan())
        .probe(|edge| {
            // Print information about each edge without affecting traversal
            println!(
                "Found edge: {:?} connecting {:?} to {:?}",
                edge.weight(),
                graph.vertex(edge.tail()).unwrap().id(),
                graph.vertex(edge.head()).unwrap().id()
            );
        })
        .count();
}

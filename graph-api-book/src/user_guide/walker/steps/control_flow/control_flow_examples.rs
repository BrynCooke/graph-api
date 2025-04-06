// ANCHOR: vertex_example
// Use control_flow to either skip a vertex (None), include it (Some), or stop traversal (Break)
let project = graph
    .walk()
    .vertices(VertexSearch::scan())
    .control_flow(|vertex, _| {
        if let Vertex::Project(project) = vertex.weight() {
            // If we find a project with "Graph" in the name, stop traversal
            if project.name.contains("Graph") {
                return ControlFlow::Break(Some(vertex));
            }
            // Include other project vertices
            return ControlFlow::Continue(Some(vertex));
        }
        // Skip non-project vertices
        ControlFlow::Continue(None)
    })
    .first();
// ANCHOR_END: vertex_example

// ANCHOR: edge_example
// Use control_flow to skip edges (None), include them (Some), or stop traversal (Break)
let earliest_connection = graph
    .walk()
    .vertices_by_id(vec![start_id])
    .edges(EdgeSearch::scan())
    .control_flow(|edge, _| {
        if let Edge::Knows { since } = edge.weight() {
            // If we found a connection from before 2010, stop traversal
            if *since < 2010 {
                return ControlFlow::Break(Some(edge));
            }
            // Include 'knows' edges
            return ControlFlow::Continue(Some(edge));
        }
        // Skip non-'knows' edges
        ControlFlow::Continue(None)
    })
    .first();
// ANCHOR_END: edge_example
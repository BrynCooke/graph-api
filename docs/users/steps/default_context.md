# Default Context Step

The `push_default_context` step is a specialized version of the `push_context` step that automatically stores the current element's ID and data in the context. This provides a convenient way to preserve information about elements as you traverse through the graph.

## Visual Diagram

Before push_default_context step (traversal with regular elements):
```text
  [Person A]* --- created ---> [Project X]*  
   |
  knows
   |
  [Person B]*
```

After push_default_context step (elements with default context):
```text
  [Person A]* + {vertex_id: id_a, vertex: Person{name: "A", age: 30}} --- created ---> [Project X]* + {vertex_id: id_x, vertex: Project{name: "X"}}
   |
  knows
   |
  [Person B]* + {vertex_id: id_b, vertex: Person{name: "B", age: 25}}
```

## Parameters

None

## Return Value

Returns a traversal with the same elements, but with each element's ID and data stored in its context.

## Examples

```rust
# use graph_api_test::Person;
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexIndex;
# use graph_api_test::EdgeIndex;
# use graph_api_derive::VertexExt;
# use graph_api_derive::EdgeExt;
# use uuid::Uuid;
# use graph_api_lib::Id;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use std::ops::Deref;
# use graph_api_lib::VertexSearch;
# let mut graph = SimpleGraph::new();

// Store vertices with their data for later access
let vertices_with_data = graph
    .walk()
    .vertices(VertexSearch::scan())
    .push_default_context()
    .collect::<Vec<_>>();
    
// Now we can access both the vertex and its data directly
for (vertex, ctx) in vertices_with_data {
    println!("Vertex ID: {:?}", vertex.id());
    println!("Vertex data: {:?}", ctx.vertex());
}

// Useful for preserving source information while traversing
let created_projects = graph
    .walk()
    .vertices(VertexIndex::person())
    .push_default_context() // Store person data
    .edges(EdgeIndex::created())
    .head() // Navigate to created projects
    .collect::<Vec<_>>()
    .into_iter()
    .map(|(project_vertex, ctx)| {
        // ctx.parent() contains the person data
        (project_vertex.id(), "Project creator")
    })
    .collect::<Vec<_>>();
```

## Notes

- Default context for vertices includes:
  - `vertex_id`: The ID of the vertex
  - `vertex`: The vertex data (cloned from the graph)
- Default context for edges includes:
  - `edge_id`: The ID of the edge
  - `edge`: The edge data (cloned from the graph)
- Type safety is maintained as the vertex/edge types are preserved
- This step requires that your vertex/edge types implement Clone + 'static
- More concise than manual context handling for common use cases
- Especially useful when you need to preserve information across multiple traversal steps
- Combines well with other context operations for complex data gathering
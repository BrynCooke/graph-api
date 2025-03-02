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
// Store vertices with their data for later access
let vertices_with_data = graph
    .walk()
    .vertices(VertexSearch::scan())
    .push_default_context()
    .collect::<Vec<_>>();
    
// Now we can access both the vertex ID and its data directly
for (id, ctx) in vertices_with_data {
    println!("Vertex ID: {:?}", id);
    println!("Vertex data: {:?}", ctx.vertex);
    
    // We can use the stored data without accessing the graph again
    if let Some(person) = ctx.vertex.as_any().downcast_ref::<Person>() {
        println!("Person name: {}", person.name);
    }
}

// Useful for preserving source information while traversing
let created_projects = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .push_default_context() // Store person data
    .out_edges(EdgeSearch::scan().with_label(Edge::created_label()))
    .head() // Navigate to created projects
    .collect::<Vec<_>>()
    .into_iter()
    .map(|(project_id, ctx)| {
        // ctx.parent() contains the person data
        let person_vertex = ctx.parent().vertex;
        let person = person_vertex.as_any().downcast_ref::<Person>().unwrap();
        
        (project_id, person.name.clone())
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
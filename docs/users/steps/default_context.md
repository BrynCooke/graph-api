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
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::Project;
# use graph_api_test::Person;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use std::ops::Deref;
# use uuid::Uuid;
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with some test data
# let refs = populate_graph(&mut graph);
#

// Use default context to access vertex information directly from prior in the traversal
let knows = graph
    .walk()
    .vertices_by_id(vec![refs.bryn, refs.julia])
    .push_context(|v, _| {
        if let Vertex::Person { name, .. } = v.weight() {
            name.clone()    
        }
        else {
            "Not a person".to_string()
        }
    })
    .edges(EdgeSearch::scan().outgoing())
    .all_knows()
    .head()
    .map(|v, ctx| {
        format!("{} knows {}", *ctx, v.project::< Person < _ >> ().unwrap().name())
    })
    .collect::<Vec<_>>();

// Check the results - should have 2 person descriptions
assert_eq!(knows.len(), 2);
assert!(knows.iter().any(|desc| desc.contains("Bryn knows Julia")));
assert!(knows.iter().any(|desc| desc.contains("Julia knows Bryn")));
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
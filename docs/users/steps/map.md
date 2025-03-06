# Map Step

The `map` step transforms elements in the traversal by applying a mapping function to each vertex or edge. Unlike other steps that continue the traversal chain, `map` returns an iterator that yields the transformed elements directly.

## Visual Diagram

Before map step:
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After map step with `vertex -> vertex.name()`:
```text
  "A", "B", "C", "D"
```

## Parameters

- `mapping`: A function that takes a vertex/edge reference and context, and returns a transformed value

## Return Value

Returns an iterator that yields the transformed elements. The type of the iterator items is determined by the return type of the mapping function.

## Examples

```rust
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::Project;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use uuid::Uuid;
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with some test data
# let refs = populate_graph(&mut graph);
# 
// Extract names from Person vertices
let names: Vec<String> = graph
    .walk()
    .vertices_by_id(vec![refs.bryn, refs.julia])
    .map(|vertex, _ctx| {
        match vertex.weight() {
            Vertex::Person { name, .. } => name.clone(),
            _ => "Unknown".to_string(),
        }
    })
    .collect();

assert_eq!(names.len(), 2);
assert!(names.contains(&"Bryn".to_string()));
assert!(names.contains(&"Julia".to_string()));

// Extract edge types
let edge_types: Vec<String> = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .edges(EdgeSearch::scan().outgoing())
    .map(|edge, _ctx| {
        match edge.weight() {
            Edge::Knows { .. } => "knows".to_string(),
            Edge::Created => "created".to_string(),
            _ => "other".to_string()
        }
    })
    .collect();

assert_eq!(edge_types.len(), 2);
assert!(edge_types.contains(&"knows".to_string()));
assert!(edge_types.contains(&"created".to_string()));
```

## Notes

- The `map` step is terminal - it returns an iterator, not a traversal builder
- Use `map` when you want to transform the traversal results into a different data type
- The mapping function has access to both the element and its context
- Unlike other traversal steps, map returns a standard Rust iterator
- Common uses include extracting specific properties, computing derived values, or creating DTOs
- For complex transformations, consider using `push_context` to accumulate data before mapping
# Edges Step

The `edges` step allows you to traverse to the edges in a graph. It moves the traversal position from vertices to their connected edges based on the provided search criteria.

## Visual Diagram

Before edges step (traversal position on vertices):
```text
  [Person A]* --- knows ---> [Person B] --- created ---> [Project]
   ^                                         
   |                                         
  owns                                       
   |                                         
  [Company C]                                        
```

After edges step with outgoing direction (traversal position moves to edges):
```text
  [Person A] --- knows --->* [Person B] --- created ---> [Project]
   ^                                         
   |                                         
  owns -*                                        
   |                                         
  [Company C]                                        
```

## Parameters

- `search`: An `EdgeSearch` that defines which edges to include. This can filter by label, direction, and other criteria.

## Return Value

A new walker where the traversal position is on the edges matching the search criteria.

## Examples

```rust
# use graph_api_test::Person;
# use graph_api_test::Knows;
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
# use graph_api_lib::EdgeSearch;
# let mut graph = SimpleGraph::new();
# let person_id = graph.add_vertex(Vertex::Person {
#     name: "Bryn".to_string(),
#     age: 45,
#     unique_id: Uuid::from_u128(1),
#     username: "bryn".to_string(),
#     biography: "Did some graph stuff".to_string(),
# });

// Get all edges in the graph
let all_edges = graph
    .walk()
    .vertices(VertexSearch::scan())
    .edges(EdgeSearch::scan())
    .collect::<Vec<_>>();

// Get only 'knows' edges from people
let knows_edges = graph
    .walk()
    .vertices(VertexIndex::person())
    .edges(EdgeIndex::knows())
    .collect::<Vec<_>>();
    
// Get edges in both directions with properties filter
let recent_edges = graph
    .walk()
    .vertices_by_id(vec![person_id])
    .edges(EdgeSearch::scan().bidirectional())
    .filter(|e| {
        if let Ok(knows) = e.project::<Knows<_>>() {
            knows.since() > 2020
        } else {
            false
        }
    })
    .collect::<Vec<_>>();
```

## Notes

- The edges step changes the traversal position from vertices to edges
- To get back to vertices after an edges step, use `head()` or `tail()`
- The search direction matters: `.outgoing()` finds edges where the current vertex is the source, `.incoming()` finds edges where the current vertex is the target, and `.bidirectional()` finds both
- The edges step can filter by label and other properties through the EdgeSearch parameter
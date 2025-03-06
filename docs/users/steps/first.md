# First Step

The `first` step returns the first element in a traversal and terminates the traversal. This is useful when you only need to retrieve a single element that matches your criteria.

## Visual Diagram

Before first step (with multiple elements in traversal):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After first step (returns only the first element):
```text
  [A]* --- edge1 ---> [B] --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

## Parameters

None

## Return Value

Returns an `Option` containing the ID of the first element in the traversal, or `None` if the traversal is empty.

## Examples

```rust
# use graph_api_test::Person;
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::Project;
# use graph_api_test::VertexIndex;
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

// Get the first vertex in the graph
let first_vertex = graph
    .walk()
    .vertices(VertexSearch::scan())
    .first();

// Get the first person over age 30
let first_person_over_30 = graph
    .walk()
    .vertices(VertexIndex::person())
    .filter(|person| {
        if let Ok(p) = person.project::<Person<_>>() {
            p.age() > 30
        } else {
            false
        }
    })
    .first();

// Check if a specific property exists in the graph
let has_project = graph
    .walk()
    .vertices(VertexIndex::project())
    .filter(|project_vertex| {
        if let Ok(p) = project_vertex.project::<Project<_>>() {
            p.name() == "Graph API"
        } else {
            false
        }
    })
    .first()
    .is_some();
```

## Notes

- The `first` step is a terminal operation - no further traversal steps can be added after it
- The element returned depends on the traversal order, which is not guaranteed unless using ordered indexes
- For deterministic results, consider using ordered indexes or adding a `filter` step with clear criteria
- Use `.first().is_some()` to check if at least one element matches your criteria
- More efficient than using `.limit(1).collect()` as it stops traversal after the first match
- If you need the actual element and not just the ID, use `.collect::<Vec<_>>().first()` instead
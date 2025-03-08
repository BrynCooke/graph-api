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
# use graph_api_test::populate_graph;
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::VertexIndex;
# use graph_api_test::EdgeIndex;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use graph_api_lib::VertexSearch;
# use graph_api_lib::EdgeSearch;
# 
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);

// Get the first vertex in the graph
let first_vertex = graph
    .walk()
    .vertices(VertexSearch::scan())
    .first();

// There should be at least one vertex in the graph
assert!(first_vertex.is_some());

// Get the first person in the graph
let first_person = graph
    .walk()
    .vertices(VertexIndex::person())
    .first();

// There should be at least one person in the graph
assert!(first_person.is_some());

// Get the first project in the graph
let first_project = graph
    .walk()
    .vertices(VertexIndex::project())
    .first();

// There should be at least one project in the graph
assert!(first_project.is_some());

// Get the first edge connecting bryn to something else
let first_bryn_edge = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .edges(EdgeSearch::scan().outgoing())
    .first();

// Bryn should have at least one outgoing edge
assert!(first_bryn_edge.is_some());
```

## Notes

- The `first` step is a terminal operation - no further traversal steps can be added after it
- The element returned depends on the traversal order, which is not guaranteed unless using ordered indexes
- For deterministic results, consider using ordered indexes or adding a `filter` step with clear criteria
- Use `.first().is_some()` to check if at least one element matches your criteria
- More efficient than using `.limit(1).collect()` as it stops traversal after the first match
- If you need the actual element and not just the ID, use `.collect::<Vec<_>>().first()` instead
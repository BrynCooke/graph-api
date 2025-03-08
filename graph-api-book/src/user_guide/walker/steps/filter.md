# Filter Step

The `filter` step allows you to keep only elements that match a predicate function. Elements that don't match the predicate are excluded from further traversal.

## Visual Diagram

Before filter step (all vertices in traversal):
```text
  [Person A]* --- knows ---> [Person B]* --- created ---> [Project]*
   ^                                         
   |                                         
  owns                                       
   |                                         
  [Company C]*                                        
```

After filter(is_person) step (only Person vertices remain in traversal):
```text
  [Person A]* --- knows ---> [Person B]* --- created ---> [Project]
   ^                                         
   |                                         
  owns                                       
   |                                         
  [Company C]                                        
```

## Parameters

- `predicate`: A function that takes a reference to an element and returns a boolean. Only elements for which this function returns `true` will be included in the traversal.

## Return Value

A new walker containing only the elements that matched the predicate.

## Examples

```rust
# use graph_api_test::populate_graph;
# use graph_api_test::Vertex;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::VertexIndex;
# use graph_api_test::EdgeIndex;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_lib::VertexSearch;
# use graph_api_lib::EdgeSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use std::ops::Deref;
# 
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate with test data
# let refs = populate_graph(&mut graph);

// Filter to keep only vertices with a specific type 
let people = graph
    .walk()
    .vertices(VertexSearch::scan())
    .all_person()
    .collect::<Vec<_>>();

assert_eq!(people.len(), 2); // bryn and julia

// Filter based on a property in the vertex
let projects = graph
    .walk()
    .vertices(VertexSearch::scan())
    .filter_by_project(|project, _| project.name == "GraphApi")
    .collect::<Vec<_>>();

assert_eq!(projects.len(), 1); // graph_api project

// Filter directly
let people_and_projects = graph
    .walk()
    .vertices(VertexSearch::scan())
    .filter(|v, _| {
        matches!(v.weight(), Vertex::Person{..} | Vertex::Project(_))
    })
    .collect::<Vec<_>>();

assert_eq!(people_and_projects.len(), 3); // graph_api project

// Filter edges
let filtered_edges = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .edges(EdgeSearch::scan())
    .filter_by_knows(|knows, _| knows.since() > 1997)
    .collect::<Vec<_>>();

// Check that we found the expected edges
assert!(filtered_edges.len() > 0);
```

## Notes

- The filter step does not modify the graph, only the traversal
- For complex filtering logic, consider breaking into multiple filter steps for better readability
- Use type projections when filtering to access type-specific methods and properties
- The filter is applied lazily during traversal, not when the step is added to the walker
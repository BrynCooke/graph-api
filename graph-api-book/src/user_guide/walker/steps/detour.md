# Detour Step

The `detour` step allows you to create a sub-traversal for each element in the current traversal. It's like a temporary branch in the traversal that returns to the main traversal when complete. This is powerful for exploring connected elements without losing your current position.

## Visual Diagram

Before detour step (traversal position on Person A):
```text
  [Person A]* --- knows ---> [Person B] --- created ---> [Project 1]
                                             
                                             created
                                             
                                             ↓
                                          
                                          [Project 2]
```

During detour execution (for each element, a sub-traversal is performed):
```text
  Main traversal:
  [Person A]* --- knows ---> [Person B]
  
  Sub-traversal from Person A:
  [Person A] --- knows ---> [Person B] 
                                            
   created
                                            
     ↓
                                          
  [Project 2]*
```

After detour step (traversal position returns to original elements):
```text
  [Person A]* --- knows ---> [Person B] 
                                                                                  
   created
                                            
     ↓
                                          
  [Project 2]
```

## Parameters

- `traversal_fn`: A function that takes a reference to the current element and returns a new traversal. The results of this traversal are collected in the context.

## Return Value

A walker with the same elements as before, but with the results of the sub-traversals stored in the context.

## Examples

```rust
# use graph_api_test::populate_graph;
# use graph_api_test::Vertex;
# use graph_api_test::VertexExt;
# use graph_api_test::Edge;
# use graph_api_test::EdgeExt;
# use graph_api_test::Project;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use graph_api_lib::VertexSearch;
# use graph_api_lib::EdgeSearch;
# use std::ops::Deref;
# 
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);

// Explore connected projects using detour
let bryn_project_count = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .push_context(|_, _| 0) // Start with count 0
    .detour(|waypoint| {
        // For bryn, find and count projects
        waypoint
            .edges(EdgeSearch::scan())
            .all_created()
            .push_context(|_, count| **count + 1) // Increment count
    })
    .map(|_, count| *count)
    .collect::<Vec<_>>();

// Bryn should have at least one project
assert_eq!(bryn_project_count.len(), 1);
```

## Notes

- The detour doesn't change the main traversal elements - it only adds context data
- Detours can be nested for complex traversals
- The detour function can return any walker, allowing for flexible sub-traversals
- Use `push_context` inside detours to store data from the sub-traversal
- Detours are executed eagerly for each element in the traversal

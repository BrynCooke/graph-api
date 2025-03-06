# Probe Step

The `probe` step allows you to execute a callback function for each element in the traversal without altering the traversal itself. This is useful for debugging, logging, or collecting information during a traversal.

## Visual Diagram

Before probe step:
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After probe step (unchanged, but callback executed for each *):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

## Parameters

- `callback`: A function that takes a reference to the current element being traversed (vertex or edge). For vertices, the function signature is `FnMut(&Graph::VertexReference<'_>)` and for edges, it's `FnMut(&Graph::EdgeReference<'_>)`.

## Return Value

A walker of the same type as the input with the probe operation added to the pipeline, allowing for further chaining of operations.

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

// Simple counting example with vertices
let mut vertex_count = 0;
let result = graph
    .walk()
    .vertices(VertexSearch::scan())
    .probe(|_| {
        vertex_count += 1;
        // println!("Visited vertex #{}", vertex_count); // Uncomment to see debug output
    })
    .collect::<Vec<_>>();

// We should have counted the same number of vertices as were in the result
assert_eq!(vertex_count, result.len());
assert!(vertex_count >= 4); // At least bryn, julia, graph_api, rust

// Simple counting example with edges
let mut edge_count = 0;
let result = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .edges(EdgeSearch::scan())
    .probe(|_| {
        edge_count += 1;
        // println!("Visited edge #{}", edge_count); // Uncomment to see debug output
    })
    .collect::<Vec<_>>();

// We should have counted the same number of edges as were in the result
assert_eq!(edge_count, result.len());
assert!(edge_count > 0); // Bryn should have some edges

// Multiple probe points in a chain
let mut vertex_start_count = 0;
let mut edge_count = 0;
let mut tail_count = 0;
let result = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .probe(|_| { 
        vertex_start_count += 1; 
        // println!("Starting at vertex: {}", vertex_start_count); // Uncomment to see debug output
    })
    .edges(EdgeSearch::scan())
    .probe(|_| { 
        edge_count += 1; 
        // println!("Following edge: {}", edge_count); // Uncomment to see debug output
    })
    .tail()
    .probe(|_| { 
        tail_count += 1; 
        // println!("Arriving at vertex: {}", tail_count); // Uncomment to see debug output
    })
    .collect::<Vec<_>>();

// The counts should match the number of elements at each stage
assert_eq!(vertex_start_count, 1); // Started with just bryn
assert_eq!(edge_count, result.len()); // Same number of edges as final results
assert_eq!(tail_count, result.len()); // Same number of tail vertices as edges
```

## Notes

- The `probe` step does not modify the traversal path or elements
- The callback function is executed for each element as it's traversed
- It's useful for debugging complex traversals without modifying the traversal logic
- Side effects in the callback function (like printing or collecting statistics) do not affect the traversal
- Can be used at multiple points in a traversal to monitor the flow at different stages
- Consider using `probe` instead of creating temporary variables outside the traversal for debugging purposes
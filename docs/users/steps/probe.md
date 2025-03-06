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

// Simple counting example
let mut count = 0;
let result = graph
    .walk()
    .vertices(VertexSearch::scan())
    .probe(|_| {
        count += 1;
        println!("Visited vertex #{}", count);
    })
    .collect::<Vec<_>>();

// Logging vertex details during traversal
let result = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .probe(|person| {
        if let Ok(p) = person.project::<Person<_>>() {
            println!("Visiting person: {}, age: {}", p.name(), p.age());
        }
    })
    .out_edges(EdgeSearch::scan().with_label(Edge::knows_label()))
    .probe(|edge| {
        println!("Following 'knows' relationship established in {}", 
                 edge.weight().project::<Knows<_>>().unwrap().since());
    })
    .tail()
    .collect::<Vec<_>>();
```

## Notes

- The `probe` step does not modify the traversal path or elements
- The callback function is executed for each element as it's traversed
- It's useful for debugging complex traversals without modifying the traversal logic
- Side effects in the callback function (like printing or collecting statistics) do not affect the traversal
- Can be used at multiple points in a traversal to monitor the flow at different stages
- Consider using `probe` instead of creating temporary variables outside the traversal for debugging purposes
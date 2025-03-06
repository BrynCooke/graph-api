# [Step Name] Step

Brief description of what the step does and its purpose in a graph traversal.

## Visual Diagram

Before [step] step:
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After [step] step:
```text
  [A]* --- edge1 ---> [B] --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

## Parameters

- `param1`: Description of the first parameter
- `param2`: Description of the second parameter

## Return Value

Description of what the step returns.

## Examples

```rust
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::VertexIndex;
# use graph_api_test::EdgeIndex;
# use graph_api_test::Person;
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
// Simple example
let result = graph
    .walk()
    .vertices(VertexSearch::scan())
    .[step_name](param1, param2)
    .collect::<Vec<_>>();

// More complex example showing common use case
let result = graph
    .walk()
    .vertices_by_id(vec![refs.bryn, refs.julia])
    .[step_name](|element| {
        // Example logic
    })
    .collect::<Vec<_>>();
```

## Notes

- Important note about the step
- Performance considerations
- Common pitfalls to avoid
- Related steps or patterns
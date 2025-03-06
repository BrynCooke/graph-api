# Limit Step

The `limit` step restricts a traversal to return at most a specified number of elements. This is useful for pagination, performance optimization, or when you only need a subset of results.

## Visual Diagram

Before limit step (with multiple elements in traversal):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After limit(2) step (only first 2 elements remain in traversal):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

## Parameters

- `limit`: A usize value specifying the maximum number of elements to include in the traversal

## Return Value

Returns a traversal containing at most the specified number of elements.

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

// Get at most 2 vertices from the graph
let some_vertices = graph
    .walk()
    .vertices(VertexSearch::scan())
    .limit(2)
    .collect::<Vec<_>>();

// Verify we got at most 2 vertices
assert!(some_vertices.len() <= 2);

// Get at most 3 edges
let some_edges = graph
    .walk()
    .vertices(VertexSearch::scan())
    .edges(EdgeSearch::scan())
    .limit(3)
    .collect::<Vec<_>>();

assert!(some_edges.len() <= 3);

```

## Notes

- The `limit` step is generally applied after other filtering operations
- It does not guarantee which elements will be returned, just how many
- For predictable results, combine with sorting operations or ordered indexes
- Can significantly improve performance by avoiding unnecessary traversal
- Particularly useful for large graphs where full traversal would be expensive
- If the traversal contains fewer elements than the limit, all elements are returned
- Different from `first()` which returns only a single element as an Option
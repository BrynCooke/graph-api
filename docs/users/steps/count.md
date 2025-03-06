# Count Step

The `count` step fully traverses the graph and returns the number of elements emitted by the traversal.

## Visual Diagram

Before count step (with elements in traversal):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After count step (consumed all elements, returned count 4):
```text
  [A] --- edge1 ---> [B] --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

## Parameters

None

## Return Value

Returns an integer representing the total number of elements in the traversal.

## Examples

```rust
# use graph_api_test::populate_graph;
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
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

// Count all vertices in the graph
let vertex_count = graph
    .walk()
    .vertices(VertexSearch::scan())
    .count();

assert!(vertex_count >= 4); // At least bryn, julia, graph_api, rust

// Count only Person vertices
let person_count = graph
    .walk()
    .vertices(VertexSearch::scan())
    .all_person()
    .count();

assert_eq!(person_count, 2); // bryn and julia

// Count Project vertices
let project_count = graph
    .walk()
    .vertices(VertexSearch::scan())
    .filter(|v, _| matches!(v.weight(), Vertex::Project { .. }))
    .count();

assert_eq!(project_count, 2); // graph_api and rust

// Count edges between vertices
let edge_count = graph
    .walk()
    .vertices(VertexSearch::scan())
    .edges(EdgeSearch::scan())
    .count();

assert!(edge_count > 0);

// Count created edges
let created_edge_count = graph
    .walk()
    .vertices(VertexSearch::scan())
    .edges(EdgeSearch::scan().with_label(Edge::created_label()))
    .count();

assert!(created_edge_count > 0);
```

## Notes

- The `count` step consumes the entire traversal
- This is a terminal operation - no further steps can be added after `count`
- For very large graphs, be aware that this will traverse the entire graph which may be expensive
- Consider using `limit` before `count` if you only need to check up to a certain number of elements
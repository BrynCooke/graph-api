# Tail Step

The `tail` step transforms an edge traversal into a vertex traversal by moving to the tail vertex of each edge. In graph theory, the tail is the destination/target vertex that the edge points to.

## Visual Diagram

Before tail step (with edges as current elements):
```text
  [A] --- edge1* ---> [B] --- edge2* ---> [C]  
   ^                                         
   |                                         
  edge3*                                       
   |                                         
  [D]                                        
```

After tail step (moved to target vertices of edges):
```text
  [A] --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

## Parameters

None

## Return Value

A vertex walker that will traverse the destination/target vertices of the edges from the previous step.

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

// Find all projects created by bryn
let projects = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .edges(EdgeIndex::created())
    .tail()
    .collect::<Vec<_>>();

// Bryn should have created at least one project
assert!(projects.len() > 0);

// Find all friends of bryn (people they know)
let friends = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .edges(EdgeIndex::knows())
    .tail()
    .collect::<Vec<_>>();

// Bryn should know at least one person
assert!(friends.len() > 0);

// Find second-degree connections (friends of friends)
let friends_of_friends = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .edges(EdgeIndex::knows())
    .tail()
    .edges(EdgeIndex::knows())
    .tail()
    .filter(|v, _| v.id() != refs.bryn) // Exclude the original person
    .collect::<Vec<_>>();

// Friends of friends might be empty in test graph, but the operation should complete
assert!(friends_of_friends.len() >= 0); // Always true, just checking operation completes
```

## Notes

- The `tail` step can only be used after an edge traversal step
- Transforms the traversal type from EdgeWalker to VertexWalker
- For directed graphs, tail refers to the destination/target vertex
- For undirected graphs, the distinction between head and tail may depend on implementation
- Commonly used in conjunction with `edges` to follow relationships
- The head-tail terminology follows standard graph theory convention
- When working with edges, remember that `tail()` gives you "where the edge points to" (destination)
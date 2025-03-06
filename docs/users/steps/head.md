# Head Step

The `head` step transforms an edge traversal into a vertex traversal by moving to the head vertex of each edge. In graph theory, the head is the source/origin vertex that the edge comes from.

## Visual Diagram

Before head step (with edges as current elements):
```text
  [A] --- edge1* ---> [B] --- edge2* ---> [C]  
   ^                                         
   |                                         
  edge3*                                       
   |                                         
  [D]                                        
```

After head step (moved to source vertices of edges):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

## Parameters

None

## Return Value

A vertex walker that will traverse the source/origin vertices of the edges from the previous step.

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

// Find all people who created projects
let creators = graph
    .walk()
    .vertices(VertexIndex::project())
    .edges(EdgeIndex::created().incoming())
    .head()
    .collect::<Vec<_>>();

// There should be at least one creator
assert!(creators.len() > 0);

// Find vertices connected to graph_api project 
let connected_to_graph_api = graph
    .walk()
    .vertices_by_id(vec![refs.graph_api])
    .edges(EdgeSearch::scan().incoming())
    .head()
    .collect::<Vec<_>>();

// There should be at least one vertex connected to graph_api
assert!(connected_to_graph_api.len() > 0);

// Find projects that use Rust
let rust_projects = graph
    .walk()
    .vertices_by_id(vec![refs.rust])
    .edges(EdgeSearch::scan().incoming())
    .head()
    .collect::<Vec<_>>();

// There should be at least one project using Rust
assert!(rust_projects.len() > 0);
```

## Notes

- The `head` step can only be used after an edge traversal step
- Transforms the traversal type from EdgeWalker to VertexWalker
- For directed graphs, head refers to the source/origin vertex
- For undirected graphs, the distinction between head and tail may depend on implementation
- Commonly used in conjunction with incoming edges to find vertices that point to certain destinations
- The head-tail terminology follows standard graph theory convention
- When working with edges, remember that `head()` gives you "where the edge comes from" (source)
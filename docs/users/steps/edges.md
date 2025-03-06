# Edges Step

The `edges` step allows you to traverse to the edges in a graph. It moves the traversal position from vertices to their connected edges based on the provided search criteria.

## Visual Diagram

Before edges step (traversal position on vertices):
```text
  [Person A]* --- knows ---> [Person B] --- created ---> [Project]
   ^                                         
   |                                         
  owns                                       
   |                                         
  [Company C]                                        
```

After edges step with outgoing direction (traversal position moves to edges):
```text
  [Person A] --- knows --->* [Person B] --- created ---> [Project]
   ^                                         
   |                                         
  owns -*                                        
   |                                         
  [Company C]                                        
```

## Parameters

- `search`: An `EdgeSearch` that defines which edges to include. This can filter by label, direction, and other criteria.

## Return Value

A new walker where the traversal position is on the edges matching the search criteria.

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

// Get all edges from bryn
let bryn_edges = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .edges(EdgeSearch::scan())
    .collect::<Vec<_>>();

// Bryn should have edges
assert!(bryn_edges.len() > 0);

// Get outgoing edges from bryn
let bryn_outgoing_edges = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .edges(EdgeSearch::scan().outgoing())
    .collect::<Vec<_>>();

// Bryn should have edges
assert!(bryn_outgoing_edges.len() > 0);

// Get only 'Created' edges (SupportsEdgeLabelIndex)
let bryn_created_edges = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .edges(EdgeIndex::created())
    .collect::<Vec<_>>();

// Bryn should have at least one `Created` edge 
assert!(bryn_edges.len() > 0);
    
// Get outgoing 'Created' edges (SupportsEdgeLabelIndex)
let bryn_outgoing_edges = graph
    .walk()
    .vertices_by_id(vec![refs.bryn])
    .edges(EdgeIndex::created().outgoing())
    .collect::<Vec<_>>();

// Bryn should have outgoing edges
assert!(bryn_outgoing_edges.len() > 0);

```

## Notes

- The edges step changes the traversal position from vertices to edges
- To get back to vertices after an edges step, use `head()` or `tail()`
- The search direction matters: `.outgoing()` finds edges where the current vertex is the source, `.incoming()` finds edges where the current vertex is the target, and `.bidirectional()` finds both
- The edges step can filter by label and other properties through the EdgeSearch parameter
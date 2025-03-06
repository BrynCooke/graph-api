# Vertices Step

The `vertices` step is the primary entry point for graph traversals, allowing you to select a set of vertices to start traversing from. It accepts a `VertexSearch` parameter that specifies which vertices to include in the traversal.

## Visual Diagram

Before vertices step (empty traversal):
```text
  [A] --- edge1 ---> [B] --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

After vertices step (with VertexSearch::scan()):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After vertices step (with VertexSearch::scan().with_label(Vertex::person_label())):
```text
  [Person A]* --- edge1 ---> [Project B]  --- edge2 ---> [Person C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [Person D]*                                        
```

## Parameters

- `vertex_search`: A `VertexSearch` object that defines the criteria for selecting vertices

## Return Value

Returns a traversal containing all vertices that match the search criteria.

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
# use uuid::Uuid;
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with some test data
# let refs = populate_graph(&mut graph);
# 
// Get all vertices in the graph
let all_vertices = graph
    .walk()
    .vertices(VertexSearch::scan())
    .collect::<Vec<_>>();

assert!(all_vertices.len() >= 4); // At least bryn, julia, graph_api, rust

// Get vertices with a specific label (SupportsVertexLabelIndex)
let people = graph
    .walk()
    .vertices(VertexIndex::person())
    .collect::<Vec<_>>();

assert_eq!(people.len(), 2); // bryn and julia

// Get vertices with a specific label (SupportsVertexIndex)
let people = graph
.walk()
.vertices(VertexIndex::person_by_name("Bryn"))
.collect::<Vec<_>>();

assert_eq!(people.len(), 1); // bryn

// Get vertices with a specific label (SupportsVertexRangeIndex)
let people = graph
.walk()
.vertices(VertexIndex::person_by_age_range(20..50))
.collect::<Vec<_>>();

assert_eq!(people.len(), 2); // bryn and julia

```

## Notes

- The `vertices` step is typically the first step in a traversal
- Use `VertexIndex` methods for faster access when you have appropriate indexes defined
- For more complex criteria, you can chain the `filter` step after this one
- When working with large graphs, consider using indexed properties for better performance
- This step supports all vertex search mechanisms, including label-based, index-based, and full scans
- The traversal order is not guaranteed unless you specifically use an ordered index
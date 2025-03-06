# Collect Step

The `collect` step finalizes a traversal and gathers the results into a collection. This is a terminal operation that ends the traversal and provides access to the traversed elements.

## Visual Diagram

Before collect step (with elements in traversal):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After collect step (all elements consumed and collected):
```text
  [A] --- edge1 ---> [B] --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        

  Collection: [A, B, C, D]
```

## Parameters

None - but the resulting collection type is determined by the type parameter provided to the collect call.

## Return Value

Returns a collection of the traversed elements. The exact type depends on what you're collecting into, commonly:
- `Vec<ElementId>` for simple ID collection
- `Vec<(ElementId, Context)>` when context is used
- Custom types when implementing `FromIterator`

## Examples

```rust
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use std::collections::HashSet;
# 
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);

// Collect vertex IDs into a vector
let vertex_ids = graph
    .walk()
    .vertices(VertexSearch::scan())
    .collect::<Vec<_>>();

assert!(vertex_ids.len() >= 4); // At least bryn, julia, graph_api, rust

// Collect into a HashSet
let vertex_id_set = graph
    .walk()
    .vertices(VertexSearch::scan())
    .collect::<HashSet<_>>();

assert!(vertex_id_set.len() >= 4);

// Filter and collect only Person vertices
let person_ids = graph
    .walk()
    .vertices(VertexSearch::scan())
    .all_person()
    .collect::<Vec<_>>();

assert_eq!(person_ids.len(), 2); // bryn and julia

// Collect edge IDs
let edge_ids = graph
    .walk()
    .vertices(VertexSearch::scan())
    .edges(EdgeSearch::scan())
    .collect::<Vec<_>>();

assert!(edge_ids.len() > 0); // We should have some edges
```

## Notes

- The `collect` step is a terminal operation - no further traversal steps can be added after it
- When collecting with context, use `map` first to format the data for collection
- The collect step fully consumes the traversal
- Most commonly used with `Vec<_>`, but can collect into any type that implements `FromIterator`
- Consider using `limit` before `collect` for large graphs to avoid excessive memory use
- For single-element queries, consider using `first()` instead of `collect` for efficiency
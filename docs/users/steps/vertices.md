# Vertices Step

The `vertices` step is the primary entry point for graph traversals, allowing you to select a set of vertices to start traversing from. It accepts a `VertexSearch` parameter that specifies which vertices to include in the traversal.

## Visual Diagram

Before vertices step (empty traversal):
```
  [A] --- edge1 ---> [B] --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

After vertices step (with VertexSearch::scan()):
```
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After vertices step (with VertexSearch::scan().with_label(Vertex::person_label())):
```
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
// Get all vertices in the graph
let all_vertices = graph
    .walk()
    .vertices(VertexSearch::scan())
    .collect::<Vec<_>>();

// Get vertices with a specific label
let people = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .collect::<Vec<_>>();

// Get vertices by a specific property using an index
let bryn = graph
    .walk()
    .vertices(VertexIndex::person_by_name("Bryn"))
    .collect::<Vec<_>>();

// Get vertices from an ID list
let specific_vertices = graph
    .walk()
    .vertices_by_id(vec![id1, id2, id3])
    .collect::<Vec<_>>();
```

## Notes

- The `vertices` step is typically the first step in a traversal
- Use `VertexIndex` methods for faster access when you have appropriate indexes defined
- For more complex criteria, you can chain the `filter` step after this one
- When working with large graphs, consider using indexed properties for better performance
- This step supports all vertex search mechanisms, including label-based, index-based, and full scans
- The traversal order is not guaranteed unless you specifically use an ordered index
# Iter Step

While there is no explicit `iter` step method in the walker API, the walker builders implement the `IntoIterator` trait, allowing you to convert a traversal into a standard Rust iterator with the `.into_iter()` method. This enables using standard iterator methods like `map`, `filter`, and `fold` on your graph traversal results.

## Visual Diagram

Before iter (walker traversal):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After iter (converted to standard Rust iterator):
```text
  Iterator<Item = VertexId>  or  Iterator<Item = EdgeId>
  A -> B -> C -> D
```

When using `map()`, the iterator yields references with contexts:
```text
  Iterator<Item = (reference, context)>
  [A, ctx] -> [B, ctx] -> [C, ctx] -> [D, ctx]
```

## Parameters

None (when using `.into_iter()`)

## Return Value

An iterator that yields element IDs (vertex or edge IDs, depending on the walker type).

When using the `map()` method, the iterator yields tuples of `(reference, context)` where:
- `reference` is either a vertex or edge reference depending on the walker type
- `context` contains any accumulated context data from the traversal

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

// Basic iteration to collect IDs
let vertex_ids = graph
    .walk()
    .vertices(VertexSearch::scan())
    .into_iter()
    .collect::<Vec<_>>();

// There should be at least 4 vertices in the graph
assert!(vertex_ids.len() >= 4);

```

## Notes

- Using `.into_iter()` consumes the walker and returns an iterator over element IDs
- This is the bridge between Graph API's walker system and Rust's standard iterator ecosystem
- After converting to an iterator, you can use all standard Rust iterator methods
- Prefer using walker steps for graph traversal logic, and iterator methods for post-traversal processing
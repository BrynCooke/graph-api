Writing a traversal is sometimes tricky. You can use the `dbg` step to print the elements that are traversed

Elements will be tagged with the provided tag.

```rust
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::VertexSearch;
# use graph_api_lib::EdgeSearch;
# use crate::graph_api_lib::Graph;
# let graph = SimpleGraph::<Vertex, Edge>::new();
graph.walk().vertices(VertexSearch::scan())
    .dbg("stage1") // elements are printed to the console 
    .count();
```

You can use `dbg` at multiple points in your traversal.
```rust
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::VertexSearch;
# use graph_api_lib::EdgeSearch;
# use crate::graph_api_lib::Graph;
# let graph = SimpleGraph::<Vertex, Edge>::new();
graph.walk().vertices(VertexSearch::scan())
    .dbg("stage 1") // elements are printed to the console 
    .out_edges(EdgeSearch::scan())
    .dbg("stage 2") // elements are printed to the console
    .count();
```

!!Remember!! Traversals are typically depth first.
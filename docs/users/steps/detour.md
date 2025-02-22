`detour` allows you to traverse .

```rust
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::VertexSearch;
# use graph_api_lib::EdgeSearch;
# use crate::graph_api_lib::Graph;
# let graph = SimpleGraph::<Vertex, Edge>::new();
graph.walk().vertices(VertexSearch::scan())
    .detour(|v|v.out_edges(EdgeSearch::))
    .count();
```


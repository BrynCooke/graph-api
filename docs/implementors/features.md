# Graph Features

Graphs have different feature sets, some are highly optimized for a single usecase, other are more general.

Graph API allows Graphs to indicate which features are supported via associated types on the `Graph` trait. These are
then used to help
filter which functions are available to users via where clause.

Current optional features are around indexing and clearing:

* `SupportsVertexLabelIndex`: Vertices by label
* `SupportsEdgeLabelIndex`: Edges by label
* `SupportsVertexIndex`: Vertices by value lookup
* `SupportsEdgeIndex`: Edges by value lookup
* `SupportsVertexOrderedIndex`: Vertices by range lookups
* `SupportsEdgeOrderedIndex`:  Edges by range lookups
* `SupportsVertexFullTextIndex`: Vertices by full text index
* `SupportsClear`: Clear all vertices and edges

When implementing a graph you must specify these as `Supported` or `Unsupported`.

Note that depending on your supported features you must also specify the corresponding feature when using the
`graph-api-test` crate.

See [testing](./testing.md) for more details.
/// Supports indexing of vertices by label
pub trait SupportsVertexLabelIndex: crate::Graph {}

/// Supports indexing of edges by label
pub trait SupportsEdgeLabelIndex: crate::Graph {}

/// Supports indexing of vertices by field using a hash index
pub trait SupportsVertexHashIndex: crate::Graph {}

/// Supports indexing of edges by field using a hash index
pub trait SupportsEdgeHashIndex: crate::Graph {}

/// Supports indexing of vertices by field with range queries
pub trait SupportsVertexRangeIndex: crate::Graph {}

/// Supports indexing of edges by field with range queries
pub trait SupportsEdgeRangeIndex: crate::Graph {}

/// Supports indexing of vertices by field using a full text index
pub trait SupportsVertexFullTextIndex: crate::Graph {}

/// Supports indexing of edges by adjacent vertex label
pub trait SupportsEdgeAdjacentLabelIndex: crate::Graph {}

/// Supports clearing all vertices and edges
pub trait SupportsClear: crate::Graph {
    /// Clears the graph, removing all vertices and edges
    fn clear(&mut self);
}

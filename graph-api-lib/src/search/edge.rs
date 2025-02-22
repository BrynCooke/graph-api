use crate::{Direction, Element, Supported};

/// A search to apply to edges when querying a graph.
/// This allows graph implementations to support vertex centric indexes.
/// In the future, this will be expanded to support more complex filters than just label and direction.
pub struct EdgeSearch<'search, Graph>
where
    Graph: crate::Graph,
{
    // Futureproof: edges may eventually be indexable
    _phantom: std::marker::PhantomData<&'search ()>,

    /// the label of the edges
    pub label: Option<<Graph::Edge as Element>::Label>,

    /// The required adjacent label
    pub adjacent_label: Option<<Graph::Vertex as Element>::Label>,

    /// The direction of the edge to match.
    pub direction: Direction,

    /// The maximum number of edges to return for the current vertex
    pub limit: Option<usize>,
}

impl<Graph> Clone for EdgeSearch<'_, Graph>
where
    Graph: crate::Graph,
{
    fn clone(&self) -> Self {
        EdgeSearch {
            _phantom: Default::default(),
            label: self.label,
            adjacent_label: self.adjacent_label,
            direction: self.direction,
            limit: self.limit,
        }
    }
}

impl<Graph> Default for EdgeSearch<'_, Graph>
where
    Graph: crate::Graph,
{
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
            label: None,
            adjacent_label: None,
            direction: Direction::All,
            limit: None,
        }
    }
}

impl<Graph> EdgeSearch<'_, Graph>
where
    Graph: crate::Graph,
{
    /// Edges must match the label
    pub fn scan() -> Self {
        Self::default()
    }

    /// Edges must match the label
    pub fn label(label: <Graph::Edge as Element>::Label) -> Self {
        Self {
            _phantom: Default::default(),
            label: Some(label),
            adjacent_label: None,
            direction: Direction::All,
            limit: None,
        }
    }

    /// Outgoing edges
    pub fn outgoing(mut self) -> Self {
        self.direction = Direction::Outgoing;
        self
    }



    /// Outgoing edges
    pub fn incoming(mut self) -> Self {
        self.direction = Direction::Incoming;
        self
    }

    /// The direction of the edges relative to the starting vertex
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// The maximum number of edges to return relative to the starting vertex.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Adjacent vertex label must match
    pub fn adjacent_labelled(mut self, adjacent_label: <Graph::Vertex as Element>::Label) -> Self
    where
        Graph: crate::Graph<SupportsEdgeAdjacentLabelIndex = Supported>,
    {
        self.adjacent_label = Some(adjacent_label);
        self
    }
}

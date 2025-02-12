use crate::{Direction, EdgeReference, Element};

/// A search to apply to edges when querying a graph.
/// This allows graph implementations to support vertex centric indexes.
/// In the future, this will be expanded to support more complex filters than just label and direction.
pub struct EdgeSearch<'a, Graph>
where
    Graph: crate::Graph,
{
    _phantom: std::marker::PhantomData<&'a Graph>,

    /// the label of the edges
    pub label: Option<<Graph::Edge as Element>::Label>,

    /// The required adjacent label
    pub adjacent_label: Option<<Graph::Vertex as Element>::Label>,

    /// The direction of the edge to match.
    pub direction: Option<Direction>,

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
            direction: None,
            limit: None,
        }
    }
}

impl<Graph> EdgeSearch<'_, Graph>
where
    Graph: crate::Graph,
{
    pub fn new() -> Self {
        Self::default()
    }

    /// Edges must match the label
    pub fn labelled(mut self, label: <Graph::Edge as Element>::Label) -> Self {
        self.label = Some(label);
        self
    }

    /// The direction of the edges relative to the starting vertex
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    /// The maximum number of edges to return relative to the starting vertex.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn evaluate<'graph, T: EdgeReference<'graph, Graph>>(
        &self,
        current: Graph::VertexId,
        edge_reference: &T,
    ) -> bool
    where
        Graph: crate::Graph,
    {
        match self.direction {
            Some(Direction::All)
                if edge_reference.head() != current && edge_reference.tail() != current =>
            {
                return false
            }
            Some(Direction::Incoming) if edge_reference.head() != current => return false,
            Some(Direction::Outgoing) => {
                if edge_reference.tail() != current {
                    return false;
                }
            }
            _ => {}
        }
        if let Some(label) = &self.label {
            let element_label = edge_reference.weight().label();
            if element_label != *label {
                return false;
            }
        }

        true
    }
}

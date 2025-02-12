use crate::{Element, ValueOrRange};

/// A search to apply to vertices when querying a graph.
#[derive(Clone)]
pub struct VertexSearch<'a, Graph>
where
    Graph: crate::Graph,
{
    pub label: Option<<Graph::Vertex as Element>::Label>,
    pub index: Option<(<Graph::Vertex as Element>::Index, ValueOrRange<'a>)>,
}

impl<Graph> Default for VertexSearch<'_, Graph>
where
    Graph: crate::Graph,
{
    fn default() -> Self {
        Self {
            label: None,
            index: None,
        }
    }
}

impl<'a, Graph> VertexSearch<'a, Graph>
where
    Graph: crate::Graph,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn labelled(mut self, label: <Graph::Vertex as Element>::Label) -> Self {
        self.label = Some(label);
        self
    }

    /// Search for vertices with a field value that matches a given value.
    /// The index parameter is the index of the field to search on.
    /// The value parameter is the field value to search for.
    /// This method can only be used if the graph supports vertex field search.
    /// The method returns a reference to the updated search object.
    /// # Arguments
    /// * index: The index of the field to search on.
    /// * value: The field value to
    pub fn indexed<V>(mut self, index: <Graph::Vertex as Element>::Index, value: V) -> Self
    where
        V: 'a + Into<ValueOrRange<'a>>,
    {
        self.index = Some((index, value.into()));
        self
    }
}

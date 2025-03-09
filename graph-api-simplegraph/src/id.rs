use crate::SimpleGraph;
use graph_api_lib::{Element, ElementId};
use std::fmt::{Display, Formatter};

/// Simple vertex identifier using direct numeric values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct VertexId {
    label: u16,
    vertex_id: u32,
}

impl VertexId {
    /// Creates a new VertexId
    ///
    /// # Arguments
    /// * `label_id` - The label identifier
    /// * `vertex_id` - The unique vertex identifier
    pub fn new(label: u16, vertex_id: u32) -> Self {
        Self { label, vertex_id }
    }

    pub fn label(&self) -> u16 {
        self.label
    }

    pub fn vertex(&self) -> u32 {
        self.vertex_id
    }
}

/// Simple edge identifier using direct numeric values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct EdgeId {
    label: u16,
    edge_id: u32,
    head: VertexId,
    tail: VertexId,
}

impl EdgeId {
    pub fn new(label: u16, edge_id: u32, tail: VertexId, head: VertexId) -> Self {
        Self {
            label,
            edge_id,
            head,
            tail,
        }
    }

    pub fn label(&self) -> u16 {
        self.label
    }

    pub fn edge(&self) -> u32 {
        self.edge_id
    }

    pub(crate) fn head(&self) -> VertexId {
        self.head
    }

    pub(crate) fn tail(&self) -> VertexId {
        self.tail
    }
}

impl<Vertex, Edge> From<VertexId> for ElementId<SimpleGraph<Vertex, Edge>>
where
    Vertex: Element,
    Edge: Element,
{
    fn from(val: VertexId) -> Self {
        ElementId::Vertex(val)
    }
}

impl<Vertex, Edge> From<EdgeId> for ElementId<SimpleGraph<Vertex, Edge>>
where
    Vertex: Element,
    Edge: Element,
{
    fn from(val: EdgeId) -> Self {
        ElementId::Edge(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_id() {
        let label_id = 42u16;
        let vertex_id = 12345u32;
        let vid = VertexId::new(label_id, vertex_id);

        assert_eq!(vid.label(), label_id);
        assert_eq!(vid.vertex(), { vertex_id });
    }

    #[test]
    fn test_edge_id() {
        let label_id = 24u16;
        let edge_id = 67890u32;
        let head = VertexId::new(1, 100);
        let tail = VertexId::new(2, 200);

        let eid = EdgeId::new(label_id, edge_id, tail, head);

        assert_eq!(eid.label(), label_id);
        assert_eq!(eid.edge(), edge_id);
        assert_eq!(eid.head().vertex(), head.vertex());
        assert_eq!(eid.tail().vertex(), tail.vertex());
    }

    #[test]
    fn test_id_conversion() {
        let vid = VertexId::new(1, 100);
        let eid = EdgeId::new(2, 200, vid, VertexId::new(3, 300));

        // Using SimpleGraph type for ElementId parameter
        type TestGraph = SimpleGraph<(), ()>;

        let id1: ElementId<TestGraph> = vid.into();
        let id2: ElementId<TestGraph> = eid.into();

        match id1 {
            ElementId::Vertex(v) => assert_eq!(v, vid),
            _ => panic!("Expected Vertex ID"),
        }

        match id2 {
            ElementId::Edge(e) => assert_eq!(e, eid),
            _ => panic!("Expected Edge ID"),
        }
    }
}

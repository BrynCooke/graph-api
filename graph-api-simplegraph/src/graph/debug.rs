use crate::SimpleGraph;
use graph_api_lib::{Element, Label};
use std::fmt::{Debug, Formatter};

impl<Vertex, Edge> Debug for SimpleGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("SimpleGraph");
        debug_struct.field("vertices", &self.vertices.len());
        debug_struct.field("edges", &self.edges.len());
        debug_struct.field("labels", &<Vertex::Label as Label>::variants());
        debug_struct.finish()
    }
}

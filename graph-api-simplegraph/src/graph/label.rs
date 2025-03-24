use crate::index::VertexIndexStorage;
use crate::tombstone_vec::TombstoneVec;
use crate::{EdgeId, VertexId};
use graph_api_lib::{Direction, Element, Index, Label};
use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::ops::RangeInclusive;

#[derive(Default)]
pub(crate) struct LabelledVertices<Vertex, Edge> {
    vertices: TombstoneVec<VertexStorage<Vertex>>,
    _phantom: PhantomData<Edge>,
}

pub(crate) struct VertexStorage<Vertex> {
    // Maps vertex ID to list of outgoing edge IDs
    pub(crate) adjacency_list: BTreeSet<Adjacency>,
    // The vertex itself
    pub(crate) weight: Vertex,
}

/// An adjacency represents a unidirectional edge.
/// The fields are range to minimize the number of distinct ranges that
/// need to be covered for the most common traversal.
/// To do this direction is first as it has the lowest cardinality.
/// Then edge label, as users will likely only have a single edge type that they want to traverse.
/// Lastly the adjacent vertex label.
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
pub(crate) struct Adjacency {
    pub(crate) direction: Direction,
    pub(crate) edge_label: u16,
    pub(crate) vertex_label: u16,
    pub(crate) edge_id: u32,
    pub(crate) vertex_id: u32,
}

impl Adjacency {}

impl Adjacency {
    pub(crate) fn reversed(&self, vertex: VertexId) -> Adjacency {
        Adjacency {
            direction: self.direction.reverse(),
            edge_label: self.edge_label,
            vertex_label: vertex.label(),
            edge_id: self.edge_id,
            vertex_id: vertex.vertex(),
        }
    }

    pub(crate) fn outgoing(edge: &EdgeId) -> Adjacency {
        Adjacency {
            direction: Direction::Outgoing,
            edge_label: edge.label(),
            vertex_label: edge.head().label(),
            edge_id: edge.edge(),
            vertex_id: edge.head().vertex(),
        }
    }

    pub(crate) fn incoming(edge: &EdgeId) -> Adjacency {
        Adjacency {
            direction: Direction::Incoming,
            edge_label: edge.label(),
            vertex_label: edge.tail().label(),
            edge_id: edge.edge(),
            vertex_id: edge.tail().vertex(),
        }
    }

    pub(crate) fn range(
        direction: Option<Direction>,
        edge_label: Option<u16>,
        vertex_label: Option<u16>,
    ) -> RangeInclusive<Adjacency> {
        Adjacency {
            direction: direction.unwrap_or(Direction::Outgoing),
            edge_label: edge_label.unwrap_or(0),
            vertex_label: vertex_label.unwrap_or(0),
            edge_id: 0,
            vertex_id: 0,
        }..=Adjacency {
            direction: direction.unwrap_or(Direction::Incoming),
            edge_label: edge_label.unwrap_or(u16::MAX),
            vertex_label: vertex_label.unwrap_or(u16::MAX),
            edge_id: u32::MAX,
            vertex_id: u32::MAX,
        }
    }
}

impl<Vertex> VertexStorage<Vertex> {
    pub(crate) fn new(weight: Vertex) -> Self {
        Self {
            adjacency_list: BTreeSet::new(),
            weight,
        }
    }
}

impl<Vertex, Edge> LabelledVertices<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    pub(crate) fn new() -> Self {
        Self {
            vertices: TombstoneVec::new(),
            _phantom: Default::default(),
        }
    }

    pub(crate) fn get(&self, vertex_id: u32) -> Option<&Vertex> {
        self.vertices.get(vertex_id as usize).map(|v| &v.weight)
    }

    pub(crate) fn get_mut(&mut self, vertex_id: u32) -> Option<&mut Vertex> {
        self.vertices
            .get_mut(vertex_id as usize)
            .map(|v| &mut v.weight)
    }

    pub(crate) fn add(&mut self, vertex: Vertex, indexes: &mut [VertexIndexStorage]) -> u32 {
        let label = vertex.label();
        let vertex_id = self.vertices.push(VertexStorage::new(vertex)) as u32;
        let storage = self
            .vertices
            .get(vertex_id as usize)
            .expect("we just inserted the vertex, qed");
        let weight = &storage.weight;

        for index in label.indexes() {
            if let Some(value) = weight.value(index) {
                let index_storage = &mut indexes[index.ordinal()];
                index_storage.insert(value, vertex_id, index);
            }
        }
        vertex_id
    }

    pub(crate) fn add_adjacency(&mut self, vertex_id: u32, adjacency: Adjacency) {
        if let Some(vertex) = self.vertices.get_mut(vertex_id as usize) {
            vertex.adjacency_list.insert(adjacency);
        }
    }

    pub(crate) fn remove(
        &mut self,
        vertex_id: u32,
        indexes: &mut [VertexIndexStorage],
    ) -> Option<VertexStorage<Vertex>> {
        // Get the vertex before removing it so we can clean up indexes
        if let Some(vertex) = self.vertices.get(vertex_id as usize) {
            let label = vertex.weight.label();
            // Remove from all indexes first
            for index in label.indexes() {
                if let Some(value) = vertex.weight.value(index) {
                    let index_storage = &mut indexes[index.ordinal()];
                    index_storage.remove(&value, vertex_id, index);
                }
            }
            // Then remove the vertex itself
            self.vertices.remove(vertex_id as usize)
        } else {
            None
        }
    }

    pub(crate) fn remove_adjacency(&mut self, vertex_id: u32, adjacency: &Adjacency) {
        if let Some(vertex) = self.vertices.get_mut(vertex_id as usize) {
            vertex.adjacency_list.remove(adjacency);
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.vertices.index_iter().map(|idx| idx as u32)
    }

    pub(crate) fn clear(&mut self) {
        self.vertices.clear();
    }
}

impl<Vertex, Edge> std::ops::Index<u32> for LabelledVertices<Vertex, Edge> {
    type Output = VertexStorage<Vertex>;

    fn index(&self, index: u32) -> &Self::Output {
        &self.vertices[index as usize]
    }
}

impl<Vertex, Edge> std::ops::IndexMut<u32> for LabelledVertices<Vertex, Edge> {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.vertices[index as usize]
    }
}

#[derive(Default)]
pub(crate) struct LabelledEdges<Edge> {
    // Stores edge data for this label
    pub(crate) edges: TombstoneVec<Edge>,
}

impl<Edge> LabelledEdges<Edge>
where
    Edge: Element,
{
    pub fn new() -> Self {
        Self {
            edges: TombstoneVec::new(),
        }
    }

    pub(crate) fn add(&mut self, edge: Edge) -> u32 {
        self.edges.push(edge) as u32
    }

    pub(crate) fn remove(&mut self, edge_id: u32) -> Option<Edge> {
        self.edges.remove(edge_id as usize)
    }

    pub(crate) fn clear(&mut self) {
        self.edges.clear();
    }

    pub(crate) fn get(&self, edge_id: u32) -> Option<&Edge> {
        self.edges.get(edge_id as usize)
    }

    pub(crate) fn get_mut(&mut self, edge_id: u32) -> Option<&mut Edge> {
        self.edges.get_mut(edge_id as usize)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_insert_and_retrieve_adjacencies() {
        let mut adjacencies = BTreeSet::new();

        let adj1 = Adjacency {
            direction: Direction::Outgoing,
            edge_label: 1,
            vertex_label: 1,
            edge_id: 1,
            vertex_id: 1,
        };

        let adj2 = Adjacency {
            direction: Direction::Outgoing,
            edge_label: 2,
            vertex_label: 2,
            edge_id: 2,
            vertex_id: 2,
        };

        let adj3 = Adjacency {
            direction: Direction::Incoming,
            edge_label: 3,
            vertex_label: 3,
            edge_id: 3,
            vertex_id: 3,
        };

        adjacencies.insert(adj1.clone());
        adjacencies.insert(adj2.clone());
        adjacencies.insert(adj3.clone());

        let range = Adjacency::range(Some(Direction::Outgoing), None, None);
        let retrieved: Vec<_> = adjacencies.range(range).cloned().collect();

        assert_eq!(retrieved, vec![adj1, adj2]);
    }

    #[test]
    fn test_empty_range() {
        let adjacencies: BTreeSet<Adjacency> = BTreeSet::new();
        let range = Adjacency::range(Some(Direction::Outgoing), None, None);
        let retrieved: Vec<_> = adjacencies.range(range).cloned().collect();

        assert!(retrieved.is_empty());
    }

    #[test]
    fn test_retrieve_specific_edge_label() {
        let mut adjacencies = BTreeSet::new();

        let adj1 = Adjacency {
            direction: Direction::Outgoing,
            edge_label: 1,
            vertex_label: 1,
            edge_id: 1,
            vertex_id: 1,
        };

        let adj2 = Adjacency {
            direction: Direction::Outgoing,
            edge_label: 2,
            vertex_label: 2,
            edge_id: 2,
            vertex_id: 2,
        };

        let adj3 = Adjacency {
            direction: Direction::Incoming,
            edge_label: 3,
            vertex_label: 3,
            edge_id: 3,
            vertex_id: 3,
        };

        adjacencies.insert(adj1.clone());
        adjacencies.insert(adj2.clone());
        adjacencies.insert(adj3.clone());

        let range = Adjacency::range(Some(Direction::Outgoing), None, None);
        let retrieved: Vec<_> = adjacencies.range(range).cloned().collect();
        assert_eq!(retrieved, vec![adj1.clone(), adj2.clone()]);

        // Test range with specific edge label
        let range = Adjacency::range(Some(Direction::Outgoing), Some(2), None);
        let retrieved: Vec<_> = adjacencies.range(range).cloned().collect();
        assert_eq!(retrieved, vec![adj2.clone()]);

        // Test range with specific edge and vertex label
        let range = Adjacency::range(Some(Direction::Outgoing), Some(2), Some(2));
        let retrieved: Vec<_> = adjacencies.range(range).cloned().collect();
        assert_eq!(retrieved, vec![adj2.clone()]);

        // Test full range
        let range = Adjacency::range(None, None, None);
        let retrieved: Vec<_> = adjacencies.range(range).cloned().collect();
        assert_eq!(retrieved, vec![adj1.clone(), adj2.clone(), adj3.clone()]);
    }
}

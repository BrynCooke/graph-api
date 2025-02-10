mod debug;
mod iter;
mod label;

use crate::graph::iter::RangeOrNoneIterator;
use crate::graph::label::{Adjacency, LabelledEdges, LabelledVertices, VertexStorage};
use crate::id::VertexId;
use crate::index::VertexIndexStorage;
use crate::EdgeId;
use graph_api_lib::{
    Direction, EdgeSearch, Element, Error, Graph, Index, Label, Project, ProjectMut, Supported,
    Value, ValueOrRange,
};
use smallbox::space::S8;
use smallbox::{smallbox, SmallBox};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

/// A graph that is backed by a simple in-memory data structure.
pub struct SimpleGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    /// The list of labels. We know this number up-front. So this is a regular vec.
    vertices: Vec<LabelledVertices<Vertex, Edge>>,
    indexes: Vec<VertexIndexStorage>,
    edges: Vec<LabelledEdges<Edge>>,
}

#[derive(Debug)]
pub struct VertexReference<'graph, Graph>
where
    Graph: graph_api_lib::Graph,
{
    id: Graph::VertexId,
    weight: &'graph Graph::Vertex,
}

impl<'graph, Graph> graph_api_lib::VertexReference<'graph, Graph> for VertexReference<'graph, Graph>
where
    Graph: graph_api_lib::Graph,
{
    fn id(&self) -> Graph::VertexId {
        self.id
    }

    fn weight(&self) -> &Graph::Vertex {
        self.weight
    }

    fn project<
        'reference,
        T: graph_api_lib::Project<'reference, <Graph as graph_api_lib::Graph>::Vertex>,
    >(
        &'reference self,
    ) -> Option<T> {
        graph_api_lib::Project::project(self.weight)
    }
}

pub struct VertexReferenceMut<'graph, Graph>
where
    Graph: graph_api_lib::Graph,
{
    indexes: &'graph mut Vec<VertexIndexStorage>,
    id: Graph::VertexId,
    weight: &'graph mut Graph::Vertex,
}

impl<Graph> Debug for VertexReferenceMut<'_, Graph>
where
    Graph: graph_api_lib::Graph,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexReferenceMut")
            .field("id", &self.id)
            .field("weight", &self.weight)
            .finish()
    }
}

impl<'graph, Graph> graph_api_lib::VertexReference<'graph, Graph>
    for VertexReferenceMut<'graph, Graph>
where
    Graph: graph_api_lib::Graph,
{
    fn id(&self) -> Graph::VertexId {
        self.id
    }

    fn weight(&self) -> &Graph::Vertex {
        self.weight
    }

    fn project<
        'reference,
        T: graph_api_lib::Project<'reference, <Graph as graph_api_lib::Graph>::Vertex>,
    >(
        &'reference self,
    ) -> Option<T> {
        graph_api_lib::Project::project(self.weight)
    }
}

impl<'graph, Graph> graph_api_lib::VertexReferenceMut<'graph, Graph>
    for VertexReferenceMut<'graph, Graph>
where
    Graph: graph_api_lib::Graph<VertexId = VertexId> + 'graph,
{
    type MutationListener<'reference> = VertexMutationListener<'reference, Graph::Vertex>;

    fn weight_mut(&mut self) -> &mut Graph::Vertex {
        self.weight
    }

    fn project_mut<
        'reference,
        T: graph_api_lib::ProjectMut<
            'reference,
            <Graph as graph_api_lib::Graph>::Vertex,
            Self::MutationListener<'reference>,
        >,
    >(
        &'reference mut self,
    ) -> Option<T> {
        graph_api_lib::ProjectMut::project_mut(
            self.weight,
            VertexMutationListener {
                phantom_data: Default::default(),
                indexes: self.indexes,
                id: self.id.vertex(),
            },
        )
    }
}

pub struct VertexMutationListener<'reference, Element> {
    phantom_data: PhantomData<Element>,
    indexes: &'reference mut Vec<VertexIndexStorage>,
    id: u32,
}

impl<'reference, Element> graph_api_lib::MutationListener<'reference, Element>
    for VertexMutationListener<'reference, Element>
where
    Element: graph_api_lib::Element,
{
    fn update(&mut self, index: Element::Index, before: Value, after: Value) {
        let actual_index = &mut self.indexes[index.ordinal()];
        actual_index.remove(&before, self.id, &index);
        actual_index.insert(after, self.id, &index);
    }
}

#[derive(Debug)]
pub struct EdgeReference<'a, Graph>
where
    Graph: graph_api_lib::Graph,
{
    id: EdgeId,
    weight: &'a Graph::Edge,
}

impl<'a, Graph> graph_api_lib::EdgeReference<'a, Graph> for EdgeReference<'a, Graph>
where
    Graph: graph_api_lib::Graph<VertexId = VertexId, EdgeId = EdgeId>,
{
    fn id(&self) -> Graph::EdgeId {
        self.id
    }

    fn tail(&self) -> Graph::VertexId {
        self.id.tail()
    }

    fn head(&self) -> Graph::VertexId {
        self.id.head()
    }

    fn weight(&self) -> &Graph::Edge {
        self.weight
    }

    fn project<'reference, T: Project<'reference, <Graph as graph_api_lib::Graph>::Edge>>(
        &'reference self,
    ) -> Option<T> {
        graph_api_lib::Project::project(self.weight)
    }
}

#[derive(Debug)]
pub struct EdgeReferenceMut<'a, Graph>
where
    Graph: graph_api_lib::Graph,
{
    id: Graph::EdgeId,
    tail: Graph::VertexId,
    head: Graph::VertexId,
    weight: &'a mut Graph::Edge,
}

impl<'a, Graph> graph_api_lib::EdgeReference<'a, Graph> for EdgeReferenceMut<'a, Graph>
where
    Graph: graph_api_lib::Graph,
{
    fn id(&self) -> Graph::EdgeId {
        self.id
    }

    fn tail(&self) -> Graph::VertexId {
        self.tail
    }

    fn head(&self) -> Graph::VertexId {
        self.head
    }

    fn weight(&self) -> &Graph::Edge {
        self.weight
    }

    fn project<'reference, T: Project<'reference, <Graph as graph_api_lib::Graph>::Edge>>(
        &'reference self,
    ) -> Option<T> {
        graph_api_lib::Project::project(self.weight)
    }
}

impl<'a, Graph> graph_api_lib::EdgeReferenceMut<'a, Graph> for EdgeReferenceMut<'a, Graph>
where
    Graph: graph_api_lib::Graph,
{
    type MutationListener<'reference> = ();

    fn weight_mut(&mut self) -> &mut Graph::Edge {
        self.weight
    }

    fn project_mut<
        'reference,
        T: ProjectMut<
            'reference,
            <Graph as graph_api_lib::Graph>::Edge,
            Self::MutationListener<'reference>,
        >,
    >(
        &'reference mut self,
    ) -> Option<T> {
        graph_api_lib::ProjectMut::project_mut(self.weight, ())
    }
}

pub struct VertexIterator<'graph, Graph>
where
    Graph: graph_api_lib::Graph + 'graph,
{
    vertices: &'graph [LabelledVertices<Graph::Vertex, Graph::Edge>], // All vertex groups
    iter: SmallBox<dyn Iterator<Item = VertexId> + 'graph, S8>, // Indexed iterator over vertices in the current group
}

impl<'a, Graph> Iterator for VertexIterator<'a, Graph>
where
    Graph: graph_api_lib::Graph<VertexId = VertexId> + 'a,
{
    type Item = VertexReference<'a, Graph>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.iter.next() {
            Some(VertexReference {
                weight: &self.vertices[id.label() as usize][id.vertex()].weight,
                id,
            })
        } else {
            None
        }
    }
}

type RangeIter = dyn Iterator<Item = (Option<Direction>, Option<u16>, Option<u16>)>;
type BoxedRangeIter = SmallBox<RangeIter, S8>;

// The edge iterator lazily creates ranges when iterating over the adjacency list
pub struct EdgeIterator<'a, Graph>
where
    Graph: graph_api_lib::Graph,
{
    vertex: VertexId,
    vertex_storage: &'a VertexStorage<Graph::Vertex>,
    edges: &'a [LabelledEdges<Graph::Edge>],
    range_iter: BoxedRangeIter,
    current_iter: Option<std::collections::btree_set::Range<'a, Adjacency>>,
}

impl<'a, Graph> Iterator for EdgeIterator<'a, Graph>
where
    Graph: graph_api_lib::Graph<VertexId = crate::id::VertexId> + 'a,
{
    type Item = EdgeReference<'a, Graph>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_iter.is_none() {
                if let Some((direction, label, adjacent_label)) = self.range_iter.next() {
                    self.current_iter =
                        Some(self.vertex_storage.adjacency_list.range(Adjacency::range(
                            direction,
                            label,
                            adjacent_label,
                        )));
                }
            }
            if let Some(iter) = &mut self.current_iter {
                if let Some(adjacency) = iter.next() {
                    match adjacency.direction {
                        Direction::Outgoing => {
                            return Some(EdgeReference {
                                id: EdgeId::new(
                                    adjacency.edge_label,
                                    adjacency.edge_id,
                                    self.vertex,
                                    VertexId::new(adjacency.vertex_label, adjacency.vertex_id),
                                ),
                                weight: &self.edges[adjacency.edge_label as usize].edges
                                    [adjacency.edge_id as usize],
                            });
                        }
                        Direction::Incoming => {
                            return Some(EdgeReference {
                                id: EdgeId::new(
                                    adjacency.edge_label,
                                    adjacency.edge_id,
                                    VertexId::new(adjacency.vertex_label, adjacency.vertex_id),
                                    self.vertex,
                                ),
                                weight: &self.edges[adjacency.edge_label as usize].edges
                                    [adjacency.edge_id as usize],
                            });
                        }
                        _ => {
                            unreachable!()
                        }
                    };
                }
            } else {
                return None;
            }
            self.current_iter = None;
        }
    }
}

impl<Vertex, Edge> Default for SimpleGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Vertex, Edge> SimpleGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    pub fn new() -> Self {
        Self {
            vertices: (0..Vertex::Label::variants().len())
                .map(|_i| LabelledVertices::new())
                .collect(),
            indexes: Vertex::Index::variants().iter().map(|i| i.into()).collect(),
            edges: (0..Edge::Label::variants().len())
                .map(|_i| LabelledEdges::new())
                .collect(),
        }
    }
}

impl<Vertex, Edge> Graph for SimpleGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element,
{
    type SupportsVertexLabelIndex = Supported;
    type SupportsEdgeLabelIndex = Supported;
    type SupportsVertexIndex = Supported;
    type SupportsEdgeIndex = Supported;
    type SupportsVertexOrderedIndex = Supported;
    type SupportsEdgeOrderedIndex = Supported;
    type SupportsVertexFullTextIndex = Supported;
    type Vertex = Vertex;
    type Edge = Edge;
    type VertexId = VertexId;
    type EdgeId = EdgeId;
    type VertexReference<'graph>
        = VertexReference<'graph, Self>
    where
        Self: 'graph;
    type VertexReferenceMut<'graph>
        = VertexReferenceMut<'graph, Self>
    where
        Self: 'graph;
    type EdgeReference<'graph>
        = EdgeReference<'graph, Self>
    where
        Self: 'graph;
    type EdgeReferenceMut<'graph>
        = EdgeReferenceMut<'graph, Self>
    where
        Self: 'graph;
    type EdgeIter<'graph>
        = EdgeIterator<'graph, Self>
    where
        Self: 'graph;
    type VertexIter<'graph>
        = VertexIterator<'graph, Self>
    where
        Self: 'graph;

    fn add_vertex(&mut self, vertex: Self::Vertex) -> Self::VertexId {
        // Get the label index from the vertex
        let label_idx = vertex.label().ordinal();

        // Get the corresponding LabelledVertices for this label
        let labelled_vertices = &mut self.vertices[label_idx];

        // Add the vertex to the label-specific storage and get its index
        let vertex_idx = labelled_vertices.add(vertex, &mut self.indexes);

        VertexId::new(label_idx as u16, vertex_idx)
    }

    fn add_edge(
        &mut self,
        from: Self::VertexId,
        to: Self::VertexId,
        edge: Self::Edge,
    ) -> Self::EdgeId {
        // 1. Get the label index from the edge
        let label_idx = edge.label().ordinal();

        // 2. Get the corresponding LabelledEdges for this label
        let labelled_edges = &mut self.edges[label_idx];

        // 3. Add the edge to the label-specific storage and get its index
        let edge_idx = labelled_edges.add(edge);

        let edge_id = EdgeId::new(label_idx as u16, edge_idx, from, to);

        // Add the edges to the adjacency lists for the vertices.
        let tail_vertex_label = &mut self.vertices[from.label() as usize];
        tail_vertex_label.add_adjacency(from.vertex(), Adjacency::outgoing(&edge_id));
        let head_vertex_label = &mut self.vertices[to.label() as usize];
        head_vertex_label.add_adjacency(to.vertex(), Adjacency::incoming(&edge_id));

        edge_id
    }

    fn remove_vertex(&mut self, id: Self::VertexId) -> Option<Self::Vertex> {
        let label_idx = id.label();
        let vertex_idx = id.vertex();

        // Get the corresponding LabelledVertices for this label
        let labelled_vertices = &mut self.vertices[label_idx as usize];
        // Remove the vertex and return it
        if let Some(vertex_storage) = labelled_vertices.remove(vertex_idx, &mut self.indexes) {
            // Remove the edges from the adjacency lists for the vertices.
            for adjacency in &vertex_storage.adjacency_list {
                let vertex_label = &mut self.vertices[adjacency.vertex_label as usize];
                vertex_label.remove_adjacency(adjacency.vertex_id, &adjacency.reversed(id));
                self.edges[adjacency.edge_label as usize].remove(adjacency.edge_id);
            }
            return Some(vertex_storage.weight);
        }
        None
    }

    fn remove_edge(&mut self, edge: Self::EdgeId) -> Option<Self::Edge> {
        let label_idx = edge.label() as usize;
        let edge_idx = edge.edge();

        // Get the corresponding LabelledEdges for this label
        let labelled_edges = &mut self.edges[label_idx];

        // Remove the edge from both vertex adjacency lists
        let tail_vertices = &mut self.vertices[edge.tail().label() as usize];
        tail_vertices.remove_adjacency(edge.tail().vertex(), &Adjacency::outgoing(&edge));

        let head_vertices = &mut self.vertices[edge.head().label() as usize];
        head_vertices.remove_adjacency(edge.head().vertex(), &Adjacency::incoming(&edge));

        // Remove and return the edge
        labelled_edges.remove(edge_idx)
    }

    fn vertex(&self, id: Self::VertexId) -> Option<Self::VertexReference<'_>> {
        let label_idx = id.label();
        let vertex_idx = id.vertex();

        // Get the corresponding LabelledVertices for this label
        let labelled_vertices = &self.vertices[label_idx as usize];

        // Get the vertex and create a reference if it exists
        labelled_vertices
            .get(vertex_idx)
            .map(|weight| VertexReference { id, weight })
    }

    fn vertex_mut(&mut self, id: Self::VertexId) -> Option<Self::VertexReferenceMut<'_>> {
        let label_idx = id.label();
        let vertex_idx = id.vertex();

        // Get the corresponding LabelledVertices for this label
        let labelled_vertices = self.vertices.get_mut(label_idx as usize)?;

        // Get mutable reference to the vertex if it exists
        let vertex = labelled_vertices.get_mut(vertex_idx);
        vertex.map(|weight| VertexReferenceMut {
            indexes: &mut self.indexes,
            id,
            weight,
        })
    }

    fn vertices<'a>(
        &'a self,
        vertex_search: &graph_api_lib::VertexSearch<Self>,
    ) -> Self::VertexIter<'a> {
        let iter: SmallBox<dyn Iterator<Item = VertexId> + 'a, S8> =
            if let Some((index, value_or_range)) = &vertex_search.index() {
                let label = vertex_search
                    .label()
                    .expect("SimpleGraph only supports indexes on labels");
                let index_storage = &self.indexes[index.ordinal()];
                match &value_or_range {
                    ValueOrRange::Value(value) => {
                        index_storage.get(value, index, label.ordinal() as u16)
                    }
                    ValueOrRange::Range(range) => {
                        index_storage.range(range, index, label.ordinal() as u16)
                    }
                }
            } else if let Some(label) = vertex_search.label() {
                // Only iterate over vertices for the specified label
                smallbox!(self.vertices[label.ordinal()]
                    .iter()
                    .map(move |idx| VertexId::new(label.ordinal() as u16, idx)))
            } else {
                // Start iterating through the first group; the iterator will handle the rest
                smallbox!(self
                    .vertices
                    .iter()
                    .enumerate()
                    .flat_map(|(ordinal, label)| label
                        .iter()
                        .map(move |idx| VertexId::new(ordinal as u16, idx))))
            };

        VertexIterator {
            vertices: &self.vertices,
            iter,
        }
    }

    fn edge(&self, id: Self::EdgeId) -> Option<Self::EdgeReference<'_>> {
        let label_idx = id.label();
        let edge_idx = id.edge();

        // Get the corresponding LabelledEdges for this label
        let labelled_edges = &self.edges[label_idx as usize];

        // Get the edge and create a reference if it exists
        labelled_edges
            .get(edge_idx)
            .map(|weight| EdgeReference { id, weight })
    }

    fn edge_mut(&mut self, edge: Self::EdgeId) -> Option<Self::EdgeReferenceMut<'_>> {
        let label_idx = edge.label();
        let edge_idx = edge.edge();

        // Get the corresponding LabelledEdges for this label
        let labelled_edges = &mut self.edges[label_idx as usize];

        // Get mutable reference to the edge if it exists
        labelled_edges
            .get_mut(edge_idx)
            .map(|weight| EdgeReferenceMut {
                id: edge,
                tail: edge.tail(),
                head: edge.head(),
                weight,
            })
    }

    fn edges(&self, vertex: Self::VertexId, search: &EdgeSearch<Self>) -> Self::EdgeIter<'_> {
        // The edges are stored in a BTreeSet so they are already sorted. If we have a label in the search we can use this to create a range iterator using VertexStorage.

        let labelled_vertices = &self.vertices[vertex.label() as usize];
        let vertex_storage = &labelled_vertices[vertex.vertex()];

        // In reverse order of specitivity we populate the ranges.
        // This is because for instance if you have a specific range for a label you will need to iterate
        // over each direction individually rather than using the full range.
        let adjacent_label_range = search
            .adjacent_label
            .map(|label| label.ordinal() as u16..label.ordinal() as u16 + 1);
        let label_range = search
            .label
            .map(|label| label.ordinal() as u16..label.ordinal() as u16 + 1)
            .or_else(|| adjacent_label_range.as_ref().map(|_| 0..u16::MAX));
        let direction_range = search
            .direction
            .map(|direction| match direction {
                Direction::All => Direction::Outgoing..Direction::All,
                Direction::Outgoing => Direction::Outgoing..Direction::Incoming,
                Direction::Incoming => Direction::Incoming..Direction::All,
            })
            .or_else(|| {
                label_range
                    .as_ref()
                    .map(|_| Direction::Incoming..Direction::All)
            });

        // Now flatmap all the ranges together.
        // This gives an iterator that can be used to generate iterators of adjacency.
        let range_iter = RangeOrNoneIterator::new(direction_range, |d| match d {
            Direction::Outgoing => Direction::Incoming,
            Direction::Incoming => Direction::All,
            Direction::All => unreachable!("range should never include all"),
        })
        .flat_map(move |direction| {
            RangeOrNoneIterator::new(label_range.clone(), |l| l + 1)
                .map(move |label| (direction, label))
        })
        .flat_map(move |(direction, label)| {
            RangeOrNoneIterator::new(adjacent_label_range.clone(), |l| l + 1)
                .map(move |adjacent_label| (direction, label, adjacent_label))
        });
        let range_iter: BoxedRangeIter = smallbox::smallbox!(range_iter);
        assert!(!range_iter.is_heap());
        EdgeIterator {
            vertex,
            vertex_storage,
            edges: &self.edges,
            range_iter,
            current_iter: None,
        }
    }

    fn clear(&mut self) -> Result<(), Error> {
        // Clear all vertex storages
        for labelled_vertices in &mut self.vertices {
            labelled_vertices.clear();
        }

        // Clear all edge storages
        for edge_label in &mut self.edges {
            edge_label.clear();
        }

        Ok(())
    }
}

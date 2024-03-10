use crate::graph::{EdgeReferenceMut, Unsupported, VertexReference, VertexReferenceMut};
use crate::search::vertex::VertexSearch;
use crate::EdgeSearch;
use crate::{Direction, EdgeReference, Graph, Id, Project, ProjectMut};
use crate::{Element, Error};
use petgraph::stable_graph::{EdgeIndex, Edges, IndexType};
use petgraph::stable_graph::{NodeIndex, NodeIndices};
use petgraph::visit::EdgeRef;
use petgraph::EdgeType;
/// Implement Graph for `petgraph::StableGraph`.
/// Ideally implementations for specific graphs are not included, but we need one reference graph.
/// If ever petgraph decided to implement graph-api directly then this could be removed as a private implementation detail of the graph-api-tests crate.
use std::fmt::Debug;
use std::marker::PhantomData;

impl<Vertex, Edge, Ty, Ix> Graph for petgraph::stable_graph::StableGraph<Vertex, Edge, Ty, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
    Vertex: Debug + Element,
    Edge: Debug + Element,
{
    type SupportsVertexLabelIndex = Unsupported;
    type SupportsEdgeLabelIndex = Unsupported;
    type SupportsVertexIndex = Unsupported;
    type SupportsEdgeIndex = Unsupported;
    type SupportsVertexOrderedIndex = Unsupported;
    type SupportsEdgeOrderedIndex = Unsupported;
    type SupportsVertexFullTextIndex = Unsupported;

    type Vertex = Vertex;
    type Edge = Edge;
    type VertexId = NodeIndex<Ix>;
    type EdgeId = EdgeIndex<Ix>;
    type VertexReference<'a>
        = VertexReferenceWrapper<'a, Self, Ix>
    where
        Self: 'a;
    type VertexReferenceMut<'a>
        = VertexReferenceWrapperMut<'a, Self, Ix>
    where
        Self: 'a;
    type VertexIter<'a>
        = VertexIter<'a, Self, Ty, Ix, NodeIndices<'a, Vertex, Ix>>
    where
        Self: 'a;
    type EdgeReference<'a>
        = EdgeReferenceWrapper<'a, Self, Ix>
    where
        Self: 'a;
    type EdgeReferenceMut<'a>
        = EdgeReferenceWrapperMut<'a, Self, Ix>
    where
        Self: 'a;
    type EdgeIter<'a>
        = EdgesIter<Self, Edges<'a, Self::Edge, Ty, Ix>>
    where
        Self: 'a;

    fn add_vertex(&mut self, vertex: Self::Vertex) -> Self::VertexId {
        petgraph::stable_graph::StableGraph::add_node(self, vertex)
    }

    fn add_edge(
        &mut self,
        from: Self::VertexId,
        to: Self::VertexId,
        edge: Self::Edge,
    ) -> Self::EdgeId {
        petgraph::stable_graph::StableGraph::add_edge(self, from, to, edge)
    }

    fn remove_vertex(&mut self, vertex: Self::VertexId) -> Option<Self::Vertex> {
        petgraph::stable_graph::StableGraph::remove_node(self, vertex)
    }

    fn remove_edge(&mut self, edge: Self::EdgeId) -> Option<Self::Edge> {
        petgraph::stable_graph::StableGraph::remove_edge(self, edge)
    }

    fn vertex(&self, id: Self::VertexId) -> Option<Self::VertexReference<'_>> {
        petgraph::stable_graph::StableGraph::node_weight(self, id)
            .map(|vertex| VertexReferenceWrapper { id, vertex })
    }

    fn vertex_mut(&mut self, id: Self::VertexId) -> Option<Self::VertexReferenceMut<'_>> {
        petgraph::stable_graph::StableGraph::node_weight_mut(self, id)
            .map(|vertex| VertexReferenceWrapperMut { id, vertex })
    }

    fn vertices(&self, _vertex_search: &VertexSearch<Self>) -> Self::VertexIter<'_> {
        VertexIter {
            graph: self,
            nodes: self.node_indices(),
        }
    }

    fn edge(&self, id: Self::EdgeId) -> Option<Self::EdgeReference<'_>> {
        let weight = petgraph::stable_graph::StableGraph::edge_weight(self, id);
        let endpoints = petgraph::stable_graph::StableGraph::edge_endpoints(self, id);
        match (weight, endpoints) {
            (Some(weight), Some(endpoints)) => Some(EdgeReferenceWrapper::Synthetic {
                id,
                weight,
                head: endpoints.1,
                tail: endpoints.0,
            }),
            _ => None,
        }
    }

    fn edge_mut(&mut self, id: Self::EdgeId) -> Option<Self::EdgeReferenceMut<'_>> {
        let endpoints = petgraph::stable_graph::StableGraph::edge_endpoints(self, id);
        let weight = petgraph::stable_graph::StableGraph::edge_weight_mut(self, id);
        match (weight, endpoints) {
            (Some(weight), Some(endpoints)) => Some(EdgeReferenceWrapperMut {
                id,
                weight,
                head: endpoints.1,
                tail: endpoints.0,
            }),
            _ => None,
        }
    }

    fn edges(&self, vertex: Self::VertexId, search: &EdgeSearch<Self>) -> Self::EdgeIter<'_> {
        EdgesIter {
            _phantom: Default::default(),
            edges: match search.direction.unwrap_or_default() {
                Direction::Incoming => [
                    None,
                    Some(self.edges_directed(vertex, petgraph::Direction::Incoming)),
                ],
                Direction::Outgoing => [
                    Some(self.edges_directed(vertex, petgraph::Direction::Outgoing)),
                    None,
                ],
                Direction::All => [
                    Some(self.edges_directed(vertex, petgraph::Direction::Outgoing)),
                    Some(self.edges_directed(vertex, petgraph::Direction::Incoming)),
                ],
            },
        }
    }

    fn clear(&mut self) -> Result<(), Error> {
        petgraph::stable_graph::StableGraph::clear(self);
        Ok(())
    }
}

impl<Ix> From<NodeIndex<Ix>> for Id<NodeIndex<Ix>, EdgeIndex<Ix>>
where
    Ix: Eq + Copy,
{
    fn from(value: NodeIndex<Ix>) -> Self {
        Id::Vertex(value)
    }
}

impl<Ix> From<EdgeIndex<Ix>> for Id<NodeIndex<Ix>, EdgeIndex<Ix>>
where
    Ix: Eq + Copy,
{
    fn from(value: EdgeIndex<Ix>) -> Self {
        Id::Edge(value)
    }
}

pub struct VertexIter<'a, Graph, Ty, Ix, Vertices>
where
    Graph: crate::Graph,
{
    nodes: Vertices,
    graph: &'a petgraph::stable_graph::StableGraph<Graph::Vertex, Graph::Edge, Ty, Ix>,
}

impl<'a, Graph, Ty, Ix, Vertices> Iterator for VertexIter<'a, Graph, Ty, Ix, Vertices>
where
    Graph: crate::Graph,
    Ty: EdgeType,
    Ix: IndexType,
    Vertices: Iterator<Item = NodeIndex<Ix>>,
    Ix: IndexType,
{
    type Item = VertexReferenceWrapper<'a, Graph, Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.next().map(|node| VertexReferenceWrapper {
            id: node,
            vertex: self
                .graph
                .node_weight(node)
                .expect("node weight should exist"),
        })
    }
}

pub struct EdgesIter<Graph, Edges> {
    _phantom: PhantomData<Graph>,
    edges: [Option<Edges>; 2],
}

impl<'a, Graph, Ty, Ix> Iterator for EdgesIter<Graph, Edges<'a, Graph::Edge, Ty, Ix>>
where
    Ty: EdgeType,
    Ix: IndexType,
    Graph: crate::Graph,
{
    type Item = EdgeReferenceWrapper<'a, Graph, Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        for edges in &mut self.edges {
            if let Some(edges) = edges.as_mut() {
                if let Some(edge) = edges.next() {
                    return Some(EdgeReferenceWrapper::Native(edge));
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum EdgeReferenceWrapper<'a, Graph, Ix>
where
    Graph: crate::Graph,
    Ix: IndexType,
{
    Native(petgraph::stable_graph::EdgeReference<'a, Graph::Edge, Ix>),
    Synthetic {
        id: EdgeIndex<Ix>,
        weight: &'a Graph::Edge,
        head: NodeIndex<Ix>,
        tail: NodeIndex<Ix>,
    },
}

impl<'a, Graph, Ix> EdgeReference<'a, Graph> for EdgeReferenceWrapper<'a, Graph, Ix>
where
    Ix: IndexType,
    Graph: crate::Graph,
    <Graph as crate::Graph>::VertexId: From<NodeIndex<Ix>>,
    <Graph as crate::Graph>::EdgeId: From<EdgeIndex<Ix>>,
{
    fn id(&self) -> Graph::EdgeId {
        match self {
            EdgeReferenceWrapper::Native(edge) => petgraph::visit::EdgeRef::id(edge).into(),
            EdgeReferenceWrapper::Synthetic { id, .. } => (*id).into(),
        }
    }

    fn tail(&self) -> Graph::VertexId {
        match self {
            EdgeReferenceWrapper::Native(edge) => edge.source().into(),
            EdgeReferenceWrapper::Synthetic { tail, .. } => (*tail).into(),
        }
    }

    fn head(&self) -> Graph::VertexId {
        match self {
            EdgeReferenceWrapper::Native(edge) => edge.target().into(),
            EdgeReferenceWrapper::Synthetic { head, .. } => (*head).into(),
        }
    }

    fn weight(&self) -> &'a Graph::Edge {
        match self {
            EdgeReferenceWrapper::Native(edge) => edge.weight(),
            EdgeReferenceWrapper::Synthetic { weight, .. } => weight,
        }
    }

    fn project<'reference, T: Project<'reference, <Graph as crate::Graph>::Edge>>(
        &'reference self,
    ) -> Option<T> {
        crate::Project::project(self.weight())
    }
}

#[derive(Debug)]
pub struct EdgeReferenceWrapperMut<'a, Graph, Ix>
where
    Ix: IndexType,
    Graph: crate::Graph,
{
    id: EdgeIndex<Ix>,
    weight: &'a mut Graph::Edge,
    head: NodeIndex<Ix>,
    tail: NodeIndex<Ix>,
}

impl<'a, Graph, Ix> EdgeReference<'a, Graph> for EdgeReferenceWrapperMut<'a, Graph, Ix>
where
    Ix: IndexType,
    Graph: crate::Graph,
    <Graph as crate::Graph>::VertexId: From<NodeIndex<Ix>>,
    <Graph as crate::Graph>::EdgeId: From<EdgeIndex<Ix>>,
{
    fn id(&self) -> Graph::EdgeId {
        self.id.into()
    }

    fn tail(&self) -> Graph::VertexId {
        self.tail.into()
    }

    fn head(&self) -> Graph::VertexId {
        self.head.into()
    }

    fn weight(&self) -> &Graph::Edge {
        self.weight
    }

    fn project<'reference, T: Project<'reference, <Graph as crate::Graph>::Edge>>(
        &'reference self,
    ) -> Option<T> {
        crate::Project::project(self.weight)
    }
}

impl<'a, Graph, Ix> EdgeReferenceMut<'a, Graph> for EdgeReferenceWrapperMut<'a, Graph, Ix>
where
    Ix: IndexType,
    Graph: crate::Graph,
    <Graph as crate::Graph>::VertexId: From<NodeIndex<Ix>>,
    <Graph as crate::Graph>::EdgeId: From<EdgeIndex<Ix>>,
{
    type MutationListener<'reference> = ();

    fn weight_mut(&mut self) -> &mut Graph::Edge {
        self.weight
    }

    fn project_mut<
        'reference,
        T: ProjectMut<'reference, <Graph as crate::Graph>::Edge, Self::MutationListener<'reference>>,
    >(
        &'reference mut self,
    ) -> Option<T> {
        crate::ProjectMut::project_mut(self.weight, ())
    }
}

#[derive(Debug)]
pub struct VertexReferenceWrapper<'a, Graph, Ix>
where
    Graph: crate::Graph,
{
    id: NodeIndex<Ix>,
    vertex: &'a Graph::Vertex,
}

impl<'a, Graph, Ix> VertexReference<'a, Graph> for VertexReferenceWrapper<'a, Graph, Ix>
where
    Ix: IndexType,
    Graph: crate::Graph,
    <Graph as crate::Graph>::VertexId: From<NodeIndex<Ix>>,
{
    fn id(&self) -> Graph::VertexId {
        self.id.into()
    }

    fn weight(&self) -> &Graph::Vertex {
        self.vertex
    }

    fn project<'reference, T: Project<'reference, <Graph as crate::Graph>::Vertex>>(
        &'reference self,
    ) -> Option<T> {
        crate::Project::project(self.vertex)
    }
}

#[derive(Debug)]
pub struct VertexReferenceWrapperMut<'a, Graph, Ix>
where
    Graph: crate::Graph,
{
    id: NodeIndex<Ix>,
    vertex: &'a mut Graph::Vertex,
}

impl<'a, Graph, Ix> VertexReference<'a, Graph> for VertexReferenceWrapperMut<'a, Graph, Ix>
where
    Ix: IndexType,
    Graph: crate::Graph,
    <Graph as crate::Graph>::VertexId: From<NodeIndex<Ix>>,
{
    fn id(&self) -> Graph::VertexId {
        self.id.into()
    }

    fn weight(&self) -> &Graph::Vertex {
        self.vertex
    }

    fn project<'reference, T: Project<'reference, <Graph as crate::Graph>::Vertex>>(
        &'reference self,
    ) -> Option<T> {
        crate::Project::project(self.vertex)
    }
}

impl<'a, Graph, Ix> VertexReferenceMut<'a, Graph> for VertexReferenceWrapperMut<'a, Graph, Ix>
where
    Ix: IndexType,
    Graph: crate::Graph + 'a,
    <Graph as crate::Graph>::VertexId: From<NodeIndex<Ix>>,
{
    type MutationListener<'reference> = ();

    fn weight_mut(&mut self) -> &mut Graph::Vertex {
        self.vertex
    }

    fn project_mut<
        'reference,
        T: ProjectMut<'reference, <Graph as crate::Graph>::Vertex, Self::MutationListener<'reference>>,
    >(
        &'reference mut self,
    ) -> Option<T> {
        ProjectMut::project_mut(self.vertex, ())
    }
}

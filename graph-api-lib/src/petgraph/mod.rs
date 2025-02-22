use crate::graph::{EdgeReferenceMut, Unsupported, VertexReference, VertexReferenceMut};
use crate::search::vertex::VertexSearch;
use crate::Element;
use crate::{Direction, EdgeReference, Graph, Id, Project, ProjectMut};
use crate::{EdgeSearch, Supported};
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
    type SupportsClear = Supported;
    type SupportsEdgeAdjacentLabelIndex = Unsupported;

    type Vertex = Vertex;
    type Edge = Edge;
    type VertexId = NodeIndex<Ix>;
    type EdgeId = EdgeIndex<Ix>;
    type VertexReference<'graph>
        = VertexReferenceWrapper<'graph, Self, Ix>
    where
        Self: 'graph;
    type VertexReferenceMut<'graph>
        = VertexReferenceWrapperMut<'graph, Self, Ix>
    where
        Self: 'graph;
    type VertexIter<'search, 'graph>
        = VertexIter<'search, 'graph, Self, Ty, Ix, NodeIndices<'graph, Vertex, Ix>>
    where
        Self: 'graph;
    type EdgeReference<'graph>
        = EdgeReferenceWrapper<'graph, Self, Ix>
    where
        Self: 'graph;
    type EdgeReferenceMut<'graph>
        = EdgeReferenceWrapperMut<'graph, Self, Ix>
    where
        Self: 'graph;

    type EdgeIter<'search, 'graph>
        = EdgeIter<'search, Self, Edges<'graph, Self::Edge, Ty, Ix>>
    where
        Self: 'graph;

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

    fn vertices<'search>(
        &self,
        vertex_search: &VertexSearch<'search, Self>,
    ) -> Self::VertexIter<'search, '_> {
        VertexIter {
            graph: self,
            nodes: self.node_indices(),
            vertex_search: vertex_search.clone(),
            count: 0,
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

    fn edges<'search>(
        &self,
        vertex: Self::VertexId,
        search: &EdgeSearch<'search, Self>,
    ) -> Self::EdgeIter<'search, '_> {
        if search.adjacent_label.is_some() {
            unreachable!("Petgraph does not support edge index via adjacent vertex label")
        }
        EdgeIter {
            _phantom: Default::default(),
            edges: match search.direction {
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
            edge_search: search.clone(),
            count: 0,
        }
    }

    fn clear(&mut self) {
        petgraph::stable_graph::StableGraph::clear(self);
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

pub struct VertexIter<'search, 'graph, Graph, Ty, Ix, Vertices>
where
    Graph: crate::Graph,
{
    nodes: Vertices,
    graph: &'graph petgraph::stable_graph::StableGraph<Graph::Vertex, Graph::Edge, Ty, Ix>,
    vertex_search: VertexSearch<'search, Graph>,
    count: usize,
}

impl<'graph, Graph, Ty, Ix, Vertices> Iterator for VertexIter<'_, 'graph, Graph, Ty, Ix, Vertices>
where
    Graph: crate::Graph,
    Ty: EdgeType,
    Ix: IndexType,
    Vertices: Iterator<Item = NodeIndex<Ix>>,
    Ix: IndexType,
    <Graph as crate::Graph>::VertexId: From<NodeIndex<Ix>>,
{
    type Item = VertexReferenceWrapper<'graph, Graph, Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.vertex_search.limit() {
            return None;
        }
        loop {
            if let Some(next) = self.nodes.next().map(|node| VertexReferenceWrapper {
                id: node,
                vertex: self
                    .graph
                    .node_weight(node)
                    .expect("node weight should exist"),
            }) {
                if let VertexSearch::Label { label, .. } = &self.vertex_search {
                    if *label != Element::label(next.weight()) {
                        continue;
                    }
                }

                self.count += 1;
                return Some(next);
            } else {
                return None;
            }
        }
    }
}

pub struct EdgeIter<'search, Graph, Edges>
where
    Graph: crate::Graph,
{
    _phantom: PhantomData<Graph>,
    edges: [Option<Edges>; 2],
    edge_search: EdgeSearch<'search, Graph>,
    count: usize,
}

impl<'graph, Graph, Ty, Ix> Iterator for EdgeIter<'_, Graph, Edges<'graph, Graph::Edge, Ty, Ix>>
where
    Ty: EdgeType,
    Ix: IndexType,
    Graph: crate::Graph,
    <Graph as crate::Graph>::VertexId: From<NodeIndex<Ix>>,
    <Graph as crate::Graph>::EdgeId: From<EdgeIndex<Ix>>,
{
    type Item = EdgeReferenceWrapper<'graph, Graph, Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.edge_search.limit.unwrap_or(usize::MAX) {
            return None;
        }
        for edges in &mut self.edges {
            if let Some(edges) = edges.as_mut() {
                for edge in edges {
                    let edge = EdgeReferenceWrapper::Native(edge);
                    // We don't have to check direction as this is supported by petgraph
                    // But we need to check everything else
                    if let Some(label) = self.edge_search.label {
                        let edge_label = Element::label(edge.weight());
                        if edge_label != label {
                            continue;
                        }
                    }

                    self.count += 1;
                    return Some(edge);
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum EdgeReferenceWrapper<'graph, Graph, Ix>
where
    Graph: crate::Graph,
    Ix: IndexType,
{
    Native(petgraph::stable_graph::EdgeReference<'graph, Graph::Edge, Ix>),
    Synthetic {
        id: EdgeIndex<Ix>,
        weight: &'graph Graph::Edge,
        head: NodeIndex<Ix>,
        tail: NodeIndex<Ix>,
    },
}

impl<'graph, Graph, Ix> EdgeReference<'graph, Graph> for EdgeReferenceWrapper<'graph, Graph, Ix>
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

    fn weight(&self) -> &'graph Graph::Edge {
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
pub struct EdgeReferenceWrapperMut<'graph, Graph, Ix>
where
    Ix: IndexType,
    Graph: crate::Graph,
{
    id: EdgeIndex<Ix>,
    weight: &'graph mut Graph::Edge,
    head: NodeIndex<Ix>,
    tail: NodeIndex<Ix>,
}

impl<'graph, Graph, Ix> EdgeReference<'graph, Graph> for EdgeReferenceWrapperMut<'graph, Graph, Ix>
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
pub struct VertexReferenceWrapper<'graph, Graph, Ix>
where
    Graph: crate::Graph,
{
    id: NodeIndex<Ix>,
    vertex: &'graph Graph::Vertex,
}

impl<'graph, Graph, Ix> VertexReference<'graph, Graph> for VertexReferenceWrapper<'graph, Graph, Ix>
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
pub struct VertexReferenceWrapperMut<'graph, Graph, Ix>
where
    Graph: crate::Graph,
{
    id: NodeIndex<Ix>,
    vertex: &'graph mut Graph::Vertex,
}

impl<'graph, Graph, Ix> VertexReference<'graph, Graph>
    for VertexReferenceWrapperMut<'graph, Graph, Ix>
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

impl<'graph, Graph, Ix> VertexReferenceMut<'graph, Graph>
    for VertexReferenceWrapperMut<'graph, Graph, Ix>
where
    Ix: IndexType,
    Graph: crate::Graph + 'graph,
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

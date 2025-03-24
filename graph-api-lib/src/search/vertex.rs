use crate::element::Element;
use crate::{Label, Supported, Value, ValueRange};
use derivative::Derivative;
use std::ops::Range;

/// A search to apply to vertices when querying a graph.
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub enum VertexSearch<'search, Graph>
where
    Graph: crate::Graph,
{
    Scan {
        limit: Option<usize>,
    },
    Label {
        label: <Graph::Vertex as Element>::Label,
        limit: Option<usize>,
    },
    Index {
        index: <<Graph::Vertex as Element>::Label as Label>::Index,
        value: Value<'search>,
        limit: Option<usize>,
    },
    Range {
        index: <<Graph::Vertex as Element>::Label as Label>::Index,
        range: Range<Value<'search>>,
        limit: Option<usize>,
    },
    FullText {
        index: <<Graph::Vertex as Element>::Label as Label>::Index,
        search: Value<'search>,
        limit: Option<usize>,
    },
}

impl<'search, Graph> VertexSearch<'search, Graph>
where
    Graph: crate::Graph,
{
    pub fn scan() -> Self {
        Self::Scan { limit: None }
    }

    pub fn label(label: <Graph::Vertex as Element>::Label) -> Self
    where
        Graph: crate::Graph<SupportsVertexLabelIndex = Supported>,
    {
        Self::Label { label, limit: None }
    }

    pub fn get<V>(index: <<Graph::Vertex as Element>::Label as Label>::Index, value: V) -> Self
    where
        V: Into<Value<'search>>,
        Graph: crate::Graph<SupportsVertexHashIndex = Supported>,
    {
        Self::Index {
            index,
            value: value.into(),
            limit: None,
        }
    }

    pub fn range<R>(index: <<Graph::Vertex as Element>::Label as Label>::Index, range: R) -> Self
    where
        R: Into<ValueRange<'search>>,
        Graph: crate::Graph<SupportsVertexRangeIndex = Supported>,
    {
        Self::Range {
            index,
            range: range.into().0,
            limit: None,
        }
    }

    pub fn full_text(
        index: <<Graph::Vertex as Element>::Label as Label>::Index,
        search: &'search str,
    ) -> Self
    where
        Graph: crate::Graph<SupportsVertexFullTextIndex = Supported>,
    {
        Self::FullText {
            index,
            search: Value::Str(search),
            limit: None,
        }
    }

    /// The maximum number of vertices to return from this search
    pub fn with_limit(mut self, new_limit: usize) -> Self {
        match &mut self {
            VertexSearch::Scan { limit } => *limit = Some(new_limit),
            VertexSearch::Label { limit, .. } => *limit = Some(new_limit),
            VertexSearch::Index { limit, .. } => *limit = Some(new_limit),
            VertexSearch::Range { limit, .. } => *limit = Some(new_limit),
            VertexSearch::FullText { limit, .. } => *limit = Some(new_limit),
        }

        self
    }

    pub fn limit(&self) -> usize {
        match self {
            VertexSearch::Scan { limit, .. } => limit,
            VertexSearch::Label { limit, .. } => limit,
            VertexSearch::Index { limit, .. } => limit,
            VertexSearch::Range { limit, .. } => limit,
            VertexSearch::FullText { limit, .. } => limit,
        }
        .unwrap_or(usize::MAX)
    }
}

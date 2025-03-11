#![doc = include_str!("../../README.md")]

pub extern crate proptest;
pub use proptest::*;
pub mod bench;
pub mod collect;
pub mod context;
pub mod count;
pub mod detour;
pub mod edges;
pub mod filter;
pub mod filter_derive;
pub mod first;
pub mod fold;
pub mod fuzz;
pub mod graph;
pub mod index;
pub mod limit;
pub mod mutation;
pub mod probe;
pub mod reduce;
pub mod vertices;

use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::ElementId;
#[allow(unused_imports)]
use graph_api_lib::{Supported, Unsupported};
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;
use uuid::Uuid;

#[cfg(feature = "vertex-label-index")]
pub(crate) type SupportsVertexLabelIndex = Supported;
#[cfg(not(feature = "vertex-label-index"))]
pub(crate) type SupportsVertexLabelIndex = Unsupported;

#[cfg(feature = "vertex-index")]
pub(crate) type SupportsVertexIndex = Supported;
#[cfg(not(feature = "vertex-index"))]
pub(crate) type SupportsVertexIndex = Unsupported;

#[cfg(feature = "vertex-full-text-index")]
pub(crate) type SupportsVertexFullTextIndex = Supported;
#[cfg(not(feature = "vertex-full-text-index"))]
pub(crate) type SupportsVertexFullTextIndex = Unsupported;

#[cfg(feature = "vertex-ordered-index")]
pub(crate) type SupportsVertexOrderedIndex = Supported;
#[cfg(not(feature = "vertex-ordered-index"))]
pub(crate) type SupportsVertexOrderedIndex = Unsupported;

#[cfg(feature = "edge-label-index")]
pub(crate) type SupportsEdgeLabelIndex = Supported;
#[cfg(not(feature = "edge-label-index"))]
pub(crate) type SupportsEdgeLabelIndex = Unsupported;

#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index]
        name: String,
        #[index(ordered)]
        age: u64,
        #[index]
        unique_id: Uuid,
        #[index(ordered)]
        username: String,
        #[index(full_text)]
        biography: String,
    },
    Project(Project),
    Rust,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows { since: i32 },
    Created,
    Language(Language),
}
#[derive(Debug, Clone)]
pub struct Language {
    pub name: String,
}
pub struct Refs<Graph>
where
    Graph: graph_api_lib::Graph,
{
    pub bryn: Graph::VertexId,
    pub julia: Graph::VertexId,
    pub graph_api: Graph::VertexId,
    pub rust: Graph::VertexId,
    pub bryn_knows_julia: Graph::EdgeId,
    pub julia_knows_bryn: Graph::EdgeId,
    pub bryn_created_graph_api: Graph::EdgeId,
    pub graph_api_language_rust: Graph::EdgeId,
}

pub fn populate_graph<Graph>(graph: &mut Graph) -> Refs<Graph>
where
    Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>,
{
    // Hey Tinkerpop folks, long time no see! Hope things are going well!
    let bryn = graph.add_vertex(Vertex::Person {
        name: "Bryn".to_string(),
        age: 45,
        unique_id: Uuid::from_u128(1),
        username: "bryn".to_string(),
        biography: "Did some graph stuff".to_string(),
    });
    let julia = graph.add_vertex(Vertex::Person {
        name: "Julia".to_string(),
        age: 48,
        unique_id: Uuid::from_u128(2),
        username: "julia".to_string(),
        biography: "Mastered the English language".to_string(),
    });
    let graph_api = graph.add_vertex(Vertex::Project(Project {
        name: "GraphApi".to_string(),
    }));

    let rust = graph.add_vertex(Vertex::Rust);

    let bryn_knows_julia = graph.add_edge(bryn, julia, Edge::Knows { since: 1999 });
    let julia_knows_bryn = graph.add_edge(julia, bryn, Edge::Knows { since: 1999 });
    let bryn_created_graph_api = graph.add_edge(bryn, graph_api, Edge::Created);
    let graph_api_language_rust = graph.add_edge(
        graph_api,
        rust,
        Edge::Language(Language {
            name: "Rust".to_string(),
        }),
    );

    Refs {
        bryn,
        julia,
        graph_api,
        rust,
        bryn_knows_julia,
        julia_knows_bryn,
        bryn_created_graph_api,
        graph_api_language_rust,
    }
}

#[derive(Error, Debug)]
pub enum TestError {
    Mismatch {
        missing: Vec<String>,
        extra: Vec<String>,
        expected: Vec<String>,
    },
    MoreThanOneElement {
        expected: Vec<String>,
        actual: Vec<String>,
    },
}

#[macro_export]
macro_rules! test {
    ($setup:expr, $name:ident, $path:path) => {
        #[test]
        fn $name() {
            let mut g = $setup;
            $path(&mut g);
        }
    };
}

#[macro_export]
macro_rules! test_suite {
    ($setup:expr) => {
        $crate::test!{$setup, graph_test_add_vertex, $crate::graph::test_add_vertex}
        $crate::test!{$setup, graph_test_mutate_vertex, $crate::graph::test_mutate_vertex}
        $crate::test!{$setup, graph_test_remove_vertex, $crate::graph::test_remove_vertex}
        $crate::test!{$setup, graph_test_add_edge, $crate::graph::test_add_edge}
        $crate::test!{$setup, graph_test_mutate_edge, $crate::graph::test_mutate_edge}
        $crate::test!{$setup, graph_test_remove_edge, $crate::graph::test_remove_edge}
        $crate::test!{$setup, graph_test_remove_vertex_with_edges, $crate::graph::test_remove_vertex_with_edges}
        $crate::test!{$setup, filter_test_vertices_filter, $crate::filter::test_vertices_filter}
        $crate::test!{$setup, filter_test_edges_filter, $crate::filter::test_edges_filter}
        $crate::test!{$setup, vertices_test_vertices_collect, $crate::collect::test_vertices_collect}
        $crate::test!{$setup, vertices_test_edges_collect, $crate::collect::test_edges_collect}
        $crate::test!{$setup, edges_test_out_edges, $crate::edges::test_out_edges}
        $crate::test!{$setup, edges_test_out_edges_limit, $crate::edges::test_out_edges_limit}
        $crate::test!{$setup, edges_test_in_edges, $crate::edges::test_in_edges}
        $crate::test!{$setup, edges_test_in_edges_limit, $crate::edges::test_in_edges_limit}
        $crate::test!{$setup, edges_test_all_edges, $crate::edges::test_all_edges}
        $crate::test!{$setup, edges_test_all_edges_limit, $crate::edges::test_all_edges_limit}
        $crate::test!{$setup, edges_test_out_edges_filtered, $crate::edges::test_out_edges_filtered}
        $crate::test!{$setup, edges_test_out_edges_filtered_limit, $crate::edges::test_out_edges_filtered_limit}
        $crate::test!{$setup, edges_test_in_edges_filtered, $crate::edges::test_in_edges_filtered}
        $crate::test!{$setup, edges_test_in_edges_filtered_limit, $crate::edges::test_in_edges_filtered_limit}
        $crate::test!{$setup, edges_test_all_edges_filtered, $crate::edges::test_all_edges_filtered}
        $crate::test!{$setup, edges_test_all_edges_filtered_limit, $crate::edges::test_all_edges_filtered_limit}
        $crate::test!{$setup, context_test_vertices_context, $crate::context::test_vertices_context}
        $crate::test!{$setup, vertices_test_limit, $crate::vertices::test_limit}
        $crate::test!{$setup, vertices_test_head, $crate::vertices::test_head}
        $crate::test!{$setup, vertices_test_tail, $crate::vertices::test_tail}
        $crate::test!{$setup, mutation_test_mutation, $crate::mutation::test_mutation}
        $crate::test!{$setup, mutation_test_edge_mutation, $crate::mutation::test_edge_mutation}
        $crate::test!{$setup, count_test_vertices_count, $crate::count::test_vertices_count}
        $crate::test!{$setup, count_test_edges_count, $crate::count::test_edges_count}
        $crate::test!{$setup, limit_test_vertices_limit, $crate::limit::test_vertices_limit}
        $crate::test!{$setup, limit_test_edges_limit, $crate::limit::test_edges_limit}
        $crate::test!{$setup, first_test_vertices_first, $crate::first::test_vertices_first}
        $crate::test!{$setup, first_test_edges_first, $crate::first::test_edges_first}
        $crate::test!{$setup, fold_test_vertices_fold, $crate::fold::test_vertices_fold}
        $crate::test!{$setup, fold_test_edges_fold, $crate::fold::test_edges_fold}
        $crate::test!{$setup, reduce_test_vertices_reduce, $crate::reduce::test_vertices_reduce}
        $crate::test!{$setup, reduce_test_edges_reduce, $crate::reduce::test_edges_reduce}
        $crate::test!{$setup, detour_test_vertices_detour, $crate::detour::test_vertices_detour}
        $crate::test!{$setup, filter_derive_test_vertices_filter, $crate::filter_derive::test_vertices_filter}
        $crate::test!{$setup, filter_derive_test_edges_filter, $crate::filter_derive::test_edges_filter}
        $crate::test!{$setup, probe_test_vertices_probe, $crate::probe::test_vertices_probe}
        $crate::test!{$setup, probe_test_edges_probe, $crate::probe::test_edges_probe}

        #[cfg(feature = "edge-label-index")]
        $crate::test!{$setup, index_edge_label_test_index, $crate::index::edge_label::test_index}
        #[cfg(feature = "edge-label-index")]
        $crate::test!{$setup, index_edge_label_test_index_limit, $crate::index::edge_label::test_index_limit}

        #[cfg(feature = "vertex-label-index")]
        $crate::test!{$setup, index_vertex_label_test_index, $crate::index::vertex_label::test_index}
        #[cfg(feature = "vertex-label-index")]
        $crate::test!{$setup, index_vertex_label_test_index_limit, $crate::index::vertex_label::test_index_limit}

        #[cfg(feature = "vertex-index")]
        $crate::test!{$setup, index_vertex_default_test_index, $crate::index::vertex_default::test_index}
        #[cfg(feature = "vertex-index")]
        $crate::test!{$setup, index_vertex_default_test_index_remove, $crate::index::vertex_default::test_index_remove}
        #[cfg(feature = "vertex-index")]
        $crate::test!{$setup, index_vertex_default_test_index_update, $crate::index::vertex_default::test_index_update}

        #[cfg(feature="vertex-full-text-index")]
        $crate::test!{$setup, index_vertex_full_text_test_index, $crate::index::vertex_full_text::test_index}
        #[cfg(feature="vertex-full-text-index")]
        $crate::test!{$setup, index_vertex_full_text_test_index_remove, $crate::index::vertex_full_text::test_index_remove}
        #[cfg(feature="vertex-full-text-index")]
        $crate::test!{$setup, index_vertex_full_text_test_index_update, $crate::index::vertex_full_text::test_index_update}

        #[cfg(feature = "vertex-ordered-index")]
        $crate::test!{$setup, index_vertex_ordered_test_index, $crate::index::vertex_ordered::test_index}
        #[cfg(feature = "vertex-ordered-index")]
        $crate::test!{$setup, index_vertex_ordered_test_index_remove, $crate::index::vertex_ordered::test_index_remove}
        #[cfg(feature = "vertex-ordered-index")]
        $crate::test!{$setup, index_vertex_ordered_test_index_update, $crate::index::vertex_ordered::test_index_update}

        $crate::proptest! {
            #[test]
            fn fuzz_test(operations in $crate::collection::vec($crate::fuzz::arb_graph_operation(), 0..100)) {
                $crate::fuzz::test_fuzz($setup, operations);
            }
        }
    };
}

impl Display for TestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestError::Mismatch {
                missing,
                extra,
                expected,
            } => {
                if !missing.is_empty() {
                    write!(f, "Missing elements:\n{}\n", missing.join("\n"),)?;
                }
                if !extra.is_empty() {
                    write!(f, "Extra elements:\n{}\n", extra.join("\n"),)?;
                }
                write!(f, "Expected elements:\n{}", expected.join("\n"))?;
            }
            TestError::MoreThanOneElement { actual, expected } => {
                write!(
                    f,
                    "Expected one of:\n{}\nBut got:\n{}",
                    expected.join("\n"),
                    actual.join("\n")
                )?;
            }
        }

        Ok(())
    }
}

pub fn assert_elements_one_of<Graph>(
    graph: &Graph,
    actual: impl IntoIterator<Item = impl Into<ElementId<Graph>>>,
    expected: impl IntoIterator<Item = impl Into<ElementId<Graph>>>,
) -> Result<(), TestError>
where
    Graph: graph_api_lib::Graph,
{
    let actual: Vec<ElementId<Graph>> = actual.into_iter().map(Into::into).collect();
    let expected: Vec<ElementId<Graph>> = expected.into_iter().map(Into::into).collect();

    // First convert to debug strings for error reporting
    let actual_strings: Vec<String> = actual.iter().map(|e| graph.dbg(*e)).collect();
    let expected_strings: Vec<String> = expected.iter().map(|e| graph.dbg(*e)).collect();

    if actual.len() != 1 {
        return Err(TestError::MoreThanOneElement {
            expected: expected_strings,
            actual: actual_strings,
        });
    }

    Ok(())
}

pub fn assert_elements_eq<Graph>(
    graph: &Graph,
    actual: impl IntoIterator<Item = impl Into<ElementId<Graph>>>,
    expected: impl IntoIterator<Item = impl Into<ElementId<Graph>>>,
) -> Result<(), TestError>
where
    Graph: graph_api_lib::Graph,
{
    let actual = actual.into_iter().map(Into::into).collect::<HashSet<_>>();
    let expected = expected.into_iter().map(Into::into).collect::<HashSet<_>>();
    if actual != expected {
        let missing: Vec<String> = expected
            .difference(&actual)
            .map(|e| graph.dbg(*e))
            .collect();
        let extra: Vec<String> = actual
            .difference(&expected)
            .map(|e| graph.dbg(*e))
            .collect();

        let expected: Vec<String> = expected.iter().map(|e| graph.dbg(*e)).collect();

        return Err(TestError::Mismatch {
            missing,
            extra,
            expected,
        });
    }
    Ok(())
}

#[macro_export]
macro_rules! assert_elements_eq {
    ($graph:expr, $actual:expr, $expected:expr) => {
        if let Err(e) = $crate::assert_elements_eq($graph, $actual, $expected) {
            panic!("{}", e);
        }
    };
}

#[macro_export]
macro_rules! assert_elements_one_of {
    ($graph:expr, $actual:expr, $expected:expr) => {
        if let Err(e) = $crate::assert_elements_one_of($graph, $actual, $expected) {
            panic!("{}", e);
        }
    };
}

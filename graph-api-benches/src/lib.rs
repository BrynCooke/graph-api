#![doc = include_str!("../../README.md")]

pub mod construction;
pub mod edge;
pub mod generators;
pub mod index;
pub mod mutation;
pub mod query;
pub mod scale;
pub mod traversal;
pub mod vertex;

use criterion::{BenchmarkGroup, Criterion};
use graph_api_lib::Graph;
#[allow(unused_imports)]
use graph_api_lib::{Supported, Unsupported};
use graph_api_test::{Edge, Vertex};
use std::time::Duration;

#[cfg(feature = "vertex-label-index")]
pub(crate) type SupportsVertexLabelIndex = Supported;
#[cfg(not(feature = "vertex-label-index"))]
pub(crate) type SupportsVertexLabelIndex = Unsupported;

#[cfg(feature = "vertex-index")]
pub(crate) type SupportsVertexHashIndex = Supported;
#[cfg(not(feature = "vertex-index"))]
pub(crate) type SupportsVertexHashIndex = Unsupported;

#[cfg(feature = "vertex-full-text-index")]
pub(crate) type SupportsVertexFullTextIndex = Supported;
#[cfg(not(feature = "vertex-full-text-index"))]
pub(crate) type SupportsVertexFullTextIndex = Unsupported;

#[cfg(feature = "vertex-range-index")]
pub(crate) type SupportsVertexRangeIndex = Supported;
#[cfg(not(feature = "vertex-range-index"))]
pub(crate) type SupportsVertexRangeIndex = Unsupported;

#[cfg(feature = "edge-label-index")]
pub(crate) type SupportsEdgeLabelIndex = Supported;
#[cfg(not(feature = "edge-label-index"))]
pub(crate) type SupportsEdgeLabelIndex = Unsupported;

/// Configures the default settings for benchmark groups
pub fn configure_group<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<criterion::measurement::WallTime>,
) {
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(50);
    group.warm_up_time(Duration::from_millis(500));
}

/// Run all benchmarks for a given graph implementation
pub fn run_benchmarks<G>(c: &mut Criterion, setup: impl Fn() -> G + Clone)
where
    G: Graph<
            Vertex = Vertex,
            Edge = Edge,
            SupportsVertexLabelIndex = SupportsVertexLabelIndex,
            SupportsVertexHashIndex = SupportsVertexHashIndex,
            SupportsVertexFullTextIndex = SupportsVertexFullTextIndex,
            SupportsVertexRangeIndex = SupportsVertexRangeIndex,
            SupportsEdgeLabelIndex = SupportsEdgeLabelIndex,
        >,
{
    // Run vertex operation benchmarks
    let mut vertex_group = c.benchmark_group("vertex_operations");
    configure_group::<G>(&mut vertex_group);
    vertex::run_benchmarks(&mut vertex_group, setup.clone());
    vertex_group.finish();

    // Run edge operation benchmarks
    let mut edge_group = c.benchmark_group("edge_operations");
    configure_group::<G>(&mut edge_group);
    edge::run_benchmarks(&mut edge_group, setup.clone());
    edge_group.finish();

    // Run traversal operation benchmarks
    let mut traversal_group = c.benchmark_group("traversal_operations");
    configure_group::<G>(&mut traversal_group);
    traversal::run_benchmarks(&mut traversal_group, setup.clone());
    traversal_group.finish();

    // Run query operation benchmarks
    let mut query_group = c.benchmark_group("query_operations");
    configure_group::<G>(&mut query_group);
    query::run_benchmarks(&mut query_group, setup.clone());
    query_group.finish();

    // Run mutation operation benchmarks
    let mut mutation_group = c.benchmark_group("mutation_operations");
    configure_group::<G>(&mut mutation_group);
    mutation::run_benchmarks(&mut mutation_group, setup.clone());
    mutation_group.finish();

    // Run construction benchmarks
    let mut construction_group = c.benchmark_group("construction");
    configure_group::<G>(&mut construction_group);
    construction::run_benchmarks(&mut construction_group, setup.clone());
    construction_group.finish();

    // Run index operation benchmarks
    let mut index_group = c.benchmark_group("index_operations");
    configure_group::<G>(&mut index_group);
    index::run_benchmarks(&mut index_group, setup.clone());
    index_group.finish();

    // Run scaling benchmarks
    let mut scale_group = c.benchmark_group("scaling");
    configure_group::<G>(&mut scale_group);
    scale::run_benchmarks(&mut scale_group, setup.clone());
    scale_group.finish();
}

#[macro_export]
macro_rules! bench_suite {
    ($criterion:expr, $setup:expr) => {
        $crate::run_benchmarks($criterion, $setup);
    };
}

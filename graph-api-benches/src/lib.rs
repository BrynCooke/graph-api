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

use criterion::BenchmarkGroup;
use graph_api_test::{Edge, Vertex};
use std::time::Duration;

/// Configures the default settings for benchmark groups
pub fn configure_group(group: &mut BenchmarkGroup<criterion::measurement::WallTime>) {
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(50);
    group.warm_up_time(Duration::from_millis(500));
}

#[macro_export]
macro_rules! bench_suite {
    ($criterion:expr, $setup:expr) => {
        let criterion: &mut criterion::Criterion = $criterion;
        let setup = $setup;

        // Run vertex operation benchmarks
        let mut vertex_group = criterion.benchmark_group("vertex_operations");
        $crate::configure_group(&mut vertex_group);
        $crate::vertex::run_benchmarks(&mut vertex_group, setup.clone());
        vertex_group.finish();

        // Run edge operation benchmarks
        let mut edge_group = criterion.benchmark_group("edge_operations");
        $crate::configure_group(&mut edge_group);
        $crate::edge::run_benchmarks(&mut edge_group, setup.clone());
        edge_group.finish();

        // Run traversal operation benchmarks
        let mut traversal_group = criterion.benchmark_group("traversal_operations");
        $crate::configure_group(&mut traversal_group);
        $crate::traversal::run_benchmarks(&mut traversal_group, setup.clone());
        traversal_group.finish();

        // Run query operation benchmarks
        let mut query_group = criterion.benchmark_group("query_operations");
        $crate::configure_group(&mut query_group);
        $crate::query::run_benchmarks(&mut query_group, setup.clone());
        query_group.finish();

        // Run mutation operation benchmarks
        let mut mutation_group = criterion.benchmark_group("mutation_operations");
        $crate::configure_group(&mut mutation_group);
        $crate::mutation::run_benchmarks(&mut mutation_group, setup.clone());
        mutation_group.finish();

        // Run construction benchmarks
        let mut construction_group = criterion.benchmark_group("construction");
        $crate::configure_group(&mut construction_group);
        $crate::construction::run_benchmarks(&mut construction_group, setup.clone());
        construction_group.finish();

        // Run index operation benchmarks
        let mut index_group = criterion.benchmark_group("index_operations");
        $crate::configure_group(&mut index_group);
        $crate::index::bench_vertex_label_index(&mut index_group, setup.clone());
        $crate::index::bench_vertex_hash_index(&mut index_group, setup.clone());
        $crate::index::bench_vertex_fulltext_index(&mut index_group, setup.clone());
        $crate::index::bench_vertex_range_index(&mut index_group, setup.clone());
        $crate::index::bench_edge_label_index(&mut index_group, setup.clone());
        index_group.finish();

        // Run scaling benchmarks
        let mut scale_group = criterion.benchmark_group("scaling");
        $crate::configure_group(&mut scale_group);
        $crate::scale::run_benchmarks(&mut scale_group, setup.clone());
        scale_group.finish();
    };
}

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

        // Run vertex operation benchmarks
        let mut vertex_group = criterion.benchmark_group("vertex_operations");
        $crate::configure_group(&mut vertex_group);
        $crate::vertex::run_benchmarks(&mut vertex_group, $setup);
        vertex_group.finish();

        // Run edge operation benchmarks
        let mut edge_group = criterion.benchmark_group("edge_operations");
        $crate::configure_group(&mut edge_group);
        $crate::edge::run_benchmarks(&mut edge_group, $setup);
        edge_group.finish();

        // Run traversal operation benchmarks
        let mut traversal_group = criterion.benchmark_group("traversal_operations");
        $crate::configure_group(&mut traversal_group);
        $crate::traversal::run_benchmarks(&mut traversal_group, $setup);
        traversal_group.finish();

        // Run query operation benchmarks
        let mut query_group = criterion.benchmark_group("query_operations");
        $crate::configure_group(&mut query_group);
        $crate::query::run_benchmarks(&mut query_group, $setup);
        query_group.finish();

        // Run mutation operation benchmarks
        let mut mutation_group = criterion.benchmark_group("mutation_operations");
        $crate::configure_group(&mut mutation_group);
        $crate::mutation::run_benchmarks(&mut mutation_group, $setup);
        mutation_group.finish();

        // Run construction benchmarks
        let mut construction_group = criterion.benchmark_group("construction");
        $crate::configure_group(&mut construction_group);
        $crate::construction::run_benchmarks(&mut construction_group, $setup);
        construction_group.finish();

        // Run vertex label index benchmarks
        let mut vertex_label_group = criterion.benchmark_group("vertex_label_index");
        $crate::configure_group(&mut vertex_label_group);
        $crate::index::run_vertex_label_benchmarks(&mut vertex_label_group, $setup);
        vertex_label_group.finish();

        // Run vertex hash index benchmarks
        let mut vertex_hash_group = criterion.benchmark_group("vertex_hash_index");
        $crate::configure_group(&mut vertex_hash_group);
        $crate::index::run_vertex_hash_benchmarks(&mut vertex_hash_group, $setup);
        vertex_hash_group.finish();

        // Run vertex full-text index benchmarks
        let mut vertex_full_text_group = criterion.benchmark_group("vertex_full_text_index");
        $crate::configure_group(&mut vertex_full_text_group);
        $crate::index::run_vertex_full_text_benchmarks(&mut vertex_full_text_group, $setup);
        vertex_full_text_group.finish();

        // Run vertex range index benchmarks
        let mut vertex_range_group = criterion.benchmark_group("vertex_range_index");
        $crate::configure_group(&mut vertex_range_group);
        $crate::index::run_vertex_range_benchmarks(&mut vertex_range_group, $setup);
        vertex_range_group.finish();

        // Run edge label index benchmarks
        let mut edge_label_group = criterion.benchmark_group("edge_label_index");
        $crate::configure_group(&mut edge_label_group);
        $crate::index::run_edge_label_benchmarks(&mut edge_label_group, $setup);
        edge_label_group.finish();

        // Run no-index benchmarks (baseline performance)
        let mut no_index_group = criterion.benchmark_group("no_index");
        $crate::configure_group(&mut no_index_group);
        $crate::index::run_no_index_benchmarks(&mut no_index_group, $setup);
        no_index_group.finish();

        // Run scaling benchmarks
        let mut scale_group = criterion.benchmark_group("scaling");
        $crate::configure_group(&mut scale_group);
        $crate::scale::run_benchmarks(&mut scale_group, $setup);
        scale_group.finish();
    };
}

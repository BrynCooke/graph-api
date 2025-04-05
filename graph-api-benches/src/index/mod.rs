// Index benchmarks
pub mod edge_label;
pub mod no_index;
pub mod vertex_full_text;
pub mod vertex_hash;
pub mod vertex_label;
pub mod vertex_range;

// Re-export run functions
pub use edge_label::run_benchmarks as run_edge_label_benchmarks;
pub use no_index::run_benchmarks as run_no_index_benchmarks;
pub use vertex_full_text::run_benchmarks as run_vertex_full_text_benchmarks;
pub use vertex_hash::run_benchmarks as run_vertex_hash_benchmarks;
pub use vertex_label::run_benchmarks as run_vertex_label_benchmarks;
pub use vertex_range::run_benchmarks as run_vertex_range_benchmarks;

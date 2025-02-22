#![doc = include_str!("../../README.md")]
mod graph;
mod id;
mod index;
mod tombstone_vec;

pub use graph::SimpleGraph;
pub use id::EdgeId;
pub use id::VertexId;
#[cfg(test)]
mod test {
    use crate::SimpleGraph;
    use graph_api_test::test_suite;

    test_suite!(SimpleGraph::new());
}

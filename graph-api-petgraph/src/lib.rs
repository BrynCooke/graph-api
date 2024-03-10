#[cfg(test)]
mod test {
    use graph_api_test::test_suite;

    test_suite!(petgraph::stable_graph::StableGraph::new());
}

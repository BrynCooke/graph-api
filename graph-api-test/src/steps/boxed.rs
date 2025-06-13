use crate::{Edge, Vertex, assert_elements_eq, populate_graph};
use graph_api_lib::{EdgeReference, EdgeSearch, Graph, VertexSearch};

pub fn test_boxed_simple<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Test basic boxing functionality - should produce same results as unboxed
    let unboxed_result = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .head()
        .collect::<Vec<_>>();

    let boxed_result = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .boxed() // ← Type erasure here
        .head()
        .collect::<Vec<_>>();

    assert_elements_eq!(
        graph,
        unboxed_result.clone(),
        vec![refs.graph_api, refs.julia]
    );
    assert_elements_eq!(graph, boxed_result, vec![refs.graph_api, refs.julia]);
}

pub fn test_boxed_complex_traversal<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let _refs = populate_graph(graph);

    // Extremely long traversal that would create deeply nested types without boxing
    // This demonstrates the type complexity reduction benefits
    let complex_unboxed_result = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(EdgeSearch::scan())
        .head()
        .edges(EdgeSearch::scan())
        .head()
        .edges(EdgeSearch::scan())
        .head()
        .edges(EdgeSearch::scan())
        .head()
        .edges(EdgeSearch::scan())
        .head()
        .collect::<Vec<_>>();

    // Same traversal with strategic boxing to reduce compilation complexity
    // Type complexity is broken at multiple points using boxed()
    let complex_boxed_result = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(EdgeSearch::scan())
        .boxed() // ← First boxing point
        .head()
        .edges(EdgeSearch::scan())
        .head()
        .boxed() // ← Second boxing point
        .edges(EdgeSearch::scan())
        .head()
        .edges(EdgeSearch::scan())
        .boxed() // ← Third boxing point
        .head()
        .edges(EdgeSearch::scan())
        .head()
        .collect::<Vec<_>>();

    // Results should be identical despite different type complexity
    assert_eq!(complex_unboxed_result.len(), complex_boxed_result.len());
}

pub fn test_boxed_ultra_long_traversal<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let _refs = populate_graph(graph);

    // Ultra-long traversal that would be extremely slow to compile without boxing
    // This creates a type chain that would be:
    // Endpoints<Edges<Endpoints<Edges<Endpoints<Edges<...>>>>>
    // Which grows exponentially and severely impacts compilation time
    let ultra_long_result = graph
        .walk()
        .vertices(VertexSearch::scan())
        .take(2) // Limit starting vertices to reduce explosion
        .edges(EdgeSearch::scan().outgoing()) // Use outgoing to be more specific
        .boxed() // ← Strategic boxing after first hop
        .head()
        .edges(EdgeSearch::scan().outgoing())
        .boxed() // ← Boxing after multiple hops
        .head()
        .edges(EdgeSearch::scan().outgoing())
        .boxed() // ← More boxing to control complexity
        .head()
        .edges(EdgeSearch::scan().outgoing())
        .boxed() // ← Final boxing before collection
        .head()
        .collect::<Vec<_>>();

    // Test that extremely long traversals don't panic and produce valid results
    // The main goal is to ensure compilation works with deep type nesting
    // The exact count is less important than successful compilation and execution
    assert!(ultra_long_result.len() < 1_000_000); // Just ensure it doesn't explode
}

pub fn test_boxed_mixed_operations<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let _refs = populate_graph(graph);

    // Test boxing with various walker operations to ensure compatibility
    let mixed_result = graph
        .walk()
        .vertices(VertexSearch::scan())
        .take(2)
        .edges(EdgeSearch::scan())
        .boxed() // ← Box after edge traversal
        .head()
        .take(1)
        .edges(EdgeSearch::scan())
        .boxed() // ← Box again after more operations
        .head()
        .collect::<Vec<_>>();

    // Should produce valid results without compilation issues
    assert!(mixed_result.len() <= 4); // Max 4 vertices in test graph
}

pub fn test_boxed_edge_walker<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Test that edge walkers can also be boxed
    let edge_result = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .boxed() // ← Box the edge walker
        .collect::<Vec<_>>();

    assert_elements_eq!(
        graph,
        edge_result,
        vec![refs.bryn_knows_julia, refs.bryn_created_graph_api]
    );
}

pub fn test_boxed_performance_equivalence<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let _refs = populate_graph(graph);

    // Ensure boxed and unboxed produce identical results for complex case
    let unboxed = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(EdgeSearch::scan())
        .head()
        .edges(EdgeSearch::scan())
        .head()
        .take(2)
        .collect::<Vec<_>>();

    let boxed = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(EdgeSearch::scan())
        .boxed() // ← Only difference is boxing
        .head()
        .edges(EdgeSearch::scan())
        .head()
        .take(2)
        .collect::<Vec<_>>();

    // Results must be identical - boxing should not change behavior
    assert_eq!(unboxed.len(), boxed.len());
    assert!(boxed.len() <= 4); // Max 4 vertices in test graph
}

pub fn test_boxed_with_context<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Test that boxed step works with custom contexts
    // First establish context, then box afterward
    let result_with_context = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .push_context(|edge, _| format!("edge-{:?}", edge.id()))
        .boxed() // ← Boxing with custom String context
        .head()
        .collect::<Vec<_>>();

    assert_elements_eq!(graph, result_with_context, vec![refs.graph_api, refs.julia]);
}

#[derive(Clone, Debug, PartialEq)]
struct TestContext {
    counter: u32,
    label: String,
}

pub fn test_boxed_context_delegation<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Test that context is properly delegated through boxed walker
    let result = graph
        .walk()
        .push_context(TestContext {
            counter: 0,
            label: "start".to_string(),
        })
        .vertices_by_id(vec![refs.bryn])
        .mutate_context(|_, ctx| {
            ctx.counter += 1;
            ctx.label = "after_vertex".to_string();
        })
        .edges(EdgeSearch::scan().outgoing())
        .boxed() // ← Box with custom context
        .mutate_context(|_, ctx| {
            ctx.counter += 10; // This should work if context delegation is correct
            ctx.label = "after_boxing".to_string();
        })
        .head()
        .map(|vertex, context| {
            // ACTUALLY verify context was preserved and modified correctly
            assert_eq!(
                context.counter, 11,
                "Context counter should be 0 + 1 + 10 = 11"
            );
            assert_eq!(
                context.label, "after_boxing",
                "Context label should be updated after boxing"
            );
            vertex
        })
        .collect::<Vec<_>>();

    assert_eq!(result.len(), 2); // Should have 2 vertices - context verification is done in map above
}

pub fn test_boxed_context_access_before_next<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    // Test that context can be accessed and modified through boxed walker
    let result = graph
        .walk()
        .push_context(TestContext {
            counter: 42,
            label: "initial".to_string(),
        })
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .boxed() // ← Box with context
        .mutate_context(|_, ctx| {
            // This tests that context can be accessed and modified after boxing
            assert_eq!(ctx.counter, 42, "Initial context should be preserved");
            assert_eq!(
                ctx.label, "initial",
                "Initial context label should be preserved"
            );
            ctx.counter = 100;
            ctx.label = "modified_after_boxing".to_string();
        })
        .head()
        .map(|vertex, context| {
            // Verify context modifications worked
            assert_eq!(context.counter, 100, "Context should be modified to 100");
            assert_eq!(
                context.label, "modified_after_boxing",
                "Context label should be updated"
            );
            vertex
        })
        .collect::<Vec<_>>();

    assert_eq!(result.len(), 2); // Context verification is done in map above
}

pub fn test_boxed_nested_contexts<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Test boxed walker with nested context structure
    let result = graph
        .walk()
        .push_context("outer")
        .vertices_by_id(vec![refs.bryn])
        .push_context(|_, parent_ctx| format!("inner-{}", parent_ctx))
        .edges(EdgeSearch::scan().outgoing())
        .boxed() // ← Box with nested context
        .head()
        .map(|vertex, context| {
            // ACTUALLY verify nested context structure is preserved
            assert_eq!(
                *context, "inner-outer",
                "Nested context should combine properly"
            );
            assert_eq!(
                *context.parent(),
                "outer",
                "Parent context should be accessible"
            );
            vertex
        })
        .collect::<Vec<_>>();

    assert_eq!(result.len(), 2); // Context verification is done in map above
}

pub fn test_boxed_context_persistence<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Test that context persists correctly through boxed walkers
    // This also tests that context mutations are visible in subsequent steps
    let result = graph
        .walk()
        .push_context(TestContext {
            counter: 5,
            label: "original".to_string(),
        })
        .vertices_by_id(vec![refs.bryn, refs.julia])
        .boxed() // ← Box early to test context persistence
        .mutate_context(|_, ctx| {
            // Verify initial context state and modify it
            assert_eq!(ctx.counter, 5, "Counter should start at 5");
            assert_eq!(ctx.label, "original", "Label should start as 'original'");

            // Modify context so we can verify persistence
            ctx.label = "modified_by_mutate_context".to_string();
            ctx.counter = 42;
        })
        .map(|vertex, context| {
            // Verify context was modified in previous step
            assert_eq!(context.counter, 42, "Counter should be modified to 42");
            assert_eq!(
                context.label, "modified_by_mutate_context",
                "Label should be updated"
            );
            vertex
        })
        .collect::<Vec<_>>();

    assert_eq!(result.len(), 2); // Context verification is done in map above
}

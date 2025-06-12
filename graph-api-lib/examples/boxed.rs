use graph_api_lib::*;
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, Vertex, populate_graph};

fn example() {
    let mut graph = SimpleGraph::<Vertex, Edge>::new();
    populate_graph(&mut graph);

    // Example 1: Complex traversal without boxing (for comparison)
    // This creates deeply nested types like:
    // Endpoints<Edges<Endpoints<Edges<Vertices<Empty>>>>>
    println!("=== Without Boxing (Complex Types) ===");
    let _complex_result: Vec<_> = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(EdgeSearch::scan())
        .head()
        .edges(EdgeSearch::scan())
        .head()
        .collect();

    // Example 2: Strategic boxing to control type complexity
    println!("=== With Strategic Boxing ===");
    let boxed_result: Vec<_> = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(EdgeSearch::scan())
        .boxed() // ← Breaks type complexity here
        .head()
        .edges(EdgeSearch::scan())
        .boxed() // ← Further complexity reduction
        .head()
        .edges(EdgeSearch::scan())
        .head()
        .collect();

    println!(
        "Found {} vertices after complex traversal",
        boxed_result.len()
    );

    // Example 3: Boxing at different strategic points
    println!("=== Boxing at Logical Checkpoints ===");
    let checkpoint_result: Vec<_> = graph
        .walk()
        .vertices(VertexSearch::scan())
        .take(10)
        .boxed() // ← Box after initial filtering
        .edges(EdgeSearch::scan())
        .boxed() // ← Box after edge filtering
        .head()
        .collect();

    println!(
        "Found {} vertices after filtered traversal",
        checkpoint_result.len()
    );

    // Example 4: When NOT to use boxing (simple chains)
    println!("=== Simple Chains (No Boxing Needed) ===");
    let simple_result: Vec<_> = graph
        .walk()
        .vertices(VertexSearch::scan())
        .take(5) // Simple chain - no boxing needed
        .collect();

    println!("Found {} vertices in simple traversal", simple_result.len());

    // Example 5: Boxing for storage in data structures
    println!("=== Storing Walkers in Data Structures ===");

    // Without boxing, this would be very difficult due to complex types
    // Note: Different walker types need different iterators
    let vertex_walkers: Vec<Box<dyn Iterator<Item = _>>> = vec![
        Box::new(
            graph
                .walk()
                .vertices(VertexSearch::scan())
                .take(5)
                .boxed() // ← Makes storage possible
                .into_iter(),
        ),
        Box::new(
            graph
                .walk()
                .vertices(VertexSearch::scan())
                .take(3)
                .boxed() // ← Makes storage possible
                .into_iter(),
        ),
    ];

    println!("Stored {} different walker types", vertex_walkers.len());
}

fn main() {
    example();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxed_examples() {
        example(); // Just ensure it runs without panicking
    }

    #[test]
    fn test_compilation_time_comparison() {
        // This test demonstrates the type complexity difference
        let mut graph = SimpleGraph::<Vertex, Edge>::new();
        populate_graph(&mut graph);

        // Measure difference in compilation for complex vs boxed chains
        // (The real benefit is seen during actual compilation, not runtime)
        // Complex nested types (slower to compile)
        let _complex: Vec<_> = graph
            .walk()
            .vertices(VertexSearch::scan())
            .edges(EdgeSearch::scan())
            .head()
            .edges(EdgeSearch::scan())
            .head()
            .edges(EdgeSearch::scan())
            .head()
            .collect();

        // Boxed types (faster to compile)
        let _boxed: Vec<_> = graph
            .walk()
            .vertices(VertexSearch::scan())
            .edges(EdgeSearch::scan())
            .boxed()
            .head()
            .edges(EdgeSearch::scan())
            .boxed()
            .head()
            .edges(EdgeSearch::scan())
            .boxed()
            .head()
            .collect();
    }

    #[test]
    fn test_performance_characteristics() {
        let mut graph = SimpleGraph::<Vertex, Edge>::new();
        populate_graph(&mut graph);

        // Demonstrate that boxed walkers work identically to unboxed ones
        let unboxed: Vec<_> = graph
            .walk()
            .vertices(VertexSearch::scan())
            .edges(EdgeSearch::scan())
            .head()
            .collect();

        let boxed: Vec<_> = graph
            .walk()
            .vertices(VertexSearch::scan())
            .edges(EdgeSearch::scan())
            .boxed()
            .head()
            .collect();

        // Results should be identical (ignoring potential order differences)
        assert_eq!(unboxed.len(), boxed.len());
    }
}

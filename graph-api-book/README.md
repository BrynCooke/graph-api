# Graph API Book

This directory contains the documentation for the Graph API project, built
with [mdBook](https://rust-lang.github.io/mdBook/).

## Documentation Standards

### Writing Style

- Use simple, concise language that's easy to understand
- Prioritize clarity over technical jargon
- Use active voice rather than passive voice
- Break complex concepts into smaller, digestible sections
- Include introductory paragraphs that explain the purpose of each feature
- Target an audience of Rust developers who might be new to graph databases

### Code Examples

- **All code examples must be externalized in this crate**
- Create example files in the same directory as the md file that will reference them with the same name as the md file.
- Each example file should focus on a single concept
- Use the `include_fn` syntax to include examples in documentation:
  ```markdown
  {% include_fn ../examples/my_example.rs:my_function %}
  ```
- Examples should be self-contained and demonstrate practical usage
- Include comments that explain what the code is doing

### Example Structure

- Create a dedicated function for each example
- Function name should clearly describe what the example demonstrates
- Include enough context for the example to be understood in isolation
- Keep examples concise but complete

Example:

```rust
// Function demonstrates how to create and traverse a simple graph
pub fn simple_traversal_example() {
    let mut graph = SimpleGraph::new();

    // Add vertices
    let v1 = graph.add_vertex(Vertex::Person { name: "Alice" });
    let v2 = graph.add_vertex(Vertex::Person { name: "Bob" });

    // Add edge
    graph.add_edge(v1, v2, Edge::Knows);

    // Traverse graph
    let friends = graph.walk()
        .vertices_by_id(vec![v1])
        .edges(EdgeIndex::knows())
        .tail()
        .collect::<Vec<_>>();
}
```

### Documentation Organization

- Each major concept should have its own page
- Use consistent section hierarchy:
    1. Introduction/Overview
    2. Basic concepts
    3. Examples
    4. Advanced usage
    5. Best practices
- Cross-reference related pages when appropriate
- Include diagrams when they clarify concepts

### Walker Step Documentation

Walker steps should be documented with a consistent format:

1. **Title and Description**: Brief explanation of what the step does
2. **Syntax**: Shows how to call the step
3. **Parameters**: Description of all parameters
4. **Return Value**: What the step returns
5. **Example**: A clear, practical example
6. **Notes**: Additional information, edge cases, and common patterns

## Building the Book

To build and view the book locally:

1. Install mdBook: `cargo install mdbook`
2. Install the include-fn preprocessor: `cargo install mdbook-include-fn`
3. Build the book: `mdbook build`
4. View the book: `mdbook serve --open`

## Contributing

When contributing to the documentation:

1. Follow the style guidelines above
2. Create examples that are easy to understand
3. Test your examples to ensure they work correctly
4. Verify that the book builds without errors
5. Review your documentation for clarity and completeness
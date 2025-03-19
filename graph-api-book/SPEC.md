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
- Create the source in a directory of the same name as the md file. Each example will be placed in that directory with a
  filename that is descriptive of what the sample does..
  For example `indexes.md` will have examples in `indexes` in the same directory
- Each example file should focus on a single concept
- Use sections to hide code that is unrelated to the concept being described
- Examples should be self-contained and demonstrate practical usage
- Include comments that explain what the code is doing
- Do not place more than one example in a single rust markdown block. For instance, when describing efficient vs
  inefficient traversals sections must be used to break each traversal into it's own rust markdown block.

In addition, we will help the user by defining one graph model that is used over and over in the examples.
We will NOT create a new model for each example, instead we will use
the [standard model](src/standard_model.rs)
This model can be imported by all other examples.
In the case where the model diagram does not show the necessary information, e.g. biography for full text indexes,
a minimal portion of the model can be restated just to illustrate the feature

### Example Structure

- Create a dedicated file for each example with shared files for graph models as appropriate
- Function name should clearly describe what the example demonstrates
- Include enough context for the example to be understood in isolation
- Keep examples concise but complete
- Use the standard model where possible.
- Models should not be included inline in the example fn, but instead separated out.

Example an external rust file (file.rs):

```rust,noplayground
/* ANCHOR: all */

// ANCHOR: model
enum Vertex {
    Person {
        name: String
    }
}

enum Edge {
    Knows
}
// ANCHOR END: model

pub fn main() {
    // ANCHOR: setup
    let mut graph = SimpleGraph::new();

    // Add vertices
    let v1 = graph.add_vertex(Vertex::Person { name: "Alice" });
    let v2 = graph.add_vertex(Vertex::Person { name: "Bob" });

    // Add edge
    graph.add_edge(v1, v2, Edge::Knows);
    // ANCHOR END: setup

    // ANCHOR: traversal
    // Traverse graph
    let friends = graph.walk()
        .vertices_by_id(vec![v1])
        .edges(EdgeIndex::knows())
        .tail()
        .collect::<Vec<_>>();
    // ANCHOR END: traversal
}
```

Use the mdbook include mechanism to pull in portions of the file:

````markdown
Setup the graph:

```rust,noplayground
{{#include file.rs:setup}}
```

And then traverse it:

```rust,noplayground
{{#include file.rs:traversal}}
```
````

### Diagrams

Diagrams are rendered using svgbob. This is a nice way to provide illustrations via ascii art.

For instance here is a diagram:

```bob
    0       3
     *-------*      +y
  1 /|    2 /|       ^
   *-------* |       |
   | |4    | |7      | ◄╮
   | *-----|-*     ⤹ +-----> +x
   |/      |/       / ⤴
   *-------*       v
  5       6      +z
```

See the [svgbob](https://github.com/ivanceras/svgbob) repo for more details.

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

Walker steps should be documented with a consistent format.

See [SPEC.md](src/user_guide/walker/steps/SPEC.md) for more details

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
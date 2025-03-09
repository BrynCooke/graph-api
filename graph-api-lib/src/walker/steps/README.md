# Walker Steps Implementation Guidelines

This directory contains the implementation of all steps in the Graph API walker system. Each step should follow these
guidelines to maintain consistency across the codebase.

## Documentation Standards

Documentation for steps should be comprehensive and follow this structure:

1. **Use explicit doc comments**, not `include_str!` to import from the book
2. **Follow the format**:
    - Start with a title (`# Step Name`)
    - Brief description explaining the step
    - Visual diagram showing the effect of the step
    - Parameters section
    - Return value section
    - Examples section with real, working code
    - Notes section for additional context, edge cases, etc.

## Doc Example Integration

Use the `include_doc::function_body!` macro to include examples from the examples directory:

```rust
#[doc = function_body!("examples/your_step.rs", example, [])]
```

Where multiple examples are used `example` can be replaced with the function name.
Each example should show one thing, however make sure to not include duplicate or redundant examples.
Keep examples small and to the point, the goal is to communicate what the step does not to give use cases.

Where there is a vertex and edge implementation of the same step they should be documented in their own right
and examples given that are specific to edge or vertex.

This approach allows examples to be:

- Tested during CI
- Kept in sync between docs and example code
- Properly compiled and checked
- Existing examples should be migrated out of the inline documentation and moved to standalone examples.

## Step Implementation Pattern

Steps generally follow this pattern:

1. Implement for both `VertexWalkerBuilder` and `EdgeWalkerBuilder`
2. Thoroughly document the vertex version with complete details
3. For the edge version, reference the vertex documentation and note any differences

## Updating Existing Steps

When updating steps currently using `include_str!`:

1. **DO NOT** use `include_str!` to import docs from the book
2. Copy the content from the markdown file as necessary
3. Format properly as doc comments
4. Use `function_body!` for examples
5. Ensure the function signatures are properly documented

## New Steps

When creating a new step:

1. Add a new file named after the step (e.g., `new_step.rs`)
2. Add the step to `mod.rs`
3. Document thoroughly following the pattern in `collect.rs`
4. Create an example in the examples directory
5. Add tests for the step

## Improving for Better Docs

After implementing or updating a step, run:

```
cargo test --examples
```

To ensure that any added examples compile, and then:

```
cargo doc --open
```

To ensure the documentation renders correctly and completely.

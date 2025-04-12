# Walker Overview

Walkers are the central concept in Graph API traversals. They provide a fluent interface for exploring and manipulating
graph data.

## What are Walkers?

A walker represents a traversal through a graph. It consists of:

1. A **current position** in the graph (vertices or edges)
2. **Context data** that can be carried along the traversal
3. A sequence of **steps** that define the traversal path

Walkers use a builder pattern where each step returns a new walker with the accumulated operations.

## How Walkers Work

1. Start with an empty walker using `graph.walk()`
2. Chain steps to define your traversal: `.vertices().edges().tail()`
3. End with a terminal operation: `.collect()`, `.first()`, `.count()`, etc.

Each step in the chain modifies the traversal in a specific way, moving to different elements, filtering, or collecting
data.

## Types of Walkers

The Graph API has two main types of walkers:

1. **Vertex Walkers**: Traverse vertex elements
2. **Edge Walkers**: Traverse edge elements

Some operations switch between these types. For example, `.edges()` converts a vertex walker to an edge walker, while
`.head()` and `.tail()` convert an edge walker to a vertex walker.

## Walker States

A walker can be in one of several states:

1. **Empty**: No elements to traverse (starting state)
2. **Active**: Contains elements to traverse
3. **Terminal**: Has performed its final operation

## Creating Walkers

You create a walker by calling the `walk()` method on a graph:

```rust,noplayground
let walker = graph.walk();
```

This returns an empty walker that can be used to start your traversal.

## Starting Traversals

There are several ways to populate a walker with initial elements:

```rust,noplayground
// Start with all vertices
graph.walk().vertices(VertexSearch::scan())

// Start with specific vertex IDs
graph.walk().vertices_by_id(vec![vertex_id1, vertex_id2])

// Start with vertices matching criteria
graph.walk().vertices(VertexSearch::scan().with_label(Person::label()))
```

## Walker Flow Control

Walkers provide several ways to control the traversal flow:

- **Filtering**: Keep only elements matching a condition
- **Limiting**: Restrict the number of elements processed
- **Branching**: Create sub-traversals that explore different paths
- **Early Termination**: Stop traversal after finding a match

## Example Walker Traversals

### Basic Traversal

```rust,noplayground
{{#include basic_traversal_example.rs:basic_walker_example}}
```

### Multi-Step Traversal

```rust,noplayground
{{#include multi_step_example.rs:multi_step_example}}
```

### Traversal with Detour

```rust,noplayground
{{#include detour_example.rs:detour_traversal_example}}
```

## Next Steps

To learn more about specific walker steps, see the [Walker Steps](./steps.md) documentation.
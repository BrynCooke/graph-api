# Graph Traversal

## Overview

The Graph API provides a powerful traversal interface called "walkers" that enables you to navigate and analyze your
graph in a flexible, type-safe way. Walkers are the primary method for querying and manipulating graph data.

## Basic Concepts

Walkers are built by chaining steps that define how to traverse the graph. Each step performs a specific operation, such
as:

- Starting at specific vertices
- Moving to connected edges
- Filtering elements
- Collecting data
- Modifying the graph

## Traversal Order (Depth-First)

By default, the Graph API walker performs a **Depth-First Search (DFS)** traversal. This means it explores as far as
possible along each branch before backtracking.

<object type="image/svg+xml" data="traversal/dfs_traversal_image.svg">
Diagram illustrating Depth-First Search traversal order
</object>

In the diagram above:

- The traversal starts at node **A**.
- It explores the **left branch** first: **A → B → D → E**.
- After reaching the end of the left branch (E), it **backtracks** to B, then A.
- It then explores the **right branch**: **A → C → F**.
- The **final DFS order** is indicated by the numbers: **A(1) → B(2) → D(3) → E(4) → C(5) → F(6)**.
- The **orange arrows** highlight the path taken during the traversal.

This DFS behaviour is fundamental to how walkers navigate the graph.

## Examples

### Basic Traversal

Here's an example that finds all Project vertices that were created by someone that a Person knows:

```rust,noplayground
{{#include traversal/basic_example.rs:all}}
```

This traversal:

1. Starts with Person vertices
2. Follows "knows" edges to find friends
3. Follows "created" edges from friends
4. Filters for Project vertices
5. Collects the results

### Working with Context

Walkers have a built-in context system that allows you to store and transform data as you traverse the graph:

```rust,noplayground
{{#include traversal/context_example.rs:all}}
```

In this example, we:

1. Start with a specific person
2. Find all friends (via "knows" edges)
3. Filter for Person vertices
4. Store each person's age in the context
5. Sum up all the ages

## Walker Steps

The Graph API provides many steps that can be combined to create complex traversals. Each step is documented with:

See the [Walker Steps](./walker/steps.md) section for detailed documentation on each available step.

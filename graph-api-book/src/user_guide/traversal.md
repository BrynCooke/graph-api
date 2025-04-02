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

- A description of what it does
- Visual diagrams showing before and after states
- Examples of how to use it
- Information about parameters and return values

See the [Walker Steps](./walker/steps.md) section for detailed documentation on each available step.

## Common Step Categories

### Traversal Position Steps

- [vertices](./walker/steps/vertices.md) - Start traversal from vertices
- [edges](./walker/steps/edges.md) - Move to edges
- [head](./walker/steps/head.md) - Move to source vertices of edges
- [tail](./walker/steps/tail.md) - Move to target vertices of edges

### Filtering Steps

- [filter](./walker/steps/filter.md) - Keep only elements that match a predicate
- [limit](./walker/steps/limit.md) - Limit the number of elements
- [first](./walker/steps/first.md) - Take only the first element

### Context Steps

- [context](./walker/steps/context.md) - Store data in context
- [default_context](./walker/steps/default_context.md) - Set default context

### Collection Steps

- [collect](./walker/steps/collect.md) - Collect elements into a collection
- [count](./walker/steps/count.md) - Count elements
- [into_iter](./walker/steps/into_iter.md) - Iterate over elements

### Advanced Steps

- [detour](./walker/steps/detour.md) - Create sub-traversals
- [mutate](./walker/steps/mutate.md) - Modify the graph during traversal
- [probe](./walker/steps/probe.md) - Debug traversal state
- [map](./walker/steps/map.md) - Transform elements during traversal

## Advanced Usage

The context system is particularly useful for:

1. Collecting metadata during traversal
2. Building complex data structures
3. Tracking traversal history
4. Performing calculations based on visited elements

See the [Context System](./walker/context_system) page for more details.

## Best Practices

For more information about effectively using walkers, see the [Best Practices](./walker/best_practices.md) guide.

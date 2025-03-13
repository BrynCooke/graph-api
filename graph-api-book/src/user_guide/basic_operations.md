# Basic Operations

Once you've defined your graph model, you can perform basic operations like adding vertices and edges, querying the
graph, and modifying elements. This guide covers the fundamental operations available in Graph API.

## Creating a Graph

To start working with graphs, first create a new graph instance:

{% include_fn ./create_graph_example.rs:create_graph_example %}

## Querying Vertices

You can query vertices using the walker API:

{% include_fn ./walker/steps/vertices.rs:vertices_step_example %}

## Working with Edges

You can work with edges using the walker API:

{% include_fn ./walker/steps/edges.rs:edges_step_example %}

## Modifying the Graph

You can modify the graph using the mutation API:

{% include_fn ./walker/steps/mutate.rs:mutate_example %}

## Next Steps

Now that you understand the basic operations, you can:

1. Learn about [Advanced Traversals](./traversal.md) using the Walker API
2. Explore [Context and State](./walker/context.md) in graph traversals
3. See [Real-world Examples](./walker/steps.md) of graph operations
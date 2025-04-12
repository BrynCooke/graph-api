# Summary

[Introduction](./introduction.md)

# User Guide

- [Getting Started](./user_guide/getting_started.md)
- [Defining a Model](./user_guide/defining_a_model.md)
- [Property Graphs](./user_guide/property_graphs.md)
    - [Exploring Without Indexes](./user_guide/property_graphs/no_index.md)
    - [Hash Indexes](./user_guide/property_graphs/hash_index.md)
    - [Range Indexes](./user_guide/property_graphs/range_index.md)
    - [Full-text Indexes](./user_guide/property_graphs/full_text_index.md)
- [Derive Macros](./user_guide/derive_macros.md)
- [Basic Operations](./user_guide/basic_operations.md)
- [Graph Traversal](./user_guide/traversal.md)
    - [Walker Overview](./user_guide/walker/overview.md)
    - [Walker Steps](./user_guide/walker/steps.md)
        - [vertices](./user_guide/walker/steps/vertices.md)
        - [vertices_by_id](./user_guide/walker/steps/vertices_by_id.md)
        - [edges](./user_guide/walker/steps/edges.md)
        - [head](./user_guide/walker/steps/head.md)
        - [tail](./user_guide/walker/steps/tail.md)
        - [filter](./user_guide/walker/steps/filter.md)
        - [control_flow](./user_guide/walker/steps/control_flow.md)
        - [map](./user_guide/walker/steps/map.md)
        - [fold](./user_guide/walker/steps/fold.md)
        - [reduce](./user_guide/walker/steps/reduce.md)
        - [take](./user_guide/walker/steps/take.md)
        - [first](./user_guide/walker/steps/first.md)
        - [context](./user_guide/walker/steps/context.md)
        - [default_context](./user_guide/walker/steps/default_context.md)
        - [mutate_context](./user_guide/walker/steps/mutate_context.md)
        - [detour](./user_guide/walker/steps/detour.md)
        - [collect](./user_guide/walker/steps/collect.md)
        - [count](./user_guide/walker/steps/count.md)
        - [into_iter](./user_guide/walker/steps/into_iter.md)
        - [probe](./user_guide/walker/steps/probe.md)
        - [mutate](./user_guide/walker/steps/mutate.md)
        - [dbg](./user_guide/walker/steps/dbg.md)
    - [Context System](./user_guide/walker/context_system.md)
    - [Best Practices](./user_guide/walker/best_practices.md)

# Implementation Guide

- [Implementation Guide for Graph Backends](./implementation/guide.md)
- [Creating a Graph Implementation](./implementation/graphs.md)
- [Testing Your Implementation](./implementation/testing.md)
- [Features and Extensions](./implementation/features.md)
- [Implementing Indexes](./implementation/indexes.md)
- [Performance Considerations](./implementation/performance.md)
- [Benchmarking](./implementation/benchmarks.md)

# Reference

- [API Reference](./reference/api.md)
- [Support Traits](./reference/support_traits.md)
- [Derive Macros](./reference/derive_macros.md)
- [Existing Implementations](./reference/implementations.md)
    - [SimpleGraph](./reference/implementations/simple_graph.md)
    - [PetGraph](./reference/implementations/pet_graph.md)



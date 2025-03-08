# Mutate Step

The `mutate` step allows you to modify the graph during traversal. For each element in the traversal (vertex or edge), the provided callback function is executed, giving you the ability to create, modify, or delete elements in the graph.

## Visual Diagram

Before mutate step (traversal position on Person vertices):
```text
  [Person A]*        [Project X]
       |
  [Person B]*
```

After mutate step (adding 'Created' edges to Project X):
```text
  [Person A]* ---- Created ----> [Project X]
       |
  [Person B]* ---- Created ----> [Project X]
```

## Parameters

- `callback`: A function that receives:
  - A mutable reference to the graph
  - The element ID (vertex ID or edge ID)
  - The context for the current element

## Requirements

- Must use `walk_mut()` instead of `walk()` to get a mutable graph reference
- Works with both vertex and edge traversals
- The traversal is collected before mutations are applied to avoid interference

## Return Value

Returns the number of elements that were modified (traversed and passed to the callback).

## Examples

```rust
# use graph_api_test::populate_graph;
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::VertexIndex;
# use graph_api_test::EdgeIndex;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use graph_api_lib::VertexSearch;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::Element;
# 
# // Create a mutable graph for testing
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);
# // Add a new project node to demonstrate mutations
# let new_project_id = graph.add_vertex(Vertex::Project(Project { name: "NewProject".to_string() }));

// Example 1: Add 'Created' edges from people to a project
let mutations_count = graph
    .walk_mut() // Must use walk_mut for mutations
    .vertices(VertexIndex::person())
    .mutate(|graph, person_id, _| {
        // Add a 'Created' edge from each person to the new project
        graph.add_edge(person_id, new_project_id, Edge::Created);
    });

// Should have created edges for all people in the graph (at least 2)
assert!(mutations_count >= 2);

// Example 2: Work with edges
let project_edges_count = graph
    .walk_mut()
    .vertices_by_id(vec![new_project_id])
    .edges(EdgeSearch::scan().incoming())
    .mutate(|graph, edge_id, _| {
        // In a real implementation, you might modify edge properties here
        // For test purposes, we're just accessing it
        if let Some(_edge) = graph.edge_mut(edge_id) {
            // Verification/modification would happen here
        }
    });

// Should match the number of people that created the project
assert_eq!(project_edges_count, mutations_count);

// Example 3: Use filter_by_person for type-safe filtering
let updated_count = graph
    .walk_mut()
    .vertices(VertexIndex::person())
    .filter_by_person(|person, _| person.name() == "Bryn")
    .mutate(|graph, person_id, _| {
        // In a real implementation, we might update person attributes here
        // For test purposes, we just verify we can get the vertex
        if let Some(_person) = graph.vertex_mut(person_id) {
            // Mutation would happen here
        }
    });

// Should have found at least one match (Bryn)
assert!(updated_count > 0);
```

## Notes

- The `mutate` step doesn't change the traversal position or context
- All modifications happen to the graph itself, not to the traversal
- The traversal is completed first, then mutations are applied afterward to avoid traversal interference
- Be cautious when performing mutations that might interact with each other
- For better performance with large traversals, consider using `limit()` before `mutate()`
- Can be used to implement complex graph algorithms like community detection, path finding, or graph rewriting
- Related steps: `filter()` for conditional traversal, `collect()` for materializing results
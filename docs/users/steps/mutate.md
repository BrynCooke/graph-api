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

### Simple Example

```rust
// Add a 'Created' edge from each person to the graph_api project
let mutations = graph
    .walk_mut()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .mutate(|graph, vertex_id, _context| {
        graph.add_edge(vertex_id, graph_api_id, Edge::Created);
    });

println!("Added {} edges", mutations);
```

### Complex Example

```rust
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_derive::VertexExt;
# use graph_api_derive::EdgeExt;
# use uuid::Uuid;
# use graph_api_lib::Id;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use std::ops::Deref;
# use graph_api_lib::VertexSearch;
# let mut graph = SimpleGraph::new();

// Identify people who know each other but don't have "Collaborates" edges,
// and add those edges when they both worked on the same project
let mutations = graph
    .walk_mut()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .edges(EdgeSearch::scan().outgoing().with_label(Edge::knows_label()))
    .head() // Get the destination person of the "knows" edge
    .mutate(|graph, target_person_id, context| {
        // Get the source person (who initiated the "knows" edge)
        let source_person_id = context.edge().unwrap().source_id();
        
        // Check if both people have created the same project
        let source_projects = graph.walk()
            .vertices_by_id(vec![source_person_id])
            .edges(EdgeSearch::scan().outgoing().with_label(Edge::created_label()))
            .head()
            .collect::<Vec<_>>();
            
        let target_projects = graph.walk()
            .vertices_by_id(vec![target_person_id])
            .edges(EdgeSearch::scan().outgoing().with_label(Edge::created_label()))
            .head()
            .collect::<Vec<_>>();
            
        // Find common projects
        for source_proj in &source_projects {
            if target_projects.contains(source_proj) {
                // Add a "Collaborates" edge if it doesn't exist already
                let existing = graph.walk()
                    .vertices_by_id(vec![source_person_id])
                    .edges(EdgeSearch::scan().outgoing().with_label(Edge::collaborates_label()))
                    .head()
                    .filter(|v| v.id() == target_person_id)
                    .count();
                    
                if existing == 0 {
                    graph.add_edge(source_person_id, target_person_id, Edge::Collaborates);
                }
                
                break;
            }
        }
    });
```

## Notes

- The `mutate` step doesn't change the traversal position or context
- All modifications happen to the graph itself, not to the traversal
- The traversal is completed first, then mutations are applied afterward to avoid traversal interference
- Be cautious when performing mutations that might interact with each other
- For better performance with large traversals, consider using `limit()` before `mutate()`
- Can be used to implement complex graph algorithms like community detection, path finding, or graph rewriting
- Related steps: `filter()` for conditional traversal, `collect()` for materializing results
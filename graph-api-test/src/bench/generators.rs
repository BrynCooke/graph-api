use crate::{Edge, Refs, Vertex, populate_graph};
use graph_api_lib::Graph;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::collections::HashSet;
use uuid::Uuid;

/// Graph size configurations
#[derive(Clone, Copy)]
pub enum GraphSize {
    Small,  // ~100 vertices, ~300 edges
    Medium, // ~1,000 vertices, ~3,000 edges
    Large,  // ~10,000 vertices, ~30,000 edges
    Huge,   // ~100,000 vertices, ~300,000 edges
}

impl GraphSize {
    pub fn vertex_count(&self) -> usize {
        match self {
            GraphSize::Small => 100,
            GraphSize::Medium => 1_000,
            GraphSize::Large => 10_000,
            GraphSize::Huge => 100_000,
        }
    }

    pub fn edge_multiplier(&self) -> usize {
        3 // Each vertex has on average 3 edges
    }
}

/// Generates a simple populated graph with the standard test data
pub fn generate_test_graph<G>(graph: &mut G) -> Refs<G>
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    populate_graph(graph)
}

/// Generates a random graph with the specified size
pub fn generate_random_graph<G>(graph: &mut G, size: GraphSize, seed: u64) -> Vec<G::VertexId>
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    let mut rng = StdRng::seed_from_u64(seed);
    let vertex_count = size.vertex_count();
    let edge_count = vertex_count * size.edge_multiplier();

    // Generate vertices
    let mut vertex_ids = Vec::with_capacity(vertex_count);
    for i in 0..vertex_count {
        let vertex = match rng.gen_range(0..3) {
            0 => Vertex::Person {
                name: format!("Person-{}", i),
                age: rng.gen_range(18..80),
                unique_id: Uuid::new_v4(),
                username: format!("user_{}", i),
                biography: format!("Bio for person {}: {}", i, random_biography(&mut rng)),
            },
            1 => Vertex::Project(crate::Project {
                name: format!("Project-{}", i),
            }),
            _ => Vertex::Rust,
        };

        vertex_ids.push(graph.add_vertex(vertex));
    }

    // Generate edges
    let mut added_edges = HashSet::new();
    for _ in 0..edge_count {
        // Pick random source and target vertices
        let src_idx = rng.gen_range(0..vertex_count);
        let tgt_idx = rng.gen_range(0..vertex_count);
        let src = vertex_ids[src_idx];
        let tgt = vertex_ids[tgt_idx];

        // Skip self-loops and duplicates
        if src == tgt || added_edges.contains(&(src, tgt)) {
            continue;
        }

        // Add edge with random type
        let edge = match rng.gen_range(0..3) {
            0 => Edge::Knows {
                since: rng.gen_range(1980..2023),
            },
            1 => Edge::Created,
            _ => Edge::Language(crate::Language {
                name: match rng.gen_range(0..4) {
                    0 => "Rust".to_string(),
                    1 => "Java".to_string(),
                    2 => "Python".to_string(),
                    _ => "JavaScript".to_string(),
                },
            }),
        };

        graph.add_edge(src, tgt, edge);
        added_edges.insert((src, tgt));
    }

    vertex_ids
}

/// Generates a social network like graph
pub fn generate_social_graph<G>(graph: &mut G, size: GraphSize, seed: u64) -> Vec<G::VertexId>
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    let mut rng = StdRng::seed_from_u64(seed);
    let vertex_count = size.vertex_count();

    // Generate vertices (all people)
    let mut vertex_ids = Vec::with_capacity(vertex_count);
    for i in 0..vertex_count {
        let vertex = Vertex::Person {
            name: format!("Person-{}", i),
            age: rng.gen_range(18..80),
            unique_id: Uuid::new_v4(),
            username: format!("user_{}", i),
            biography: format!("Social network user {}", i),
        };

        vertex_ids.push(graph.add_vertex(vertex));
    }

    // Create a social network structure - people tend to know people nearby in the list
    // This creates a more realistic social graph structure than purely random connections
    for i in 0..vertex_count {
        // Each person knows 5-15 other people
        let num_connections = rng.gen_range(5..15).min(vertex_count - 1);
        let mut connections = HashSet::new();

        while connections.len() < num_connections {
            // 80% chance to connect to someone "nearby" (community structure)
            let target_idx = if rng.gen_bool(0.8) {
                // Connect to someone nearby in the list (community)
                let range = 50; // community size
                let start = i.saturating_sub(range / 2);
                let end = (i + range / 2).min(vertex_count - 1);
                rng.gen_range(start..=end)
            } else {
                // Random connection anywhere
                rng.gen_range(0..vertex_count)
            };

            // Avoid self-loops
            if target_idx != i {
                connections.insert(target_idx);
            }
        }

        // Create edges for connections
        for target_idx in connections {
            let target = vertex_ids[target_idx];
            let year = rng.gen_range(1980..2023);
            graph.add_edge(vertex_ids[i], target, Edge::Knows { since: year });
        }
    }

    vertex_ids
}

/// Generate a project dependency graph
pub fn generate_project_graph<G>(graph: &mut G, size: GraphSize, seed: u64) -> Vec<G::VertexId>
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    let mut rng = StdRng::seed_from_u64(seed);
    let project_count = size.vertex_count() / 3; // 1/3 projects
    let person_count = size.vertex_count() / 3; // 1/3 people
    let rust_count = size.vertex_count() - project_count - person_count; // 1/3 Rust (language nodes)

    // Generate project vertices
    let mut project_ids = Vec::with_capacity(project_count);
    for i in 0..project_count {
        let vertex = Vertex::Project(crate::Project {
            name: format!("Project-{}", i),
        });
        project_ids.push(graph.add_vertex(vertex));
    }

    // Generate person vertices
    let mut person_ids = Vec::with_capacity(person_count);
    for i in 0..person_count {
        let vertex = Vertex::Person {
            name: format!("Developer-{}", i),
            age: rng.gen_range(18..80),
            unique_id: Uuid::new_v4(),
            username: format!("dev_{}", i),
            biography: format!("Developer working on project {}", i % project_count),
        };
        person_ids.push(graph.add_vertex(vertex));
    }

    // Generate language vertices (all Rust in this case)
    let mut rust_ids = Vec::with_capacity(rust_count);
    for _ in 0..rust_count {
        rust_ids.push(graph.add_vertex(Vertex::Rust));
    }

    // Create project dependencies (project -> project edges)
    for (i, &project_id) in project_ids.iter().enumerate() {
        // Each project depends on 0-5 other projects
        let num_deps = rng.gen_range(0..=5).min(project_count - 1);
        let mut deps = HashSet::new();

        while deps.len() < num_deps {
            let target_idx = rng.gen_range(0..project_count);
            if target_idx != i {
                // Avoid self-dependencies
                deps.insert(target_idx);
            }
        }

        // Create dependency edges
        for target_idx in deps {
            let target = project_ids[target_idx];
            graph.add_edge(project_id, target, Edge::Created);
        }
    }

    // Create author relationships (person -> project edges)
    for (i, &person_id) in person_ids.iter().enumerate() {
        // Each person contributes to 1-3 projects
        let num_projects = rng.gen_range(1..=3).min(project_count);
        let mut projects = HashSet::new();

        // First project is often related to the person's index (simulates main project)
        projects.insert(i % project_count);

        // Add more random projects
        while projects.len() < num_projects {
            projects.insert(rng.gen_range(0..project_count));
        }

        // Create author edges
        for &project_idx in &projects {
            let project_id = project_ids[project_idx];
            graph.add_edge(person_id, project_id, Edge::Created);
        }
    }

    // Create language relationships (project -> language edges)
    for &project_id in &project_ids {
        // Each project uses 1-2 languages
        let num_langs = rng.gen_range(1..=2).min(rust_count);

        for _ in 0..num_langs {
            let lang_idx = rng.gen_range(0..rust_count);
            let lang_id = rust_ids[lang_idx];

            graph.add_edge(
                project_id,
                lang_id,
                Edge::Language(crate::Language {
                    name: "Rust".to_string(),
                }),
            );
        }
    }

    // Combine all vertex IDs and return
    let mut all_vertices = Vec::with_capacity(size.vertex_count());
    all_vertices.extend(project_ids);
    all_vertices.extend(person_ids);
    all_vertices.extend(rust_ids);
    all_vertices
}

/// Helper to generate random biography text
fn random_biography(rng: &mut StdRng) -> String {
    let adjectives = [
        "creative",
        "diligent",
        "innovative",
        "experienced",
        "passionate",
    ];
    let roles = ["developer", "engineer", "architect", "programmer", "coder"];
    let interests = [
        "graph databases",
        "distributed systems",
        "machine learning",
        "web development",
        "mobile apps",
    ];

    format!(
        "A {} {} interested in {}",
        adjectives.choose(rng).unwrap(),
        roles.choose(rng).unwrap(),
        interests.choose(rng).unwrap()
    )
}

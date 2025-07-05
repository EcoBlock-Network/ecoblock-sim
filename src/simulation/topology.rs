///! Define standard topologies for simulation

/// Return a list of directed edges representing a linear topology
/// Example: A -> B -> C -> D
pub fn linear_topology() -> Vec<(&'static str, &'static str)> {
    vec![
        ("A", "B"),
        ("B", "C"),
        ("C", "D"),
    ]
}

/// Return a list of directed edges representing a ring topology
/// Example: A -> B -> C -> D -> A
pub fn ring_topology() -> Vec<(&'static str, &'static str)> {
    vec![
        ("A", "B"),
        ("B", "C"),
        ("C", "D"),
        ("D", "A"),
    ]
}

/// Return a list of directed edges for a full mesh
/// Each node is connected to every other node (except self)
pub fn full_mesh(nodes: &[&'static str]) -> Vec<(&'static str, &'static str)> {
    let mut edges = Vec::new();
    for &from in nodes {
        for &to in nodes {
            if from != to {
                edges.push((from, to));
            }
        }
    }
    edges
}

/// Return a list of directed edges representing a binary tree topology
/// Example:
///        A
///      /   \
///     B     C
///    / \   / \
///   D   E F   G
pub fn tree_topology() -> Vec<(&'static str, &'static str)> {
    vec![
        ("A", "B"), ("A", "C"),
        ("B", "D"), ("B", "E"),
        ("C", "F"), ("C", "G"),
    ]
}

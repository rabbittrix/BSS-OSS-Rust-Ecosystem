//! Dependency Management for Order Fulfillment

use uuid::Uuid;

/// Dependency graph for order fulfillment
pub struct DependencyGraph {
    nodes: Vec<DependencyNode>,
}

/// Dependency node
pub struct DependencyNode {
    pub id: Uuid,
    pub dependencies: Vec<Uuid>,
    pub dependents: Vec<Uuid>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn add_node(&mut self, id: Uuid, dependencies: Vec<Uuid>) {
        // Update dependents for each dependency
        for dep_id in &dependencies {
            if let Some(dep_node) = self.nodes.iter_mut().find(|n| n.id == *dep_id) {
                dep_node.dependents.push(id);
            }
        }

        self.nodes.push(DependencyNode {
            id,
            dependencies,
            dependents: vec![],
        });
    }

    pub fn get_ready_nodes(&self, completed: &[Uuid]) -> Vec<Uuid> {
        self.nodes
            .iter()
            .filter(|node| {
                // All dependencies are completed
                node.dependencies
                    .iter()
                    .all(|dep_id| completed.contains(dep_id))
            })
            .map(|node| node.id)
            .collect()
    }

    pub fn get_dependents(&self, node_id: Uuid) -> Vec<Uuid> {
        self.nodes
            .iter()
            .find(|n| n.id == node_id)
            .map(|n| n.dependents.clone())
            .unwrap_or_default()
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

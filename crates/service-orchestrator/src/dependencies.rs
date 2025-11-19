//! Service Dependency Management

use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Service dependency relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDependency {
    /// ID of the service that has the dependency
    pub service_id: Uuid,
    /// ID of the service specification that this service depends on
    pub depends_on_spec_id: Uuid,
    /// Type of dependency
    pub dependency_type: DependencyType,
    /// Whether the dependency is required
    pub required: bool,
}

/// Type of service dependency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DependencyType {
    /// Service requires another service to be active
    RequiresActive,
    /// Service requires another service to be provisioned
    RequiresProvisioned,
    /// Service requires another service to be configured
    RequiresConfigured,
    /// Service is optional enhancement of another service
    OptionalEnhancement,
}

/// Service dependency graph
pub struct ServiceDependencyGraph {
    nodes: Vec<ServiceDependencyNode>,
}

/// Service dependency node
struct ServiceDependencyNode {
    pub service_spec_id: Uuid,
    pub service_id: Option<Uuid>,
    pub dependencies: Vec<Uuid>, // IDs of service specs this depends on
    pub dependents: Vec<Uuid>,   // IDs of service specs that depend on this
    pub state: DependencyNodeState,
}

/// Dependency node state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DependencyNodeState {
    NotProvisioned,
    Provisioning,
    Active,
    Inactive,
}

impl ServiceDependencyGraph {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    /// Load dependencies from database
    pub async fn load_from_db(pool: &PgPool) -> Result<Self, sqlx::Error> {
        let mut graph = Self::new();

        // Load all service dependencies
        let rows = sqlx::query(
            "SELECT service_specification_id, depends_on_specification_id, dependency_type, required
             FROM service_dependencies",
        )
        .fetch_all(pool)
        .await?;

        let mut spec_deps: std::collections::HashMap<Uuid, Vec<Uuid>> =
            std::collections::HashMap::new();
        for row in rows {
            let spec_id: Uuid = row.get(0);
            let dep_spec_id: Uuid = row.get(1);
            spec_deps.entry(spec_id).or_default().push(dep_spec_id);
        }

        // Load service specification states
        let state_rows = sqlx::query(
            "SELECT service_specification_id, service_id, state
             FROM service_specification_states",
        )
        .fetch_all(pool)
        .await?;

        let mut spec_states: std::collections::HashMap<Uuid, (Option<Uuid>, DependencyNodeState)> =
            std::collections::HashMap::new();
        for row in state_rows {
            let spec_id: Uuid = row.get(0);
            let service_id: Option<Uuid> = row.get(1);
            let state_str: String = row.get(2);
            let state = match state_str.as_str() {
                "NOT_PROVISIONED" => DependencyNodeState::NotProvisioned,
                "PROVISIONING" => DependencyNodeState::Provisioning,
                "ACTIVE" => DependencyNodeState::Active,
                "INACTIVE" => DependencyNodeState::Inactive,
                _ => DependencyNodeState::NotProvisioned,
            };
            spec_states.insert(spec_id, (service_id, state));
        }

        // Build graph nodes
        for (spec_id, deps) in spec_deps {
            let (service_id, state) = spec_states
                .get(&spec_id)
                .cloned()
                .unwrap_or((None, DependencyNodeState::NotProvisioned));
            graph.nodes.push(ServiceDependencyNode {
                service_spec_id: spec_id,
                service_id,
                dependencies: deps.clone(),
                dependents: vec![],
                state,
            });
        }

        // Build dependents relationships
        // First collect all (dependent_id, dependency_id) pairs
        let mut dependent_pairs: Vec<(Uuid, Uuid)> = Vec::new();
        for node in &graph.nodes {
            for dep_spec_id in &node.dependencies {
                dependent_pairs.push((node.service_spec_id, *dep_spec_id));
            }
        }

        // Then update dependents for each dependency
        for (dependent_id, dep_spec_id) in dependent_pairs {
            if let Some(dep_node) = graph
                .nodes
                .iter_mut()
                .find(|n| n.service_spec_id == dep_spec_id)
            {
                dep_node.dependents.push(dependent_id);
            }
        }

        Ok(graph)
    }

    /// Save dependencies to database
    pub async fn save_to_db(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        // Clear existing dependencies (in a real implementation, you'd want to merge/update)
        sqlx::query("DELETE FROM service_dependencies")
            .execute(pool)
            .await?;

        // Insert all dependencies
        for node in &self.nodes {
            for dep_spec_id in &node.dependencies {
                sqlx::query(
                    "INSERT INTO service_dependencies (service_specification_id, depends_on_specification_id, dependency_type, required)
                     VALUES ($1, $2, $3, $4)
                     ON CONFLICT (service_specification_id, depends_on_specification_id) DO NOTHING",
                )
                .bind(node.service_spec_id)
                .bind(dep_spec_id)
                .bind("REQUIRES_ACTIVE")
                .bind(true)
                .execute(pool)
                .await?;
            }
        }

        // Update service specification states
        for node in &self.nodes {
            let state_str = match node.state {
                DependencyNodeState::NotProvisioned => "NOT_PROVISIONED",
                DependencyNodeState::Provisioning => "PROVISIONING",
                DependencyNodeState::Active => "ACTIVE",
                DependencyNodeState::Inactive => "INACTIVE",
            };

            sqlx::query(
                "INSERT INTO service_specification_states (service_specification_id, service_id, state, updated_at)
                 VALUES ($1, $2, $3, NOW())
                 ON CONFLICT (service_specification_id)
                 DO UPDATE SET service_id = $2, state = $3, updated_at = NOW()",
            )
            .bind(node.service_spec_id)
            .bind(node.service_id)
            .bind(state_str)
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    /// Load dependencies for a specific service specification from database
    pub async fn load_dependencies_for_spec(
        pool: &PgPool,
        service_spec_id: Uuid,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT depends_on_specification_id
             FROM service_dependencies
             WHERE service_specification_id = $1",
        )
        .bind(service_spec_id)
        .fetch_all(pool)
        .await?;

        Ok(rows.iter().map(|row| row.get(0)).collect())
    }

    /// Add a service specification with its dependencies
    pub fn add_service_spec(&mut self, service_spec_id: Uuid, dependencies: Vec<Uuid>) {
        // Update dependents for each dependency
        for dep_spec_id in &dependencies {
            if let Some(dep_node) = self
                .nodes
                .iter_mut()
                .find(|n| n.service_spec_id == *dep_spec_id)
            {
                dep_node.dependents.push(service_spec_id);
            }
        }

        // Check if node already exists
        if let Some(node) = self
            .nodes
            .iter_mut()
            .find(|n| n.service_spec_id == service_spec_id)
        {
            node.dependencies = dependencies;
        } else {
            self.nodes.push(ServiceDependencyNode {
                service_spec_id,
                service_id: None,
                dependencies,
                dependents: vec![],
                state: DependencyNodeState::NotProvisioned,
            });
        }
    }

    /// Get service specifications that are ready to be provisioned
    /// (all their dependencies are active)
    pub fn get_ready_specs(&self) -> Vec<Uuid> {
        self.nodes
            .iter()
            .filter(|node| {
                // All dependencies are active
                node.dependencies.iter().all(|dep_spec_id| {
                    self.nodes
                        .iter()
                        .find(|n| n.service_spec_id == *dep_spec_id)
                        .map(|n| n.state == DependencyNodeState::Active)
                        .unwrap_or(false)
                })
            })
            .filter(|node| {
                // Service is not yet provisioned
                node.state == DependencyNodeState::NotProvisioned
            })
            .map(|node| node.service_spec_id)
            .collect()
    }

    /// Mark a service specification as provisioning
    pub fn mark_provisioning(&mut self, service_spec_id: Uuid, service_id: Uuid) {
        if let Some(node) = self
            .nodes
            .iter_mut()
            .find(|n| n.service_spec_id == service_spec_id)
        {
            node.service_id = Some(service_id);
            node.state = DependencyNodeState::Provisioning;
        }
    }

    /// Mark a service specification as active
    pub fn mark_active(&mut self, service_spec_id: Uuid) {
        if let Some(node) = self
            .nodes
            .iter_mut()
            .find(|n| n.service_spec_id == service_spec_id)
        {
            node.state = DependencyNodeState::Active;
        }
    }

    /// Mark a service specification as inactive
    pub fn mark_inactive(&mut self, service_spec_id: Uuid) {
        if let Some(node) = self
            .nodes
            .iter_mut()
            .find(|n| n.service_spec_id == service_spec_id)
        {
            node.state = DependencyNodeState::Inactive;
        }
    }

    /// Get dependents of a service specification
    pub fn get_dependents(&self, service_spec_id: Uuid) -> Vec<Uuid> {
        self.nodes
            .iter()
            .find(|n| n.service_spec_id == service_spec_id)
            .map(|n| n.dependents.clone())
            .unwrap_or_default()
    }

    /// Get dependencies of a service specification
    pub fn get_dependencies(&self, service_spec_id: Uuid) -> Vec<Uuid> {
        self.nodes
            .iter()
            .find(|n| n.service_spec_id == service_spec_id)
            .map(|n| n.dependencies.clone())
            .unwrap_or_default()
    }

    /// Check if a service specification can be provisioned
    pub fn can_provision(&self, service_spec_id: Uuid) -> bool {
        self.nodes
            .iter()
            .find(|n| n.service_spec_id == service_spec_id)
            .map(|node| {
                // All dependencies are active
                node.dependencies.iter().all(|dep_spec_id| {
                    self.nodes
                        .iter()
                        .find(|n| n.service_spec_id == *dep_spec_id)
                        .map(|n| n.state == DependencyNodeState::Active)
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false)
    }
}

impl Default for ServiceDependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

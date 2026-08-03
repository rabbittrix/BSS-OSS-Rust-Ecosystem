//! Service Activation Automation

use crate::dependencies::ServiceDependencyGraph;
use crate::state::{ServiceLifecycleState, ServiceWorkflowContext};
use sqlx::PgPool;
use std::sync::Arc;
use tmf638_service_inventory::CreateServiceInventoryRequest;
use tmf640_service_activation::CreateServiceActivationRequest;
use uuid::Uuid;

/// Service activation automation engine (shares the orchestrator dependency graph).
pub struct ServiceActivationEngine {
    pool: Arc<PgPool>,
    dependency_graph: Arc<tokio::sync::RwLock<ServiceDependencyGraph>>,
}

impl ServiceActivationEngine {
    pub fn new(
        pool: PgPool,
        dependency_graph: Arc<tokio::sync::RwLock<ServiceDependencyGraph>>,
    ) -> Self {
        Self {
            pool: Arc::new(pool),
            dependency_graph,
        }
    }

    /// Automatically trigger service activation when dependencies are met
    pub async fn auto_activate_service(
        &self,
        context: &mut ServiceWorkflowContext,
        service_order_id: Uuid,
        service_spec_id: Uuid,
    ) -> Result<Uuid, ActivationError> {
        {
            let dependency_graph = self.dependency_graph.read().await;
            if !dependency_graph.can_provision(service_spec_id) {
                return Err(ActivationError::DependenciesNotMet);
            }
        }

        let activation_request = CreateServiceActivationRequest {
            name: format!("Auto-activation for service order {}", service_order_id),
            description: Some("Automatically triggered service activation".to_string()),
            version: Some("1.0".to_string()),
            service_id: None,
            service_order_id: Some(service_order_id),
            configuration: None,
        };

        let activation_id = self.create_service_activation(activation_request).await?;

        if let Some(task) = context
            .tasks
            .iter_mut()
            .find(|t| matches!(t.task_type, crate::state::ServiceTaskType::CreateActivation))
        {
            task.activation_id = Some(activation_id);
            task.state = ServiceLifecycleState::ReadyForActivation;
        }

        self.execute_activation(activation_id).await?;

        if let Some(task) = context.tasks.iter_mut().find(|t| {
            matches!(
                t.task_type,
                crate::state::ServiceTaskType::ExecuteActivation
            )
        }) {
            task.activation_id = Some(activation_id);
            task.state = ServiceLifecycleState::Activated;
        }

        {
            let mut dependency_graph = self.dependency_graph.write().await;
            dependency_graph.mark_provisioning(service_spec_id, activation_id);
        }

        context.state = ServiceLifecycleState::Activated;
        Ok(activation_id)
    }

    /// Create service inventory after activation
    pub async fn create_service_inventory(
        &self,
        context: &mut ServiceWorkflowContext,
        service_order_id: Uuid,
        service_spec_id: Uuid,
        _activation_id: Uuid,
    ) -> Result<Uuid, ActivationError> {
        let inventory_request = CreateServiceInventoryRequest {
            name: format!("Service inventory for order {}", service_order_id),
            description: Some("Automatically created service inventory".to_string()),
            version: Some("1.0".to_string()),
            service_specification_id: Some(service_spec_id),
            service_id: None,
            related_party: None,
        };

        let inventory_id = self.create_inventory(inventory_request).await?;

        if let Some(task) = context
            .tasks
            .iter_mut()
            .find(|t| matches!(t.task_type, crate::state::ServiceTaskType::CreateInventory))
        {
            task.inventory_id = Some(inventory_id);
            task.state = ServiceLifecycleState::InventoryCreated;
        }

        context.state = ServiceLifecycleState::InventoryCreated;

        {
            let mut dependency_graph = self.dependency_graph.write().await;
            dependency_graph.mark_active(service_spec_id);
        }

        Ok(inventory_id)
    }

    async fn create_service_activation(
        &self,
        request: CreateServiceActivationRequest,
    ) -> Result<Uuid, ActivationError> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();

        sqlx::query(
            "INSERT INTO service_activations (id, name, description, version, state, activation_date, service_order_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(&request.version)
        .bind("PENDING")
        .bind(now)
        .bind(request.service_order_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(ActivationError::Database)?;

        Ok(id)
    }

    async fn execute_activation(&self, activation_id: Uuid) -> Result<(), ActivationError> {
        sqlx::query("UPDATE service_activations SET state = $1 WHERE id = $2")
            .bind("IN_PROGRESS")
            .bind(activation_id)
            .execute(self.pool.as_ref())
            .await
            .map_err(ActivationError::Database)?;

        // Production: call external provisioning / NE configuration here.
        let completion_date = chrono::Utc::now();
        sqlx::query(
            "UPDATE service_activations SET state = $1, completion_date = $2 WHERE id = $3",
        )
        .bind("COMPLETED")
        .bind(completion_date)
        .bind(activation_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(ActivationError::Database)?;

        Ok(())
    }

    async fn create_inventory(
        &self,
        request: CreateServiceInventoryRequest,
    ) -> Result<Uuid, ActivationError> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();

        sqlx::query(
            "INSERT INTO service_inventories (id, name, description, version, state, activation_date, service_specification_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(&request.version)
        .bind("ACTIVE")
        .bind(now)
        .bind(request.service_specification_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(ActivationError::Database)?;

        Ok(id)
    }
}

/// Activation errors
#[derive(Debug, thiserror::Error)]
pub enum ActivationError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Dependencies not met")]
    DependenciesNotMet,
    #[error("Service activation not found")]
    ActivationNotFound,
    #[error("Service inventory not found")]
    InventoryNotFound,
    #[error("Invalid activation state")]
    InvalidState,
}

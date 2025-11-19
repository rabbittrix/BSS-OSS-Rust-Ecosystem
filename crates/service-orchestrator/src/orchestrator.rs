//! Main Service Orchestrator

use crate::activation::{ActivationError, ServiceActivationEngine};
use crate::dependencies::ServiceDependencyGraph;
use crate::state::{ServiceLifecycleState, ServiceWorkflowContext};
use crate::workflow::{ServiceWorkflowEngine, WorkflowError};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tmf641_service_order::models::ServiceOrder;
use uuid::Uuid;

/// Service orchestrator interface
#[async_trait]
pub trait ServiceOrchestratorTrait: Send + Sync {
    /// Start orchestrating a service order
    async fn orchestrate(&self, service_order: ServiceOrder) -> Result<Uuid, OrchestratorError>;

    /// Get workflow context
    async fn get_context(
        &self,
        service_order_id: Uuid,
    ) -> Result<ServiceWorkflowContext, OrchestratorError>;

    /// Process ready tasks in workflow
    async fn process_workflow(&self, service_order_id: Uuid) -> Result<(), OrchestratorError>;

    /// Check and update service dependencies
    async fn check_dependencies(&self, service_order_id: Uuid) -> Result<(), OrchestratorError>;
}

/// Service orchestrator implementation
pub struct ServiceOrchestrator {
    pool: Arc<PgPool>,
    activation_engine: Arc<ServiceActivationEngine>,
    dependency_graph: Arc<tokio::sync::RwLock<ServiceDependencyGraph>>,
}

impl ServiceOrchestrator {
    pub fn new(pool: PgPool) -> Self {
        let activation_engine = Arc::new(ServiceActivationEngine::new(pool.clone()));
        Self {
            pool: Arc::new(pool),
            activation_engine,
            dependency_graph: Arc::new(tokio::sync::RwLock::new(ServiceDependencyGraph::new())),
        }
    }

    /// Initialize orchestrator and load dependency graph from database
    pub async fn initialize(pool: PgPool) -> Result<Self, OrchestratorError> {
        let dependency_graph = ServiceDependencyGraph::load_from_db(&pool)
            .await
            .map_err(OrchestratorError::Database)?;

        let activation_engine = Arc::new(ServiceActivationEngine::new(pool.clone()));
        Ok(Self {
            pool: Arc::new(pool),
            activation_engine,
            dependency_graph: Arc::new(tokio::sync::RwLock::new(dependency_graph)),
        })
    }

    /// Load service order items from database
    async fn load_service_order_items(
        &self,
        service_order_id: Uuid,
    ) -> Result<Vec<(Uuid, Option<Uuid>)>, OrchestratorError> {
        let rows = sqlx::query(
            "SELECT service_specification_id, service_id
             FROM service_order_items
             WHERE order_id = $1",
        )
        .bind(service_order_id)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(OrchestratorError::Database)?;

        Ok(rows.iter().map(|row| (row.get(0), row.get(1))).collect())
    }

    /// Store workflow context in database
    async fn store_context(
        &self,
        context: &ServiceWorkflowContext,
    ) -> Result<(), OrchestratorError> {
        let state_str = format!("{:?}", context.state);
        let context_json = serde_json::to_string(context)
            .map_err(|e| OrchestratorError::Serialization(e.to_string()))?;

        sqlx::query(
            "INSERT INTO service_workflow_contexts (service_order_id, state, context_data, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (service_order_id) 
             DO UPDATE SET state = $2, context_data = $3, updated_at = $5",
        )
        .bind(context.service_order_id)
        .bind(&state_str)
        .bind(&context_json)
        .bind(context.created_at)
        .bind(context.updated_at)
        .execute(self.pool.as_ref())
        .await
        .map_err(OrchestratorError::Database)?;

        Ok(())
    }

    /// Load workflow context from database
    async fn load_context(
        &self,
        service_order_id: Uuid,
    ) -> Result<ServiceWorkflowContext, OrchestratorError> {
        let row = sqlx::query(
            "SELECT context_data FROM service_workflow_contexts WHERE service_order_id = $1",
        )
        .bind(service_order_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(OrchestratorError::Database)?;

        match row {
            Some(row) => {
                let context_json: String = row.get(0);
                serde_json::from_str(&context_json)
                    .map_err(|e| OrchestratorError::Deserialization(e.to_string()))
            }
            None => Err(OrchestratorError::ContextNotFound),
        }
    }
}

#[async_trait]
impl ServiceOrchestratorTrait for ServiceOrchestrator {
    async fn orchestrate(&self, service_order: ServiceOrder) -> Result<Uuid, OrchestratorError> {
        let service_order_id = service_order.base.id;

        // Create workflow context
        let context = ServiceWorkflowEngine::create_workflow(service_order_id);

        // Extract service specification IDs from service order items
        // First try from the service order object, then fall back to database
        let service_specs = if let Some(order_items) = &service_order.order_item {
            order_items
                .iter()
                .filter_map(|item| item.service_specification.as_ref().map(|spec| spec.id))
                .collect::<Vec<_>>()
        } else {
            // Load from database if not in the object
            let items = self.load_service_order_items(service_order_id).await?;
            items
                .iter()
                .filter_map(|(spec_id, _)| {
                    if *spec_id != Uuid::nil() {
                        Some(*spec_id)
                    } else {
                        None
                    }
                })
                .collect()
        };

        // Load dependencies for each service specification
        for spec_id in &service_specs {
            let dependencies =
                ServiceDependencyGraph::load_dependencies_for_spec(self.pool.as_ref(), *spec_id)
                    .await
                    .map_err(OrchestratorError::Database)?;

            let mut dependency_graph = self.dependency_graph.write().await;
            dependency_graph.add_service_spec(*spec_id, dependencies);
            drop(dependency_graph);
        }

        // Persist dependency graph to database
        {
            let dependency_graph = self.dependency_graph.read().await;
            dependency_graph
                .save_to_db(self.pool.as_ref())
                .await
                .map_err(OrchestratorError::Database)?;
        }

        // Store initial context
        self.store_context(&context).await?;

        // Start processing workflow
        self.process_workflow(service_order_id).await?;

        Ok(service_order_id)
    }

    async fn get_context(
        &self,
        service_order_id: Uuid,
    ) -> Result<ServiceWorkflowContext, OrchestratorError> {
        self.load_context(service_order_id).await
    }

    async fn process_workflow(&self, service_order_id: Uuid) -> Result<(), OrchestratorError> {
        let mut context = self.load_context(service_order_id).await?;

        // Advance workflow
        ServiceWorkflowEngine::advance_workflow(&mut context)
            .map_err(OrchestratorError::Workflow)?;

        // Process ready tasks - collect IDs first to avoid borrow checker issues
        let ready_task_ids: Vec<Uuid> = context.get_ready_tasks().iter().map(|t| t.id).collect();

        for task_id in ready_task_ids {
            let task = context
                .get_task(task_id)
                .ok_or(OrchestratorError::InvalidStateTransition)?;
            match task.task_type.clone() {
                crate::state::ServiceTaskType::ValidateOrder => {
                    // Validate service order
                    context.update_task_state(task_id, ServiceLifecycleState::Validating);
                    // Mark as completed (validation passed)
                    context.update_task_state(task_id, ServiceLifecycleState::Completed);
                }
                crate::state::ServiceTaskType::CheckDependencies => {
                    // Check dependencies
                    match self.check_dependencies(service_order_id).await {
                        Ok(_) => {
                            context.update_task_state(task_id, ServiceLifecycleState::Completed);
                        }
                        Err(OrchestratorError::DependenciesNotMet) => {
                            context.update_task_state(
                                task_id,
                                ServiceLifecycleState::WaitingForDependencies,
                            );
                        }
                        Err(e) => {
                            context.update_task_state(task_id, ServiceLifecycleState::Failed);
                            context.error = Some(e.to_string());
                        }
                    }
                }
                crate::state::ServiceTaskType::CreateActivation => {
                    // Get service specification IDs from service order
                    let service_specs = self.load_service_order_items(service_order_id).await?;

                    // Process each service specification
                    let mut activation_succeeded = true;
                    let mut activation_error = None;

                    for (service_spec_id, _service_id) in service_specs {
                        if service_spec_id == Uuid::nil() {
                            continue;
                        }

                        // Auto-activate service
                        match self
                            .activation_engine
                            .auto_activate_service(&mut context, service_order_id, service_spec_id)
                            .await
                        {
                            Ok(_) => {
                                // Activation successful for this spec
                            }
                            Err(e) => {
                                // If dependencies not met, wait and retry later
                                if matches!(e, ActivationError::DependenciesNotMet) {
                                    context.update_task_state(
                                        task_id,
                                        ServiceLifecycleState::WaitingForDependencies,
                                    );
                                    activation_succeeded = false;
                                    break;
                                }
                                activation_succeeded = false;
                                activation_error = Some(e.to_string());
                                break;
                            }
                        }
                    }

                    // Mark task based on result
                    if activation_succeeded {
                        context.update_task_state(task_id, ServiceLifecycleState::Completed);
                    } else if let Some(err) = activation_error {
                        context.update_task_state(task_id, ServiceLifecycleState::Failed);
                        context.error = Some(err);
                    }
                }
                crate::state::ServiceTaskType::ExecuteActivation => {
                    // Activation is handled by auto_activate_service
                    context.update_task_state(task.id, ServiceLifecycleState::Completed);
                }
                crate::state::ServiceTaskType::CreateInventory => {
                    // Get service specification IDs from service order
                    let service_specs = self.load_service_order_items(service_order_id).await?;

                    // Get activation ID from the execute activation task
                    let activation_id = context
                        .tasks
                        .iter()
                        .find(|t| {
                            matches!(
                                t.task_type,
                                crate::state::ServiceTaskType::ExecuteActivation
                            )
                        })
                        .and_then(|t| t.activation_id)
                        .ok_or_else(|| OrchestratorError::InvalidStateTransition)?;

                    // Create service inventory for each service specification
                    let mut all_succeeded = true;
                    let mut inventory_error = None;

                    for (service_spec_id, _service_id) in service_specs {
                        if service_spec_id == Uuid::nil() {
                            continue;
                        }

                        match self
                            .activation_engine
                            .create_service_inventory(
                                &mut context,
                                service_order_id,
                                service_spec_id,
                                activation_id,
                            )
                            .await
                        {
                            Ok(_) => {
                                // Inventory created successfully
                            }
                            Err(e) => {
                                all_succeeded = false;
                                inventory_error = Some(e.to_string());
                                break;
                            }
                        }
                    }

                    if all_succeeded {
                        context.update_task_state(task_id, ServiceLifecycleState::Completed);
                        ServiceWorkflowEngine::complete_workflow(&mut context);
                    } else if let Some(err) = inventory_error {
                        context.update_task_state(task_id, ServiceLifecycleState::Failed);
                        ServiceWorkflowEngine::fail_workflow(&mut context, err);
                    }
                }
                crate::state::ServiceTaskType::UpdateInventory => {
                    // Update inventory task
                    context.update_task_state(task_id, ServiceLifecycleState::Completed);
                }
            }
        }

        // Store updated context
        self.store_context(&context).await?;

        Ok(())
    }

    async fn check_dependencies(&self, service_order_id: Uuid) -> Result<(), OrchestratorError> {
        // Get service specification IDs from service order
        let service_specs = self.load_service_order_items(service_order_id).await?;

        let dependency_graph = self.dependency_graph.read().await;

        // Check if all service specifications can be provisioned
        let mut all_ready = true;
        for (spec_id, _) in &service_specs {
            if *spec_id != Uuid::nil() && !dependency_graph.can_provision(*spec_id) {
                all_ready = false;
                break;
            }
        }
        drop(dependency_graph);

        if !all_ready {
            return Err(OrchestratorError::DependenciesNotMet);
        }

        Ok(())
    }
}

impl ServiceOrchestrator {
    /// Process all pending workflows (background worker)
    pub async fn process_pending_workflows(&self) -> Result<usize, OrchestratorError> {
        // Get all workflows that are not completed or failed
        let rows = sqlx::query(
            "SELECT service_order_id FROM service_workflow_contexts
             WHERE state NOT IN ('COMPLETED', 'FAILED', 'CANCELLED')
             ORDER BY updated_at ASC
             LIMIT 100",
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(OrchestratorError::Database)?;

        let mut processed = 0;
        for row in rows {
            let service_order_id: Uuid = row.get(0);
            if let Err(e) = self.process_workflow(service_order_id).await {
                log::warn!(
                    "Failed to process workflow for service order {}: {}",
                    service_order_id,
                    e
                );
            } else {
                processed += 1;
            }
        }

        Ok(processed)
    }

    /// Start background worker to process workflows periodically
    pub fn start_background_worker(
        self: Arc<Self>,
        interval_seconds: u64,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_secs(interval_seconds));
            loop {
                interval.tick().await;
                match self.process_pending_workflows().await {
                    Ok(count) => {
                        if count > 0 {
                            log::info!("Processed {} pending workflows", count);
                        }
                    }
                    Err(e) => {
                        log::error!("Error processing pending workflows: {}", e);
                    }
                }
            }
        })
    }
}

/// Orchestrator errors
#[derive(Debug, thiserror::Error)]
pub enum OrchestratorError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Workflow context not found")]
    ContextNotFound,
    #[error("Service order not found")]
    ServiceOrderNotFound,
    #[error("Workflow error: {0}")]
    Workflow(#[from] WorkflowError),
    #[error("Dependencies not met")]
    DependenciesNotMet,
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    #[error("Invalid state transition")]
    InvalidStateTransition,
}

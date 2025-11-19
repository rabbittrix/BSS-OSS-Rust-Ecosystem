//! Service Orchestration Workflows

use crate::state::{
    ServiceLifecycleState, ServiceTaskType, ServiceWorkflowContext, ServiceWorkflowTask,
};
use chrono::Utc;
use uuid::Uuid;

/// Service workflow engine
pub struct ServiceWorkflowEngine;

impl ServiceWorkflowEngine {
    /// Create initial workflow context from a service order
    pub fn create_workflow(service_order_id: Uuid) -> ServiceWorkflowContext {
        let mut context = ServiceWorkflowContext::new(service_order_id);

        // Create initial workflow tasks
        let validate_task = ServiceWorkflowTask {
            id: Uuid::new_v4(),
            service_order_id,
            task_type: ServiceTaskType::ValidateOrder,
            state: ServiceLifecycleState::OrderReceived,
            dependencies: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
            error: None,
            service_id: None,
            activation_id: None,
            inventory_id: None,
        };

        let check_deps_task = ServiceWorkflowTask {
            id: Uuid::new_v4(),
            service_order_id,
            task_type: ServiceTaskType::CheckDependencies,
            state: ServiceLifecycleState::CheckingDependencies,
            dependencies: vec![validate_task.id],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
            error: None,
            service_id: None,
            activation_id: None,
            inventory_id: None,
        };

        let create_activation_task = ServiceWorkflowTask {
            id: Uuid::new_v4(),
            service_order_id,
            task_type: ServiceTaskType::CreateActivation,
            state: ServiceLifecycleState::WaitingForDependencies,
            dependencies: vec![check_deps_task.id],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
            error: None,
            service_id: None,
            activation_id: None,
            inventory_id: None,
        };

        let execute_activation_task = ServiceWorkflowTask {
            id: Uuid::new_v4(),
            service_order_id,
            task_type: ServiceTaskType::ExecuteActivation,
            state: ServiceLifecycleState::WaitingForDependencies,
            dependencies: vec![create_activation_task.id],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
            error: None,
            service_id: None,
            activation_id: None,
            inventory_id: None,
        };

        let create_inventory_task = ServiceWorkflowTask {
            id: Uuid::new_v4(),
            service_order_id,
            task_type: ServiceTaskType::CreateInventory,
            state: ServiceLifecycleState::WaitingForDependencies,
            dependencies: vec![execute_activation_task.id],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
            error: None,
            service_id: None,
            activation_id: None,
            inventory_id: None,
        };

        context.add_task(validate_task);
        context.add_task(check_deps_task);
        context.add_task(create_activation_task);
        context.add_task(execute_activation_task);
        context.add_task(create_inventory_task);

        context
    }

    /// Advance workflow to next state
    pub fn advance_workflow(context: &mut ServiceWorkflowContext) -> Result<(), WorkflowError> {
        let ready_task_ids: Vec<Uuid> = context.get_ready_tasks().iter().map(|t| t.id).collect();

        for task_id in ready_task_ids {
            let task = context
                .get_task(task_id)
                .ok_or(WorkflowError::TaskNotFound(task_id))?;
            let task_type = task.task_type.clone();
            let task_state = task.state;

            match task_type {
                ServiceTaskType::ValidateOrder => {
                    if task_state == ServiceLifecycleState::OrderReceived {
                        context.update_task_state(task_id, ServiceLifecycleState::Validating);
                        context.state = ServiceLifecycleState::Validating;
                    }
                }
                ServiceTaskType::CheckDependencies => {
                    if task_state == ServiceLifecycleState::CheckingDependencies {
                        // Dependencies will be checked by the orchestrator
                        // This task will be marked complete when dependencies are verified
                    }
                }
                ServiceTaskType::CreateActivation => {
                    if task_state == ServiceLifecycleState::WaitingForDependencies {
                        context
                            .update_task_state(task_id, ServiceLifecycleState::ReadyForActivation);
                        context.state = ServiceLifecycleState::ReadyForActivation;
                    }
                }
                ServiceTaskType::ExecuteActivation => {
                    if task_state == ServiceLifecycleState::WaitingForDependencies
                        || task_state == ServiceLifecycleState::ReadyForActivation
                    {
                        context.update_task_state(task_id, ServiceLifecycleState::Activating);
                        context.state = ServiceLifecycleState::Activating;
                    }
                }
                ServiceTaskType::CreateInventory => {
                    if task_state == ServiceLifecycleState::WaitingForDependencies
                        || task_state == ServiceLifecycleState::Activated
                    {
                        context.state = ServiceLifecycleState::InventoryCreated;
                    }
                }
                ServiceTaskType::UpdateInventory => {
                    // Update inventory task
                    if task_state == ServiceLifecycleState::InventoryCreated {
                        context.update_task_state(task_id, ServiceLifecycleState::Completed);
                    }
                }
            }
        }

        Ok(())
    }

    /// Retry failed tasks in workflow
    pub fn retry_failed_tasks(context: &mut ServiceWorkflowContext) {
        for task in &mut context.tasks {
            if task.state == ServiceLifecycleState::Failed && task.error.is_some() {
                // Reset task to previous state based on task type
                match task.task_type {
                    ServiceTaskType::ValidateOrder => {
                        task.state = ServiceLifecycleState::OrderReceived;
                    }
                    ServiceTaskType::CheckDependencies => {
                        task.state = ServiceLifecycleState::CheckingDependencies;
                    }
                    ServiceTaskType::CreateActivation => {
                        task.state = ServiceLifecycleState::WaitingForDependencies;
                    }
                    ServiceTaskType::ExecuteActivation => {
                        task.state = ServiceLifecycleState::ReadyForActivation;
                    }
                    ServiceTaskType::CreateInventory => {
                        task.state = ServiceLifecycleState::Activated;
                    }
                    ServiceTaskType::UpdateInventory => {
                        task.state = ServiceLifecycleState::InventoryCreated;
                    }
                }
                task.error = None;
                task.completed_at = None;
            }
        }
        context.error = None;
        context.updated_at = Utc::now();
    }

    /// Mark workflow as completed
    pub fn complete_workflow(context: &mut ServiceWorkflowContext) {
        context.state = ServiceLifecycleState::Completed;
        context.completed_at = Some(Utc::now());
        context.updated_at = Utc::now();
    }

    /// Mark workflow as failed
    pub fn fail_workflow(context: &mut ServiceWorkflowContext, error: String) {
        context.state = ServiceLifecycleState::Failed;
        context.error = Some(error);
        context.updated_at = Utc::now();
    }
}

/// Workflow errors
#[derive(Debug, thiserror::Error)]
pub enum WorkflowError {
    #[error("Invalid workflow state transition")]
    InvalidStateTransition,
    #[error("Task not found: {0}")]
    TaskNotFound(Uuid),
    #[error("Dependencies not met")]
    DependenciesNotMet,
}

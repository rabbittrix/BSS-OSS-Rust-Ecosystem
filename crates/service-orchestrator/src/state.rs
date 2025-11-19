//! Service Lifecycle State Management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Service lifecycle state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServiceLifecycleState {
    /// Service order received
    OrderReceived,
    /// Validating service order
    Validating,
    /// Checking dependencies
    CheckingDependencies,
    /// Waiting for dependencies
    WaitingForDependencies,
    /// Dependencies met, ready for activation
    ReadyForActivation,
    /// Activating service
    Activating,
    /// Service activated
    Activated,
    /// Service inventory created
    InventoryCreated,
    /// Service lifecycle completed
    Completed,
    /// Service lifecycle failed
    Failed,
    /// Service lifecycle cancelled
    Cancelled,
}

/// Service workflow task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceWorkflowTask {
    pub id: Uuid,
    pub service_order_id: Uuid,
    pub task_type: ServiceTaskType,
    pub state: ServiceLifecycleState,
    pub dependencies: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub service_id: Option<Uuid>,
    pub activation_id: Option<Uuid>,
    pub inventory_id: Option<Uuid>,
}

/// Service task type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServiceTaskType {
    /// Service order validation
    ValidateOrder,
    /// Check service dependencies
    CheckDependencies,
    /// Create service activation
    CreateActivation,
    /// Execute service activation
    ExecuteActivation,
    /// Create service inventory
    CreateInventory,
    /// Update service inventory
    UpdateInventory,
}

/// Service workflow context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceWorkflowContext {
    pub service_order_id: Uuid,
    pub state: ServiceLifecycleState,
    pub tasks: Vec<ServiceWorkflowTask>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

impl ServiceWorkflowContext {
    pub fn new(service_order_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            service_order_id,
            state: ServiceLifecycleState::OrderReceived,
            tasks: vec![],
            created_at: now,
            updated_at: now,
            completed_at: None,
            error: None,
        }
    }

    pub fn add_task(&mut self, task: ServiceWorkflowTask) {
        self.tasks.push(task);
        self.updated_at = Utc::now();
    }

    pub fn update_task_state(&mut self, task_id: Uuid, state: ServiceLifecycleState) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.state = state;
            task.updated_at = Utc::now();
            if state == ServiceLifecycleState::Completed || state == ServiceLifecycleState::Failed {
                task.completed_at = Some(Utc::now());
            }
            self.updated_at = Utc::now();
        }
    }

    pub fn get_task(&self, task_id: Uuid) -> Option<&ServiceWorkflowTask> {
        self.tasks.iter().find(|t| t.id == task_id)
    }

    pub fn get_task_mut(&mut self, task_id: Uuid) -> Option<&mut ServiceWorkflowTask> {
        self.tasks.iter_mut().find(|t| t.id == task_id)
    }

    pub fn get_ready_tasks(&self) -> Vec<&ServiceWorkflowTask> {
        self.tasks
            .iter()
            .filter(|task| {
                // Task is ready if:
                // 1. It's in a state that allows progression
                // 2. All its dependencies are completed
                let dependencies_met = task.dependencies.iter().all(|dep_id| {
                    self.tasks
                        .iter()
                        .find(|t| t.id == *dep_id)
                        .map(|t| t.state == ServiceLifecycleState::Completed)
                        .unwrap_or(true)
                });

                matches!(
                    task.state,
                    ServiceLifecycleState::OrderReceived
                        | ServiceLifecycleState::Validating
                        | ServiceLifecycleState::CheckingDependencies
                        | ServiceLifecycleState::ReadyForActivation
                ) && dependencies_met
            })
            .collect()
    }
}

//! Order Fulfillment State Management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Order fulfillment state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FulfillmentState {
    /// Order received and acknowledged
    Acknowledged,
    /// Order is being validated
    Validating,
    /// Order is being decomposed
    Decomposing,
    /// Service orders are being created
    CreatingServiceOrders,
    /// Resource orders are being created
    CreatingResourceOrders,
    /// Orders are in progress
    InProgress,
    /// All orders completed successfully
    Completed,
    /// Order failed
    Failed,
    /// Order cancelled
    Cancelled,
}

/// Order fulfillment task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentTask {
    pub id: Uuid,
    pub order_id: Uuid,
    pub task_type: TaskType,
    pub state: FulfillmentState,
    pub dependencies: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

/// Task type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskType {
    /// Product order task
    ProductOrder,
    /// Service order task
    ServiceOrder(Uuid),
    /// Resource order task
    ResourceOrder(Uuid),
    /// Service activation task
    ServiceActivation(Uuid),
    /// Resource activation task
    ResourceActivation(Uuid),
}

/// Order fulfillment context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentContext {
    pub product_order_id: Uuid,
    pub state: FulfillmentState,
    pub tasks: Vec<FulfillmentTask>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl FulfillmentContext {
    pub fn new(product_order_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            product_order_id,
            state: FulfillmentState::Acknowledged,
            tasks: vec![],
            created_at: now,
            updated_at: now,
            completed_at: None,
        }
    }

    pub fn add_task(&mut self, task: FulfillmentTask) {
        self.tasks.push(task);
        self.updated_at = Utc::now();
    }

    pub fn update_task_state(&mut self, task_id: Uuid, state: FulfillmentState) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.state = state;
            task.updated_at = Utc::now();
            if state == FulfillmentState::Completed {
                task.completed_at = Some(Utc::now());
            }
            self.updated_at = Utc::now();
        }
    }

    pub fn get_ready_tasks(&self) -> Vec<&FulfillmentTask> {
        self.tasks
            .iter()
            .filter(|task| {
                // Task is ready if all dependencies are completed
                task.dependencies.iter().all(|dep_id| {
                    self.tasks
                        .iter()
                        .find(|t| t.id == *dep_id)
                        .map(|t| t.state == FulfillmentState::Completed)
                        .unwrap_or(true)
                })
            })
            .filter(|task| {
                // Task is not yet completed or failed
                matches!(
                    task.state,
                    FulfillmentState::Acknowledged
                        | FulfillmentState::Validating
                        | FulfillmentState::InProgress
                )
            })
            .collect()
    }

    pub fn update_state(&mut self) {
        // Update overall state based on task states
        if self.tasks.is_empty() {
            self.state = FulfillmentState::Acknowledged;
            return;
        }

        let all_completed = self
            .tasks
            .iter()
            .all(|t| t.state == FulfillmentState::Completed);
        let any_failed = self
            .tasks
            .iter()
            .any(|t| t.state == FulfillmentState::Failed);

        if all_completed {
            self.state = FulfillmentState::Completed;
            self.completed_at = Some(Utc::now());
        } else if any_failed {
            self.state = FulfillmentState::Failed;
        } else {
            // Check if any task is in progress
            let any_in_progress = self.tasks.iter().any(|t| {
                matches!(
                    t.state,
                    FulfillmentState::InProgress
                        | FulfillmentState::CreatingServiceOrders
                        | FulfillmentState::CreatingResourceOrders
                )
            });

            if any_in_progress {
                self.state = FulfillmentState::InProgress;
            }
        }
    }
}

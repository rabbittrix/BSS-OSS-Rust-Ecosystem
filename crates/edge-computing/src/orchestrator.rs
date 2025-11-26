//! Edge Task Orchestrator

use crate::error::EdgeComputingError;
use crate::models::{EdgeTask, TaskPriority, TaskStatus, TaskType};
use crate::node::EdgeNodeManager;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Edge Task Orchestrator
pub struct EdgeOrchestrator {
    node_manager: Arc<EdgeNodeManager>,
    tasks: Arc<RwLock<std::collections::HashMap<Uuid, EdgeTask>>>,
}

impl EdgeOrchestrator {
    /// Create a new edge orchestrator
    pub fn new(node_manager: Arc<EdgeNodeManager>) -> Self {
        Self {
            node_manager,
            tasks: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Submit a task for execution
    pub async fn submit_task(
        &self,
        task_type: TaskType,
        payload: serde_json::Value,
        priority: TaskPriority,
    ) -> Uuid {
        let task_id = Uuid::new_v4();
        let task = EdgeTask {
            id: task_id,
            task_type,
            payload,
            priority,
            assigned_node: None,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            result: None,
        };

        self.tasks.write().await.insert(task_id, task);
        task_id
    }

    /// Assign task to best available node
    pub async fn assign_task(&self, task_id: Uuid) -> Result<Uuid, EdgeComputingError> {
        let online_nodes = self.node_manager.get_online_nodes().await;

        if online_nodes.is_empty() {
            return Err(EdgeComputingError::ResourceUnavailable(
                "No online nodes available".to_string(),
            ));
        }

        // Simple load balancing: select node with most available resources
        let best_node = online_nodes
            .iter()
            .max_by_key(|node| {
                (node.capacity.available_cpu * 100.0) as u64 + node.capacity.available_memory_mb
            })
            .ok_or_else(|| {
                EdgeComputingError::ResourceUnavailable("No suitable node found".to_string())
            })?;

        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.assigned_node = Some(best_node.id);
            task.status = TaskStatus::Assigned;
            Ok(best_node.id)
        } else {
            Err(EdgeComputingError::TaskExecutionFailed(
                "Task not found".to_string(),
            ))
        }
    }

    /// Get task status
    pub async fn get_task_status(&self, task_id: Uuid) -> Option<TaskStatus> {
        self.tasks.read().await.get(&task_id).map(|t| t.status)
    }

    /// Get task result
    pub async fn get_task_result(&self, task_id: Uuid) -> Option<serde_json::Value> {
        self.tasks
            .read()
            .await
            .get(&task_id)
            .and_then(|t| t.result.clone())
    }

    /// Complete a task
    pub async fn complete_task(
        &self,
        task_id: Uuid,
        result: serde_json::Value,
    ) -> Result<(), EdgeComputingError> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.status = TaskStatus::Completed;
            task.completed_at = Some(Utc::now());
            task.result = Some(result);
            Ok(())
        } else {
            Err(EdgeComputingError::TaskExecutionFailed(
                "Task not found".to_string(),
            ))
        }
    }

    /// Fail a task
    pub async fn fail_task(&self, task_id: Uuid, error: String) -> Result<(), EdgeComputingError> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.status = TaskStatus::Failed;
            task.completed_at = Some(Utc::now());
            task.result = Some(serde_json::json!({ "error": error }));
            Ok(())
        } else {
            Err(EdgeComputingError::TaskExecutionFailed(
                "Task not found".to_string(),
            ))
        }
    }

    /// Get pending tasks
    pub async fn get_pending_tasks(&self) -> Vec<EdgeTask> {
        self.tasks
            .read()
            .await
            .values()
            .filter(|t| t.status == TaskStatus::Pending)
            .cloned()
            .collect()
    }
}

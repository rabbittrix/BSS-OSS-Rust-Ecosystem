//! Edge Computing models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Edge node status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Online,
    Offline,
    Maintenance,
    Overloaded,
}

/// Edge node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeNode {
    pub id: Uuid,
    pub name: String,
    pub location: String,
    pub endpoint: String,
    pub status: NodeStatus,
    pub capacity: NodeCapacity,
    pub last_heartbeat: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

/// Node capacity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapacity {
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub storage_gb: u64,
    pub available_cpu: f64,
    pub available_memory_mb: u64,
    pub available_storage_gb: u64,
}

/// Edge task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeTask {
    pub id: Uuid,
    pub task_type: TaskType,
    pub payload: serde_json::Value,
    pub priority: TaskPriority,
    pub assigned_node: Option<Uuid>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub result: Option<serde_json::Value>,
}

/// Task type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    DataProcessing,
    Analytics,
    CacheUpdate,
    Synchronization,
    IotDataCollection,
    RealTimeProcessing,
}

/// Task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Assigned,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Sync operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOperation {
    pub id: Uuid,
    pub source_node: Uuid,
    pub target_node: Uuid,
    pub data_type: String,
    pub data: serde_json::Value,
    pub status: SyncStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Sync status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

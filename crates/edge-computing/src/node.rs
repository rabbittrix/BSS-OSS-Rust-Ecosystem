//! Edge Node Management

use crate::error::EdgeComputingError;
use crate::models::{EdgeNode, NodeCapacity, NodeStatus};
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Edge Node Manager
pub struct EdgeNodeManager {
    nodes: Arc<RwLock<std::collections::HashMap<Uuid, EdgeNode>>>,
}

impl EdgeNodeManager {
    /// Create a new edge node manager
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Register a new edge node
    pub async fn register_node(
        &self,
        name: String,
        location: String,
        endpoint: String,
        capacity: NodeCapacity,
    ) -> Uuid {
        let id = Uuid::new_v4();
        let node = EdgeNode {
            id,
            name,
            location,
            endpoint,
            status: NodeStatus::Online,
            capacity,
            last_heartbeat: Utc::now(),
            metadata: serde_json::json!({}),
        };

        self.nodes.write().await.insert(id, node);
        id
    }

    /// Update node heartbeat
    pub async fn update_heartbeat(&self, node_id: Uuid) -> Result<(), EdgeComputingError> {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(&node_id) {
            node.last_heartbeat = Utc::now();
            Ok(())
        } else {
            Err(EdgeComputingError::NodeNotFound(node_id.to_string()))
        }
    }

    /// Get node by ID
    pub async fn get_node(&self, node_id: Uuid) -> Option<EdgeNode> {
        self.nodes.read().await.get(&node_id).cloned()
    }

    /// Get all online nodes
    pub async fn get_online_nodes(&self) -> Vec<EdgeNode> {
        self.nodes
            .read()
            .await
            .values()
            .filter(|n| n.status == NodeStatus::Online)
            .cloned()
            .collect()
    }

    /// Update node status
    pub async fn update_node_status(
        &self,
        node_id: Uuid,
        status: NodeStatus,
    ) -> Result<(), EdgeComputingError> {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(&node_id) {
            node.status = status;
            Ok(())
        } else {
            Err(EdgeComputingError::NodeNotFound(node_id.to_string()))
        }
    }

    /// Update node capacity
    pub async fn update_node_capacity(
        &self,
        node_id: Uuid,
        capacity: NodeCapacity,
    ) -> Result<(), EdgeComputingError> {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(&node_id) {
            node.capacity = capacity;
            Ok(())
        } else {
            Err(EdgeComputingError::NodeNotFound(node_id.to_string()))
        }
    }

    /// Remove a node
    pub async fn remove_node(&self, node_id: Uuid) -> Result<(), EdgeComputingError> {
        let mut nodes = self.nodes.write().await;
        if nodes.remove(&node_id).is_some() {
            Ok(())
        } else {
            Err(EdgeComputingError::NodeNotFound(node_id.to_string()))
        }
    }
}

impl Default for EdgeNodeManager {
    fn default() -> Self {
        Self::new()
    }
}

//! Resource Management Models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Capacity Type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapacityType {
    Bandwidth,
    Cpu,
    Memory,
    Storage,
    Connections,
    Custom(String),
}

impl CapacityType {
    pub fn as_str(&self) -> &str {
        match self {
            CapacityType::Bandwidth => "BANDWIDTH",
            CapacityType::Cpu => "CPU",
            CapacityType::Memory => "MEMORY",
            CapacityType::Storage => "STORAGE",
            CapacityType::Connections => "CONNECTIONS",
            CapacityType::Custom(s) => s.as_str(),
        }
    }
}

/// Resource Capacity
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResourceCapacity {
    pub id: Uuid,
    pub resource_inventory_id: Uuid,
    pub capacity_type: String,
    pub total_capacity: f64,
    pub used_capacity: f64,
    pub reserved_capacity: f64,
    pub available_capacity: f64,
    pub unit: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create Resource Capacity Request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateResourceCapacityRequest {
    pub resource_inventory_id: Uuid,
    pub capacity_type: String,
    pub total_capacity: f64,
    pub unit: String,
}

/// Update Resource Capacity Request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateResourceCapacityRequest {
    pub total_capacity: Option<f64>,
    pub used_capacity: Option<f64>,
    pub reserved_capacity: Option<f64>,
}

/// Reservation Status
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Active,
    Completed,
    Cancelled,
}

/// Resource Reservation
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResourceReservation {
    pub id: Uuid,
    pub resource_inventory_id: Uuid,
    pub reservation_name: String,
    pub description: Option<String>,
    pub reservation_status: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub resource_order_id: Option<Uuid>,
    pub service_order_id: Option<Uuid>,
    pub reserved_by_party_id: Option<Uuid>,
    pub capacity_requirements: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>,
}

/// Create Resource Reservation Request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateResourceReservationRequest {
    pub resource_inventory_id: Uuid,
    pub reservation_name: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub resource_order_id: Option<Uuid>,
    pub service_order_id: Option<Uuid>,
    pub reserved_by_party_id: Option<Uuid>,
    pub capacity_requirements: serde_json::Value,
}

/// Update Resource Reservation Request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateResourceReservationRequest {
    pub reservation_status: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>,
}

/// Connection Type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConnectionType {
    Physical,
    Logical,
    Virtual,
    Overlay,
}

/// Relationship Type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RelationshipType {
    ConnectedTo,
    DependsOn,
    ParentOf,
    ChildOf,
    Peer,
}

/// Connection Status
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConnectionStatus {
    Active,
    Inactive,
    Planned,
    Failed,
}

/// Network Topology Connection
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkTopology {
    pub id: Uuid,
    pub source_resource_id: Uuid,
    pub target_resource_id: Uuid,
    pub connection_type: String,
    pub relationship_type: String,
    pub connection_status: String,
    pub bandwidth_mbps: Option<f64>,
    pub latency_ms: Option<f64>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create Network Topology Request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateNetworkTopologyRequest {
    pub source_resource_id: Uuid,
    pub target_resource_id: Uuid,
    pub connection_type: String,
    pub relationship_type: String,
    pub bandwidth_mbps: Option<f64>,
    pub latency_ms: Option<f64>,
    pub description: Option<String>,
}

/// Update Network Topology Request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateNetworkTopologyRequest {
    pub connection_status: Option<String>,
    pub bandwidth_mbps: Option<f64>,
    pub latency_ms: Option<f64>,
    pub description: Option<String>,
}

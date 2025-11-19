//! TMF656 Slice Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Slice State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SliceState {
    Planned,
    Active,
    Inactive,
    Terminated,
}

/// Slice Type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SliceType {
    EnhancedMobileBroadband,
    UltraReliableLowLatency,
    MassiveMachineTypeCommunications,
    Custom,
}

/// Network Slice - Represents a 5G network slice
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkSlice {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Slice state
    pub state: SliceState,
    /// Slice type
    pub slice_type: SliceType,
    /// Service Level Agreement (SLA) parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_parameters: Option<SLAParameters>,
    /// Network function references
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_functions: Option<Vec<NetworkFunctionRef>>,
    /// Activation date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub activation_date: Option<DateTime<Utc>>,
    /// Termination date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub termination_date: Option<DateTime<Utc>>,
}

/// SLA Parameters
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SLAParameters {
    /// Maximum latency in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency_ms: Option<u32>,
    /// Minimum throughput in Mbps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_throughput_mbps: Option<u32>,
    /// Maximum number of devices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_devices: Option<u32>,
    /// Coverage area
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_area: Option<String>,
}

/// Network Function Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkFunctionRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_type: Option<String>,
}

/// Request to create a network slice
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateNetworkSliceRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub slice_type: SliceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_parameters: Option<CreateSLAParametersRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_functions: Option<Vec<CreateNetworkFunctionRefRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub activation_date: Option<DateTime<Utc>>,
}

/// Request to create SLA parameters
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateSLAParametersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency_ms: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_throughput_mbps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_devices: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_area: Option<String>,
}

/// Request to create a network function reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateNetworkFunctionRefRequest {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_type: Option<String>,
}

/// Request to update a network slice
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateNetworkSliceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<SliceState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub activation_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub termination_date: Option<DateTime<Utc>>,
}

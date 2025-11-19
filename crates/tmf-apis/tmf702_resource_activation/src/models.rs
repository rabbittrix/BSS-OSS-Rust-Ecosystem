//! TMF702 Resource Activation & Configuration models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Resource Activation State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceActivationState {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Resource Activation - Represents a resource activation/configuration request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResourceActivation {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Activation state
    pub state: ResourceActivationState,
    /// Resource reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceRef>,
    /// Service activation reference (if triggered by service activation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_activation: Option<ServiceActivationRef>,
    /// Activation date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub activation_date: Option<DateTime<Utc>>,
    /// Completion date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub completion_date: Option<DateTime<Utc>>,
    /// Configuration parameters (key-value pairs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Vec<ConfigurationParameter>>,
}

/// Resource Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResourceRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// Service Activation Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceActivationRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
}

/// Configuration Parameter
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConfigurationParameter {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to create a resource activation
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateResourceActivationRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub resource_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub service_activation_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Vec<CreateConfigurationParameterRequest>>,
}

/// Request to create a configuration parameter
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateConfigurationParameterRequest {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

//! TMF640 Service Activation & Configuration models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Service Activation State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServiceActivationState {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Service Activation - Represents a service activation/configuration request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceActivation {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Activation state
    pub state: ServiceActivationState,
    /// Service reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceRef>,
    /// Service order reference (if triggered by service order)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_order: Option<ServiceOrderRef>,
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

/// Service Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
}

/// Service Order Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceOrderRef {
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

/// Request to create a service activation
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateServiceActivationRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub service_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub service_order_id: Option<Uuid>,
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

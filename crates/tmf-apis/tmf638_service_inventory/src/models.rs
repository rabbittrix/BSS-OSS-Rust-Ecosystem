//! TMF638 Service Inventory models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Service Inventory State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServiceInventoryState {
    Reserved,
    Active,
    Inactive,
    Terminated,
    Suspended,
}

/// Service Inventory - Represents inventory of a service (provisioned service instance)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceInventory {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Service inventory state
    pub state: ServiceInventoryState,
    /// Service specification reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecificationRef>,
    /// Service reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceRef>,
    /// Related party (customer who owns this service)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Activation date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub activation_date: Option<DateTime<Utc>>,
    /// Last modified date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub last_modified_date: Option<DateTime<Utc>>,
}

/// Service Specification Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceSpecificationRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
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

/// Related Party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelatedParty {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
    pub role: String,
}

/// Request to create a service inventory
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateServiceInventoryRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub service_specification_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub service_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
}

/// Request to create a related party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRelatedPartyRequest {
    pub name: String,
    pub role: String,
}

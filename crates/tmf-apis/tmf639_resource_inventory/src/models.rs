//! TMF639 Resource Inventory models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Resource Inventory State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceInventoryState {
    Reserved,
    Available,
    InUse,
    Maintenance,
    Retired,
}

/// Resource Inventory - Represents inventory of physical/virtual network resources
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResourceInventory {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Resource inventory state
    pub state: ResourceInventoryState,
    /// Resource specification reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ResourceSpecificationRef>,
    /// Resource reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceRef>,
    /// Resource type (physical, virtual, logical)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// Related party (owner/operator)
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

/// Resource Specification Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResourceSpecificationRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
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

/// Request to create a resource inventory
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateResourceInventoryRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub resource_specification_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub resource_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
}

/// Request to create a related party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRelatedPartyRequest {
    pub name: String,
    pub role: String,
}

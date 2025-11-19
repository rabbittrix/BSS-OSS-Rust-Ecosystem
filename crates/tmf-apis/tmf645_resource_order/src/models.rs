//! TMF645 Resource Order Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Resource Order State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceOrderState {
    Acknowledged,
    InProgress,
    Completed,
    Cancelled,
    Rejected,
    Held,
    Failed,
}

/// Resource Order - Represents a resource-level order (network resource provisioning)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResourceOrder {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Resource order state
    pub state: ResourceOrderState,
    /// Resource order items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_item: Option<Vec<ResourceOrderItem>>,
    /// Related party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Order date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub order_date: Option<DateTime<Utc>>,
    /// Expected completion date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub expected_completion_date: Option<DateTime<Utc>>,
    /// Priority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// External ID (e.g., from service order)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

/// Resource Order Item - Individual item within a resource order
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResourceOrderItem {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    /// Item action (add, modify, delete, noChange)
    pub action: String,
    /// Resource specification reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ResourceSpecificationRef>,
    /// Resource reference (if modifying existing resource)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceRef>,
    /// Item state
    pub state: ResourceOrderState,
    /// Quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
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

/// Request to create a resource order
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateResourceOrderRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_item: Option<Vec<CreateResourceOrderItemRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
}

/// Request to create a resource order item
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateResourceOrderItemRequest {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub resource_specification_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub resource_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

/// Request to create a related party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRelatedPartyRequest {
    pub name: String,
    pub role: String,
}

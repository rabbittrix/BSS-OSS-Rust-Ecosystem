//! TMF641 Service Order Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Service Order State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServiceOrderState {
    Acknowledged,
    InProgress,
    Completed,
    Cancelled,
    Rejected,
    Held,
    Failed,
}

/// Service Order - Represents a service-level order (network/service provisioning)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceOrder {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Service order state
    pub state: ServiceOrderState,
    /// Service order items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_item: Option<Vec<ServiceOrderItem>>,
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
    /// External ID (e.g., from product order)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

/// Service Order Item - Individual item within a service order
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceOrderItem {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    /// Item action (add, modify, delete, noChange)
    pub action: String,
    /// Service specification reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecificationRef>,
    /// Service reference (if modifying existing service)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceRef>,
    /// Item state
    pub state: ServiceOrderState,
    /// Quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
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

/// Request to create a service order
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateServiceOrderRequest {
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
    pub order_item: Option<Vec<CreateServiceOrderItemRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
}

/// Request to create a service order item
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateServiceOrderItemRequest {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub service_specification_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub service_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

/// Request to create a related party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRelatedPartyRequest {
    pub name: String,
    pub role: String,
}

//! TMF622 Product Ordering models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Product Order State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderState {
    Acknowledged,
    InProgress,
    Completed,
    Cancelled,
    Rejected,
    Held,
    Failed,
}

/// Product Order - Represents a customer order for products
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductOrder {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Order state
    pub state: OrderState,
    /// Order items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_item: Option<Vec<OrderItem>>,
    /// Related party (customer)
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
}

/// Order Item - Individual item within a product order
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OrderItem {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    /// Item action (add, modify, delete, noChange)
    pub action: String,
    /// Product offering reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering: Option<ProductOfferingRef>,
    /// Product specification reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_specification: Option<ProductSpecificationRef>,
    /// Item state
    pub state: OrderState,
    /// Quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

/// Product Offering Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductOfferingRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
}

/// Product Specification Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductSpecificationRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
}

/// Related Party - Customer or other party related to the order
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelatedParty {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
    pub role: String,
}

/// Request to create a product order
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateProductOrderRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_item: Option<Vec<CreateOrderItemRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
}

/// Request to create an order item
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderItemRequest {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_specification_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

/// Request to create a related party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRelatedPartyRequest {
    pub name: String,
    pub role: String,
}

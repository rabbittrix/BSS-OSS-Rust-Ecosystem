//! TMF637 Product Inventory models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Product Inventory State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InventoryState {
    Reserved,
    Available,
    InUse,
    Retired,
    ReservedForCustomer,
}

/// Product Inventory - Represents inventory of a product
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductInventory {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Inventory state
    pub state: InventoryState,
    /// Product specification reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_specification: Option<ProductSpecificationRef>,
    /// Product offering reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering: Option<ProductOfferingRef>,
    /// Quantity available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    /// Reserved quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_quantity: Option<i32>,
    /// Related party (customer who owns/reserves this inventory)
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

/// Product Specification Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductSpecificationRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
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

/// Related Party - Customer or other party related to the inventory
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelatedParty {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
    pub role: String,
}

/// Request to create a product inventory
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateProductInventoryRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub product_specification_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub product_offering_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
}

/// Request to create a related party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRelatedPartyRequest {
    pub name: String,
    pub role: String,
}

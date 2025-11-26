//! TMF634 Quote Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Quote State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuoteState {
    InProgress,
    Ready,
    Cancelled,
    Accepted,
    Rejected,
    Expired,
}

/// Quote - Represents a price quote for products or services
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Quote {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Quote state
    pub state: QuoteState,
    /// Quote items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_item: Option<Vec<QuoteItem>>,
    /// Related party (customer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Quote date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub quote_date: Option<DateTime<Utc>>,
    /// Valid until date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub valid_until: Option<DateTime<Utc>>,
    /// Total price amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<Money>,
    /// Expected order date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub expected_order_date: Option<DateTime<Utc>>,
}

/// Quote Item - Individual item within a quote
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct QuoteItem {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    /// Product offering reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering: Option<ProductOfferingRef>,
    /// Product specification reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_specification: Option<ProductSpecificationRef>,
    /// Quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    /// Unit price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Money>,
    /// Total price for this item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<Money>,
}

/// Money - Represents a monetary amount
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Money {
    /// Amount value
    pub value: f64,
    /// Currency code (ISO 4217)
    pub unit: String,
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

/// Related Party - Customer or other party related to the quote
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelatedParty {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
    pub role: String,
}

/// Request to create a quote
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateQuoteRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_item: Option<Vec<CreateQuoteItemRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub valid_until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub expected_order_date: Option<DateTime<Utc>>,
}

/// Request to create a quote item
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateQuoteItemRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub product_offering_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub product_specification_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Money>,
}

/// Request to create a related party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRelatedPartyRequest {
    pub name: String,
    pub role: String,
}

/// Request to update a quote
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateQuoteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<QuoteState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub valid_until: Option<DateTime<Utc>>,
}

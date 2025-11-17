//! TMF678 Customer Bill Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Bill State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BillState {
    Pending,
    Paid,
    Overdue,
    Cancelled,
}

/// Customer Bill - Represents a customer bill
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CustomerBill {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Bill state
    pub state: BillState,
    /// Bill date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub bill_date: Option<DateTime<Utc>>,
    /// Due date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub due_date: Option<DateTime<Utc>>,
    /// Total amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<Money>,
    /// Tax included
    #[serde(default)]
    pub tax_included: bool,
    /// Bill items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_item: Option<Vec<BillItem>>,
    /// Related party (customer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

/// Bill Item - Individual item within a bill
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BillItem {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    /// Item description
    pub description: String,
    /// Item amount
    pub amount: Money,
    /// Item quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    /// Product offering reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering: Option<ProductOfferingRef>,
}

/// Money representation
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Money {
    pub value: f64,
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

/// Related Party - Customer or other party related to the bill
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelatedParty {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
    pub role: String,
}

/// Request to create a customer bill
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCustomerBillRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub bill_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub due_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<Money>,
    #[serde(default)]
    pub tax_included: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_item: Option<Vec<CreateBillItemRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
}

/// Request to create a bill item
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateBillItemRequest {
    pub description: String,
    pub amount: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub product_offering_id: Option<Uuid>,
}

/// Request to create a related party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRelatedPartyRequest {
    pub name: String,
    pub role: String,
}

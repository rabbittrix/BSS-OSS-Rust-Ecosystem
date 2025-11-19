//! TMF635 Usage Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Usage State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UsageState {
    Captured,
    Rated,
    Billed,
    Rejected,
}

/// Usage - Represents usage records (CDRs, event consumption)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Usage {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Usage state
    pub state: UsageState,
    /// Usage type (voice, data, sms, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
    /// Usage date/time
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub usage_date: Option<DateTime<Utc>>,
    /// Start date/time
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub start_date: Option<DateTime<Utc>>,
    /// End date/time
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub end_date: Option<DateTime<Utc>>,
    /// Amount (quantity consumed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Unit (MB, minutes, messages, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Product offering reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering: Option<ProductOfferingRef>,
    /// Related party (customer, subscriber)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Rating reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<RatingRef>,
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

/// Rating Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RatingRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_amount: Option<f64>,
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

/// Request to create a usage record
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateUsageRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub usage_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub product_offering_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub rating_id: Option<Uuid>,
}

/// Request to create a related party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRelatedPartyRequest {
    pub name: String,
    pub role: String,
}

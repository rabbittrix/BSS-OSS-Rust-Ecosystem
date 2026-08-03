//! TMF676 Payment Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Payment Status
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

/// Money - Represents a monetary amount
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Money {
    /// Amount value
    pub value: f64,
    /// Currency code (ISO 4217)
    pub unit: String,
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

/// Payment
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Payment {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Payment status
    pub status: PaymentStatus,
    /// Payment amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// Payment date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub payment_date: Option<DateTime<Utc>>,
    /// Related billing account
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub billing_account_id: Option<Uuid>,
    /// Related parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

/// Refund
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Refund {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Refund status
    pub status: PaymentStatus,
    /// Refund amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// Refund date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub refund_date: Option<DateTime<Utc>>,
    /// Related payment
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub payment_id: Option<Uuid>,
    /// Related parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

/// Request to create a payment
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePaymentRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub payment_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub billing_account_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

/// Request to update a payment
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdatePaymentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
}

/// Request to create a refund
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRefundRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub refund_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub payment_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

/// Request to update a refund
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateRefundRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
}

//! TMF651 Agreement Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Agreement Status
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AgreementStatus {
    InProcess,
    Active,
    Suspended,
    Terminated,
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

/// Agreement
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Agreement {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Agreement status
    pub status: AgreementStatus,
    /// Agreement type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_type: Option<String>,
    /// Agreement period start
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub agreement_period_start: Option<DateTime<Utc>>,
    /// Agreement period end
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub agreement_period_end: Option<DateTime<Utc>>,
    /// Related parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

/// Request to create an agreement
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAgreementRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub agreement_period_start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub agreement_period_end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

/// Request to update an agreement
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateAgreementRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AgreementStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub agreement_period_start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub agreement_period_end: Option<DateTime<Utc>>,
}

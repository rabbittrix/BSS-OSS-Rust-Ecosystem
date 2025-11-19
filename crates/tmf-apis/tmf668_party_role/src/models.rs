//! TMF668 Party Role Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Party Role State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PartyRoleState {
    Initialized,
    Validated,
    Active,
    Suspended,
    Terminated,
}

/// Party Role - Represents a party (organization, person) with a specific role
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PartyRole {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Party role state
    pub state: PartyRoleState,
    /// Role name (e.g., CUSTOMER, PARTNER, VENDOR, RESELLER)
    pub role: String,
    /// Party type (INDIVIDUAL, ORGANIZATION)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_type: Option<String>,
    /// Contact information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<ContactMedium>>,
    /// Related party (parent organization, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Engagement date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub engagement_date: Option<DateTime<Utc>>,
}

/// Contact Medium
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContactMedium {
    pub medium_type: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<bool>,
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

/// Request to create a party role
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePartyRoleRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<CreateContactMediumRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub engagement_date: Option<DateTime<Utc>>,
}

/// Request to create a contact medium
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateContactMediumRequest {
    pub medium_type: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<bool>,
}

/// Request to create a related party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRelatedPartyRequest {
    pub name: String,
    pub role: String,
}

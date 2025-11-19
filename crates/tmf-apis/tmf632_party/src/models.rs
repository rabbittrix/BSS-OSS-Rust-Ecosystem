//! TMF632 Party Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Party State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PartyState {
    Initialized,
    Validated,
    Active,
    Suspended,
    Terminated,
}

/// Party Type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PartyType {
    Individual,
    Organization,
}

/// Party - Represents an individual or organization
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Party {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Party state
    pub state: PartyState,
    /// Party type (individual or organization)
    pub party_type: PartyType,
    /// Contact information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<ContactMedium>>,
    /// Related party (parent organization, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Account reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Vec<AccountRef>>,
    /// Characteristic (attributes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<Vec<Characteristic>>,
    /// Registration date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub registration_date: Option<DateTime<Utc>>,
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

/// Account Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AccountRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
}

/// Characteristic (attribute)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Characteristic {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

/// Request to create a party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePartyRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub party_type: PartyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<CreateContactMediumRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Vec<CreateAccountRefRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<Vec<CreateCharacteristicRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub registration_date: Option<DateTime<Utc>>,
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

/// Request to create an account reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAccountRefRequest {
    pub name: String,
}

/// Request to create a characteristic
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCharacteristicRequest {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

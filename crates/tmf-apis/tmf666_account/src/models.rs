//! TMF666 Account Management models

use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Account State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountState {
    Active,
    Inactive,
    Closed,
}

/// Billing Account
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BillingAccount {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Account state
    pub state: AccountState,
    /// Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Related parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

/// Party Account
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PartyAccount {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Account state
    pub state: AccountState,
    /// Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Related parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
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

/// Request to create a billing account
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateBillingAccountRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

/// Request to update a billing account
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateBillingAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<AccountState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
}

/// Request to create a party account
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePartyAccountRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

/// Request to update a party account
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdatePartyAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<AccountState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
}

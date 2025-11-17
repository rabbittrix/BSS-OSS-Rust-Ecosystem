//! TMF629 Customer Management models

use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Customer State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomerState {
    Initial,
    Active,
    Suspended,
    Terminated,
}

/// Customer - Represents a customer profile
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Customer {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Customer state
    pub state: CustomerState,
    /// Customer status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Contact medium (email, phone, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<ContactMedium>>,
    /// Account reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Vec<AccountRef>>,
    /// Related party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Customer characteristic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<Vec<Characteristic>>,
}

/// Contact Medium - Customer contact information
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContactMedium {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    /// Contact medium type (email, phone, postal, etc.)
    pub medium_type: String,
    /// Preferred flag
    #[serde(default)]
    pub preferred: bool,
    /// Contact details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<ContactCharacteristic>,
}

/// Contact Characteristic
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContactCharacteristic {
    /// Contact value (email address, phone number, etc.)
    pub value: String,
    /// Contact type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<String>,
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

/// Related Party - Party related to the customer
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelatedParty {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
    pub role: String,
}

/// Characteristic - Customer characteristic
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Characteristic {
    pub name: String,
    pub value: String,
}

/// Request to create a customer
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCustomerRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<CreateContactMediumRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
}

/// Request to create a contact medium
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateContactMediumRequest {
    pub medium_type: String,
    #[serde(default)]
    pub preferred: bool,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<String>,
}

/// Request to create a related party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRelatedPartyRequest {
    pub name: String,
    pub role: String,
}

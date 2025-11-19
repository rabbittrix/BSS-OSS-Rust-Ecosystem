//! TMF669 Identity & Credential Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Identity State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IdentityState {
    Created,
    Active,
    Suspended,
    Revoked,
    Expired,
}

/// Credential Type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CredentialType {
    Password,
    OAuth,
    Jwt,
    ApiKey,
    Certificate,
}

/// Identity - Represents a digital identity
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Identity {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Identity state
    pub state: IdentityState,
    /// Identity type (user, service, device)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    /// Party reference (who this identity belongs to)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<PartyRef>,
    /// Credentials associated with this identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<Vec<Credential>>,
    /// OAuth client ID (if OAuth-based)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_client_id: Option<String>,
    /// OAuth client secret (hashed, never returned in responses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_client_secret: Option<String>,
    /// JWT issuer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_issuer: Option<String>,
    /// Expiration date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub expiration_date: Option<DateTime<Utc>>,
}

/// Party Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PartyRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
}

/// Credential
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Credential {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    /// Credential type
    pub credential_type: CredentialType,
    /// Credential value (hashed, never returned in responses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_value: Option<String>,
    /// Created date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub created_date: Option<DateTime<Utc>>,
    /// Expiration date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub expiration_date: Option<DateTime<Utc>>,
}

/// Request to create an identity
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateIdentityRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub party_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<Vec<CreateCredentialRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub expiration_date: Option<DateTime<Utc>>,
}

/// Request to create a credential
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCredentialRequest {
    pub credential_type: CredentialType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub expiration_date: Option<DateTime<Utc>>,
}

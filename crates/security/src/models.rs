//! Security Models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// OAuth 2.0 Client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClient {
    pub id: Uuid,
    pub client_id: String,
    pub client_secret_hash: String,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<GrantType>,
    pub scopes: Vec<String>,
    pub identity_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// OAuth Grant Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GrantType {
    AuthorizationCode,
    ClientCredentials,
    RefreshToken,
    Implicit,
}

/// OAuth Authorization Code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationCode {
    pub code: String,
    pub client_id: String,
    pub user_id: Uuid,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// OAuth Access Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: Vec<String>,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// MFA Method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MfaMethod {
    Totp,
    Sms,
    Email,
    BackupCode,
}

/// MFA Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    pub id: Uuid,
    pub identity_id: Uuid,
    pub method: MfaMethod,
    pub secret: Option<String>,       // Encrypted secret for TOTP
    pub phone_number: Option<String>, // For SMS
    pub email: Option<String>,        // For Email
    pub backup_codes: Vec<String>,    // Hashed backup codes
    pub is_enabled: bool,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
}

/// MFA Challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaChallenge {
    pub id: Uuid,
    pub identity_id: Uuid,
    pub method: MfaMethod,
    pub challenge_code: String, // For SMS/Email
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub verified: bool,
}

/// Role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<Permission>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Permission
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Permission {
    pub resource: String,
    pub action: String,
}

impl Permission {
    pub fn new(resource: String, action: String) -> Self {
        Self { resource, action }
    }
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.resource, self.action)
    }
}

/// User Role Assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub id: Uuid,
    pub identity_id: Uuid,
    pub role_id: Uuid,
    pub assigned_at: DateTime<Utc>,
    pub assigned_by: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Audit Event Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditEventType {
    Authentication,
    Authorization,
    RoleAssignment,
    PermissionChange,
    OAuthTokenIssued,
    OAuthTokenRevoked,
    MfaEnabled,
    MfaDisabled,
    MfaVerified,
    PasswordChange,
    AccountLocked,
    AccountUnlocked,
    SecurityPolicyViolation,
}

/// Audit Log Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: Uuid,
    pub event_type: AuditEventType,
    pub identity_id: Option<Uuid>,
    pub user_id: Option<String>,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub result: AuditResult,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub details: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

/// Audit Result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditResult {
    Success,
    Failure,
    Denied,
}

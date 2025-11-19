//! Database operations for TMF669 Identity & Credential Management

use crate::models::{CreateIdentityRequest, CredentialType, Identity, IdentityState};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse identity state from database string
fn parse_identity_state(s: &str) -> IdentityState {
    match s.to_uppercase().as_str() {
        "CREATED" => IdentityState::Created,
        "ACTIVE" => IdentityState::Active,
        "SUSPENDED" => IdentityState::Suspended,
        "REVOKED" => IdentityState::Revoked,
        "EXPIRED" => IdentityState::Expired,
        _ => IdentityState::Created,
    }
}

/// Convert identity state to database string
fn identity_state_to_string(state: &IdentityState) -> String {
    match state {
        IdentityState::Created => "CREATED".to_string(),
        IdentityState::Active => "ACTIVE".to_string(),
        IdentityState::Suspended => "SUSPENDED".to_string(),
        IdentityState::Revoked => "REVOKED".to_string(),
        IdentityState::Expired => "EXPIRED".to_string(),
    }
}

/// Parse credential type from database string
#[allow(dead_code)]
fn parse_credential_type(s: &str) -> CredentialType {
    match s.to_uppercase().as_str() {
        "PASSWORD" => CredentialType::Password,
        "OAUTH" => CredentialType::OAuth,
        "JWT" => CredentialType::Jwt,
        "API_KEY" => CredentialType::ApiKey,
        "CERTIFICATE" => CredentialType::Certificate,
        _ => CredentialType::Password,
    }
}

/// Convert credential type to database string
fn credential_type_to_string(cred_type: &CredentialType) -> String {
    match cred_type {
        CredentialType::Password => "PASSWORD".to_string(),
        CredentialType::OAuth => "OAUTH".to_string(),
        CredentialType::Jwt => "JWT".to_string(),
        CredentialType::ApiKey => "API_KEY".to_string(),
        CredentialType::Certificate => "CERTIFICATE".to_string(),
    }
}

/// Get all identities
pub async fn get_identities(pool: &Pool<Postgres>) -> TmfResult<Vec<Identity>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, identity_type, party_id, 
         oauth_client_id, jwt_issuer, expiration_date, href, last_update
         FROM identities ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut identities = Vec::new();
    for row in rows {
        identities.push(Identity {
            base: tmf_apis_core::BaseEntity {
                id: row.get::<Uuid, _>("id"),
                href: row.get::<Option<String>, _>("href"),
                name: row.get::<String, _>("name"),
                description: row.get::<Option<String>, _>("description"),
                version: row.get::<Option<String>, _>("version"),
                lifecycle_status: tmf_apis_core::LifecycleStatus::Active,
                last_update: row.get::<Option<DateTime<Utc>>, _>("last_update"),
                valid_for: None,
            },
            state: parse_identity_state(&row.get::<String, _>("state")),
            identity_type: row.get::<Option<String>, _>("identity_type"),
            party: None,      // Load separately if needed
            credential: None, // Load separately if needed
            oauth_client_id: row.get::<Option<String>, _>("oauth_client_id"),
            oauth_client_secret: None, // Never return secrets
            jwt_issuer: row.get::<Option<String>, _>("jwt_issuer"),
            expiration_date: row.get::<Option<DateTime<Utc>>, _>("expiration_date"),
        });
    }

    Ok(identities)
}

/// Get identity by ID
pub async fn get_identity_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<Identity> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, identity_type, party_id, 
         oauth_client_id, jwt_issuer, expiration_date, href, last_update
         FROM identities WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Identity with id {} not found", id)))?;

    Ok(Identity {
        base: tmf_apis_core::BaseEntity {
            id: row.get::<Uuid, _>("id"),
            href: row.get::<Option<String>, _>("href"),
            name: row.get::<String, _>("name"),
            description: row.get::<Option<String>, _>("description"),
            version: row.get::<Option<String>, _>("version"),
            lifecycle_status: tmf_apis_core::LifecycleStatus::Active,
            last_update: row.get::<Option<DateTime<Utc>>, _>("last_update"),
            valid_for: None,
        },
        state: parse_identity_state(&row.get::<String, _>("state")),
        identity_type: row.get::<Option<String>, _>("identity_type"),
        party: None,
        credential: None,
        oauth_client_id: row.get::<Option<String>, _>("oauth_client_id"),
        oauth_client_secret: None, // Never return secrets
        jwt_issuer: row.get::<Option<String>, _>("jwt_issuer"),
        expiration_date: row.get::<Option<DateTime<Utc>>, _>("expiration_date"),
    })
}

/// Create a new identity
pub async fn create_identity(
    pool: &Pool<Postgres>,
    request: CreateIdentityRequest,
) -> TmfResult<Identity> {
    let id = Uuid::new_v4();
    let state = identity_state_to_string(&IdentityState::Created);

    sqlx::query(
        "INSERT INTO identities (id, name, description, version, state, identity_type, 
         party_id, oauth_client_id, oauth_client_secret, jwt_issuer, expiration_date)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(&request.identity_type)
    .bind(request.party_id)
    .bind(&request.oauth_client_id)
    .bind(&request.oauth_client_secret)
    .bind(&request.jwt_issuer)
    .bind(request.expiration_date)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create credentials if provided
    if let Some(credentials) = request.credential {
        for cred in credentials {
            let cred_id = Uuid::new_v4();
            let cred_type = credential_type_to_string(&cred.credential_type);
            let now = Utc::now();

            sqlx::query(
                "INSERT INTO identity_credentials (id, identity_id, credential_type, 
                 credential_value, created_date, expiration_date)
                 VALUES ($1, $2, $3, $4, $5, $6)",
            )
            .bind(cred_id)
            .bind(id)
            .bind(&cred_type)
            .bind(&cred.credential_value)
            .bind(now)
            .bind(cred.expiration_date)
            .execute(pool)
            .await
            .map_err(map_sqlx_error)?;
        }
    }

    // Fetch the created identity
    get_identity_by_id(pool, id).await
}

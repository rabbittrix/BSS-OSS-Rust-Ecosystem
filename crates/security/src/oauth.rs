//! OAuth 2.0 / OIDC Integration
//!
//! Implements OAuth 2.0 authorization server and OpenID Connect (OIDC) support

use crate::error::SecurityError;
use crate::models::{AccessToken, AuthorizationCode, GrantType, OAuthClient};
use chrono::{Duration, Utc};
use log::info;
use rand::Rng;
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// OAuth 2.0 Provider
pub struct OAuthProvider {
    pool: PgPool,
    issuer: String,
    access_token_ttl: i64,       // in seconds
    refresh_token_ttl: i64,      // in seconds
    authorization_code_ttl: i64, // in seconds
}

impl OAuthProvider {
    /// Create a new OAuth provider
    pub fn new(pool: PgPool, issuer: String) -> Self {
        Self {
            pool,
            issuer,
            access_token_ttl: 3600,        // 1 hour
            refresh_token_ttl: 86400 * 30, // 30 days
            authorization_code_ttl: 600,   // 10 minutes
        }
    }

    /// Register a new OAuth client
    pub async fn register_client(
        &self,
        client_id: String,
        client_secret: String,
        redirect_uris: Vec<String>,
        grant_types: Vec<GrantType>,
        scopes: Vec<String>,
        identity_id: Uuid,
    ) -> Result<OAuthClient, SecurityError> {
        // Hash the client secret
        let client_secret_hash = self.hash_secret(&client_secret);

        let id = Uuid::new_v4();
        let grant_types_str: Vec<String> = grant_types.iter().map(grant_type_to_string).collect();
        let scopes_str = scopes.join(" ");

        sqlx::query(
            "INSERT INTO oauth_clients (id, client_id, client_secret_hash, redirect_uris, 
             grant_types, scopes, identity_id, is_active, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(id)
        .bind(&client_id)
        .bind(&client_secret_hash)
        .bind(&redirect_uris)
        .bind(&grant_types_str)
        .bind(&scopes_str)
        .bind(identity_id)
        .bind(true)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        info!("Registered OAuth client: {}", client_id);

        Ok(OAuthClient {
            id,
            client_id,
            client_secret_hash,
            redirect_uris,
            grant_types,
            scopes,
            identity_id,
            created_at: Utc::now(),
            expires_at: None,
            is_active: true,
        })
    }

    /// Validate OAuth client credentials
    pub async fn validate_client(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<OAuthClient, SecurityError> {
        let row = sqlx::query_as::<_, OAuthClientRow>(
            "SELECT id, client_id, client_secret_hash, redirect_uris, grant_types, scopes,
             identity_id, created_at, expires_at, is_active
             FROM oauth_clients WHERE client_id = $1 AND is_active = true",
        )
        .bind(client_id)
        .fetch_optional(&self.pool)
        .await?;

        let client_row =
            row.ok_or_else(|| SecurityError::OAuth("Invalid client credentials".to_string()))?;

        // Verify client secret
        let provided_hash = self.hash_secret(client_secret);
        if provided_hash != client_row.client_secret_hash {
            return Err(SecurityError::OAuth(
                "Invalid client credentials".to_string(),
            ));
        }

        Ok(OAuthClient {
            id: client_row.id,
            client_id: client_row.client_id,
            client_secret_hash: client_row.client_secret_hash,
            redirect_uris: client_row.redirect_uris,
            grant_types: client_row
                .grant_types
                .iter()
                .map(|s| string_to_grant_type(s))
                .collect(),
            scopes: client_row
                .scopes
                .split(' ')
                .map(|s| s.to_string())
                .collect(),
            identity_id: client_row.identity_id,
            created_at: client_row.created_at,
            expires_at: client_row.expires_at,
            is_active: client_row.is_active,
        })
    }

    /// Generate authorization code
    pub async fn generate_authorization_code(
        &self,
        client_id: String,
        user_id: Uuid,
        redirect_uri: String,
        scopes: Vec<String>,
        code_challenge: Option<String>,
        code_challenge_method: Option<String>,
    ) -> Result<AuthorizationCode, SecurityError> {
        let code = self.generate_random_code(32);
        let expires_at = Utc::now() + Duration::seconds(self.authorization_code_ttl);

        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO authorization_codes (id, code, client_id, user_id, redirect_uri, scopes,
             code_challenge, code_challenge_method, expires_at, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(id)
        .bind(&code)
        .bind(&client_id)
        .bind(user_id)
        .bind(&redirect_uri)
        .bind(&scopes)
        .bind(&code_challenge)
        .bind(&code_challenge_method)
        .bind(expires_at)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        info!("Generated authorization code for client: {}", client_id);

        Ok(AuthorizationCode {
            code,
            client_id,
            user_id,
            redirect_uri,
            scopes,
            code_challenge,
            code_challenge_method,
            expires_at,
            created_at: Utc::now(),
        })
    }

    /// Exchange authorization code for access token
    pub async fn exchange_code_for_token(
        &self,
        code: &str,
        client_id: &str,
        redirect_uri: &str,
        code_verifier: Option<&str>,
    ) -> Result<AccessToken, SecurityError> {
        // Get authorization code
        let row = sqlx::query_as::<_, AuthorizationCodeRow>(
            "SELECT code, client_id, user_id, redirect_uri, scopes, code_challenge,
             code_challenge_method, expires_at, created_at
             FROM authorization_codes WHERE code = $1 AND client_id = $2",
        )
        .bind(code)
        .bind(client_id)
        .fetch_optional(&self.pool)
        .await?;

        let auth_code =
            row.ok_or_else(|| SecurityError::OAuth("Invalid authorization code".to_string()))?;

        // Check expiration
        if auth_code.expires_at < Utc::now() {
            return Err(SecurityError::OAuth(
                "Authorization code expired".to_string(),
            ));
        }

        // Verify redirect URI
        if auth_code.redirect_uri != redirect_uri {
            return Err(SecurityError::OAuth("Invalid redirect URI".to_string()));
        }

        // Verify PKCE if present
        if let Some(ref challenge) = auth_code.code_challenge {
            if let Some(verifier) = code_verifier {
                let verifier_hash = self.hash_secret(verifier);
                if verifier_hash != *challenge {
                    return Err(SecurityError::OAuth("Invalid code verifier".to_string()));
                }
            } else {
                return Err(SecurityError::OAuth("Code verifier required".to_string()));
            }
        }

        // Generate access token
        let access_token = self
            .generate_access_token(client_id, Some(auth_code.user_id), &auth_code.scopes)
            .await?;

        // Delete used authorization code
        sqlx::query("DELETE FROM authorization_codes WHERE code = $1")
            .bind(code)
            .execute(&self.pool)
            .await?;

        info!(
            "Exchanged authorization code for access token for client: {}",
            client_id
        );

        Ok(access_token)
    }

    /// Generate access token (client credentials flow)
    pub async fn generate_client_credentials_token(
        &self,
        client_id: &str,
        scopes: &[String],
    ) -> Result<AccessToken, SecurityError> {
        self.generate_access_token(client_id, None, scopes).await
    }

    /// Generate access token
    async fn generate_access_token(
        &self,
        client_id: &str,
        user_id: Option<Uuid>,
        scopes: &[String],
    ) -> Result<AccessToken, SecurityError> {
        let token = self.generate_random_code(64);
        let refresh_token = Some(self.generate_random_code(64));
        let expires_at = Utc::now() + Duration::seconds(self.access_token_ttl);
        let refresh_expires_at = Utc::now() + Duration::seconds(self.refresh_token_ttl);

        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO access_tokens (id, token, token_type, expires_in, refresh_token,
             refresh_expires_at, scope, client_id, user_id, created_at, expires_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        )
        .bind(id)
        .bind(&token)
        .bind("Bearer")
        .bind(self.access_token_ttl)
        .bind(refresh_token.as_ref())
        .bind(refresh_expires_at)
        .bind(scopes.join(" "))
        .bind(client_id)
        .bind(user_id)
        .bind(Utc::now())
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        Ok(AccessToken {
            token,
            token_type: "Bearer".to_string(),
            expires_in: self.access_token_ttl,
            refresh_token,
            scope: scopes.to_vec(),
            client_id: client_id.to_string(),
            user_id,
            created_at: Utc::now(),
            expires_at,
        })
    }

    /// Validate access token
    pub async fn validate_access_token(&self, token: &str) -> Result<AccessToken, SecurityError> {
        let row = sqlx::query_as::<_, AccessTokenRow>(
            "SELECT token, token_type, expires_in, refresh_token, scope, client_id, user_id,
             created_at, expires_at
             FROM access_tokens WHERE token = $1 AND expires_at > CURRENT_TIMESTAMP",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        let token_row =
            row.ok_or_else(|| SecurityError::OAuth("Invalid or expired access token".to_string()))?;

        Ok(AccessToken {
            token: token_row.token,
            token_type: token_row.token_type,
            expires_in: token_row.expires_in,
            refresh_token: token_row.refresh_token,
            scope: token_row.scope.split(' ').map(|s| s.to_string()).collect(),
            client_id: token_row.client_id,
            user_id: token_row.user_id,
            created_at: token_row.created_at,
            expires_at: token_row.expires_at,
        })
    }

    /// Refresh access token
    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<AccessToken, SecurityError> {
        let row = sqlx::query_as::<_, AccessTokenRow>(
            "SELECT token, token_type, expires_in, refresh_token, scope, client_id, user_id,
             created_at, expires_at
             FROM access_tokens WHERE refresh_token = $1",
        )
        .bind(refresh_token)
        .fetch_optional(&self.pool)
        .await?;

        let old_token =
            row.ok_or_else(|| SecurityError::OAuth("Invalid refresh token".to_string()))?;

        // Check if refresh token is still valid
        let refresh_expires_at = old_token.created_at + Duration::seconds(self.refresh_token_ttl);
        if refresh_expires_at < Utc::now() {
            return Err(SecurityError::OAuth("Refresh token expired".to_string()));
        }

        // Revoke old token
        sqlx::query(
            "UPDATE access_tokens SET expires_at = CURRENT_TIMESTAMP WHERE refresh_token = $1",
        )
        .bind(refresh_token)
        .execute(&self.pool)
        .await?;

        // Generate new access token
        let scopes: Vec<String> = old_token.scope.split(' ').map(|s| s.to_string()).collect();
        self.generate_access_token(&old_token.client_id, old_token.user_id, &scopes)
            .await
    }

    /// Revoke access token
    pub async fn revoke_token(&self, token: &str) -> Result<(), SecurityError> {
        sqlx::query("UPDATE access_tokens SET expires_at = CURRENT_TIMESTAMP WHERE token = $1 OR refresh_token = $1")
            .bind(token)
            .execute(&self.pool)
            .await?;

        info!("Revoked access token");
        Ok(())
    }

    /// Get OIDC discovery document
    pub fn get_discovery_document(&self) -> serde_json::Value {
        serde_json::json!({
            "issuer": self.issuer,
            "authorization_endpoint": format!("{}/oauth/authorize", self.issuer),
            "token_endpoint": format!("{}/oauth/token", self.issuer),
            "userinfo_endpoint": format!("{}/oauth/userinfo", self.issuer),
            "jwks_uri": format!("{}/oauth/jwks", self.issuer),
            "response_types_supported": ["code", "token", "id_token"],
            "grant_types_supported": ["authorization_code", "client_credentials", "refresh_token"],
            "scopes_supported": ["openid", "profile", "email", "offline_access"],
            "token_endpoint_auth_methods_supported": ["client_secret_basic", "client_secret_post"],
            "code_challenge_methods_supported": ["plain", "S256"]
        })
    }

    /// Helper: Generate random code
    fn generate_random_code(&self, length: usize) -> String {
        const CHARSET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Helper: Hash secret
    fn hash_secret(&self, secret: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// Helper functions
fn grant_type_to_string(grant_type: &GrantType) -> String {
    match grant_type {
        GrantType::AuthorizationCode => "AUTHORIZATION_CODE".to_string(),
        GrantType::ClientCredentials => "CLIENT_CREDENTIALS".to_string(),
        GrantType::RefreshToken => "REFRESH_TOKEN".to_string(),
        GrantType::Implicit => "IMPLICIT".to_string(),
    }
}

fn string_to_grant_type(s: &str) -> GrantType {
    match s {
        "AUTHORIZATION_CODE" => GrantType::AuthorizationCode,
        "CLIENT_CREDENTIALS" => GrantType::ClientCredentials,
        "REFRESH_TOKEN" => GrantType::RefreshToken,
        "IMPLICIT" => GrantType::Implicit,
        _ => GrantType::AuthorizationCode,
    }
}

/// Internal row structures
#[derive(Debug, FromRow)]
struct OAuthClientRow {
    id: Uuid,
    client_id: String,
    client_secret_hash: String,
    redirect_uris: Vec<String>,
    grant_types: Vec<String>,
    scopes: String,
    identity_id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    expires_at: Option<chrono::DateTime<chrono::Utc>>,
    is_active: bool,
}

#[allow(dead_code)]
#[derive(Debug, FromRow)]
struct AuthorizationCodeRow {
    code: String,
    client_id: String,
    user_id: Uuid,
    redirect_uri: String,
    scopes: Vec<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
    expires_at: chrono::DateTime<chrono::Utc>,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow)]
struct AccessTokenRow {
    token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: Option<String>,
    scope: String,
    client_id: String,
    user_id: Option<Uuid>,
    created_at: chrono::DateTime<chrono::Utc>,
    expires_at: chrono::DateTime<chrono::Utc>,
}

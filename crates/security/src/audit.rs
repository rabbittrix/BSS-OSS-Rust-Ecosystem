//! Audit Logging for Security Events
//!
//! Logs all security-related events for compliance and forensics

use crate::error::SecurityError;
use crate::models::{AuditEventType, AuditLogEntry, AuditResult};
use chrono::Utc;
use log::info;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// Audit Logger
pub struct AuditLogger {
    pool: PgPool,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Log an audit event
    #[allow(clippy::too_many_arguments)]
    pub async fn log_event(
        &self,
        event_type: AuditEventType,
        identity_id: Option<Uuid>,
        user_id: Option<String>,
        resource: Option<String>,
        action: Option<String>,
        result: AuditResult,
        ip_address: Option<String>,
        user_agent: Option<String>,
        details: Option<serde_json::Value>,
    ) -> Result<Uuid, SecurityError> {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();

        sqlx::query(
            "INSERT INTO audit_logs (id, event_type, identity_id, user_id, resource, action,
             result, ip_address, user_agent, details, timestamp)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        )
        .bind(id)
        .bind(event_type_to_string(&event_type))
        .bind(identity_id)
        .bind(&user_id)
        .bind(&resource)
        .bind(&action)
        .bind(result_to_string(&result))
        .bind(&ip_address)
        .bind(&user_agent)
        .bind(&details)
        .bind(timestamp)
        .execute(&self.pool)
        .await?;

        info!("Audit event logged: {:?} - {:?}", event_type, result);

        Ok(id)
    }

    /// Log authentication event
    pub async fn log_authentication(
        &self,
        identity_id: Option<Uuid>,
        user_id: Option<String>,
        result: AuditResult,
        ip_address: Option<String>,
        user_agent: Option<String>,
        details: Option<serde_json::Value>,
    ) -> Result<Uuid, SecurityError> {
        self.log_event(
            AuditEventType::Authentication,
            identity_id,
            user_id,
            None,
            None,
            result,
            ip_address,
            user_agent,
            details,
        )
        .await
    }

    /// Log authorization event
    #[allow(clippy::too_many_arguments)]
    pub async fn log_authorization(
        &self,
        identity_id: Option<Uuid>,
        user_id: Option<String>,
        resource: String,
        action: String,
        result: AuditResult,
        ip_address: Option<String>,
        user_agent: Option<String>,
        details: Option<serde_json::Value>,
    ) -> Result<Uuid, SecurityError> {
        self.log_event(
            AuditEventType::Authorization,
            identity_id,
            user_id,
            Some(resource),
            Some(action),
            result,
            ip_address,
            user_agent,
            details,
        )
        .await
    }

    /// Log role assignment event
    pub async fn log_role_assignment(
        &self,
        identity_id: Uuid,
        role_id: Uuid,
        assigned_by: Option<Uuid>,
        result: AuditResult,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Uuid, SecurityError> {
        let details = serde_json::json!({
            "role_id": role_id,
            "assigned_by": assigned_by
        });

        self.log_event(
            AuditEventType::RoleAssignment,
            Some(identity_id),
            None,
            Some("role".to_string()),
            Some("assign".to_string()),
            result,
            ip_address,
            user_agent,
            Some(details),
        )
        .await
    }

    /// Log OAuth token issued event
    pub async fn log_oauth_token_issued(
        &self,
        identity_id: Option<Uuid>,
        client_id: String,
        scopes: Vec<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Uuid, SecurityError> {
        let details = serde_json::json!({
            "client_id": client_id,
            "scopes": scopes
        });

        self.log_event(
            AuditEventType::OAuthTokenIssued,
            identity_id,
            None,
            Some("oauth_token".to_string()),
            Some("issue".to_string()),
            AuditResult::Success,
            ip_address,
            user_agent,
            Some(details),
        )
        .await
    }

    /// Log OAuth token revoked event
    pub async fn log_oauth_token_revoked(
        &self,
        identity_id: Option<Uuid>,
        client_id: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Uuid, SecurityError> {
        let details = serde_json::json!({
            "client_id": client_id
        });

        self.log_event(
            AuditEventType::OAuthTokenRevoked,
            identity_id,
            None,
            Some("oauth_token".to_string()),
            Some("revoke".to_string()),
            AuditResult::Success,
            ip_address,
            user_agent,
            Some(details),
        )
        .await
    }

    /// Log MFA enabled event
    pub async fn log_mfa_enabled(
        &self,
        identity_id: Uuid,
        method: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Uuid, SecurityError> {
        let details = serde_json::json!({
            "method": method
        });

        self.log_event(
            AuditEventType::MfaEnabled,
            Some(identity_id),
            None,
            Some("mfa".to_string()),
            Some("enable".to_string()),
            AuditResult::Success,
            ip_address,
            user_agent,
            Some(details),
        )
        .await
    }

    /// Log MFA disabled event
    pub async fn log_mfa_disabled(
        &self,
        identity_id: Uuid,
        method: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Uuid, SecurityError> {
        let details = serde_json::json!({
            "method": method
        });

        self.log_event(
            AuditEventType::MfaDisabled,
            Some(identity_id),
            None,
            Some("mfa".to_string()),
            Some("disable".to_string()),
            AuditResult::Success,
            ip_address,
            user_agent,
            Some(details),
        )
        .await
    }

    /// Log security policy violation
    #[allow(clippy::too_many_arguments)]
    pub async fn log_security_policy_violation(
        &self,
        identity_id: Option<Uuid>,
        user_id: Option<String>,
        resource: String,
        action: String,
        violation_type: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
        details: Option<serde_json::Value>,
    ) -> Result<Uuid, SecurityError> {
        let mut violation_details = details.unwrap_or_else(|| serde_json::json!({}));
        violation_details["violation_type"] = serde_json::Value::String(violation_type);

        self.log_event(
            AuditEventType::SecurityPolicyViolation,
            identity_id,
            user_id,
            Some(resource),
            Some(action),
            AuditResult::Denied,
            ip_address,
            user_agent,
            Some(violation_details),
        )
        .await
    }

    /// Get audit logs for an identity
    pub async fn get_identity_logs(
        &self,
        identity_id: Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<AuditLogEntry>, SecurityError> {
        let limit = limit.unwrap_or(100);

        let rows = sqlx::query_as::<_, AuditLogRow>(
            "SELECT id, event_type, identity_id, user_id, resource, action, result,
             ip_address, user_agent, details, timestamp
             FROM audit_logs
             WHERE identity_id = $1
             ORDER BY timestamp DESC
             LIMIT $2",
        )
        .bind(identity_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| AuditLogEntry {
                id: r.id,
                event_type: string_to_event_type(&r.event_type),
                identity_id: r.identity_id,
                user_id: r.user_id,
                resource: r.resource,
                action: r.action,
                result: string_to_result(&r.result),
                ip_address: r.ip_address,
                user_agent: r.user_agent,
                details: r.details,
                timestamp: r.timestamp,
            })
            .collect())
    }

    /// Get audit logs by event type
    pub async fn get_logs_by_event_type(
        &self,
        event_type: AuditEventType,
        limit: Option<i64>,
    ) -> Result<Vec<AuditLogEntry>, SecurityError> {
        let limit = limit.unwrap_or(100);

        let rows = sqlx::query_as::<_, AuditLogRow>(
            "SELECT id, event_type, identity_id, user_id, resource, action, result,
             ip_address, user_agent, details, timestamp
             FROM audit_logs
             WHERE event_type = $1
             ORDER BY timestamp DESC
             LIMIT $2",
        )
        .bind(event_type_to_string(&event_type))
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| AuditLogEntry {
                id: r.id,
                event_type: string_to_event_type(&r.event_type),
                identity_id: r.identity_id,
                user_id: r.user_id,
                resource: r.resource,
                action: r.action,
                result: string_to_result(&r.result),
                ip_address: r.ip_address,
                user_agent: r.user_agent,
                details: r.details,
                timestamp: r.timestamp,
            })
            .collect())
    }

    /// Get audit logs by date range
    pub async fn get_logs_by_date_range(
        &self,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        limit: Option<i64>,
    ) -> Result<Vec<AuditLogEntry>, SecurityError> {
        let limit = limit.unwrap_or(1000);

        let rows = sqlx::query_as::<_, AuditLogRow>(
            "SELECT id, event_type, identity_id, user_id, resource, action, result,
             ip_address, user_agent, details, timestamp
             FROM audit_logs
             WHERE timestamp >= $1 AND timestamp <= $2
             ORDER BY timestamp DESC
             LIMIT $3",
        )
        .bind(start_date)
        .bind(end_date)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| AuditLogEntry {
                id: r.id,
                event_type: string_to_event_type(&r.event_type),
                identity_id: r.identity_id,
                user_id: r.user_id,
                resource: r.resource,
                action: r.action,
                result: string_to_result(&r.result),
                ip_address: r.ip_address,
                user_agent: r.user_agent,
                details: r.details,
                timestamp: r.timestamp,
            })
            .collect())
    }
}

/// Helper functions
fn event_type_to_string(event_type: &AuditEventType) -> String {
    match event_type {
        AuditEventType::Authentication => "AUTHENTICATION".to_string(),
        AuditEventType::Authorization => "AUTHORIZATION".to_string(),
        AuditEventType::RoleAssignment => "ROLE_ASSIGNMENT".to_string(),
        AuditEventType::PermissionChange => "PERMISSION_CHANGE".to_string(),
        AuditEventType::OAuthTokenIssued => "OAUTH_TOKEN_ISSUED".to_string(),
        AuditEventType::OAuthTokenRevoked => "OAUTH_TOKEN_REVOKED".to_string(),
        AuditEventType::MfaEnabled => "MFA_ENABLED".to_string(),
        AuditEventType::MfaDisabled => "MFA_DISABLED".to_string(),
        AuditEventType::MfaVerified => "MFA_VERIFIED".to_string(),
        AuditEventType::PasswordChange => "PASSWORD_CHANGE".to_string(),
        AuditEventType::AccountLocked => "ACCOUNT_LOCKED".to_string(),
        AuditEventType::AccountUnlocked => "ACCOUNT_UNLOCKED".to_string(),
        AuditEventType::SecurityPolicyViolation => "SECURITY_POLICY_VIOLATION".to_string(),
    }
}

fn string_to_event_type(s: &str) -> AuditEventType {
    match s {
        "AUTHENTICATION" => AuditEventType::Authentication,
        "AUTHORIZATION" => AuditEventType::Authorization,
        "ROLE_ASSIGNMENT" => AuditEventType::RoleAssignment,
        "PERMISSION_CHANGE" => AuditEventType::PermissionChange,
        "OAUTH_TOKEN_ISSUED" => AuditEventType::OAuthTokenIssued,
        "OAUTH_TOKEN_REVOKED" => AuditEventType::OAuthTokenRevoked,
        "MFA_ENABLED" => AuditEventType::MfaEnabled,
        "MFA_DISABLED" => AuditEventType::MfaDisabled,
        "MFA_VERIFIED" => AuditEventType::MfaVerified,
        "PASSWORD_CHANGE" => AuditEventType::PasswordChange,
        "ACCOUNT_LOCKED" => AuditEventType::AccountLocked,
        "ACCOUNT_UNLOCKED" => AuditEventType::AccountUnlocked,
        "SECURITY_POLICY_VIOLATION" => AuditEventType::SecurityPolicyViolation,
        _ => AuditEventType::Authentication,
    }
}

fn result_to_string(result: &AuditResult) -> String {
    match result {
        AuditResult::Success => "SUCCESS".to_string(),
        AuditResult::Failure => "FAILURE".to_string(),
        AuditResult::Denied => "DENIED".to_string(),
    }
}

fn string_to_result(s: &str) -> AuditResult {
    match s {
        "SUCCESS" => AuditResult::Success,
        "FAILURE" => AuditResult::Failure,
        "DENIED" => AuditResult::Denied,
        _ => AuditResult::Failure,
    }
}

/// Internal row structure
#[derive(Debug, FromRow)]
struct AuditLogRow {
    id: Uuid,
    event_type: String,
    identity_id: Option<Uuid>,
    user_id: Option<String>,
    resource: Option<String>,
    action: Option<String>,
    result: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    details: Option<serde_json::Value>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

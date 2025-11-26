//! Audit logger implementation

use crate::models::{AuditEventType, AuditLogEntry, AuditResult};
use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Comprehensive audit logger
pub struct AuditLogger {
    pool: PgPool,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Log any operation
    #[allow(clippy::too_many_arguments)]
    pub async fn log_operation(
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
    ) -> Result<Uuid, sqlx::Error> {
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

        Ok(id)
    }

    /// Get audit logs for an identity
    pub async fn get_identity_logs(
        &self,
        identity_id: Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<AuditLogEntry>, sqlx::Error> {
        let limit = limit.unwrap_or(100);
        let rows = sqlx::query(
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

        rows.into_iter()
            .map(|row| {
                Ok(AuditLogEntry {
                    id: row.get("id"),
                    event_type: string_to_event_type(&row.get::<String, _>("event_type"))
                        .map_err(|e| sqlx::Error::Decode(Box::new(std::io::Error::other(e))))?,
                    identity_id: row.get("identity_id"),
                    user_id: row.get("user_id"),
                    resource: row.get("resource"),
                    action: row.get("action"),
                    result: string_to_result(&row.get::<String, _>("result"))
                        .map_err(|e| sqlx::Error::Decode(Box::new(std::io::Error::other(e))))?,
                    ip_address: row.get("ip_address"),
                    user_agent: row.get("user_agent"),
                    details: row.get("details"),
                    timestamp: row.get("timestamp"),
                })
            })
            .collect()
    }

    /// Get audit logs by resource
    pub async fn get_resource_logs(
        &self,
        resource: &str,
        limit: Option<i64>,
    ) -> Result<Vec<AuditLogEntry>, sqlx::Error> {
        let limit = limit.unwrap_or(100);
        let rows = sqlx::query(
            "SELECT id, event_type, identity_id, user_id, resource, action, result,
             ip_address, user_agent, details, timestamp
             FROM audit_logs
             WHERE resource = $1
             ORDER BY timestamp DESC
             LIMIT $2",
        )
        .bind(resource)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                Ok(AuditLogEntry {
                    id: row.get("id"),
                    event_type: string_to_event_type(&row.get::<String, _>("event_type"))
                        .map_err(|e| sqlx::Error::Decode(Box::new(std::io::Error::other(e))))?,
                    identity_id: row.get("identity_id"),
                    user_id: row.get("user_id"),
                    resource: row.get("resource"),
                    action: row.get("action"),
                    result: string_to_result(&row.get::<String, _>("result"))
                        .map_err(|e| sqlx::Error::Decode(Box::new(std::io::Error::other(e))))?,
                    ip_address: row.get("ip_address"),
                    user_agent: row.get("user_agent"),
                    details: row.get("details"),
                    timestamp: row.get("timestamp"),
                })
            })
            .collect()
    }
}

fn event_type_to_string(event_type: &AuditEventType) -> String {
    format!("{:?}", event_type)
}

fn result_to_string(result: &AuditResult) -> String {
    format!("{:?}", result)
}

fn string_to_event_type(s: &str) -> Result<AuditEventType, String> {
    match s {
        "AUTHENTICATION" => Ok(AuditEventType::Authentication),
        "AUTHORIZATION" => Ok(AuditEventType::Authorization),
        "ROLE_ASSIGNMENT" => Ok(AuditEventType::RoleAssignment),
        "OAUTH_TOKEN_ISSUED" => Ok(AuditEventType::OAuthTokenIssued),
        "CATALOG_CREATED" => Ok(AuditEventType::CatalogCreated),
        "CATALOG_UPDATED" => Ok(AuditEventType::CatalogUpdated),
        "CATALOG_DELETED" => Ok(AuditEventType::CatalogDeleted),
        "PRODUCT_OFFERING_CREATED" => Ok(AuditEventType::ProductOfferingCreated),
        "PRODUCT_OFFERING_UPDATED" => Ok(AuditEventType::ProductOfferingUpdated),
        "PRODUCT_OFFERING_DELETED" => Ok(AuditEventType::ProductOfferingDeleted),
        "ORDER_CREATED" => Ok(AuditEventType::OrderCreated),
        "ORDER_UPDATED" => Ok(AuditEventType::OrderUpdated),
        "ORDER_CANCELLED" => Ok(AuditEventType::OrderCancelled),
        "ORDER_COMPLETED" => Ok(AuditEventType::OrderCompleted),
        "CUSTOMER_CREATED" => Ok(AuditEventType::CustomerCreated),
        "CUSTOMER_UPDATED" => Ok(AuditEventType::CustomerUpdated),
        "CUSTOMER_DELETED" => Ok(AuditEventType::CustomerDeleted),
        "SERVICE_ORDER_CREATED" => Ok(AuditEventType::ServiceOrderCreated),
        "SERVICE_ORDER_UPDATED" => Ok(AuditEventType::ServiceOrderUpdated),
        "SERVICE_ACTIVATED" => Ok(AuditEventType::ServiceActivated),
        "SERVICE_DEACTIVATED" => Ok(AuditEventType::ServiceDeactivated),
        "RESOURCE_ORDER_CREATED" => Ok(AuditEventType::ResourceOrderCreated),
        "RESOURCE_RESERVED" => Ok(AuditEventType::ResourceReserved),
        "RESOURCE_RELEASED" => Ok(AuditEventType::ResourceReleased),
        "BILL_GENERATED" => Ok(AuditEventType::BillGenerated),
        "PAYMENT_PROCESSED" => Ok(AuditEventType::PaymentProcessed),
        "REFUND_ISSUED" => Ok(AuditEventType::RefundIssued),
        "USAGE_RECORDED" => Ok(AuditEventType::UsageRecorded),
        "USAGE_AGGREGATED" => Ok(AuditEventType::UsageAggregated),
        "CONFIGURATION_CHANGED" => Ok(AuditEventType::ConfigurationChanged),
        "POLICY_UPDATED" => Ok(AuditEventType::PolicyUpdated),
        "SYSTEM_STARTUP" => Ok(AuditEventType::SystemStartup),
        "SYSTEM_SHUTDOWN" => Ok(AuditEventType::SystemShutdown),
        "MAINTENANCE_MODE" => Ok(AuditEventType::MaintenanceMode),
        _ => Err(format!("Unknown event type: {}", s)),
    }
}

fn string_to_result(s: &str) -> Result<AuditResult, String> {
    match s {
        "SUCCESS" => Ok(AuditResult::Success),
        "FAILURE" => Ok(AuditResult::Failure),
        "DENIED" => Ok(AuditResult::Denied),
        "PARTIAL" => Ok(AuditResult::Partial),
        _ => Err(format!("Unknown result: {}", s)),
    }
}

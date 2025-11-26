//! Audit logging models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of audit events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditEventType {
    // Security events
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
    // Catalog events
    CatalogCreated,
    CatalogUpdated,
    CatalogDeleted,
    ProductOfferingCreated,
    ProductOfferingUpdated,
    ProductOfferingDeleted,
    // Order events
    OrderCreated,
    OrderUpdated,
    OrderCancelled,
    OrderCompleted,
    // Customer events
    CustomerCreated,
    CustomerUpdated,
    CustomerDeleted,
    // Service events
    ServiceOrderCreated,
    ServiceOrderUpdated,
    ServiceActivated,
    ServiceDeactivated,
    // Resource events
    ResourceOrderCreated,
    ResourceReserved,
    ResourceReleased,
    // Billing events
    BillGenerated,
    PaymentProcessed,
    RefundIssued,
    // Usage events
    UsageRecorded,
    UsageAggregated,
    // System events
    ConfigurationChanged,
    PolicyUpdated,
    SystemStartup,
    SystemShutdown,
    MaintenanceMode,
}

/// Result of an audit event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditResult {
    Success,
    Failure,
    Denied,
    Partial,
}

/// Audit log entry
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

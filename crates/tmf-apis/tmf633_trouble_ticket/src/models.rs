//! TMF633 Trouble Ticket Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Trouble Ticket Status
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TroubleTicketStatus {
    Submitted,
    Acknowledged,
    InProgress,
    Resolved,
    Closed,
    Cancelled,
}

/// Trouble Ticket Priority
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TroubleTicketPriority {
    Critical,
    High,
    Medium,
    Low,
}

/// Trouble Ticket Type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TroubleTicketType {
    ServiceIssue,
    BillingIssue,
    TechnicalIssue,
    AccountIssue,
    Other,
}

/// Related Entity Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelatedEntity {
    pub id: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Trouble Ticket - Represents a customer service ticket
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TroubleTicket {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub status: TroubleTicketStatus,
    pub priority: TroubleTicketPriority,
    pub ticket_type: TroubleTicketType,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_entity: Option<Vec<RelatedEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<Uuid>,
}

/// Create Trouble Ticket Request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateTroubleTicketRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub ticket_type: TroubleTicketType,
    pub priority: TroubleTicketPriority,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_entity: Option<Vec<RelatedEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
}

/// Update Trouble Ticket Request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateTroubleTicketRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TroubleTicketStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TroubleTicketPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
}

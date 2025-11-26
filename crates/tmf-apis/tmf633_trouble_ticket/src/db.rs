//! Database operations for TMF633 Trouble Ticket Management

use crate::models::{
    CreateTroubleTicketRequest, TroubleTicket, TroubleTicketPriority, TroubleTicketStatus,
    TroubleTicketType, UpdateTroubleTicketRequest,
};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse trouble ticket status from database string
fn parse_ticket_status(s: &str) -> TroubleTicketStatus {
    match s.to_uppercase().as_str() {
        "SUBMITTED" => TroubleTicketStatus::Submitted,
        "ACKNOWLEDGED" => TroubleTicketStatus::Acknowledged,
        "IN_PROGRESS" => TroubleTicketStatus::InProgress,
        "RESOLVED" => TroubleTicketStatus::Resolved,
        "CLOSED" => TroubleTicketStatus::Closed,
        "CANCELLED" => TroubleTicketStatus::Cancelled,
        _ => TroubleTicketStatus::Submitted,
    }
}

/// Convert trouble ticket status to database string
fn ticket_status_to_string(status: &TroubleTicketStatus) -> String {
    match status {
        TroubleTicketStatus::Submitted => "SUBMITTED".to_string(),
        TroubleTicketStatus::Acknowledged => "ACKNOWLEDGED".to_string(),
        TroubleTicketStatus::InProgress => "IN_PROGRESS".to_string(),
        TroubleTicketStatus::Resolved => "RESOLVED".to_string(),
        TroubleTicketStatus::Closed => "CLOSED".to_string(),
        TroubleTicketStatus::Cancelled => "CANCELLED".to_string(),
    }
}

/// Parse trouble ticket priority from database string
fn parse_ticket_priority(s: &str) -> TroubleTicketPriority {
    match s.to_uppercase().as_str() {
        "CRITICAL" => TroubleTicketPriority::Critical,
        "HIGH" => TroubleTicketPriority::High,
        "MEDIUM" => TroubleTicketPriority::Medium,
        "LOW" => TroubleTicketPriority::Low,
        _ => TroubleTicketPriority::Medium,
    }
}

/// Convert trouble ticket priority to database string
fn ticket_priority_to_string(priority: &TroubleTicketPriority) -> String {
    match priority {
        TroubleTicketPriority::Critical => "CRITICAL".to_string(),
        TroubleTicketPriority::High => "HIGH".to_string(),
        TroubleTicketPriority::Medium => "MEDIUM".to_string(),
        TroubleTicketPriority::Low => "LOW".to_string(),
    }
}

/// Parse trouble ticket type from database string
fn parse_ticket_type(s: &str) -> TroubleTicketType {
    match s.to_uppercase().as_str() {
        "SERVICE_ISSUE" => TroubleTicketType::ServiceIssue,
        "BILLING_ISSUE" => TroubleTicketType::BillingIssue,
        "TECHNICAL_ISSUE" => TroubleTicketType::TechnicalIssue,
        "ACCOUNT_ISSUE" => TroubleTicketType::AccountIssue,
        _ => TroubleTicketType::Other,
    }
}

/// Convert trouble ticket type to database string
fn ticket_type_to_string(ticket_type: &TroubleTicketType) -> String {
    match ticket_type {
        TroubleTicketType::ServiceIssue => "SERVICE_ISSUE".to_string(),
        TroubleTicketType::BillingIssue => "BILLING_ISSUE".to_string(),
        TroubleTicketType::TechnicalIssue => "TECHNICAL_ISSUE".to_string(),
        TroubleTicketType::AccountIssue => "ACCOUNT_ISSUE".to_string(),
        TroubleTicketType::Other => "OTHER".to_string(),
    }
}

/// Helper to convert database row to TroubleTicket
fn row_to_trouble_ticket(row: &sqlx::postgres::PgRow) -> TroubleTicket {
    use tmf_apis_core::BaseEntity;
    use tmf_apis_core::LifecycleStatus;

    TroubleTicket {
        base: BaseEntity {
            id: row.get("id"),
            href: row.get("href"),
            name: row.get("name"),
            description: row.get("description"),
            version: row.get("version"),
            lifecycle_status: LifecycleStatus::Active,
            last_update: row.get("last_update"),
            valid_for: None,
        },
        status: parse_ticket_status(row.get("status")),
        priority: parse_ticket_priority(row.get("priority")),
        ticket_type: parse_ticket_type(row.get("ticket_type")),
        description: row.get("description"),
        resolution: row.get("resolution"),
        resolution_date: row.get("resolution_date"),
        related_entity: row
            .try_get::<Option<serde_json::Value>, _>("related_entity")
            .ok()
            .flatten()
            .and_then(|v| serde_json::from_value(v).ok()),
        customer_id: row.get("customer_id"),
        assigned_to: row.get("assigned_to"),
        tenant_id: row.get("tenant_id"),
    }
}

/// Get all trouble tickets
pub async fn get_trouble_tickets(pool: &Pool<Postgres>) -> TmfResult<Vec<TroubleTicket>> {
    let rows = sqlx::query(
        "SELECT id, href, name, description, version, status, priority, ticket_type, 
         description, resolution, resolution_date, related_entity, customer_id, 
         assigned_to, tenant_id, last_update
         FROM trouble_tickets ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(rows.iter().map(row_to_trouble_ticket).collect())
}

/// Get trouble ticket by ID
pub async fn get_trouble_ticket_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<Option<TroubleTicket>> {
    let row = sqlx::query(
        "SELECT id, href, name, description, version, status, priority, ticket_type, 
         description, resolution, resolution_date, related_entity, customer_id, 
         assigned_to, tenant_id, last_update
         FROM trouble_tickets WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(row.map(|r| row_to_trouble_ticket(&r)))
}

/// Create a new trouble ticket
pub async fn create_trouble_ticket(
    pool: &Pool<Postgres>,
    request: CreateTroubleTicketRequest,
) -> TmfResult<TroubleTicket> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let href = format!("/tmf-api/troubleTicket/v4/troubleTicket/{}", id);
    let status = ticket_status_to_string(&TroubleTicketStatus::Submitted);
    let priority = ticket_priority_to_string(&request.priority);
    let ticket_type = ticket_type_to_string(&request.ticket_type);
    let description = request.description.as_deref().unwrap_or("");

    let related_entity_json = request
        .related_entity
        .as_ref()
        .map(|entities| serde_json::to_value(entities).unwrap_or(serde_json::Value::Null));

    sqlx::query(
        "INSERT INTO trouble_tickets (
            id, href, name, description, version, status, priority, ticket_type,
            resolution, resolution_date, related_entity, customer_id, assigned_to,
            tenant_id, created_at, last_update
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)",
    )
    .bind(id)
    .bind(&href)
    .bind(&request.name)
    .bind(description)
    .bind("1.0.0")
    .bind(&status)
    .bind(&priority)
    .bind(&ticket_type)
    .bind::<Option<String>>(None)
    .bind::<Option<DateTime<Utc>>>(None)
    .bind(related_entity_json.as_ref())
    .bind(request.customer_id)
    .bind(request.assigned_to.as_ref())
    .bind::<Option<Uuid>>(None)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_trouble_ticket_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Trouble ticket not found after creation".to_string()))
}

/// Update a trouble ticket
pub async fn update_trouble_ticket(
    pool: &Pool<Postgres>,
    id: Uuid,
    request: UpdateTroubleTicketRequest,
) -> TmfResult<TroubleTicket> {
    let status_str = request.status.as_ref().map(ticket_status_to_string);
    let priority_str = request.priority.as_ref().map(ticket_priority_to_string);
    let resolution_date = if request.resolution.is_some() {
        Some(Utc::now())
    } else {
        None
    };

    sqlx::query(
        "UPDATE trouble_tickets SET 
         status = COALESCE($1, status), 
         priority = COALESCE($2, priority),
         description = COALESCE($3, description),
         resolution = COALESCE($4, resolution),
         resolution_date = COALESCE($5, resolution_date),
         assigned_to = COALESCE($6, assigned_to),
         last_update = CURRENT_TIMESTAMP
         WHERE id = $7",
    )
    .bind(status_str)
    .bind(priority_str)
    .bind(&request.description)
    .bind(&request.resolution)
    .bind(resolution_date)
    .bind(&request.assigned_to)
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_trouble_ticket_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Trouble ticket not found".to_string()))
}

/// Delete a trouble ticket
pub async fn delete_trouble_ticket(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<()> {
    let result = sqlx::query("DELETE FROM trouble_tickets WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

    if result.rows_affected() == 0 {
        return Err(TmfError::NotFound("Trouble ticket not found".to_string()));
    }

    Ok(())
}

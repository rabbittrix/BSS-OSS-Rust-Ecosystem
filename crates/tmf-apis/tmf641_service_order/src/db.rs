//! Database operations for TMF641 Service Order Management

use crate::models::{CreateServiceOrderRequest, ServiceOrder, ServiceOrderState};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse service order state from database string
fn parse_service_order_state(s: &str) -> ServiceOrderState {
    match s.to_uppercase().as_str() {
        "ACKNOWLEDGED" => ServiceOrderState::Acknowledged,
        "IN_PROGRESS" => ServiceOrderState::InProgress,
        "COMPLETED" => ServiceOrderState::Completed,
        "CANCELLED" => ServiceOrderState::Cancelled,
        "REJECTED" => ServiceOrderState::Rejected,
        "HELD" => ServiceOrderState::Held,
        "FAILED" => ServiceOrderState::Failed,
        _ => ServiceOrderState::Acknowledged,
    }
}

/// Convert service order state to database string
fn service_order_state_to_string(state: &ServiceOrderState) -> String {
    match state {
        ServiceOrderState::Acknowledged => "ACKNOWLEDGED".to_string(),
        ServiceOrderState::InProgress => "IN_PROGRESS".to_string(),
        ServiceOrderState::Completed => "COMPLETED".to_string(),
        ServiceOrderState::Cancelled => "CANCELLED".to_string(),
        ServiceOrderState::Rejected => "REJECTED".to_string(),
        ServiceOrderState::Held => "HELD".to_string(),
        ServiceOrderState::Failed => "FAILED".to_string(),
    }
}

/// Get all service orders
pub async fn get_service_orders(pool: &Pool<Postgres>) -> TmfResult<Vec<ServiceOrder>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, order_date, 
         expected_completion_date, priority, external_id, href, last_update
         FROM service_orders ORDER BY order_date DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut orders = Vec::new();
    for row in rows {
        orders.push(ServiceOrder {
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
            state: parse_service_order_state(&row.get::<String, _>("state")),
            order_item: None,    // Load separately if needed
            related_party: None, // Load separately if needed
            order_date: row.get::<Option<DateTime<Utc>>, _>("order_date"),
            expected_completion_date: row
                .get::<Option<DateTime<Utc>>, _>("expected_completion_date"),
            priority: row.get::<Option<String>, _>("priority"),
            external_id: row.get::<Option<String>, _>("external_id"),
        });
    }

    Ok(orders)
}

/// Get service order by ID
pub async fn get_service_order_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<ServiceOrder> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, order_date, 
         expected_completion_date, priority, external_id, href, last_update
         FROM service_orders WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Service order with id {} not found", id)))?;

    Ok(ServiceOrder {
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
        state: parse_service_order_state(&row.get::<String, _>("state")),
        order_item: None,
        related_party: None,
        order_date: row.get::<Option<DateTime<Utc>>, _>("order_date"),
        expected_completion_date: row.get::<Option<DateTime<Utc>>, _>("expected_completion_date"),
        priority: row.get::<Option<String>, _>("priority"),
        external_id: row.get::<Option<String>, _>("external_id"),
    })
}

/// Create a new service order
pub async fn create_service_order(
    pool: &Pool<Postgres>,
    request: CreateServiceOrderRequest,
) -> TmfResult<ServiceOrder> {
    let id = Uuid::new_v4();
    let state = service_order_state_to_string(&ServiceOrderState::Acknowledged);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO service_orders (id, name, description, version, state, order_date, priority, external_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(now)
    .bind(&request.priority)
    .bind(&request.external_id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create service order items if provided
    if let Some(items) = request.order_item {
        for item in items {
            let item_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO service_order_items (id, order_id, action, service_specification_id, 
                 service_id, state, quantity)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)",
            )
            .bind(item_id)
            .bind(id)
            .bind(&item.action)
            .bind(item.service_specification_id)
            .bind(item.service_id)
            .bind(&state)
            .bind(item.quantity)
            .execute(pool)
            .await
            .map_err(map_sqlx_error)?;
        }
    }

    // Create related parties if provided
    if let Some(parties) = request.related_party {
        for party in parties {
            let party_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO service_order_related_parties (id, order_id, name, role)
                 VALUES ($1, $2, $3, $4)",
            )
            .bind(party_id)
            .bind(id)
            .bind(&party.name)
            .bind(&party.role)
            .execute(pool)
            .await
            .map_err(map_sqlx_error)?;
        }
    }

    // Fetch the created service order
    get_service_order_by_id(pool, id).await
}


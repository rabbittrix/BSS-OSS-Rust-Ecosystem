//! Database operations for TMF645 Resource Order Management

use crate::models::{CreateResourceOrderRequest, ResourceOrder, ResourceOrderState};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse resource order state from database string
fn parse_resource_order_state(s: &str) -> ResourceOrderState {
    match s.to_uppercase().as_str() {
        "ACKNOWLEDGED" => ResourceOrderState::Acknowledged,
        "IN_PROGRESS" => ResourceOrderState::InProgress,
        "COMPLETED" => ResourceOrderState::Completed,
        "CANCELLED" => ResourceOrderState::Cancelled,
        "REJECTED" => ResourceOrderState::Rejected,
        "HELD" => ResourceOrderState::Held,
        "FAILED" => ResourceOrderState::Failed,
        _ => ResourceOrderState::Acknowledged,
    }
}

/// Convert resource order state to database string
fn resource_order_state_to_string(state: &ResourceOrderState) -> String {
    match state {
        ResourceOrderState::Acknowledged => "ACKNOWLEDGED".to_string(),
        ResourceOrderState::InProgress => "IN_PROGRESS".to_string(),
        ResourceOrderState::Completed => "COMPLETED".to_string(),
        ResourceOrderState::Cancelled => "CANCELLED".to_string(),
        ResourceOrderState::Rejected => "REJECTED".to_string(),
        ResourceOrderState::Held => "HELD".to_string(),
        ResourceOrderState::Failed => "FAILED".to_string(),
    }
}

/// Get all resource orders
pub async fn get_resource_orders(pool: &Pool<Postgres>) -> TmfResult<Vec<ResourceOrder>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, order_date, 
         expected_completion_date, priority, external_id, href, last_update
         FROM resource_orders ORDER BY order_date DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut orders = Vec::new();
    for row in rows {
        orders.push(ResourceOrder {
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
            state: parse_resource_order_state(&row.get::<String, _>("state")),
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

/// Get resource order by ID
pub async fn get_resource_order_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<ResourceOrder> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, order_date, 
         expected_completion_date, priority, external_id, href, last_update
         FROM resource_orders WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Resource order with id {} not found", id)))?;

    Ok(ResourceOrder {
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
        state: parse_resource_order_state(&row.get::<String, _>("state")),
        order_item: None,
        related_party: None,
        order_date: row.get::<Option<DateTime<Utc>>, _>("order_date"),
        expected_completion_date: row.get::<Option<DateTime<Utc>>, _>("expected_completion_date"),
        priority: row.get::<Option<String>, _>("priority"),
        external_id: row.get::<Option<String>, _>("external_id"),
    })
}

/// Create a new resource order
pub async fn create_resource_order(
    pool: &Pool<Postgres>,
    request: CreateResourceOrderRequest,
) -> TmfResult<ResourceOrder> {
    let id = Uuid::new_v4();
    let state = resource_order_state_to_string(&ResourceOrderState::Acknowledged);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO resource_orders (id, name, description, version, state, order_date, priority, external_id)
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

    // Create resource order items if provided
    if let Some(items) = request.order_item {
        for item in items {
            let item_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO resource_order_items (id, order_id, action, resource_specification_id, 
                 resource_id, state, quantity)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)",
            )
            .bind(item_id)
            .bind(id)
            .bind(&item.action)
            .bind(item.resource_specification_id)
            .bind(item.resource_id)
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
                "INSERT INTO resource_order_related_parties (id, order_id, name, role)
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

    // Fetch the created resource order
    get_resource_order_by_id(pool, id).await
}


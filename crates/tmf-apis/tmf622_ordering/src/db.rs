//! Database operations for TMF622 Product Ordering

use crate::models::{CreateProductOrderRequest, OrderState, ProductOrder};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse order state from database string
fn parse_order_state(s: &str) -> OrderState {
    match s.to_uppercase().as_str() {
        "ACKNOWLEDGED" => OrderState::Acknowledged,
        "IN_PROGRESS" => OrderState::InProgress,
        "COMPLETED" => OrderState::Completed,
        "CANCELLED" => OrderState::Cancelled,
        "REJECTED" => OrderState::Rejected,
        "HELD" => OrderState::Held,
        "FAILED" => OrderState::Failed,
        _ => OrderState::Acknowledged,
    }
}

/// Convert order state to database string
fn order_state_to_string(state: &OrderState) -> String {
    match state {
        OrderState::Acknowledged => "ACKNOWLEDGED".to_string(),
        OrderState::InProgress => "IN_PROGRESS".to_string(),
        OrderState::Completed => "COMPLETED".to_string(),
        OrderState::Cancelled => "CANCELLED".to_string(),
        OrderState::Rejected => "REJECTED".to_string(),
        OrderState::Held => "HELD".to_string(),
        OrderState::Failed => "FAILED".to_string(),
    }
}

/// Get all product orders
pub async fn get_orders(pool: &Pool<Postgres>) -> TmfResult<Vec<ProductOrder>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, order_date, 
         expected_completion_date, priority, href, last_update
         FROM product_orders ORDER BY order_date DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut orders = Vec::new();
    for row in rows {
        orders.push(ProductOrder {
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
            state: parse_order_state(&row.get::<String, _>("state")),
            order_item: None,    // Load separately if needed
            related_party: None, // Load separately if needed
            order_date: row.get::<Option<DateTime<Utc>>, _>("order_date"),
            expected_completion_date: row
                .get::<Option<DateTime<Utc>>, _>("expected_completion_date"),
            priority: row.get::<Option<String>, _>("priority"),
        });
    }

    Ok(orders)
}

/// Get product order by ID
pub async fn get_order_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<ProductOrder> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, order_date, 
         expected_completion_date, priority, href, last_update
         FROM product_orders WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Product order with id {} not found", id)))?;

    Ok(ProductOrder {
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
        state: parse_order_state(&row.get::<String, _>("state")),
        order_item: None,
        related_party: None,
        order_date: row.get::<Option<DateTime<Utc>>, _>("order_date"),
        expected_completion_date: row.get::<Option<DateTime<Utc>>, _>("expected_completion_date"),
        priority: row.get::<Option<String>, _>("priority"),
    })
}

/// Create a new product order
pub async fn create_order(
    pool: &Pool<Postgres>,
    request: CreateProductOrderRequest,
) -> TmfResult<ProductOrder> {
    let id = Uuid::new_v4();
    let state = order_state_to_string(&OrderState::Acknowledged);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO product_orders (id, name, description, version, state, order_date, priority)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(now)
    .bind(&request.priority)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create order items if provided
    if let Some(items) = request.order_item {
        for item in items {
            let item_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO order_items (id, order_id, action, product_offering_id, 
                 product_specification_id, state, quantity)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)",
            )
            .bind(item_id)
            .bind(id)
            .bind(&item.action)
            .bind(item.product_offering_id)
            .bind(item.product_specification_id)
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
                "INSERT INTO related_parties (id, order_id, name, role)
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

    // Fetch the created order
    get_order_by_id(pool, id).await
}

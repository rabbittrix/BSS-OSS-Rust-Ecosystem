//! Database operations for TMF634 Quote Management

use crate::models::{CreateQuoteRequest, Quote, QuoteState, UpdateQuoteRequest};
use chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse quote state from database string
fn parse_quote_state(s: &str) -> QuoteState {
    match s.to_uppercase().as_str() {
        "IN_PROGRESS" => QuoteState::InProgress,
        "READY" => QuoteState::Ready,
        "CANCELLED" => QuoteState::Cancelled,
        "ACCEPTED" => QuoteState::Accepted,
        "REJECTED" => QuoteState::Rejected,
        "EXPIRED" => QuoteState::Expired,
        _ => QuoteState::InProgress,
    }
}

/// Convert quote state to database string
fn quote_state_to_string(state: &QuoteState) -> String {
    match state {
        QuoteState::InProgress => "IN_PROGRESS".to_string(),
        QuoteState::Ready => "READY".to_string(),
        QuoteState::Cancelled => "CANCELLED".to_string(),
        QuoteState::Accepted => "ACCEPTED".to_string(),
        QuoteState::Rejected => "REJECTED".to_string(),
        QuoteState::Expired => "EXPIRED".to_string(),
    }
}

/// Helper to convert database row to Quote
fn row_to_quote(row: &sqlx::postgres::PgRow) -> Quote {
    use tmf_apis_core::BaseEntity;
    use tmf_apis_core::LifecycleStatus;

    Quote {
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
        state: parse_quote_state(row.get("state")),
        quote_item: None,    // Load separately if needed
        related_party: None, // Load separately if needed
        quote_date: row.get("quote_date"),
        valid_until: row.get("valid_until"),
        total_price: row
            .try_get::<Option<serde_json::Value>, _>("total_price")
            .ok()
            .flatten()
            .and_then(|v| serde_json::from_value(v).ok()),
        expected_order_date: row.get("expected_order_date"),
    }
}

/// Get all quotes
pub async fn get_quotes(pool: &Pool<Postgres>) -> TmfResult<Vec<Quote>> {
    let rows = sqlx::query(
        "SELECT id, href, name, description, version, state, quote_date, 
         valid_until, total_price, expected_order_date, last_update
         FROM quotes ORDER BY quote_date DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(rows.iter().map(row_to_quote).collect())
}

/// Get quote by ID
pub async fn get_quote_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<Option<Quote>> {
    let row = sqlx::query(
        "SELECT id, href, name, description, version, state, quote_date, 
         valid_until, total_price, expected_order_date, last_update
         FROM quotes WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(row.map(|r| row_to_quote(&r)))
}

/// Create a new quote
pub async fn create_quote(pool: &Pool<Postgres>, request: CreateQuoteRequest) -> TmfResult<Quote> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let href = format!("/tmf-api/quoteManagement/v4/quote/{}", id);
    let state = quote_state_to_string(&QuoteState::InProgress);

    let total_price_json = None::<serde_json::Value>; // Calculate from items if needed

    sqlx::query(
        "INSERT INTO quotes (
            id, href, name, description, version, state, quote_date,
            valid_until, total_price, expected_order_date, created_at, last_update
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
    )
    .bind(id)
    .bind(&href)
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.version.as_deref().unwrap_or("1.0.0"))
    .bind(&state)
    .bind(now)
    .bind(request.valid_until)
    .bind(total_price_json)
    .bind(request.expected_order_date)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create quote items if provided
    if let Some(items) = request.quote_item {
        for item in items {
            let item_id = Uuid::new_v4();
            let unit_price_json = item
                .unit_price
                .as_ref()
                .and_then(|m| serde_json::to_value(m).ok());
            let item_total_price = item.unit_price.as_ref().and_then(|up| {
                item.quantity.map(|q| crate::models::Money {
                    value: up.value * q as f64,
                    unit: up.unit.clone(),
                })
            });
            let item_total_price_json = item_total_price
                .as_ref()
                .and_then(|m| serde_json::to_value(m).ok());

            sqlx::query(
                "INSERT INTO quote_items (
                    id, quote_id, product_offering_id, product_specification_id,
                    quantity, unit_price, total_price
                ) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            )
            .bind(item_id)
            .bind(id)
            .bind(item.product_offering_id)
            .bind(item.product_specification_id)
            .bind(item.quantity)
            .bind(unit_price_json)
            .bind(item_total_price_json)
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
                "INSERT INTO quote_related_parties (id, quote_id, name, role)
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

    get_quote_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Quote not found after creation".to_string()))
}

/// Update a quote
pub async fn update_quote(
    pool: &Pool<Postgres>,
    id: Uuid,
    request: UpdateQuoteRequest,
) -> TmfResult<Quote> {
    let state_str = request.state.as_ref().map(quote_state_to_string);

    sqlx::query(
        "UPDATE quotes SET 
         state = COALESCE($1, state), 
         description = COALESCE($2, description),
         valid_until = COALESCE($3, valid_until),
         last_update = CURRENT_TIMESTAMP
         WHERE id = $4",
    )
    .bind(state_str)
    .bind(&request.description)
    .bind(request.valid_until)
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_quote_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Quote not found".to_string()))
}

/// Delete a quote
pub async fn delete_quote(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<()> {
    let result = sqlx::query("DELETE FROM quotes WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

    if result.rows_affected() == 0 {
        return Err(TmfError::NotFound("Quote not found".to_string()));
    }

    Ok(())
}

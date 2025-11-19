//! Database operations for TMF635 Usage Management

use crate::models::{CreateUsageRequest, Usage, UsageState};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse usage state from database string
fn parse_usage_state(s: &str) -> UsageState {
    match s.to_uppercase().as_str() {
        "CAPTURED" => UsageState::Captured,
        "RATED" => UsageState::Rated,
        "BILLED" => UsageState::Billed,
        "REJECTED" => UsageState::Rejected,
        _ => UsageState::Captured,
    }
}

/// Convert usage state to database string
fn usage_state_to_string(state: &UsageState) -> String {
    match state {
        UsageState::Captured => "CAPTURED".to_string(),
        UsageState::Rated => "RATED".to_string(),
        UsageState::Billed => "BILLED".to_string(),
        UsageState::Rejected => "REJECTED".to_string(),
    }
}

/// Get all usage records
pub async fn get_usages(pool: &Pool<Postgres>) -> TmfResult<Vec<Usage>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, usage_type, usage_date, start_date, end_date, 
         amount, unit, href, last_update
         FROM usages ORDER BY usage_date DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut usages = Vec::new();
    for row in rows {
        usages.push(Usage {
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
            state: parse_usage_state(&row.get::<String, _>("state")),
            usage_type: row.get::<Option<String>, _>("usage_type"),
            usage_date: row.get::<Option<DateTime<Utc>>, _>("usage_date"),
            start_date: row.get::<Option<DateTime<Utc>>, _>("start_date"),
            end_date: row.get::<Option<DateTime<Utc>>, _>("end_date"),
            amount: row.get::<Option<f64>, _>("amount"),
            unit: row.get::<Option<String>, _>("unit"),
            product_offering: None, // Load separately if needed
            related_party: None,    // Load separately if needed
            rating: None,           // Load separately if needed
        });
    }

    Ok(usages)
}

/// Get usage by ID
pub async fn get_usage_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<Usage> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, usage_type, usage_date, start_date, end_date, 
         amount, unit, href, last_update
         FROM usages WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Usage with id {} not found", id)))?;

    Ok(Usage {
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
        state: parse_usage_state(&row.get::<String, _>("state")),
        usage_type: row.get::<Option<String>, _>("usage_type"),
        usage_date: row.get::<Option<DateTime<Utc>>, _>("usage_date"),
        start_date: row.get::<Option<DateTime<Utc>>, _>("start_date"),
        end_date: row.get::<Option<DateTime<Utc>>, _>("end_date"),
        amount: row.get::<Option<f64>, _>("amount"),
        unit: row.get::<Option<String>, _>("unit"),
        product_offering: None,
        related_party: None,
        rating: None,
    })
}

/// Create a new usage record
pub async fn create_usage(pool: &Pool<Postgres>, request: CreateUsageRequest) -> TmfResult<Usage> {
    let id = Uuid::new_v4();
    let state = usage_state_to_string(&UsageState::Captured);

    sqlx::query(
        "INSERT INTO usages (id, name, description, version, state, usage_type, usage_date, 
         start_date, end_date, amount, unit, product_offering_id, rating_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(&request.usage_type)
    .bind(request.usage_date)
    .bind(request.start_date)
    .bind(request.end_date)
    .bind(request.amount)
    .bind(&request.unit)
    .bind(request.product_offering_id)
    .bind(request.rating_id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create related parties if provided
    if let Some(parties) = request.related_party {
        for party in parties {
            let party_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO usage_related_parties (id, usage_id, name, role)
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

    // Fetch the created usage
    get_usage_by_id(pool, id).await
}

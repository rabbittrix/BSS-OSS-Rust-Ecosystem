//! Database operations for TMF656 Slice Management

use crate::models::{
    CreateNetworkSliceRequest, NetworkSlice, SliceState, SliceType,
};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse slice state from database string
fn parse_slice_state(s: &str) -> SliceState {
    match s.to_uppercase().as_str() {
        "PLANNED" => SliceState::Planned,
        "ACTIVE" => SliceState::Active,
        "INACTIVE" => SliceState::Inactive,
        "TERMINATED" => SliceState::Terminated,
        _ => SliceState::Planned,
    }
}

/// Convert slice state to database string
fn slice_state_to_string(state: &SliceState) -> String {
    match state {
        SliceState::Planned => "PLANNED".to_string(),
        SliceState::Active => "ACTIVE".to_string(),
        SliceState::Inactive => "INACTIVE".to_string(),
        SliceState::Terminated => "TERMINATED".to_string(),
    }
}

/// Parse slice type from database string
fn parse_slice_type(s: &str) -> SliceType {
    match s.to_uppercase().as_str() {
        "ENHANCED_MOBILE_BROADBAND" => SliceType::EnhancedMobileBroadband,
        "ULTRA_RELIABLE_LOW_LATENCY" => SliceType::UltraReliableLowLatency,
        "MASSIVE_MACHINE_TYPE_COMMUNICATIONS" => SliceType::MassiveMachineTypeCommunications,
        "CUSTOM" => SliceType::Custom,
        _ => SliceType::Custom,
    }
}

/// Convert slice type to database string
fn slice_type_to_string(slice_type: &SliceType) -> String {
    match slice_type {
        SliceType::EnhancedMobileBroadband => "ENHANCED_MOBILE_BROADBAND".to_string(),
        SliceType::UltraReliableLowLatency => "ULTRA_RELIABLE_LOW_LATENCY".to_string(),
        SliceType::MassiveMachineTypeCommunications => {
            "MASSIVE_MACHINE_TYPE_COMMUNICATIONS".to_string()
        }
        SliceType::Custom => "CUSTOM".to_string(),
    }
}

/// Get all network slices
pub async fn get_network_slices(pool: &Pool<Postgres>) -> TmfResult<Vec<NetworkSlice>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, slice_type, 
         activation_date, termination_date, href, last_update
         FROM network_slices ORDER BY activation_date DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut slices = Vec::new();
    for row in rows {
        slices.push(NetworkSlice {
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
            state: parse_slice_state(&row.get::<String, _>("state")),
            slice_type: parse_slice_type(&row.get::<String, _>("slice_type")),
            sla_parameters: None,        // Load separately if needed
            network_functions: None,     // Load separately if needed
            activation_date: row.get::<Option<DateTime<Utc>>, _>("activation_date"),
            termination_date: row.get::<Option<DateTime<Utc>>, _>("termination_date"),
        });
    }

    Ok(slices)
}

/// Get network slice by ID
pub async fn get_network_slice_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<NetworkSlice> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, slice_type, 
         activation_date, termination_date, href, last_update
         FROM network_slices WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Network slice with id {} not found", id)))?;

    Ok(NetworkSlice {
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
        state: parse_slice_state(&row.get::<String, _>("state")),
        slice_type: parse_slice_type(&row.get::<String, _>("slice_type")),
        sla_parameters: None,        // Load separately if needed
        network_functions: None,     // Load separately if needed
        activation_date: row.get::<Option<DateTime<Utc>>, _>("activation_date"),
        termination_date: row.get::<Option<DateTime<Utc>>, _>("termination_date"),
    })
}

/// Create a new network slice
pub async fn create_network_slice(
    pool: &Pool<Postgres>,
    request: CreateNetworkSliceRequest,
) -> TmfResult<NetworkSlice> {
    let id = Uuid::new_v4();
    let href = Some(format!("/tmf-api/sliceManagement/v4/networkSlice/{}", id));

    sqlx::query(
        "INSERT INTO network_slices (id, name, description, version, state, slice_type, 
         activation_date, href)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(slice_state_to_string(&SliceState::Planned))
    .bind(slice_type_to_string(&request.slice_type))
    .bind(request.activation_date)
    .bind(&href)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Fetch the created network slice
    get_network_slice_by_id(pool, id).await
}

/// Update a network slice
pub async fn update_network_slice(
    pool: &Pool<Postgres>,
    id: Uuid,
    state: Option<SliceState>,
    activation_date: Option<DateTime<Utc>>,
    termination_date: Option<DateTime<Utc>>,
) -> TmfResult<NetworkSlice> {
    sqlx::query(
        "UPDATE network_slices SET 
         state = COALESCE($1, state), 
         activation_date = COALESCE($2, activation_date),
         termination_date = COALESCE($3, termination_date),
         last_update = CURRENT_TIMESTAMP
         WHERE id = $4",
    )
    .bind(state.as_ref().map(slice_state_to_string))
    .bind(activation_date)
    .bind(termination_date)
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Fetch the updated network slice
    get_network_slice_by_id(pool, id).await
}

/// Delete a network slice
pub async fn delete_network_slice(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<()> {
    let result = sqlx::query("DELETE FROM network_slices WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

    if result.rows_affected() == 0 {
        return Err(TmfError::NotFound(format!(
            "Network slice with id {} not found",
            id
        )));
    }

    Ok(())
}


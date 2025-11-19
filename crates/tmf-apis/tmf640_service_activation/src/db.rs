//! Database operations for TMF640 Service Activation & Configuration

use crate::models::{
    CreateServiceActivationRequest, ServiceActivation, ServiceActivationState,
};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse service activation state from database string
fn parse_service_activation_state(s: &str) -> ServiceActivationState {
    match s.to_uppercase().as_str() {
        "PENDING" => ServiceActivationState::Pending,
        "IN_PROGRESS" => ServiceActivationState::InProgress,
        "COMPLETED" => ServiceActivationState::Completed,
        "FAILED" => ServiceActivationState::Failed,
        "CANCELLED" => ServiceActivationState::Cancelled,
        _ => ServiceActivationState::Pending,
    }
}

/// Convert service activation state to database string
fn service_activation_state_to_string(state: &ServiceActivationState) -> String {
    match state {
        ServiceActivationState::Pending => "PENDING".to_string(),
        ServiceActivationState::InProgress => "IN_PROGRESS".to_string(),
        ServiceActivationState::Completed => "COMPLETED".to_string(),
        ServiceActivationState::Failed => "FAILED".to_string(),
        ServiceActivationState::Cancelled => "CANCELLED".to_string(),
    }
}

/// Get all service activations
pub async fn get_service_activations(
    pool: &Pool<Postgres>,
) -> TmfResult<Vec<ServiceActivation>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, activation_date, 
         completion_date, service_id, service_order_id, href, last_update
         FROM service_activations ORDER BY activation_date DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut activations = Vec::new();
    for row in rows {
        activations.push(ServiceActivation {
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
            state: parse_service_activation_state(&row.get::<String, _>("state")),
            service: None,      // Load separately if needed
            service_order: None, // Load separately if needed
            activation_date: row.get::<Option<DateTime<Utc>>, _>("activation_date"),
            completion_date: row.get::<Option<DateTime<Utc>>, _>("completion_date"),
            configuration: None, // Load separately if needed
        });
    }

    Ok(activations)
}

/// Get service activation by ID
pub async fn get_service_activation_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<ServiceActivation> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, activation_date, 
         completion_date, service_id, service_order_id, href, last_update
         FROM service_activations WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Service activation with id {} not found", id)))?;

    Ok(ServiceActivation {
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
        state: parse_service_activation_state(&row.get::<String, _>("state")),
        service: None,
        service_order: None,
        activation_date: row.get::<Option<DateTime<Utc>>, _>("activation_date"),
        completion_date: row.get::<Option<DateTime<Utc>>, _>("completion_date"),
        configuration: None,
    })
}

/// Create a new service activation
pub async fn create_service_activation(
    pool: &Pool<Postgres>,
    request: CreateServiceActivationRequest,
) -> TmfResult<ServiceActivation> {
    let id = Uuid::new_v4();
    let state = service_activation_state_to_string(&ServiceActivationState::Pending);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO service_activations (id, name, description, version, state, 
         activation_date, service_id, service_order_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(now)
    .bind(request.service_id)
    .bind(request.service_order_id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create configuration parameters if provided
    if let Some(config) = request.configuration {
        for param in config {
            let param_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO service_activation_configurations (id, activation_id, name, value, description)
                 VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(param_id)
            .bind(id)
            .bind(&param.name)
            .bind(&param.value)
            .bind(&param.description)
            .execute(pool)
            .await
            .map_err(map_sqlx_error)?;
        }
    }

    // Fetch the created service activation
    get_service_activation_by_id(pool, id).await
}


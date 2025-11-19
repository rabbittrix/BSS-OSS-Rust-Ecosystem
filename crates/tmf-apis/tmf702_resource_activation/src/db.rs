//! Database operations for TMF702 Resource Activation & Configuration

use crate::models::{CreateResourceActivationRequest, ResourceActivation, ResourceActivationState};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse resource activation state from database string
fn parse_resource_activation_state(s: &str) -> ResourceActivationState {
    match s.to_uppercase().as_str() {
        "PENDING" => ResourceActivationState::Pending,
        "IN_PROGRESS" => ResourceActivationState::InProgress,
        "COMPLETED" => ResourceActivationState::Completed,
        "FAILED" => ResourceActivationState::Failed,
        "CANCELLED" => ResourceActivationState::Cancelled,
        _ => ResourceActivationState::Pending,
    }
}

/// Convert resource activation state to database string
fn resource_activation_state_to_string(state: &ResourceActivationState) -> String {
    match state {
        ResourceActivationState::Pending => "PENDING".to_string(),
        ResourceActivationState::InProgress => "IN_PROGRESS".to_string(),
        ResourceActivationState::Completed => "COMPLETED".to_string(),
        ResourceActivationState::Failed => "FAILED".to_string(),
        ResourceActivationState::Cancelled => "CANCELLED".to_string(),
    }
}

/// Get all resource activations
pub async fn get_resource_activations(pool: &Pool<Postgres>) -> TmfResult<Vec<ResourceActivation>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, activation_date, 
         completion_date, resource_id, service_activation_id, href, last_update
         FROM resource_activations ORDER BY activation_date DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut activations = Vec::new();
    for row in rows {
        activations.push(ResourceActivation {
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
            state: parse_resource_activation_state(&row.get::<String, _>("state")),
            resource: None,           // Load separately if needed
            service_activation: None, // Load separately if needed
            activation_date: row.get::<Option<DateTime<Utc>>, _>("activation_date"),
            completion_date: row.get::<Option<DateTime<Utc>>, _>("completion_date"),
            configuration: None, // Load separately if needed
        });
    }

    Ok(activations)
}

/// Get resource activation by ID
pub async fn get_resource_activation_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<ResourceActivation> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, activation_date, 
         completion_date, resource_id, service_activation_id, href, last_update
         FROM resource_activations WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Resource activation with id {} not found", id)))?;

    Ok(ResourceActivation {
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
        state: parse_resource_activation_state(&row.get::<String, _>("state")),
        resource: None,
        service_activation: None,
        activation_date: row.get::<Option<DateTime<Utc>>, _>("activation_date"),
        completion_date: row.get::<Option<DateTime<Utc>>, _>("completion_date"),
        configuration: None,
    })
}

/// Create a new resource activation
pub async fn create_resource_activation(
    pool: &Pool<Postgres>,
    request: CreateResourceActivationRequest,
) -> TmfResult<ResourceActivation> {
    let id = Uuid::new_v4();
    let state = resource_activation_state_to_string(&ResourceActivationState::Pending);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO resource_activations (id, name, description, version, state, 
         activation_date, resource_id, service_activation_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(now)
    .bind(request.resource_id)
    .bind(request.service_activation_id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create configuration parameters if provided
    if let Some(config) = request.configuration {
        for param in config {
            let param_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO resource_activation_configurations (id, activation_id, name, value, description)
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

    // Fetch the created resource activation
    get_resource_activation_by_id(pool, id).await
}

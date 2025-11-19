//! Database operations for TMF638 Service Inventory

use crate::models::{CreateServiceInventoryRequest, ServiceInventory, ServiceInventoryState};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse service inventory state from database string
fn parse_service_inventory_state(s: &str) -> ServiceInventoryState {
    match s.to_uppercase().as_str() {
        "RESERVED" => ServiceInventoryState::Reserved,
        "ACTIVE" => ServiceInventoryState::Active,
        "INACTIVE" => ServiceInventoryState::Inactive,
        "TERMINATED" => ServiceInventoryState::Terminated,
        "SUSPENDED" => ServiceInventoryState::Suspended,
        _ => ServiceInventoryState::Active,
    }
}

/// Convert service inventory state to database string
fn service_inventory_state_to_string(state: &ServiceInventoryState) -> String {
    match state {
        ServiceInventoryState::Reserved => "RESERVED".to_string(),
        ServiceInventoryState::Active => "ACTIVE".to_string(),
        ServiceInventoryState::Inactive => "INACTIVE".to_string(),
        ServiceInventoryState::Terminated => "TERMINATED".to_string(),
        ServiceInventoryState::Suspended => "SUSPENDED".to_string(),
    }
}

/// Get all service inventories
pub async fn get_service_inventories(pool: &Pool<Postgres>) -> TmfResult<Vec<ServiceInventory>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, activation_date, 
         last_modified_date, href, last_update
         FROM service_inventories ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut inventories = Vec::new();
    for row in rows {
        inventories.push(ServiceInventory {
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
            state: parse_service_inventory_state(&row.get::<String, _>("state")),
            service_specification: None, // Load separately if needed
            service: None,               // Load separately if needed
            related_party: None,         // Load separately if needed
            activation_date: row.get::<Option<DateTime<Utc>>, _>("activation_date"),
            last_modified_date: row.get::<Option<DateTime<Utc>>, _>("last_modified_date"),
        });
    }

    Ok(inventories)
}

/// Get service inventory by ID
pub async fn get_service_inventory_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<ServiceInventory> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, activation_date, 
         last_modified_date, href, last_update
         FROM service_inventories WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Service inventory with id {} not found", id)))?;

    Ok(ServiceInventory {
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
        state: parse_service_inventory_state(&row.get::<String, _>("state")),
        service_specification: None,
        service: None,
        related_party: None,
        activation_date: row.get::<Option<DateTime<Utc>>, _>("activation_date"),
        last_modified_date: row.get::<Option<DateTime<Utc>>, _>("last_modified_date"),
    })
}

/// Create a new service inventory
pub async fn create_service_inventory(
    pool: &Pool<Postgres>,
    request: CreateServiceInventoryRequest,
) -> TmfResult<ServiceInventory> {
    let id = Uuid::new_v4();
    let state = service_inventory_state_to_string(&ServiceInventoryState::Active);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO service_inventories (id, name, description, version, state, 
         service_specification_id, service_id, activation_date, last_modified_date)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(request.service_specification_id)
    .bind(request.service_id)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create related parties if provided
    if let Some(parties) = request.related_party {
        for party in parties {
            let party_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO service_inventory_related_parties (id, inventory_id, name, role)
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

    // Fetch the created service inventory
    get_service_inventory_by_id(pool, id).await
}

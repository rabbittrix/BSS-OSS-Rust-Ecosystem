//! Database operations for TMF639 Resource Inventory

use crate::models::{CreateResourceInventoryRequest, ResourceInventory, ResourceInventoryState};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse resource inventory state from database string
fn parse_resource_inventory_state(s: &str) -> ResourceInventoryState {
    match s.to_uppercase().as_str() {
        "RESERVED" => ResourceInventoryState::Reserved,
        "AVAILABLE" => ResourceInventoryState::Available,
        "IN_USE" => ResourceInventoryState::InUse,
        "MAINTENANCE" => ResourceInventoryState::Maintenance,
        "RETIRED" => ResourceInventoryState::Retired,
        _ => ResourceInventoryState::Available,
    }
}

/// Convert resource inventory state to database string
fn resource_inventory_state_to_string(state: &ResourceInventoryState) -> String {
    match state {
        ResourceInventoryState::Reserved => "RESERVED".to_string(),
        ResourceInventoryState::Available => "AVAILABLE".to_string(),
        ResourceInventoryState::InUse => "IN_USE".to_string(),
        ResourceInventoryState::Maintenance => "MAINTENANCE".to_string(),
        ResourceInventoryState::Retired => "RETIRED".to_string(),
    }
}

/// Get all resource inventories
pub async fn get_resource_inventories(pool: &Pool<Postgres>) -> TmfResult<Vec<ResourceInventory>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, resource_type, activation_date, 
         last_modified_date, href, last_update
         FROM resource_inventories ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut inventories = Vec::new();
    for row in rows {
        inventories.push(ResourceInventory {
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
            state: parse_resource_inventory_state(&row.get::<String, _>("state")),
            resource_specification: None, // Load separately if needed
            resource: None,               // Load separately if needed
            resource_type: row.get::<Option<String>, _>("resource_type"),
            related_party: None, // Load separately if needed
            activation_date: row.get::<Option<DateTime<Utc>>, _>("activation_date"),
            last_modified_date: row.get::<Option<DateTime<Utc>>, _>("last_modified_date"),
        });
    }

    Ok(inventories)
}

/// Get resource inventory by ID
pub async fn get_resource_inventory_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<ResourceInventory> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, resource_type, activation_date, 
         last_modified_date, href, last_update
         FROM resource_inventories WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Resource inventory with id {} not found", id)))?;

    Ok(ResourceInventory {
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
        state: parse_resource_inventory_state(&row.get::<String, _>("state")),
        resource_specification: None,
        resource: None,
        resource_type: row.get::<Option<String>, _>("resource_type"),
        related_party: None,
        activation_date: row.get::<Option<DateTime<Utc>>, _>("activation_date"),
        last_modified_date: row.get::<Option<DateTime<Utc>>, _>("last_modified_date"),
    })
}

/// Create a new resource inventory
pub async fn create_resource_inventory(
    pool: &Pool<Postgres>,
    request: CreateResourceInventoryRequest,
) -> TmfResult<ResourceInventory> {
    let id = Uuid::new_v4();
    let state = resource_inventory_state_to_string(&ResourceInventoryState::Available);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO resource_inventories (id, name, description, version, state, 
         resource_type, resource_specification_id, resource_id, activation_date, last_modified_date)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(&request.resource_type)
    .bind(request.resource_specification_id)
    .bind(request.resource_id)
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
                "INSERT INTO resource_inventory_related_parties (id, inventory_id, name, role)
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

    // Fetch the created resource inventory
    get_resource_inventory_by_id(pool, id).await
}

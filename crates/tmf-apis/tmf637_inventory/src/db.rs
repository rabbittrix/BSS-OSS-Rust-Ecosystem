//! Database operations for TMF637 Product Inventory

use crate::models::{CreateProductInventoryRequest, InventoryState, ProductInventory};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse inventory state from database string
fn parse_inventory_state(s: &str) -> InventoryState {
    match s.to_uppercase().as_str() {
        "RESERVED" => InventoryState::Reserved,
        "AVAILABLE" => InventoryState::Available,
        "IN_USE" => InventoryState::InUse,
        "RETIRED" => InventoryState::Retired,
        "RESERVED_FOR_CUSTOMER" => InventoryState::ReservedForCustomer,
        _ => InventoryState::Available,
    }
}

/// Convert inventory state to database string
fn inventory_state_to_string(state: &InventoryState) -> String {
    match state {
        InventoryState::Reserved => "RESERVED".to_string(),
        InventoryState::Available => "AVAILABLE".to_string(),
        InventoryState::InUse => "IN_USE".to_string(),
        InventoryState::Retired => "RETIRED".to_string(),
        InventoryState::ReservedForCustomer => "RESERVED_FOR_CUSTOMER".to_string(),
    }
}

/// Get all product inventories
pub async fn get_inventories(pool: &Pool<Postgres>) -> TmfResult<Vec<ProductInventory>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, quantity, reserved_quantity,
         activation_date, last_modified_date, href, last_update
         FROM product_inventories ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut inventories = Vec::new();
    for row in rows {
        inventories.push(ProductInventory {
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
            state: parse_inventory_state(&row.get::<String, _>("state")),
            product_specification: None, // Load separately if needed
            product_offering: None,      // Load separately if needed
            quantity: row.get::<Option<i32>, _>("quantity"),
            reserved_quantity: row.get::<Option<i32>, _>("reserved_quantity"),
            related_party: None, // Load separately if needed
            activation_date: row.get::<Option<DateTime<Utc>>, _>("activation_date"),
            last_modified_date: row.get::<Option<DateTime<Utc>>, _>("last_modified_date"),
        });
    }

    Ok(inventories)
}

/// Get product inventory by ID
pub async fn get_inventory_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<ProductInventory> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, quantity, reserved_quantity,
         activation_date, last_modified_date, href, last_update
         FROM product_inventories WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Product inventory with id {} not found", id)))?;

    Ok(ProductInventory {
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
        state: parse_inventory_state(&row.get::<String, _>("state")),
        product_specification: None,
        product_offering: None,
        quantity: row.get::<Option<i32>, _>("quantity"),
        reserved_quantity: row.get::<Option<i32>, _>("reserved_quantity"),
        related_party: None,
        activation_date: row.get::<Option<DateTime<Utc>>, _>("activation_date"),
        last_modified_date: row.get::<Option<DateTime<Utc>>, _>("last_modified_date"),
    })
}

/// Create a new product inventory
pub async fn create_inventory(
    pool: &Pool<Postgres>,
    request: CreateProductInventoryRequest,
) -> TmfResult<ProductInventory> {
    let id = Uuid::new_v4();
    let state = inventory_state_to_string(&InventoryState::Available);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO product_inventories (id, name, description, version, state, quantity, 
         reserved_quantity, activation_date, last_modified_date)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(&id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(&request.quantity)
    .bind(&0i32) // reserved_quantity defaults to 0
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create related parties if provided
    if let Some(parties) = request.related_party {
        for party in parties {
            let party_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO inventory_related_parties (id, inventory_id, name, role)
                 VALUES ($1, $2, $3, $4)",
            )
            .bind(&party_id)
            .bind(&id)
            .bind(&party.name)
            .bind(&party.role)
            .execute(pool)
            .await
            .map_err(map_sqlx_error)?;
        }
    }

    // Fetch the created inventory
    get_inventory_by_id(pool, id).await
}

//! Resource Capacity Management

use crate::error::{ResourceManagementError, ResourceManagementResult};
use crate::models::{
    CreateResourceCapacityRequest, ResourceCapacity, UpdateResourceCapacityRequest,
};
use chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

/// Get all capacities for a resource
pub async fn get_resource_capacities(
    pool: &Pool<Postgres>,
    resource_inventory_id: Uuid,
) -> ResourceManagementResult<Vec<ResourceCapacity>> {
    let rows = sqlx::query(
        "SELECT id, resource_inventory_id, capacity_type, total_capacity, 
         used_capacity, reserved_capacity, unit, created_at, updated_at
         FROM resource_capacities
         WHERE resource_inventory_id = $1
         ORDER BY capacity_type",
    )
    .bind(resource_inventory_id)
    .fetch_all(pool)
    .await?;

    let mut capacities = Vec::new();
    for row in rows {
        let total: f64 = row.get("total_capacity");
        let used: f64 = row.get("used_capacity");
        let reserved: f64 = row.get("reserved_capacity");
        let available = total - used - reserved;

        capacities.push(ResourceCapacity {
            id: row.get("id"),
            resource_inventory_id: row.get("resource_inventory_id"),
            capacity_type: row.get("capacity_type"),
            total_capacity: total,
            used_capacity: used,
            reserved_capacity: reserved,
            available_capacity: available,
            unit: row.get("unit"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    Ok(capacities)
}

/// Get capacity by ID
pub async fn get_capacity_by_id(
    pool: &Pool<Postgres>,
    capacity_id: Uuid,
) -> ResourceManagementResult<ResourceCapacity> {
    let row = sqlx::query(
        "SELECT id, resource_inventory_id, capacity_type, total_capacity, 
         used_capacity, reserved_capacity, unit, created_at, updated_at
         FROM resource_capacities
         WHERE id = $1",
    )
    .bind(capacity_id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => {
            let total: f64 = row.get("total_capacity");
            let used: f64 = row.get("used_capacity");
            let reserved: f64 = row.get("reserved_capacity");
            let available = total - used - reserved;

            Ok(ResourceCapacity {
                id: row.get("id"),
                resource_inventory_id: row.get("resource_inventory_id"),
                capacity_type: row.get("capacity_type"),
                total_capacity: total,
                used_capacity: used,
                reserved_capacity: reserved,
                available_capacity: available,
                unit: row.get("unit"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
        }
        None => Err(ResourceManagementError::ResourceNotFound(format!(
            "Capacity with id {} not found",
            capacity_id
        ))),
    }
}

/// Create resource capacity
pub async fn create_resource_capacity(
    pool: &Pool<Postgres>,
    request: CreateResourceCapacityRequest,
) -> ResourceManagementResult<ResourceCapacity> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO resource_capacities 
         (id, resource_inventory_id, capacity_type, total_capacity, used_capacity, 
          reserved_capacity, unit, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(id)
    .bind(request.resource_inventory_id)
    .bind(&request.capacity_type)
    .bind(request.total_capacity)
    .bind(0.0) // used_capacity
    .bind(0.0) // reserved_capacity
    .bind(&request.unit)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    get_capacity_by_id(pool, id).await
}

/// Update resource capacity
pub async fn update_resource_capacity(
    pool: &Pool<Postgres>,
    capacity_id: Uuid,
    request: UpdateResourceCapacityRequest,
) -> ResourceManagementResult<ResourceCapacity> {
    // Get current capacity
    let current = get_capacity_by_id(pool, capacity_id).await?;

    let total = request.total_capacity.unwrap_or(current.total_capacity);
    let used = request.used_capacity.unwrap_or(current.used_capacity);
    let reserved = request
        .reserved_capacity
        .unwrap_or(current.reserved_capacity);

    // Validate capacity
    if used + reserved > total {
        return Err(ResourceManagementError::InsufficientCapacity(format!(
            "Used ({}) + Reserved ({}) exceeds total capacity ({})",
            used, reserved, total
        )));
    }

    let now = Utc::now();

    sqlx::query(
        "UPDATE resource_capacities 
         SET total_capacity = $1, used_capacity = $2, reserved_capacity = $3, updated_at = $4
         WHERE id = $5",
    )
    .bind(total)
    .bind(used)
    .bind(reserved)
    .bind(now)
    .bind(capacity_id)
    .execute(pool)
    .await?;

    get_capacity_by_id(pool, capacity_id).await
}

/// Check if resource has sufficient capacity
pub async fn check_capacity_availability(
    pool: &Pool<Postgres>,
    resource_inventory_id: Uuid,
    capacity_type: &str,
    required_amount: f64,
) -> ResourceManagementResult<bool> {
    let capacities = get_resource_capacities(pool, resource_inventory_id).await?;

    for capacity in capacities {
        if capacity.capacity_type == capacity_type {
            return Ok(capacity.available_capacity >= required_amount);
        }
    }

    Ok(false)
}

/// Reserve capacity
pub async fn reserve_capacity(
    pool: &Pool<Postgres>,
    resource_inventory_id: Uuid,
    capacity_type: &str,
    amount: f64,
) -> ResourceManagementResult<()> {
    let capacities = get_resource_capacities(pool, resource_inventory_id).await?;

    for capacity in capacities {
        if capacity.capacity_type == capacity_type {
            let new_reserved = capacity.reserved_capacity + amount;
            if new_reserved > capacity.total_capacity - capacity.used_capacity {
                return Err(ResourceManagementError::InsufficientCapacity(format!(
                    "Cannot reserve {} {}: only {} available",
                    amount, capacity.unit, capacity.available_capacity
                )));
            }

            let update_request = UpdateResourceCapacityRequest {
                total_capacity: None,
                used_capacity: None,
                reserved_capacity: Some(new_reserved),
            };

            update_resource_capacity(pool, capacity.id, update_request).await?;
            return Ok(());
        }
    }

    Err(ResourceManagementError::ResourceNotFound(format!(
        "Capacity type {} not found for resource {}",
        capacity_type, resource_inventory_id
    )))
}

/// Release reserved capacity
pub async fn release_reserved_capacity(
    pool: &Pool<Postgres>,
    resource_inventory_id: Uuid,
    capacity_type: &str,
    amount: f64,
) -> ResourceManagementResult<()> {
    let capacities = get_resource_capacities(pool, resource_inventory_id).await?;

    for capacity in capacities {
        if capacity.capacity_type == capacity_type {
            let new_reserved = (capacity.reserved_capacity - amount).max(0.0);

            let update_request = UpdateResourceCapacityRequest {
                total_capacity: None,
                used_capacity: None,
                reserved_capacity: Some(new_reserved),
            };

            update_resource_capacity(pool, capacity.id, update_request).await?;
            return Ok(());
        }
    }

    Err(ResourceManagementError::ResourceNotFound(format!(
        "Capacity type {} not found for resource {}",
        capacity_type, resource_inventory_id
    )))
}

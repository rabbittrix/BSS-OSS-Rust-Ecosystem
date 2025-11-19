//! Resource Reservation System

use crate::capacity::check_capacity_availability;
use crate::error::{ResourceManagementError, ResourceManagementResult};
use crate::models::{
    CreateResourceReservationRequest, ResourceReservation, UpdateResourceReservationRequest,
};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

/// Get all reservations for a resource
pub async fn get_resource_reservations(
    pool: &Pool<Postgres>,
    resource_inventory_id: Uuid,
) -> ResourceManagementResult<Vec<ResourceReservation>> {
    let rows = sqlx::query(
        "SELECT id, resource_inventory_id, reservation_name, description, reservation_status,
         start_time, end_time, resource_order_id, service_order_id, reserved_by_party_id,
         capacity_requirements, created_at, updated_at, confirmed_at, cancelled_at, cancellation_reason
         FROM resource_reservations
         WHERE resource_inventory_id = $1
         ORDER BY start_time DESC",
    )
    .bind(resource_inventory_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(row_to_reservation).collect())
}

/// Get reservation by ID
pub async fn get_reservation_by_id(
    pool: &Pool<Postgres>,
    reservation_id: Uuid,
) -> ResourceManagementResult<ResourceReservation> {
    let row = sqlx::query(
        "SELECT id, resource_inventory_id, reservation_name, description, reservation_status,
         start_time, end_time, resource_order_id, service_order_id, reserved_by_party_id,
         capacity_requirements, created_at, updated_at, confirmed_at, cancelled_at, cancellation_reason
         FROM resource_reservations
         WHERE id = $1",
    )
    .bind(reservation_id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => Ok(row_to_reservation(&row)),
        None => Err(ResourceManagementError::ReservationNotFound(format!(
            "Reservation with id {} not found",
            reservation_id
        ))),
    }
}

/// Create resource reservation
pub async fn create_resource_reservation(
    pool: &Pool<Postgres>,
    request: CreateResourceReservationRequest,
) -> ResourceManagementResult<ResourceReservation> {
    // Validate time range
    if request.end_time <= request.start_time {
        return Err(ResourceManagementError::InvalidTimeRange);
    }

    // Check for overlapping reservations
    let overlapping = check_reservation_conflicts(
        pool,
        request.resource_inventory_id,
        request.start_time,
        request.end_time,
        None,
    )
    .await?;

    if !overlapping.is_empty() {
        return Err(ResourceManagementError::ReservationConflict(format!(
            "Reservation conflicts with {} existing reservation(s)",
            overlapping.len()
        )));
    }

    // Check capacity availability if requirements are specified
    if let Some(capacity_reqs) = request.capacity_requirements.as_object() {
        for (capacity_type, value) in capacity_reqs {
            if let Some(amount) = value.as_f64() {
                let available = check_capacity_availability(
                    pool,
                    request.resource_inventory_id,
                    capacity_type,
                    amount,
                )
                .await?;

                if !available {
                    return Err(ResourceManagementError::InsufficientCapacity(format!(
                        "Insufficient {} capacity for reservation",
                        capacity_type
                    )));
                }
            }
        }
    }

    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO resource_reservations 
         (id, resource_inventory_id, reservation_name, description, reservation_status,
          start_time, end_time, resource_order_id, service_order_id, reserved_by_party_id,
          capacity_requirements, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
    )
    .bind(id)
    .bind(request.resource_inventory_id)
    .bind(&request.reservation_name)
    .bind(&request.description)
    .bind("PENDING")
    .bind(request.start_time)
    .bind(request.end_time)
    .bind(request.resource_order_id)
    .bind(request.service_order_id)
    .bind(request.reserved_by_party_id)
    .bind(&request.capacity_requirements)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    get_reservation_by_id(pool, id).await
}

/// Update resource reservation
pub async fn update_resource_reservation(
    pool: &Pool<Postgres>,
    reservation_id: Uuid,
    request: UpdateResourceReservationRequest,
) -> ResourceManagementResult<ResourceReservation> {
    let current = get_reservation_by_id(pool, reservation_id).await?;

    let start_time = request.start_time.unwrap_or(current.start_time);
    let end_time = request.end_time.unwrap_or(current.end_time);

    // Validate time range if changed
    if end_time <= start_time {
        return Err(ResourceManagementError::InvalidTimeRange);
    }

    // Check for conflicts if time range changed
    if request.start_time.is_some() || request.end_time.is_some() {
        let overlapping = check_reservation_conflicts(
            pool,
            current.resource_inventory_id,
            start_time,
            end_time,
            Some(reservation_id),
        )
        .await?;

        if !overlapping.is_empty() {
            return Err(ResourceManagementError::ReservationConflict(format!(
                "Updated reservation conflicts with {} existing reservation(s)",
                overlapping.len()
            )));
        }
    }

    let current_status = current.reservation_status.clone();
    let status = request.reservation_status.unwrap_or(current_status.clone());
    let now = Utc::now();
    let confirmed_at = if status == "CONFIRMED" && current_status != "CONFIRMED" {
        Some(now)
    } else {
        current.confirmed_at
    };

    let cancelled_at = if status == "CANCELLED" && current_status != "CANCELLED" {
        Some(now)
    } else {
        current.cancelled_at
    };

    sqlx::query(
        "UPDATE resource_reservations 
         SET reservation_status = $1, start_time = $2, end_time = $3, 
             cancellation_reason = $4, confirmed_at = $5, cancelled_at = $6, updated_at = $7
         WHERE id = $8",
    )
    .bind(&status)
    .bind(start_time)
    .bind(end_time)
    .bind(&request.cancellation_reason)
    .bind(confirmed_at)
    .bind(cancelled_at)
    .bind(now)
    .bind(reservation_id)
    .execute(pool)
    .await?;

    get_reservation_by_id(pool, reservation_id).await
}

/// Check for reservation conflicts
async fn check_reservation_conflicts(
    pool: &Pool<Postgres>,
    resource_inventory_id: Uuid,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    exclude_reservation_id: Option<Uuid>,
) -> ResourceManagementResult<Vec<Uuid>> {
    let mut query = sqlx::query(
        "SELECT id FROM resource_reservations
         WHERE resource_inventory_id = $1
         AND reservation_status NOT IN ('COMPLETED', 'CANCELLED')
         AND (
             (start_time <= $2 AND end_time > $2) OR
             (start_time < $3 AND end_time >= $3) OR
             (start_time >= $2 AND end_time <= $3)
         )",
    )
    .bind(resource_inventory_id)
    .bind(start_time)
    .bind(end_time);

    if let Some(exclude_id) = exclude_reservation_id {
        query = query.bind(exclude_id);
        // Note: This would need a proper SQL query modification
        // For now, we'll filter in application code
    }

    let rows = query.fetch_all(pool).await?;

    let mut conflicts = Vec::new();
    for row in rows {
        let id: Uuid = row.get("id");
        if exclude_reservation_id != Some(id) {
            conflicts.push(id);
        }
    }

    Ok(conflicts)
}

/// Get active reservations for a resource
pub async fn get_active_reservations(
    pool: &Pool<Postgres>,
    resource_inventory_id: Uuid,
) -> ResourceManagementResult<Vec<ResourceReservation>> {
    let now = Utc::now();

    let rows = sqlx::query(
        "SELECT id, resource_inventory_id, reservation_name, description, reservation_status,
         start_time, end_time, resource_order_id, service_order_id, reserved_by_party_id,
         capacity_requirements, created_at, updated_at, confirmed_at, cancelled_at, cancellation_reason
         FROM resource_reservations
         WHERE resource_inventory_id = $1
         AND reservation_status IN ('CONFIRMED', 'ACTIVE')
         AND start_time <= $2
         AND end_time > $2
         ORDER BY start_time",
    )
    .bind(resource_inventory_id)
    .bind(now)
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(row_to_reservation).collect())
}

/// Helper to convert database row to ResourceReservation
fn row_to_reservation(row: &sqlx::postgres::PgRow) -> ResourceReservation {
    ResourceReservation {
        id: row.get("id"),
        resource_inventory_id: row.get("resource_inventory_id"),
        reservation_name: row.get("reservation_name"),
        description: row.get("description"),
        reservation_status: row.get("reservation_status"),
        start_time: row.get("start_time"),
        end_time: row.get("end_time"),
        resource_order_id: row.get("resource_order_id"),
        service_order_id: row.get("service_order_id"),
        reserved_by_party_id: row.get("reserved_by_party_id"),
        capacity_requirements: row.get("capacity_requirements"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        confirmed_at: row.get("confirmed_at"),
        cancelled_at: row.get("cancelled_at"),
        cancellation_reason: row.get("cancellation_reason"),
    }
}

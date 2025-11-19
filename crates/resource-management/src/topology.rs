//! Network Topology Management

use crate::error::{ResourceManagementError, ResourceManagementResult};
use crate::models::{
    CreateNetworkTopologyRequest, NetworkTopology, UpdateNetworkTopologyRequest,
};
use chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

/// Get all topology connections for a resource
pub async fn get_resource_topology(
    pool: &Pool<Postgres>,
    resource_inventory_id: Uuid,
) -> ResourceManagementResult<Vec<NetworkTopology>> {
    let rows = sqlx::query(
        "SELECT id, source_resource_id, target_resource_id, connection_type, relationship_type,
         connection_status, bandwidth_mbps, latency_ms, description, created_at, updated_at
         FROM network_topology
         WHERE source_resource_id = $1 OR target_resource_id = $1
         ORDER BY created_at DESC",
    )
    .bind(resource_inventory_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(|row| row_to_topology(row)).collect())
}

/// Get topology connection by ID
pub async fn get_topology_by_id(
    pool: &Pool<Postgres>,
    topology_id: Uuid,
) -> ResourceManagementResult<NetworkTopology> {
    let row = sqlx::query(
        "SELECT id, source_resource_id, target_resource_id, connection_type, relationship_type,
         connection_status, bandwidth_mbps, latency_ms, description, created_at, updated_at
         FROM network_topology
         WHERE id = $1",
    )
    .bind(topology_id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => Ok(row_to_topology(&row)),
        None => Err(ResourceManagementError::TopologyNotFound(format!(
            "Topology connection with id {} not found",
            topology_id
        ))),
    }
}

/// Create network topology connection
pub async fn create_network_topology(
    pool: &Pool<Postgres>,
    request: CreateNetworkTopologyRequest,
) -> ResourceManagementResult<NetworkTopology> {
    // Validate that source and target are different
    if request.source_resource_id == request.target_resource_id {
        return Err(ResourceManagementError::InvalidTopologyRelationship(
            "Source and target resources cannot be the same".to_string(),
        ));
    }

    // Check if connection already exists
    let existing = sqlx::query(
        "SELECT id FROM network_topology
         WHERE source_resource_id = $1
         AND target_resource_id = $2
         AND relationship_type = $3",
    )
    .bind(request.source_resource_id)
    .bind(request.target_resource_id)
    .bind(&request.relationship_type)
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        return Err(ResourceManagementError::InvalidTopologyRelationship(
            "Topology connection already exists".to_string(),
        ));
    }

    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO network_topology 
         (id, source_resource_id, target_resource_id, connection_type, relationship_type,
          connection_status, bandwidth_mbps, latency_ms, description, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
    )
    .bind(id)
    .bind(request.source_resource_id)
    .bind(request.target_resource_id)
    .bind(&request.connection_type)
    .bind(&request.relationship_type)
    .bind("ACTIVE")
    .bind(request.bandwidth_mbps)
    .bind(request.latency_ms)
    .bind(&request.description)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    get_topology_by_id(pool, id).await
}

/// Update network topology connection
pub async fn update_network_topology(
    pool: &Pool<Postgres>,
    topology_id: Uuid,
    request: UpdateNetworkTopologyRequest,
) -> ResourceManagementResult<NetworkTopology> {
    let current = get_topology_by_id(pool, topology_id).await?;

    let status = request
        .connection_status
        .unwrap_or(current.connection_status);
    let bandwidth = request.bandwidth_mbps.or(current.bandwidth_mbps);
    let latency = request.latency_ms.or(current.latency_ms);
    let description = request.description.or(current.description);

    let now = Utc::now();

    sqlx::query(
        "UPDATE network_topology 
         SET connection_status = $1, bandwidth_mbps = $2, latency_ms = $3, 
             description = $4, updated_at = $5
         WHERE id = $6",
    )
    .bind(&status)
    .bind(bandwidth)
    .bind(latency)
    .bind(&description)
    .bind(now)
    .bind(topology_id)
    .execute(pool)
    .await?;

    get_topology_by_id(pool, topology_id).await
}

/// Delete network topology connection
pub async fn delete_network_topology(
    pool: &Pool<Postgres>,
    topology_id: Uuid,
) -> ResourceManagementResult<()> {
    let result = sqlx::query("DELETE FROM network_topology WHERE id = $1")
        .bind(topology_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ResourceManagementError::TopologyNotFound(format!(
            "Topology connection with id {} not found",
            topology_id
        )));
    }

    Ok(())
}

/// Get topology path between two resources
pub async fn get_topology_path(
    pool: &Pool<Postgres>,
    source_resource_id: Uuid,
    target_resource_id: Uuid,
) -> ResourceManagementResult<Vec<NetworkTopology>> {
    // Simple path finding: get direct connections first
    let direct = sqlx::query(
        "SELECT id, source_resource_id, target_resource_id, connection_type, relationship_type,
         connection_status, bandwidth_mbps, latency_ms, description, created_at, updated_at
         FROM network_topology
         WHERE (source_resource_id = $1 AND target_resource_id = $2)
         OR (source_resource_id = $2 AND target_resource_id = $1)
         AND connection_status = 'ACTIVE'",
    )
    .bind(source_resource_id)
    .bind(target_resource_id)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = direct {
        return Ok(vec![row_to_topology(&row)]);
    }

    // For now, return empty - could implement BFS/DFS for multi-hop paths
    Ok(Vec::new())
}

/// Get all resources connected to a resource
pub async fn get_connected_resources(
    pool: &Pool<Postgres>,
    resource_inventory_id: Uuid,
) -> ResourceManagementResult<Vec<Uuid>> {
    let rows = sqlx::query(
        "SELECT DISTINCT 
         CASE 
             WHEN source_resource_id = $1 THEN target_resource_id
             ELSE source_resource_id
         END as connected_resource_id
         FROM network_topology
         WHERE (source_resource_id = $1 OR target_resource_id = $1)
         AND connection_status = 'ACTIVE'",
    )
    .bind(resource_inventory_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(|row| row.get("connected_resource_id")).collect())
}

/// Helper to convert database row to NetworkTopology
fn row_to_topology(row: &sqlx::postgres::PgRow) -> NetworkTopology {
    NetworkTopology {
        id: row.get("id"),
        source_resource_id: row.get("source_resource_id"),
        target_resource_id: row.get("target_resource_id"),
        connection_type: row.get("connection_type"),
        relationship_type: row.get("relationship_type"),
        connection_status: row.get("connection_status"),
        bandwidth_mbps: row.get("bandwidth_mbps"),
        latency_ms: row.get("latency_ms"),
        description: row.get("description"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}


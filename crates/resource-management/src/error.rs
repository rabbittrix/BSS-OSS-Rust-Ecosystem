//! Error types for Resource Management

use thiserror::Error;

/// Resource Management errors
#[derive(Debug, Error)]
pub enum ResourceManagementError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Insufficient capacity: {0}")]
    InsufficientCapacity(String),

    #[error("Reservation conflict: {0}")]
    ReservationConflict(String),

    #[error("Invalid reservation time range")]
    InvalidTimeRange,

    #[error("Reservation not found: {0}")]
    ReservationNotFound(String),

    #[error("Topology connection not found: {0}")]
    TopologyNotFound(String),

    #[error("Invalid topology relationship: {0}")]
    InvalidTopologyRelationship(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

pub type ResourceManagementResult<T> = Result<T, ResourceManagementError>;

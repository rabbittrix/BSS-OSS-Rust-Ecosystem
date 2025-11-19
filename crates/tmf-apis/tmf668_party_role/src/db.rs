//! Database operations for TMF668 Party Role Management

use crate::models::{CreatePartyRoleRequest, PartyRole, PartyRoleState};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse party role state from database string
fn parse_party_role_state(s: &str) -> PartyRoleState {
    match s.to_uppercase().as_str() {
        "INITIALIZED" => PartyRoleState::Initialized,
        "VALIDATED" => PartyRoleState::Validated,
        "ACTIVE" => PartyRoleState::Active,
        "SUSPENDED" => PartyRoleState::Suspended,
        "TERMINATED" => PartyRoleState::Terminated,
        _ => PartyRoleState::Initialized,
    }
}

/// Convert party role state to database string
fn party_role_state_to_string(state: &PartyRoleState) -> String {
    match state {
        PartyRoleState::Initialized => "INITIALIZED".to_string(),
        PartyRoleState::Validated => "VALIDATED".to_string(),
        PartyRoleState::Active => "ACTIVE".to_string(),
        PartyRoleState::Suspended => "SUSPENDED".to_string(),
        PartyRoleState::Terminated => "TERMINATED".to_string(),
    }
}

/// Get all party roles
pub async fn get_party_roles(pool: &Pool<Postgres>) -> TmfResult<Vec<PartyRole>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, role, party_type, engagement_date, 
         href, last_update
         FROM party_roles ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut party_roles = Vec::new();
    for row in rows {
        party_roles.push(PartyRole {
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
            state: parse_party_role_state(&row.get::<String, _>("state")),
            role: row.get::<String, _>("role"),
            party_type: row.get::<Option<String>, _>("party_type"),
            contact_medium: None, // Load separately if needed
            related_party: None,  // Load separately if needed
            engagement_date: row.get::<Option<DateTime<Utc>>, _>("engagement_date"),
        });
    }

    Ok(party_roles)
}

/// Get party role by ID
pub async fn get_party_role_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<PartyRole> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, role, party_type, engagement_date, 
         href, last_update
         FROM party_roles WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Party role with id {} not found", id)))?;

    Ok(PartyRole {
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
        state: parse_party_role_state(&row.get::<String, _>("state")),
        role: row.get::<String, _>("role"),
        party_type: row.get::<Option<String>, _>("party_type"),
        contact_medium: None,
        related_party: None,
        engagement_date: row.get::<Option<DateTime<Utc>>, _>("engagement_date"),
    })
}

/// Create a new party role
pub async fn create_party_role(
    pool: &Pool<Postgres>,
    request: CreatePartyRoleRequest,
) -> TmfResult<PartyRole> {
    let id = Uuid::new_v4();
    let state = party_role_state_to_string(&PartyRoleState::Initialized);

    sqlx::query(
        "INSERT INTO party_roles (id, name, description, version, state, role, party_type, engagement_date)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(&request.role)
    .bind(&request.party_type)
    .bind(request.engagement_date)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create contact mediums if provided
    if let Some(contacts) = request.contact_medium {
        for contact in contacts {
            let contact_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO party_role_contact_mediums (id, party_role_id, medium_type, value, preferred)
                 VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(contact_id)
            .bind(id)
            .bind(&contact.medium_type)
            .bind(&contact.value)
            .bind(contact.preferred.unwrap_or(false))
            .execute(pool)
            .await
            .map_err(map_sqlx_error)?;
        }
    }

    // Create related parties if provided
    if let Some(parties) = request.related_party {
        for party in parties {
            let party_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO party_role_related_parties (id, party_role_id, name, role)
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

    // Fetch the created party role
    get_party_role_by_id(pool, id).await
}

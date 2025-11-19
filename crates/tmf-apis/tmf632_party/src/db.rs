//! Database operations for TMF632 Party Management

use crate::models::{CreatePartyRequest, Party, PartyState, PartyType};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse party state from database string
fn parse_party_state(s: &str) -> PartyState {
    match s.to_uppercase().as_str() {
        "INITIALIZED" => PartyState::Initialized,
        "VALIDATED" => PartyState::Validated,
        "ACTIVE" => PartyState::Active,
        "SUSPENDED" => PartyState::Suspended,
        "TERMINATED" => PartyState::Terminated,
        _ => PartyState::Initialized,
    }
}

/// Convert party state to database string
fn party_state_to_string(state: &PartyState) -> String {
    match state {
        PartyState::Initialized => "INITIALIZED".to_string(),
        PartyState::Validated => "VALIDATED".to_string(),
        PartyState::Active => "ACTIVE".to_string(),
        PartyState::Suspended => "SUSPENDED".to_string(),
        PartyState::Terminated => "TERMINATED".to_string(),
    }
}

/// Parse party type from database string
fn parse_party_type(s: &str) -> PartyType {
    match s.to_uppercase().as_str() {
        "INDIVIDUAL" => PartyType::Individual,
        "ORGANIZATION" => PartyType::Organization,
        _ => PartyType::Individual,
    }
}

/// Convert party type to database string
fn party_type_to_string(party_type: &PartyType) -> String {
    match party_type {
        PartyType::Individual => "INDIVIDUAL".to_string(),
        PartyType::Organization => "ORGANIZATION".to_string(),
    }
}

/// Get all parties
pub async fn get_parties(pool: &Pool<Postgres>) -> TmfResult<Vec<Party>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, party_type, registration_date, 
         href, last_update
         FROM parties ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut parties = Vec::new();
    for row in rows {
        parties.push(Party {
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
            state: parse_party_state(&row.get::<String, _>("state")),
            party_type: parse_party_type(&row.get::<String, _>("party_type")),
            contact_medium: None, // Load separately if needed
            related_party: None,  // Load separately if needed
            account: None,        // Load separately if needed
            characteristic: None, // Load separately if needed
            registration_date: row.get::<Option<DateTime<Utc>>, _>("registration_date"),
        });
    }

    Ok(parties)
}

/// Get party by ID
pub async fn get_party_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<Party> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, party_type, registration_date, 
         href, last_update
         FROM parties WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Party with id {} not found", id)))?;

    Ok(Party {
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
        state: parse_party_state(&row.get::<String, _>("state")),
        party_type: parse_party_type(&row.get::<String, _>("party_type")),
        contact_medium: None,
        related_party: None,
        account: None,
        characteristic: None,
        registration_date: row.get::<Option<DateTime<Utc>>, _>("registration_date"),
    })
}

/// Create a new party
pub async fn create_party(pool: &Pool<Postgres>, request: CreatePartyRequest) -> TmfResult<Party> {
    let id = Uuid::new_v4();
    let state = party_state_to_string(&PartyState::Initialized);
    let party_type = party_type_to_string(&request.party_type);

    sqlx::query(
        "INSERT INTO parties (id, name, description, version, state, party_type, registration_date)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(&party_type)
    .bind(request.registration_date)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create contact mediums if provided
    if let Some(contacts) = request.contact_medium {
        for contact in contacts {
            let contact_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO party_contact_mediums (id, party_id, medium_type, value, preferred)
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
                "INSERT INTO party_related_parties (id, party_id, name, role)
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

    // Create accounts if provided
    if let Some(accounts) = request.account {
        for account in accounts {
            let account_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO party_accounts (id, party_id, name)
                 VALUES ($1, $2, $3)",
            )
            .bind(account_id)
            .bind(id)
            .bind(&account.name)
            .execute(pool)
            .await
            .map_err(map_sqlx_error)?;
        }
    }

    // Create characteristics if provided
    if let Some(characteristics) = request.characteristic {
        for char in characteristics {
            let char_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO party_characteristics (id, party_id, name, value, value_type)
                 VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(char_id)
            .bind(id)
            .bind(&char.name)
            .bind(&char.value)
            .bind(&char.value_type)
            .execute(pool)
            .await
            .map_err(map_sqlx_error)?;
        }
    }

    // Fetch the created party
    get_party_by_id(pool, id).await
}

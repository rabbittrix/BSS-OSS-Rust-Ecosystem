//! Database operations for TMF629 Customer Management

use crate::models::{CreateCustomerRequest, Customer, CustomerState};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse customer state from database string
fn parse_customer_state(s: &str) -> CustomerState {
    match s.to_uppercase().as_str() {
        "INITIAL" => CustomerState::Initial,
        "ACTIVE" => CustomerState::Active,
        "SUSPENDED" => CustomerState::Suspended,
        "TERMINATED" => CustomerState::Terminated,
        _ => CustomerState::Initial,
    }
}

/// Convert customer state to database string
fn customer_state_to_string(state: &CustomerState) -> String {
    match state {
        CustomerState::Initial => "INITIAL".to_string(),
        CustomerState::Active => "ACTIVE".to_string(),
        CustomerState::Suspended => "SUSPENDED".to_string(),
        CustomerState::Terminated => "TERMINATED".to_string(),
    }
}

/// Get all customers
pub async fn get_customers(pool: &Pool<Postgres>) -> TmfResult<Vec<Customer>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, status, href, last_update
         FROM customers ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut customers = Vec::new();
    for row in rows {
        customers.push(Customer {
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
            state: parse_customer_state(&row.get::<String, _>("state")),
            status: row.get::<Option<String>, _>("status"),
            contact_medium: None, // Load separately if needed
            account: None,        // Load separately if needed
            related_party: None,  // Load separately if needed
            characteristic: None, // Load separately if needed
        });
    }

    Ok(customers)
}

/// Get customer by ID
pub async fn get_customer_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<Customer> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, status, href, last_update
         FROM customers WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Customer with id {} not found", id)))?;

    Ok(Customer {
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
        state: parse_customer_state(&row.get::<String, _>("state")),
        status: row.get::<Option<String>, _>("status"),
        contact_medium: None,
        account: None,
        related_party: None,
        characteristic: None,
    })
}

/// Create a new customer
pub async fn create_customer(
    pool: &Pool<Postgres>,
    request: CreateCustomerRequest,
) -> TmfResult<Customer> {
    let id = Uuid::new_v4();
    let state = customer_state_to_string(&CustomerState::Initial);

    sqlx::query(
        "INSERT INTO customers (id, name, description, version, state, status)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(&request.status)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create contact mediums if provided
    if let Some(contacts) = request.contact_medium {
        for contact in contacts {
            let contact_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO customer_contact_mediums (id, customer_id, medium_type, preferred, value, contact_type)
                 VALUES ($1, $2, $3, $4, $5, $6)",
            )
            .bind(contact_id)
            .bind(id)
            .bind(&contact.medium_type)
            .bind(contact.preferred)
            .bind(&contact.value)
            .bind(&contact.contact_type)
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
                "INSERT INTO customer_related_parties (id, customer_id, name, role)
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

    // Fetch the created customer
    get_customer_by_id(pool, id).await
}

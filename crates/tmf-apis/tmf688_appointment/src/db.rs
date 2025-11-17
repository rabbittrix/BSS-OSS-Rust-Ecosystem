//! Database operations for TMF688 Appointment Management

use crate::models::{CreateAppointmentRequest, Appointment, AppointmentState};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse appointment state from database string
fn parse_appointment_state(s: &str) -> AppointmentState {
    match s.to_uppercase().as_str() {
        "INITIAL" => AppointmentState::Initial,
        "CONFIRMED" => AppointmentState::Confirmed,
        "IN_PROGRESS" => AppointmentState::InProgress,
        "COMPLETED" => AppointmentState::Completed,
        "CANCELLED" => AppointmentState::Cancelled,
        "FAILED" => AppointmentState::Failed,
        _ => AppointmentState::Initial,
    }
}

/// Convert appointment state to database string
fn appointment_state_to_string(state: &AppointmentState) -> String {
    match state {
        AppointmentState::Initial => "INITIAL".to_string(),
        AppointmentState::Confirmed => "CONFIRMED".to_string(),
        AppointmentState::InProgress => "IN_PROGRESS".to_string(),
        AppointmentState::Completed => "COMPLETED".to_string(),
        AppointmentState::Cancelled => "CANCELLED".to_string(),
        AppointmentState::Failed => "FAILED".to_string(),
    }
}

/// Get all appointments
pub async fn get_appointments(pool: &Pool<Postgres>) -> TmfResult<Vec<Appointment>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, appointment_date, duration, 
         appointment_type, href, last_update
         FROM appointments ORDER BY appointment_date DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut appointments = Vec::new();
    for row in rows {
        appointments.push(Appointment {
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
            state: parse_appointment_state(&row.get::<String, _>("state")),
            appointment_date: row.get::<Option<DateTime<Utc>>, _>("appointment_date"),
            duration: row.get::<Option<i32>, _>("duration"),
            appointment_type: row.get::<Option<String>, _>("appointment_type"),
            description: row.get::<Option<String>, _>("description"),
            related_party: None,  // Load separately if needed
            contact_medium: None, // Load separately if needed
        });
    }

    Ok(appointments)
}

/// Get appointment by ID
pub async fn get_appointment_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<Appointment> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, appointment_date, duration, 
         appointment_type, href, last_update
         FROM appointments WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Appointment with id {} not found", id)))?;

    Ok(Appointment {
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
        state: parse_appointment_state(&row.get::<String, _>("state")),
        appointment_date: row.get::<Option<DateTime<Utc>>, _>("appointment_date"),
        duration: row.get::<Option<i32>, _>("duration"),
        appointment_type: row.get::<Option<String>, _>("appointment_type"),
        description: row.get::<Option<String>, _>("description"),
        related_party: None,
        contact_medium: None,
    })
}

/// Create a new appointment
pub async fn create_appointment(
    pool: &Pool<Postgres>,
    request: CreateAppointmentRequest,
) -> TmfResult<Appointment> {
    let id = Uuid::new_v4();
    let state = appointment_state_to_string(&AppointmentState::Initial);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO appointments (id, name, description, version, state, appointment_date, duration, appointment_type)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(request.appointment_date.unwrap_or(now))
    .bind(&request.duration)
    .bind(&request.appointment_type)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create related parties if provided
    if let Some(parties) = request.related_party {
        for party in parties {
            let party_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO appointment_related_parties (id, appointment_id, name, role)
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

    // Create contact mediums if provided
    if let Some(contacts) = request.contact_medium {
        for contact in contacts {
            let contact_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO appointment_contact_mediums (id, appointment_id, medium_type, value)
                 VALUES ($1, $2, $3, $4)",
            )
            .bind(contact_id)
            .bind(id)
            .bind(&contact.medium_type)
            .bind(&contact.value)
            .execute(pool)
            .await
            .map_err(map_sqlx_error)?;
        }
    }

    // Fetch the created appointment
    get_appointment_by_id(pool, id).await
}


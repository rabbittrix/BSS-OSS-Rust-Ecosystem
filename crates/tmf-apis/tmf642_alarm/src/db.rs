//! Database operations for TMF642 Alarm Management

use crate::models::{Alarm, AlarmSeverity, AlarmState, AlarmType, CreateAlarmRequest};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse alarm state from database string
fn parse_alarm_state(s: &str) -> AlarmState {
    match s.to_uppercase().as_str() {
        "RAISED" => AlarmState::Raised,
        "ACKNOWLEDGED" => AlarmState::Acknowledged,
        "CLEARED" => AlarmState::Cleared,
        "CLOSED" => AlarmState::Closed,
        _ => AlarmState::Raised,
    }
}

/// Convert alarm state to database string
fn alarm_state_to_string(state: &AlarmState) -> String {
    match state {
        AlarmState::Raised => "RAISED".to_string(),
        AlarmState::Acknowledged => "ACKNOWLEDGED".to_string(),
        AlarmState::Cleared => "CLEARED".to_string(),
        AlarmState::Closed => "CLOSED".to_string(),
    }
}

/// Parse alarm severity from database string
fn parse_alarm_severity(s: &str) -> AlarmSeverity {
    match s.to_uppercase().as_str() {
        "CRITICAL" => AlarmSeverity::Critical,
        "MAJOR" => AlarmSeverity::Major,
        "MINOR" => AlarmSeverity::Minor,
        "WARNING" => AlarmSeverity::Warning,
        "INDETERMINATE" => AlarmSeverity::Indeterminate,
        _ => AlarmSeverity::Indeterminate,
    }
}

/// Convert alarm severity to database string
fn alarm_severity_to_string(severity: &AlarmSeverity) -> String {
    match severity {
        AlarmSeverity::Critical => "CRITICAL".to_string(),
        AlarmSeverity::Major => "MAJOR".to_string(),
        AlarmSeverity::Minor => "MINOR".to_string(),
        AlarmSeverity::Warning => "WARNING".to_string(),
        AlarmSeverity::Indeterminate => "INDETERMINATE".to_string(),
    }
}

/// Parse alarm type from database string
fn parse_alarm_type(s: &str) -> AlarmType {
    match s.to_uppercase().as_str() {
        "COMMUNICATIONS_ALARM" => AlarmType::CommunicationsAlarm,
        "QUALITY_OF_SERVICE_ALARM" => AlarmType::QualityOfServiceAlarm,
        "PROCESSING_ERROR_ALARM" => AlarmType::ProcessingErrorAlarm,
        "EQUIPMENT_ALARM" => AlarmType::EquipmentAlarm,
        "ENVIRONMENTAL_ALARM" => AlarmType::EnvironmentalAlarm,
        "INTEGRITY_VIOLATION" => AlarmType::IntegrityViolation,
        "OPERATIONAL_VIOLATION" => AlarmType::OperationalViolation,
        "PHYSICAL_VIOLATION" => AlarmType::PhysicalViolation,
        "SECURITY_SERVICE_OR_MECHANISM_VIOLATION" => AlarmType::SecurityServiceOrMechanismViolation,
        "TIME_DOMAIN_VIOLATION" => AlarmType::TimeDomainViolation,
        _ => AlarmType::ProcessingErrorAlarm,
    }
}

/// Convert alarm type to database string
fn alarm_type_to_string(alarm_type: &AlarmType) -> String {
    match alarm_type {
        AlarmType::CommunicationsAlarm => "COMMUNICATIONS_ALARM".to_string(),
        AlarmType::QualityOfServiceAlarm => "QUALITY_OF_SERVICE_ALARM".to_string(),
        AlarmType::ProcessingErrorAlarm => "PROCESSING_ERROR_ALARM".to_string(),
        AlarmType::EquipmentAlarm => "EQUIPMENT_ALARM".to_string(),
        AlarmType::EnvironmentalAlarm => "ENVIRONMENTAL_ALARM".to_string(),
        AlarmType::IntegrityViolation => "INTEGRITY_VIOLATION".to_string(),
        AlarmType::OperationalViolation => "OPERATIONAL_VIOLATION".to_string(),
        AlarmType::PhysicalViolation => "PHYSICAL_VIOLATION".to_string(),
        AlarmType::SecurityServiceOrMechanismViolation => {
            "SECURITY_SERVICE_OR_MECHANISM_VIOLATION".to_string()
        }
        AlarmType::TimeDomainViolation => "TIME_DOMAIN_VIOLATION".to_string(),
    }
}

/// Get all alarms
pub async fn get_alarms(pool: &Pool<Postgres>) -> TmfResult<Vec<Alarm>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, severity, alarm_type, 
         source_resource_id, raised_time, acknowledged_time, cleared_time, 
         alarm_details, href, last_update
         FROM alarms ORDER BY raised_time DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut alarms = Vec::new();
    for row in rows {
        alarms.push(Alarm {
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
            state: parse_alarm_state(&row.get::<String, _>("state")),
            severity: parse_alarm_severity(&row.get::<String, _>("severity")),
            alarm_type: parse_alarm_type(&row.get::<String, _>("alarm_type")),
            source_resource: None, // Load separately if needed
            raised_time: row.get::<Option<DateTime<Utc>>, _>("raised_time"),
            acknowledged_time: row.get::<Option<DateTime<Utc>>, _>("acknowledged_time"),
            cleared_time: row.get::<Option<DateTime<Utc>>, _>("cleared_time"),
            alarm_details: row.get::<Option<String>, _>("alarm_details"),
        });
    }

    Ok(alarms)
}

/// Get alarm by ID
pub async fn get_alarm_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<Alarm> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, severity, alarm_type, 
         source_resource_id, raised_time, acknowledged_time, cleared_time, 
         alarm_details, href, last_update
         FROM alarms WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Alarm with id {} not found", id)))?;

    Ok(Alarm {
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
        state: parse_alarm_state(&row.get::<String, _>("state")),
        severity: parse_alarm_severity(&row.get::<String, _>("severity")),
        alarm_type: parse_alarm_type(&row.get::<String, _>("alarm_type")),
        source_resource: None, // Load separately if needed
        raised_time: row.get::<Option<DateTime<Utc>>, _>("raised_time"),
        acknowledged_time: row.get::<Option<DateTime<Utc>>, _>("acknowledged_time"),
        cleared_time: row.get::<Option<DateTime<Utc>>, _>("cleared_time"),
        alarm_details: row.get::<Option<String>, _>("alarm_details"),
    })
}

/// Create a new alarm
pub async fn create_alarm(pool: &Pool<Postgres>, request: CreateAlarmRequest) -> TmfResult<Alarm> {
    let id = Uuid::new_v4();
    let href = Some(format!("/tmf-api/alarmManagement/v4/alarm/{}", id));
    let raised_time = request.raised_time.unwrap_or_else(Utc::now);

    sqlx::query(
        "INSERT INTO alarms (id, name, description, version, state, severity, alarm_type, 
         source_resource_id, raised_time, alarm_details, href)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(alarm_state_to_string(&AlarmState::Raised))
    .bind(alarm_severity_to_string(&request.severity))
    .bind(alarm_type_to_string(&request.alarm_type))
    .bind(request.source_resource_id)
    .bind(raised_time)
    .bind(&request.alarm_details)
    .bind(&href)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Fetch the created alarm
    get_alarm_by_id(pool, id).await
}

/// Update an alarm
pub async fn update_alarm(
    pool: &Pool<Postgres>,
    id: Uuid,
    state: Option<AlarmState>,
    acknowledged_time: Option<DateTime<Utc>>,
    cleared_time: Option<DateTime<Utc>>,
) -> TmfResult<Alarm> {
    sqlx::query(
        "UPDATE alarms SET 
         state = COALESCE($1, state), 
         acknowledged_time = COALESCE($2, acknowledged_time),
         cleared_time = COALESCE($3, cleared_time),
         last_update = CURRENT_TIMESTAMP
         WHERE id = $4",
    )
    .bind(state.as_ref().map(alarm_state_to_string))
    .bind(acknowledged_time)
    .bind(cleared_time)
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Fetch the updated alarm
    get_alarm_by_id(pool, id).await
}

/// Delete an alarm
pub async fn delete_alarm(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<()> {
    let result = sqlx::query("DELETE FROM alarms WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

    if result.rows_affected() == 0 {
        return Err(TmfError::NotFound(format!(
            "Alarm with id {} not found",
            id
        )));
    }

    Ok(())
}

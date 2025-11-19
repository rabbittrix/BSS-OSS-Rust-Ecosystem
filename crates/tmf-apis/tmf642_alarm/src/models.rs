//! TMF642 Alarm Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Alarm State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlarmState {
    Raised,
    Acknowledged,
    Cleared,
    Closed,
}

/// Alarm Severity
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlarmSeverity {
    Critical,
    Major,
    Minor,
    Warning,
    Indeterminate,
}

/// Alarm Type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlarmType {
    CommunicationsAlarm,
    QualityOfServiceAlarm,
    ProcessingErrorAlarm,
    EquipmentAlarm,
    EnvironmentalAlarm,
    IntegrityViolation,
    OperationalViolation,
    PhysicalViolation,
    SecurityServiceOrMechanismViolation,
    TimeDomainViolation,
}

/// Alarm - Represents a network alarm
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Alarm {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Alarm state
    pub state: AlarmState,
    /// Alarm severity
    pub severity: AlarmSeverity,
    /// Alarm type
    pub alarm_type: AlarmType,
    /// Source resource reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_resource: Option<ResourceRef>,
    /// Raised time
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub raised_time: Option<DateTime<Utc>>,
    /// Acknowledged time
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub acknowledged_time: Option<DateTime<Utc>>,
    /// Cleared time
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub cleared_time: Option<DateTime<Utc>>,
    /// Alarm specific information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_details: Option<String>,
}

/// Resource Reference
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResourceRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// Request to create an alarm
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAlarmRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub severity: AlarmSeverity,
    pub alarm_type: AlarmType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "uuid")]
    pub source_resource_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub raised_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_details: Option<String>,
}

/// Request to update an alarm
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateAlarmRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<AlarmState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub acknowledged_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub cleared_time: Option<DateTime<Utc>>,
}

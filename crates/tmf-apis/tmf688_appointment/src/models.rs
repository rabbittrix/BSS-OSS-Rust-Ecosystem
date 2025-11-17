//! TMF688 Appointment Management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Appointment State
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppointmentState {
    Initial,
    Confirmed,
    InProgress,
    Completed,
    Cancelled,
    Failed,
}

/// Appointment - Represents a scheduled appointment
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Appointment {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Appointment state
    pub state: AppointmentState,
    /// Appointment date/time
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub appointment_date: Option<DateTime<Utc>>,
    /// Duration in minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// Appointment type (installation, repair, maintenance, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointment_type: Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Related party (customer, technician, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Contact medium (address, phone, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<ContactMedium>>,
}

/// Related Party - Party related to the appointment
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelatedParty {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
    pub role: String,
}

/// Contact Medium - Contact information for the appointment
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContactMedium {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    /// Contact medium type (address, phone, email, etc.)
    pub medium_type: String,
    /// Contact value
    pub value: String,
}

/// Request to create an appointment
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAppointmentRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub appointment_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointment_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<CreateRelatedPartyRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<CreateContactMediumRequest>>,
}

/// Request to create a related party
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRelatedPartyRequest {
    pub name: String,
    pub role: String,
}

/// Request to create a contact medium
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateContactMediumRequest {
    pub medium_type: String,
    pub value: String,
}

//! IoT Device Management Models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

/// Device Status
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceStatus {
    Registered,
    Provisioned,
    Active,
    Inactive,
    Offline,
    Maintenance,
    Decommissioned,
}

/// Device Type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceType {
    Sensor,
    Actuator,
    Gateway,
    Controller,
    SmartMeter,
    Router,
    Switch,
    Other,
}

/// IoT Device
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IoTDevice {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub device_type: DeviceType,
    pub status: DeviceStatus,
    pub manufacturer: String,
    pub model: String,
    pub serial_number: String,
    pub firmware_version: Option<String>,
    pub hardware_version: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
    pub location: Option<DeviceLocation>,
    pub capabilities: Vec<DeviceCapability>,
    pub configuration: Option<serde_json::Value>,
    pub last_seen: Option<DateTime<Utc>>,
    pub tenant_id: Option<Uuid>,
}

/// Device Location
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeviceLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub address: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub room: Option<String>,
}

/// Device Capability
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeviceCapability {
    pub name: String,
    pub capability_type: String,
    pub description: Option<String>,
    pub parameters: Option<serde_json::Value>,
}

/// Device Telemetry Data
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeviceTelemetry {
    pub device_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub metrics: serde_json::Value,
    pub tags: Option<serde_json::Value>,
}

/// Create Device Request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateDeviceRequest {
    pub name: String,
    pub description: Option<String>,
    pub device_type: DeviceType,
    pub manufacturer: String,
    pub model: String,
    pub serial_number: String,
    pub firmware_version: Option<String>,
    pub hardware_version: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
    pub location: Option<DeviceLocation>,
    pub capabilities: Vec<DeviceCapability>,
    pub configuration: Option<serde_json::Value>,
    pub tenant_id: Option<Uuid>,
}

/// Update Device Request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateDeviceRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<DeviceStatus>,
    pub firmware_version: Option<String>,
    pub ip_address: Option<String>,
    pub location: Option<DeviceLocation>,
    pub configuration: Option<serde_json::Value>,
}

/// Device Control Command
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeviceCommand {
    pub device_id: Uuid,
    pub command: String,
    pub parameters: Option<serde_json::Value>,
    pub timeout_seconds: Option<u64>,
}

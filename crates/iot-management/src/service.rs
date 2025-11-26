//! IoT Device Management Service

use crate::error::IoTError;
use crate::models::{
    CreateDeviceRequest, DeviceStatus, DeviceTelemetry, IoTDevice, UpdateDeviceRequest,
};
use chrono::Utc;
use sqlx::PgPool;
use tmf_apis_core::BaseEntity;
use uuid::Uuid;

/// IoT Device Management Service
pub struct IoTService {
    pool: PgPool,
}

impl IoTService {
    /// Create a new IoT service
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Register a new IoT device
    pub async fn register_device(
        &self,
        request: CreateDeviceRequest,
    ) -> Result<IoTDevice, IoTError> {
        // Check if device with same serial number already exists
        let existing = sqlx::query_scalar::<_, Option<Uuid>>(
            "SELECT id FROM iot_devices WHERE serial_number = $1",
        )
        .bind(&request.serial_number)
        .fetch_optional(&self.pool)
        .await?;

        if existing.is_some() {
            return Err(IoTError::DeviceAlreadyExists(format!(
                "Device with serial number {} already exists",
                request.serial_number
            )));
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO iot_devices (
                id, name, description, device_type, manufacturer, model, 
                serial_number, firmware_version, hardware_version, mac_address,
                ip_address, status, location, capabilities, configuration,
                tenant_id, created_at, last_update
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $17)",
        )
        .bind(id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(format!("{:?}", request.device_type))
        .bind(&request.manufacturer)
        .bind(&request.model)
        .bind(&request.serial_number)
        .bind(&request.firmware_version)
        .bind(&request.hardware_version)
        .bind(&request.mac_address)
        .bind(&request.ip_address)
        .bind(format!("{:?}", DeviceStatus::Registered))
        .bind(&serde_json::to_value(&request.location)?)
        .bind(&serde_json::to_value(&request.capabilities)?)
        .bind(&request.configuration)
        .bind(&request.tenant_id)
        .bind(now)
        .execute(&self.pool)
        .await?;

        // Fetch the created device
        self.get_device(id).await
    }

    /// Get device by ID
    pub async fn get_device(&self, device_id: Uuid) -> Result<IoTDevice, IoTError> {
        let row = sqlx::query(
            "SELECT id, name, description, device_type, manufacturer, model,
             serial_number, firmware_version, hardware_version, mac_address,
             ip_address, status, location, capabilities, configuration,
             tenant_id, created_at, last_update, last_seen
             FROM iot_devices WHERE id = $1",
        )
        .bind(device_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(self.row_to_device(&row)?),
            None => Err(IoTError::DeviceNotFound(device_id.to_string())),
        }
    }

    /// List all devices
    pub async fn list_devices(
        &self,
        tenant_id: Option<Uuid>,
        status: Option<DeviceStatus>,
    ) -> Result<Vec<IoTDevice>, IoTError> {
        let mut query = "SELECT id, name, description, device_type, manufacturer, model,
             serial_number, firmware_version, hardware_version, mac_address,
             ip_address, status, location, capabilities, configuration,
             tenant_id, created_at, last_update, last_seen
             FROM iot_devices WHERE 1=1"
            .to_string();

        if tenant_id.is_some() {
            query.push_str(" AND tenant_id = $1");
        }
        if status.is_some() {
            query.push_str(if tenant_id.is_some() {
                " AND status = $2"
            } else {
                " AND status = $1"
            });
        }

        let rows = if let Some(tid) = tenant_id {
            if let Some(s) = status {
                sqlx::query(&query)
                    .bind(tid)
                    .bind(format!("{:?}", s))
                    .fetch_all(&self.pool)
                    .await?
            } else {
                sqlx::query(&query).bind(tid).fetch_all(&self.pool).await?
            }
        } else if let Some(s) = status {
            sqlx::query(&query)
                .bind(format!("{:?}", s))
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query(&query).fetch_all(&self.pool).await?
        };

        rows.iter().map(|row| self.row_to_device(row)).collect()
    }

    /// Update device
    pub async fn update_device(
        &self,
        device_id: Uuid,
        request: UpdateDeviceRequest,
    ) -> Result<IoTDevice, IoTError> {
        // Verify device exists
        self.get_device(device_id).await?;

        let mut updates = Vec::new();
        let mut bind_index = 1;

        if let Some(_) = &request.name {
            updates.push(format!("name = ${}", bind_index));
            bind_index += 1;
        }
        if let Some(_) = &request.description {
            updates.push(format!("description = ${}", bind_index));
            bind_index += 1;
        }
        if let Some(_) = &request.status {
            updates.push(format!("status = ${}", bind_index));
            bind_index += 1;
        }
        if let Some(_) = &request.firmware_version {
            updates.push(format!("firmware_version = ${}", bind_index));
            bind_index += 1;
        }
        if let Some(_) = &request.ip_address {
            updates.push(format!("ip_address = ${}", bind_index));
            bind_index += 1;
        }
        if let Some(_) = &request.location {
            updates.push(format!("location = ${}", bind_index));
            bind_index += 1;
        }
        if let Some(_) = &request.configuration {
            updates.push(format!("configuration = ${}", bind_index));
            bind_index += 1;
        }

        if updates.is_empty() {
            return self.get_device(device_id).await;
        }

        updates.push(format!("last_update = ${}", bind_index));

        let query = format!(
            "UPDATE iot_devices SET {} WHERE id = ${}",
            updates.join(", "),
            bind_index + 1
        );

        let mut query_builder = sqlx::query(&query);
        if let Some(name) = &request.name {
            query_builder = query_builder.bind(name);
        }
        if let Some(description) = &request.description {
            query_builder = query_builder.bind(description);
        }
        if let Some(status) = &request.status {
            query_builder = query_builder.bind(format!("{:?}", status));
        }
        if let Some(firmware_version) = &request.firmware_version {
            query_builder = query_builder.bind(firmware_version);
        }
        if let Some(ip_address) = &request.ip_address {
            query_builder = query_builder.bind(ip_address);
        }
        if let Some(location) = &request.location {
            let location_value = serde_json::to_value(location)?;
            query_builder = query_builder.bind(location_value);
        }
        if let Some(configuration) = &request.configuration {
            query_builder = query_builder.bind(configuration);
        }
        query_builder = query_builder.bind(Utc::now());
        query_builder = query_builder.bind(device_id);

        query_builder.execute(&self.pool).await?;

        self.get_device(device_id).await
    }

    /// Update device heartbeat (last_seen timestamp)
    pub async fn update_device_heartbeat(&self, device_id: Uuid) -> Result<(), IoTError> {
        sqlx::query("UPDATE iot_devices SET last_seen = $1 WHERE id = $2")
            .bind(Utc::now())
            .bind(device_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Store device telemetry data
    pub async fn store_telemetry(&self, telemetry: DeviceTelemetry) -> Result<(), IoTError> {
        sqlx::query(
            "INSERT INTO iot_telemetry (device_id, timestamp, metrics, tags)
             VALUES ($1, $2, $3, $4)",
        )
        .bind(telemetry.device_id)
        .bind(telemetry.timestamp)
        .bind(&telemetry.metrics)
        .bind(&telemetry.tags)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Delete device
    pub async fn delete_device(&self, device_id: Uuid) -> Result<(), IoTError> {
        let result = sqlx::query("DELETE FROM iot_devices WHERE id = $1")
            .bind(device_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(IoTError::DeviceNotFound(device_id.to_string()));
        }

        Ok(())
    }

    /// Convert database row to IoTDevice
    fn row_to_device(&self, row: &sqlx::postgres::PgRow) -> Result<IoTDevice, IoTError> {
        use sqlx::Row;

        let id: Uuid = row.get("id");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        let device_type_str: String = row.get("device_type");
        let manufacturer: String = row.get("manufacturer");
        let model: String = row.get("model");
        let serial_number: String = row.get("serial_number");
        let firmware_version: Option<String> = row.get("firmware_version");
        let hardware_version: Option<String> = row.get("hardware_version");
        let mac_address: Option<String> = row.get("mac_address");
        let ip_address: Option<String> = row.get("ip_address");
        let status_str: String = row.get("status");
        let location_json: Option<serde_json::Value> = row.get("location");
        let capabilities_json: Option<serde_json::Value> = row.get("capabilities");
        let configuration: Option<serde_json::Value> = row.get("configuration");
        let tenant_id: Option<Uuid> = row.get("tenant_id");
        let _created_at: chrono::DateTime<Utc> = row.get("created_at");
        let last_update: Option<chrono::DateTime<Utc>> = row.get("last_update");
        let last_seen: Option<chrono::DateTime<Utc>> = row.get("last_seen");

        let device_type = serde_json::from_str(&format!("\"{}\"", device_type_str))
            .map_err(|e| IoTError::SerializationError(format!("Invalid device_type: {}", e)))?;

        let status = serde_json::from_str(&format!("\"{}\"", status_str))
            .map_err(|e| IoTError::SerializationError(format!("Invalid status: {}", e)))?;

        let location = location_json
            .map(|v| serde_json::from_value(v))
            .transpose()
            .map_err(|e| IoTError::SerializationError(format!("Invalid location: {}", e)))?;

        let capabilities = capabilities_json
            .map(|v| serde_json::from_value(v))
            .transpose()
            .map(|opt| opt.unwrap_or_else(|| Vec::new()))
            .map_err(|e| IoTError::SerializationError(format!("Invalid capabilities: {}", e)))?;

        Ok(IoTDevice {
            base: BaseEntity {
                id,
                href: Some(format!("/iot/devices/{}", id)),
                name,
                description,
                version: None,
                lifecycle_status: tmf_apis_core::LifecycleStatus::Active,
                last_update,
                valid_for: None,
            },
            device_type,
            status,
            manufacturer,
            model,
            serial_number,
            firmware_version,
            hardware_version,
            mac_address,
            ip_address,
            location,
            capabilities,
            configuration,
            last_seen,
            tenant_id,
        })
    }
}

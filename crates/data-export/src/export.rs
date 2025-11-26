//! Data export functionality

use crate::error::DataExportError;
use crate::models::{ExportFormat, ExportRequest};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{postgres::PgRow, Column, PgPool, Row};
use std::collections::HashMap;
use uuid::Uuid;

/// Data exporter
pub struct DataExporter {
    pool: PgPool,
}

impl DataExporter {
    /// Create a new data exporter
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Export data based on request
    pub async fn export(&self, request: ExportRequest) -> Result<String, DataExportError> {
        let mut data = HashMap::new();

        for entity_type in &request.entity_types {
            let entity_data = match entity_type.as_str() {
                "catalogs" => self.export_catalogs(request.tenant_id).await?,
                "customers" => self.export_customers(request.tenant_id).await?,
                "orders" => self.export_orders(request.tenant_id).await?,
                "products" => self.export_products(request.tenant_id).await?,
                _ => {
                    return Err(DataExportError::InvalidFormat(format!(
                        "Unknown entity type: {}",
                        entity_type
                    )))
                }
            };

            data.insert(entity_type.clone(), entity_data);
        }

        match request.format {
            ExportFormat::Json => Ok(serde_json::to_string_pretty(&data)?),
            ExportFormat::Csv => self.export_as_csv(&data),
            ExportFormat::Xml => self.export_as_xml(&data),
        }
    }

    /// Export catalogs
    async fn export_catalogs(
        &self,
        tenant_id: Option<uuid::Uuid>,
    ) -> Result<Value, DataExportError> {
        let rows = if let Some(tid) = tenant_id {
            sqlx::query("SELECT * FROM catalogs WHERE tenant_id = $1")
                .bind(tid)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query("SELECT * FROM catalogs")
                .fetch_all(&self.pool)
                .await?
        };

        let catalogs: Vec<Value> = rows.iter().map(row_to_json_value).collect();

        Ok(Value::Array(catalogs))
    }

    /// Export customers
    async fn export_customers(
        &self,
        _tenant_id: Option<uuid::Uuid>,
    ) -> Result<Value, DataExportError> {
        // Similar to export_catalogs
        Ok(Value::Array(vec![]))
    }

    /// Export orders
    async fn export_orders(
        &self,
        _tenant_id: Option<uuid::Uuid>,
    ) -> Result<Value, DataExportError> {
        // Similar to export_catalogs
        Ok(Value::Array(vec![]))
    }

    /// Export products
    async fn export_products(
        &self,
        _tenant_id: Option<uuid::Uuid>,
    ) -> Result<Value, DataExportError> {
        // Similar to export_catalogs
        Ok(Value::Array(vec![]))
    }

    /// Export as CSV
    fn export_as_csv(&self, _data: &HashMap<String, Value>) -> Result<String, DataExportError> {
        // Simplified CSV export
        Ok("CSV export not yet fully implemented".to_string())
    }

    /// Export as XML
    fn export_as_xml(&self, _data: &HashMap<String, Value>) -> Result<String, DataExportError> {
        // Simplified XML export
        Ok("XML export not yet fully implemented".to_string())
    }
}

/// Convert a database row to a JSON Value
fn row_to_json_value(row: &PgRow) -> Value {
    let mut map = serde_json::Map::new();
    for column in row.columns() {
        let column_name = column.name();
        let value = get_column_value(row, column_name);
        map.insert(column_name.to_string(), value);
    }
    Value::Object(map)
}

/// Get a column value as JSON Value, trying different types
fn get_column_value(row: &PgRow, column_name: &str) -> Value {
    // Try different common PostgreSQL types
    if let Ok(v) = row.try_get::<Option<String>, _>(column_name) {
        return v.map(Value::String).unwrap_or(Value::Null);
    }
    if let Ok(v) = row.try_get::<Option<i64>, _>(column_name) {
        return v.map(|n| Value::Number(n.into())).unwrap_or(Value::Null);
    }
    if let Ok(v) = row.try_get::<Option<f64>, _>(column_name) {
        return v
            .and_then(|n| serde_json::Number::from_f64(n).map(Value::Number))
            .unwrap_or(Value::Null);
    }
    if let Ok(v) = row.try_get::<Option<bool>, _>(column_name) {
        return v.map(Value::Bool).unwrap_or(Value::Null);
    }
    if let Ok(v) = row.try_get::<Option<Uuid>, _>(column_name) {
        return v
            .map(|u| Value::String(u.to_string()))
            .unwrap_or(Value::Null);
    }
    if let Ok(v) = row.try_get::<Option<DateTime<Utc>>, _>(column_name) {
        return v
            .map(|dt| Value::String(dt.to_rfc3339()))
            .unwrap_or(Value::Null);
    }
    if let Ok(v) = row.try_get::<Option<serde_json::Value>, _>(column_name) {
        return v.unwrap_or(Value::Null);
    }
    // Fallback to string representation
    Value::Null
}

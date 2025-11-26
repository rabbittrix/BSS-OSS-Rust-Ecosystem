//! Tenant service for managing tenants

use crate::error::TenantError;
use crate::models::{CreateTenantRequest, Tenant, TenantConfig, TenantStatus, UpdateTenantRequest};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Tenant service
pub struct TenantService {
    pool: PgPool,
}

impl TenantService {
    /// Create a new tenant service
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new tenant
    pub async fn create_tenant(&self, req: CreateTenantRequest) -> Result<Tenant, TenantError> {
        // Check if tenant with same name or domain already exists
        if let Some(ref domain) = req.domain {
            let existing =
                sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE domain = $1 OR name = $2")
                    .bind(domain)
                    .bind(&req.name)
                    .fetch_optional(&self.pool)
                    .await?;

            if existing.is_some() {
                return Err(TenantError::AlreadyExists(format!(
                    "Tenant with domain {} or name {} already exists",
                    domain, req.name
                )));
            }
        } else {
            let existing = sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE name = $1")
                .bind(&req.name)
                .fetch_optional(&self.pool)
                .await?;

            if existing.is_some() {
                return Err(TenantError::AlreadyExists(format!(
                    "Tenant with name {} already exists",
                    req.name
                )));
            }
        }

        let id = Uuid::new_v4();
        let config = req.config.unwrap_or_else(|| TenantConfig {
            max_users: None,
            max_storage_gb: None,
            features: vec![],
            custom_settings: serde_json::json!({}),
        });

        let tenant =
            sqlx::query_as::<_, Tenant>("INSERT INTO tenants (id, name, domain, status, config, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *")
                .bind(id)
                .bind(&req.name)
                .bind(&req.domain)
                .bind(TenantStatus::Active)
                .bind(serde_json::to_value(&config)?)
                .bind(Utc::now())
                .bind(Utc::now())
                .fetch_one(&self.pool)
                .await?;

        log::info!("Created tenant: {} ({})", tenant.name, tenant.id);
        Ok(tenant)
    }

    /// Get tenant by ID
    pub async fn get_tenant(&self, tenant_id: Uuid) -> Result<Tenant, TenantError> {
        let tenant = sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE id = $1")
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await?;

        tenant.ok_or_else(|| TenantError::NotFound(tenant_id.to_string()))
    }

    /// Get tenant by domain
    pub async fn get_tenant_by_domain(&self, domain: &str) -> Result<Tenant, TenantError> {
        let tenant = sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE domain = $1")
            .bind(domain)
            .fetch_optional(&self.pool)
            .await?;

        tenant.ok_or_else(|| TenantError::NotFound(domain.to_string()))
    }

    /// List all tenants
    pub async fn list_tenants(&self) -> Result<Vec<Tenant>, TenantError> {
        let tenants = sqlx::query_as::<_, Tenant>("SELECT * FROM tenants ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        Ok(tenants)
    }

    /// Update tenant
    pub async fn update_tenant(
        &self,
        tenant_id: Uuid,
        req: UpdateTenantRequest,
    ) -> Result<Tenant, TenantError> {
        // Check if tenant exists
        let existing = self.get_tenant(tenant_id).await?;

        let name = req.name.unwrap_or(existing.name);
        let domain = req.domain.or(existing.domain);
        let status = req.status.unwrap_or(existing.status);

        let config = if let Some(new_config) = req.config {
            serde_json::to_value(&new_config)?
        } else {
            existing.config
        };

        let tenant = sqlx::query_as::<_, Tenant>(
            "UPDATE tenants 
             SET name = $1, domain = $2, status = $3, config = $4, updated_at = $5
             WHERE id = $6
             RETURNING *",
        )
        .bind(&name)
        .bind(&domain)
        .bind(status)
        .bind(&config)
        .bind(Utc::now())
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await?;

        log::info!("Updated tenant: {} ({})", tenant.name, tenant.id);
        Ok(tenant)
    }

    /// Delete tenant
    pub async fn delete_tenant(&self, tenant_id: Uuid) -> Result<(), TenantError> {
        let rows_affected = sqlx::query("DELETE FROM tenants WHERE id = $1")
            .bind(tenant_id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(TenantError::NotFound(tenant_id.to_string()));
        }

        log::info!("Deleted tenant: {}", tenant_id);
        Ok(())
    }

    /// Verify tenant is active
    pub async fn verify_tenant_active(&self, tenant_id: Uuid) -> Result<(), TenantError> {
        let tenant = self.get_tenant(tenant_id).await?;

        if tenant.status != TenantStatus::Active {
            return Err(TenantError::Inactive(format!(
                "Tenant {} is not active (status: {:?})",
                tenant_id, tenant.status
            )));
        }

        Ok(())
    }
}

//! Role-Based Access Control (RBAC)
//!
//! Manages roles, permissions, and user-role assignments

use crate::error::SecurityError;
use crate::models::{Permission, Role, UserRole};
use chrono::Utc;
use log::info;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// RBAC Service
pub struct RbacService {
    pool: PgPool,
}

impl RbacService {
    /// Create a new RBAC service
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new role
    pub async fn create_role(
        &self,
        name: String,
        description: Option<String>,
        permissions: Vec<Permission>,
    ) -> Result<Role, SecurityError> {
        let id = Uuid::new_v4();
        let permissions_json = serde_json::to_string(&permissions)
            .map_err(|e| SecurityError::Rbac(format!("Failed to serialize permissions: {}", e)))?;

        sqlx::query(
            "INSERT INTO roles (id, name, description, permissions, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(id)
        .bind(&name)
        .bind(&description)
        .bind(&permissions_json)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        info!("Created role: {}", name);

        Ok(Role {
            id,
            name,
            description,
            permissions,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Get role by ID
    pub async fn get_role(&self, role_id: Uuid) -> Result<Role, SecurityError> {
        let row = sqlx::query_as::<_, RoleRow>(
            "SELECT id, name, description, permissions, created_at, updated_at
             FROM roles WHERE id = $1",
        )
        .bind(role_id)
        .fetch_optional(&self.pool)
        .await?;

        let role_row =
            row.ok_or_else(|| SecurityError::NotFound(format!("Role {} not found", role_id)))?;

        let permissions: Vec<Permission> =
            serde_json::from_str(&role_row.permissions).map_err(|e| {
                SecurityError::Rbac(format!("Failed to deserialize permissions: {}", e))
            })?;

        Ok(Role {
            id: role_row.id,
            name: role_row.name,
            description: role_row.description,
            permissions,
            created_at: role_row.created_at,
            updated_at: role_row.updated_at,
        })
    }

    /// Get role by name
    pub async fn get_role_by_name(&self, name: &str) -> Result<Role, SecurityError> {
        let row = sqlx::query_as::<_, RoleRow>(
            "SELECT id, name, description, permissions, created_at, updated_at
             FROM roles WHERE name = $1",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        let role_row =
            row.ok_or_else(|| SecurityError::NotFound(format!("Role {} not found", name)))?;

        let permissions: Vec<Permission> =
            serde_json::from_str(&role_row.permissions).map_err(|e| {
                SecurityError::Rbac(format!("Failed to deserialize permissions: {}", e))
            })?;

        Ok(Role {
            id: role_row.id,
            name: role_row.name,
            description: role_row.description,
            permissions,
            created_at: role_row.created_at,
            updated_at: role_row.updated_at,
        })
    }

    /// List all roles
    pub async fn list_roles(&self) -> Result<Vec<Role>, SecurityError> {
        let rows = sqlx::query_as::<_, RoleRow>(
            "SELECT id, name, description, permissions, created_at, updated_at
             FROM roles ORDER BY name",
        )
        .fetch_all(&self.pool)
        .await?;

        let mut roles = Vec::new();
        for row in rows {
            let permissions: Vec<Permission> =
                serde_json::from_str(&row.permissions).map_err(|e| {
                    SecurityError::Rbac(format!("Failed to deserialize permissions: {}", e))
                })?;

            roles.push(Role {
                id: row.id,
                name: row.name,
                description: row.description,
                permissions,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }

        Ok(roles)
    }

    /// Update role permissions
    pub async fn update_role_permissions(
        &self,
        role_id: Uuid,
        permissions: Vec<Permission>,
    ) -> Result<(), SecurityError> {
        let permissions_json = serde_json::to_string(&permissions)
            .map_err(|e| SecurityError::Rbac(format!("Failed to serialize permissions: {}", e)))?;

        sqlx::query("UPDATE roles SET permissions = $1, updated_at = $2 WHERE id = $3")
            .bind(&permissions_json)
            .bind(Utc::now())
            .bind(role_id)
            .execute(&self.pool)
            .await?;

        info!("Updated permissions for role: {}", role_id);
        Ok(())
    }

    /// Delete role
    pub async fn delete_role(&self, role_id: Uuid) -> Result<(), SecurityError> {
        sqlx::query("DELETE FROM roles WHERE id = $1")
            .bind(role_id)
            .execute(&self.pool)
            .await?;

        info!("Deleted role: {}", role_id);
        Ok(())
    }

    /// Assign role to identity
    pub async fn assign_role(
        &self,
        identity_id: Uuid,
        role_id: Uuid,
        assigned_by: Option<Uuid>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<UserRole, SecurityError> {
        // Check if role exists
        self.get_role(role_id).await?;

        // Check if already assigned
        let existing = sqlx::query_as::<_, UserRoleRow>(
            "SELECT id, identity_id, role_id, assigned_at, assigned_by, expires_at
             FROM user_roles WHERE identity_id = $1 AND role_id = $2",
        )
        .bind(identity_id)
        .bind(role_id)
        .fetch_optional(&self.pool)
        .await?;

        if existing.is_some() {
            return Err(SecurityError::Rbac(
                "Role already assigned to identity".to_string(),
            ));
        }

        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO user_roles (id, identity_id, role_id, assigned_at, assigned_by, expires_at)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(id)
        .bind(identity_id)
        .bind(role_id)
        .bind(Utc::now())
        .bind(assigned_by)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        info!("Assigned role {} to identity {}", role_id, identity_id);

        Ok(UserRole {
            id,
            identity_id,
            role_id,
            assigned_at: Utc::now(),
            assigned_by,
            expires_at,
        })
    }

    /// Remove role from identity
    pub async fn remove_role(&self, identity_id: Uuid, role_id: Uuid) -> Result<(), SecurityError> {
        sqlx::query("DELETE FROM user_roles WHERE identity_id = $1 AND role_id = $2")
            .bind(identity_id)
            .bind(role_id)
            .execute(&self.pool)
            .await?;

        info!("Removed role {} from identity {}", role_id, identity_id);
        Ok(())
    }

    /// Get roles for an identity
    pub async fn get_identity_roles(&self, identity_id: Uuid) -> Result<Vec<Role>, SecurityError> {
        let rows = sqlx::query_as::<_, RoleRow>(
            "SELECT r.id, r.name, r.description, r.permissions, r.created_at, r.updated_at
             FROM roles r
             INNER JOIN user_roles ur ON r.id = ur.role_id
             WHERE ur.identity_id = $1
             AND (ur.expires_at IS NULL OR ur.expires_at > CURRENT_TIMESTAMP)
             ORDER BY r.name",
        )
        .bind(identity_id)
        .fetch_all(&self.pool)
        .await?;

        let mut roles = Vec::new();
        for row in rows {
            let permissions: Vec<Permission> =
                serde_json::from_str(&row.permissions).map_err(|e| {
                    SecurityError::Rbac(format!("Failed to deserialize permissions: {}", e))
                })?;

            roles.push(Role {
                id: row.id,
                name: row.name,
                description: row.description,
                permissions,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }

        Ok(roles)
    }

    /// Get all permissions for an identity (from all roles)
    pub async fn get_identity_permissions(
        &self,
        identity_id: Uuid,
    ) -> Result<Vec<Permission>, SecurityError> {
        let roles = self.get_identity_roles(identity_id).await?;

        let mut permissions = Vec::new();
        for role in roles {
            for permission in role.permissions {
                // Avoid duplicates
                if !permissions.contains(&permission) {
                    permissions.push(permission);
                }
            }
        }

        Ok(permissions)
    }

    /// Check if identity has a specific role
    pub async fn has_role(
        &self,
        identity_id: Uuid,
        role_name: &str,
    ) -> Result<bool, SecurityError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)
             FROM user_roles ur
             INNER JOIN roles r ON ur.role_id = r.id
             WHERE ur.identity_id = $1 AND r.name = $2
             AND (ur.expires_at IS NULL OR ur.expires_at > CURRENT_TIMESTAMP)",
        )
        .bind(identity_id)
        .bind(role_name)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    /// Check if identity has a specific permission
    pub async fn has_permission(
        &self,
        identity_id: Uuid,
        resource: &str,
        action: &str,
    ) -> Result<bool, SecurityError> {
        let permission = Permission::new(resource.to_string(), action.to_string());
        let permissions = self.get_identity_permissions(identity_id).await?;

        Ok(permissions.contains(&permission))
    }

    /// Check if identity has any of the specified permissions
    pub async fn has_any_permission(
        &self,
        identity_id: Uuid,
        required_permissions: &[Permission],
    ) -> Result<bool, SecurityError> {
        let identity_permissions = self.get_identity_permissions(identity_id).await?;

        for required in required_permissions {
            if identity_permissions.contains(required) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Check if identity has all of the specified permissions
    pub async fn has_all_permissions(
        &self,
        identity_id: Uuid,
        required_permissions: &[Permission],
    ) -> Result<bool, SecurityError> {
        let identity_permissions = self.get_identity_permissions(identity_id).await?;

        for required in required_permissions {
            if !identity_permissions.contains(required) {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

/// Internal row structures
#[derive(Debug, FromRow)]
struct RoleRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    permissions: String, // JSON string
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
#[derive(Debug, FromRow)]
struct UserRoleRow {
    id: Uuid,
    identity_id: Uuid,
    role_id: Uuid,
    assigned_at: chrono::DateTime<chrono::Utc>,
    assigned_by: Option<Uuid>,
    expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

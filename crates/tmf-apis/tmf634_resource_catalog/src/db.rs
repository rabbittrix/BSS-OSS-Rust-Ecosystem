//! Database operations for TMF634 Resource Catalog

use crate::models::{
    CreateResourceCatalogRequest, CreateResourceSpecificationRequest, ResourceCatalog,
    ResourceSpecification,
};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{LifecycleStatus, TmfError, TmfResult};
use uuid::Uuid;

fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

fn parse_lifecycle_status(s: &str) -> LifecycleStatus {
    match s.to_uppercase().as_str() {
        "IN_STUDY" => LifecycleStatus::InStudy,
        "IN_DESIGN" => LifecycleStatus::InDesign,
        "IN_TEST" => LifecycleStatus::InTest,
        "ACTIVE" => LifecycleStatus::Active,
        "LAUNCHED" => LifecycleStatus::Launched,
        "RETIRED" => LifecycleStatus::Retired,
        "OBSOLETE" => LifecycleStatus::Obsolete,
        "REJECTED" => LifecycleStatus::Rejected,
        _ => LifecycleStatus::Active,
    }
}

fn lifecycle_status_to_string(status: &LifecycleStatus) -> String {
    match status {
        LifecycleStatus::InStudy => "IN_STUDY".to_string(),
        LifecycleStatus::InDesign => "IN_DESIGN".to_string(),
        LifecycleStatus::InTest => "IN_TEST".to_string(),
        LifecycleStatus::Active => "ACTIVE".to_string(),
        LifecycleStatus::Launched => "LAUNCHED".to_string(),
        LifecycleStatus::Retired => "RETIRED".to_string(),
        LifecycleStatus::Obsolete => "OBSOLETE".to_string(),
        LifecycleStatus::Rejected => "REJECTED".to_string(),
    }
}

fn row_to_catalog(row: sqlx::postgres::PgRow) -> ResourceCatalog {
    ResourceCatalog {
        base: tmf_apis_core::BaseEntity {
            id: row.get("id"),
            href: row.get("href"),
            name: row.get("name"),
            description: row.get("description"),
            version: row.get("version"),
            lifecycle_status: parse_lifecycle_status(&row.get::<String, _>("lifecycle_status")),
            valid_for: None,
            last_update: row.get("last_update"),
        },
        resource_specification: None,
    }
}

fn row_to_spec(row: sqlx::postgres::PgRow) -> ResourceSpecification {
    ResourceSpecification {
        base: tmf_apis_core::BaseEntity {
            id: row.get("id"),
            href: row.get("href"),
            name: row.get("name"),
            description: row.get("description"),
            version: row.get("version"),
            lifecycle_status: parse_lifecycle_status(&row.get::<String, _>("lifecycle_status")),
            valid_for: None,
            last_update: row.get("last_update"),
        },
        category: row.get("category"),
        is_bundle: row.get("is_bundle"),
    }
}

pub async fn list_resource_catalogs(pool: &Pool<Postgres>) -> TmfResult<Vec<ResourceCatalog>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, lifecycle_status, href, last_update
         FROM resource_catalogs ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;
    Ok(rows.into_iter().map(row_to_catalog).collect())
}

pub async fn get_resource_catalog_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<Option<ResourceCatalog>> {
    let row = sqlx::query(
        "SELECT id, name, description, version, lifecycle_status, href, last_update
         FROM resource_catalogs WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;
    Ok(row.map(row_to_catalog))
}

pub async fn create_resource_catalog(
    pool: &Pool<Postgres>,
    req: CreateResourceCatalogRequest,
) -> TmfResult<ResourceCatalog> {
    let id = Uuid::new_v4();
    let href = format!(
        "/tmf-api/resourceCatalogManagement/v4/resourceCatalog/{}",
        id
    );
    let version = req.version.unwrap_or_else(|| "1.0".to_string());
    sqlx::query(
        "INSERT INTO resource_catalogs (id, name, description, version, lifecycle_status, href, last_update)
         VALUES ($1,$2,$3,$4,$5,$6,NOW())",
    )
    .bind(id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&version)
    .bind(lifecycle_status_to_string(&req.lifecycle_status))
    .bind(&href)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;
    get_resource_catalog_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("resource catalog".into()))
}

pub async fn list_resource_specifications(
    pool: &Pool<Postgres>,
) -> TmfResult<Vec<ResourceSpecification>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, lifecycle_status, href, last_update, category, is_bundle
         FROM resource_catalog_specifications ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;
    Ok(rows.into_iter().map(row_to_spec).collect())
}

pub async fn get_resource_specification_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<Option<ResourceSpecification>> {
    let row = sqlx::query(
        "SELECT id, name, description, version, lifecycle_status, href, last_update, category, is_bundle
         FROM resource_catalog_specifications WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;
    Ok(row.map(row_to_spec))
}

pub async fn create_resource_specification(
    pool: &Pool<Postgres>,
    req: CreateResourceSpecificationRequest,
) -> TmfResult<ResourceSpecification> {
    let id = Uuid::new_v4();
    let href = format!(
        "/tmf-api/resourceCatalogManagement/v4/resourceSpecification/{}",
        id
    );
    let version = req.version.unwrap_or_else(|| "1.0".to_string());
    sqlx::query(
        "INSERT INTO resource_catalog_specifications
         (id, name, description, version, lifecycle_status, href, last_update, category, is_bundle)
         VALUES ($1,$2,$3,$4,$5,$6,NOW(),$7,$8)",
    )
    .bind(id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&version)
    .bind(lifecycle_status_to_string(&req.lifecycle_status))
    .bind(&href)
    .bind(&req.category)
    .bind(req.is_bundle)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;
    get_resource_specification_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("resource specification".into()))
}

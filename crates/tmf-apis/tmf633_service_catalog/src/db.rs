//! Database operations for TMF633 Service Catalog

use crate::models::{
    CreateServiceCatalogRequest, CreateServiceSpecificationRequest, ServiceCatalog,
    ServiceSpecification,
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

fn row_to_catalog(row: sqlx::postgres::PgRow) -> ServiceCatalog {
    ServiceCatalog {
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
        service_specification: None,
    }
}

fn row_to_spec(row: sqlx::postgres::PgRow) -> ServiceSpecification {
    ServiceSpecification {
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

pub async fn list_service_catalogs(pool: &Pool<Postgres>) -> TmfResult<Vec<ServiceCatalog>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, lifecycle_status, href, last_update
         FROM service_catalogs ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;
    Ok(rows.into_iter().map(row_to_catalog).collect())
}

pub async fn get_service_catalog_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<Option<ServiceCatalog>> {
    let row = sqlx::query(
        "SELECT id, name, description, version, lifecycle_status, href, last_update
         FROM service_catalogs WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;
    Ok(row.map(row_to_catalog))
}

pub async fn create_service_catalog(
    pool: &Pool<Postgres>,
    req: CreateServiceCatalogRequest,
) -> TmfResult<ServiceCatalog> {
    let id = Uuid::new_v4();
    let href = format!("/tmf-api/serviceCatalogManagement/v4/serviceCatalog/{}", id);
    let version = req.version.unwrap_or_else(|| "1.0".to_string());
    sqlx::query(
        "INSERT INTO service_catalogs (id, name, description, version, lifecycle_status, href, last_update)
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
    get_service_catalog_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("service catalog".into()))
}

pub async fn list_service_specifications(
    pool: &Pool<Postgres>,
) -> TmfResult<Vec<ServiceSpecification>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, lifecycle_status, href, last_update, category, is_bundle
         FROM service_specifications ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;
    Ok(rows.into_iter().map(row_to_spec).collect())
}

pub async fn get_service_specification_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<Option<ServiceSpecification>> {
    let row = sqlx::query(
        "SELECT id, name, description, version, lifecycle_status, href, last_update, category, is_bundle
         FROM service_specifications WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;
    Ok(row.map(row_to_spec))
}

pub async fn create_service_specification(
    pool: &Pool<Postgres>,
    req: CreateServiceSpecificationRequest,
) -> TmfResult<ServiceSpecification> {
    let id = Uuid::new_v4();
    let href = format!(
        "/tmf-api/serviceCatalogManagement/v4/serviceSpecification/{}",
        id
    );
    let version = req.version.unwrap_or_else(|| "1.0".to_string());
    sqlx::query(
        "INSERT INTO service_specifications
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
    get_service_specification_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("service specification".into()))
}

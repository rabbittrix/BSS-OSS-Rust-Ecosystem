//! Database operations for TMF620 Product Catalog

use crate::models::{Catalog, CreateCatalogRequest, CreateProductOfferingRequest, ProductOffering};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{LifecycleStatus, TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse lifecycle status from database string
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

/// Convert lifecycle status to database string
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

/// Initialize database connection pool
pub async fn init_db() -> Pool<Postgres> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Retry connection with exponential backoff
    let mut retries = 5;
    let mut delay = 1;

    loop {
        match sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(10))
            .connect(&db_url)
            .await
        {
            Ok(pool) => return pool,
            Err(e) if retries > 0 => {
                log::warn!(
                    "Failed to connect to database ({} retries left): {}",
                    retries,
                    e
                );
                retries -= 1;
                tokio::time::sleep(std::time::Duration::from_secs(delay)).await;
                delay *= 2;
            }
            Err(e) => panic!("Failed to connect to database after retries: {}", e),
        }
    }
}

/// Get all catalogs
pub async fn get_catalogs(pool: &Pool<Postgres>) -> TmfResult<Vec<Catalog>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, lifecycle_status, 
         href, last_update, valid_for_start, valid_for_end
         FROM catalogs ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let catalogs = rows
        .into_iter()
        .map(|row| Catalog {
            base: tmf_apis_core::BaseEntity {
                id: row.get("id"),
                href: row.get("href"),
                name: row.get("name"),
                description: row.get("description"),
                version: row.get("version"),
                lifecycle_status: parse_lifecycle_status(&row.get::<String, _>("lifecycle_status")),
                valid_for: match (
                    row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("valid_for_start"),
                    row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("valid_for_end"),
                ) {
                    (Some(start), end) => Some(tmf_apis_core::TimePeriod {
                        start_date_time: start,
                        end_date_time: end,
                    }),
                    _ => None,
                },
                last_update: row.get("last_update"),
            },
            product_offering: None, // Load separately if needed
        })
        .collect();

    Ok(catalogs)
}

/// Get a catalog by ID
pub async fn get_catalog_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<Catalog> {
    let row = sqlx::query(
        "SELECT id, name, description, version, lifecycle_status,
         href, last_update, valid_for_start, valid_for_end
         FROM catalogs WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;

    let row = row.ok_or_else(|| TmfError::NotFound(format!("Catalog with id {} not found", id)))?;

    Ok(Catalog {
        base: tmf_apis_core::BaseEntity {
            id: row.get("id"),
            href: row.get("href"),
            name: row.get("name"),
            description: row.get("description"),
            version: row.get("version"),
            lifecycle_status: parse_lifecycle_status(&row.get::<String, _>("lifecycle_status")),
            valid_for: match (
                row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("valid_for_start"),
                row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("valid_for_end"),
            ) {
                (Some(start), end) => Some(tmf_apis_core::TimePeriod {
                    start_date_time: start,
                    end_date_time: end,
                }),
                _ => None,
            },
            last_update: row.get("last_update"),
        },
        product_offering: None,
    })
}

/// Create a new catalog
pub async fn create_catalog(
    pool: &Pool<Postgres>,
    request: CreateCatalogRequest,
) -> TmfResult<Catalog> {
    let id = Uuid::new_v4();
    let lifecycle_status = lifecycle_status_to_string(&request.lifecycle_status);

    sqlx::query(
        "INSERT INTO catalogs (id, name, description, version, lifecycle_status)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&lifecycle_status)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_catalog_by_id(pool, id).await
}

/// Get all product offerings
pub async fn get_product_offerings(pool: &Pool<Postgres>) -> TmfResult<Vec<ProductOffering>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, lifecycle_status,
         href, last_update, valid_for_start, valid_for_end,
         is_sellable, is_bundle
         FROM product_offerings ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let offerings = rows
        .into_iter()
        .map(|row| ProductOffering {
            base: tmf_apis_core::BaseEntity {
                id: row.get("id"),
                href: row.get("href"),
                name: row.get("name"),
                description: row.get("description"),
                version: row.get("version"),
                lifecycle_status: parse_lifecycle_status(&row.get::<String, _>("lifecycle_status")),
                valid_for: match (
                    row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("valid_for_start"),
                    row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("valid_for_end"),
                ) {
                    (Some(start), end) => Some(tmf_apis_core::TimePeriod {
                        start_date_time: start,
                        end_date_time: end,
                    }),
                    _ => None,
                },
                last_update: row.get("last_update"),
            },
            is_sellable: row.get("is_sellable"),
            is_bundle: row.get("is_bundle"),
            product_specification: None,
            bundled_product_offering: None,
            product_offering_price: None,
        })
        .collect();

    Ok(offerings)
}

/// Create a new product offering
pub async fn create_product_offering(
    pool: &Pool<Postgres>,
    request: CreateProductOfferingRequest,
) -> TmfResult<ProductOffering> {
    let id = Uuid::new_v4();
    let lifecycle_status = lifecycle_status_to_string(&request.lifecycle_status);

    sqlx::query(
        "INSERT INTO product_offerings (id, name, description, version, lifecycle_status, is_sellable, is_bundle)
         VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&lifecycle_status)
    .bind(request.is_sellable)
    .bind(request.is_bundle)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    let row = sqlx::query(
        "SELECT id, name, description, version, lifecycle_status,
         href, last_update, valid_for_start, valid_for_end,
         is_sellable, is_bundle
         FROM product_offerings WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(ProductOffering {
        base: tmf_apis_core::BaseEntity {
            id: row.get("id"),
            href: row.get("href"),
            name: row.get("name"),
            description: row.get("description"),
            version: row.get("version"),
            lifecycle_status: parse_lifecycle_status(&row.get::<String, _>("lifecycle_status")),
            valid_for: match (
                row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("valid_for_start"),
                row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("valid_for_end"),
            ) {
                (Some(start), end) => Some(tmf_apis_core::TimePeriod {
                    start_date_time: start,
                    end_date_time: end,
                }),
                _ => None,
            },
            last_update: row.get("last_update"),
        },
        is_sellable: row.get("is_sellable"),
        is_bundle: row.get("is_bundle"),
        product_specification: None,
        bundled_product_offering: None,
        product_offering_price: None,
    })
}

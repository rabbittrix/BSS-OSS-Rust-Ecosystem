//! Request handlers for TMF620 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all catalogs
#[utoipa::path(
    get,
    path = "/tmf-api/productCatalogManagement/v4/catalog",
    responses(
        (status = 200, description = "List of catalogs", body = Vec<Catalog>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF620"
)]
pub async fn get_catalogs(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_catalogs(pool.get_ref()).await {
        Ok(catalogs) => Ok(HttpResponse::Ok().json(catalogs)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get catalog by ID
#[utoipa::path(
    get,
    path = "/tmf-api/productCatalogManagement/v4/catalog/{id}",
    responses(
        (status = 200, description = "Catalog found", body = Catalog),
        (status = 404, description = "Catalog not found"),
        (status = 400, description = "Invalid catalog ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Catalog ID (UUID)")
    ),
    tag = "TMF620"
)]
pub async fn get_catalog_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid catalog ID format. Expected UUID."
            })));
        }
    };

    match db::get_catalog_by_id(pool.get_ref(), id).await {
        Ok(catalog) => Ok(HttpResponse::Ok().json(catalog)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new catalog
#[utoipa::path(
    post,
    path = "/tmf-api/productCatalogManagement/v4/catalog",
    request_body = CreateCatalogRequest,
    responses(
        (status = 201, description = "Catalog created", body = Catalog),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF620"
)]
pub async fn create_catalog(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateCatalogRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_catalog(pool.get_ref(), body.into_inner()).await {
        Ok(catalog) => Ok(HttpResponse::Created().json(catalog)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get all product offerings
#[utoipa::path(
    get,
    path = "/tmf-api/productCatalogManagement/v4/productOffering",
    responses(
        (status = 200, description = "List of product offerings", body = Vec<ProductOffering>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF620"
)]
pub async fn get_product_offerings(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_product_offerings(pool.get_ref()).await {
        Ok(offerings) => Ok(HttpResponse::Ok().json(offerings)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new product offering
#[utoipa::path(
    post,
    path = "/tmf-api/productCatalogManagement/v4/productOffering",
    request_body = CreateProductOfferingRequest,
    responses(
        (status = 201, description = "Product offering created", body = ProductOffering),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF620"
)]
pub async fn create_product_offering(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateProductOfferingRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_product_offering(pool.get_ref(), body.into_inner()).await {
        Ok(offering) => Ok(HttpResponse::Created().json(offering)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

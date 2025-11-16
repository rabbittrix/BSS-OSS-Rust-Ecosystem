//! Request handlers for TMF637 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all product inventories
#[utoipa::path(
    get,
    path = "/tmf-api/productInventoryManagement/v4/productInventory",
    responses(
        (status = 200, description = "List of product inventories", body = Vec<ProductInventory>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF637"
)]
pub async fn get_inventories(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_inventories(pool.get_ref()).await {
        Ok(inventories) => Ok(HttpResponse::Ok().json(inventories)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get product inventory by ID
#[utoipa::path(
    get,
    path = "/tmf-api/productInventoryManagement/v4/productInventory/{id}",
    responses(
        (status = 200, description = "Product inventory found", body = ProductInventory),
        (status = 404, description = "Product inventory not found"),
        (status = 400, description = "Invalid inventory ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Product Inventory ID (UUID)")
    ),
    tag = "TMF637"
)]
pub async fn get_inventory_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid product inventory ID format. Expected UUID."
            })));
        }
    };

    match db::get_inventory_by_id(pool.get_ref(), id).await {
        Ok(inventory) => Ok(HttpResponse::Ok().json(inventory)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new product inventory
#[utoipa::path(
    post,
    path = "/tmf-api/productInventoryManagement/v4/productInventory",
    request_body = CreateProductInventoryRequest,
    responses(
        (status = 201, description = "Product inventory created", body = ProductInventory),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF637"
)]
pub async fn create_inventory(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateProductInventoryRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_inventory(pool.get_ref(), body.into_inner()).await {
        Ok(inventory) => Ok(HttpResponse::Created().json(inventory)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}


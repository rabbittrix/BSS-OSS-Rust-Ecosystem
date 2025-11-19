//! Request handlers for TMF638 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all service inventories
#[utoipa::path(
    get,
    path = "/tmf-api/serviceInventoryManagement/v4/serviceInventory",
    responses(
        (status = 200, description = "List of service inventories", body = Vec<ServiceInventory>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF638"
)]
pub async fn get_service_inventories(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_service_inventories(pool.get_ref()).await {
        Ok(inventories) => Ok(HttpResponse::Ok().json(inventories)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get service inventory by ID
#[utoipa::path(
    get,
    path = "/tmf-api/serviceInventoryManagement/v4/serviceInventory/{id}",
    responses(
        (status = 200, description = "Service inventory found", body = ServiceInventory),
        (status = 404, description = "Service inventory not found"),
        (status = 400, description = "Invalid inventory ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Service Inventory ID (UUID)")
    ),
    tag = "TMF638"
)]
pub async fn get_service_inventory_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid service inventory ID format. Expected UUID."
            })));
        }
    };

    match db::get_service_inventory_by_id(pool.get_ref(), id).await {
        Ok(inventory) => Ok(HttpResponse::Ok().json(inventory)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new service inventory
#[utoipa::path(
    post,
    path = "/tmf-api/serviceInventoryManagement/v4/serviceInventory",
    request_body = CreateServiceInventoryRequest,
    responses(
        (status = 201, description = "Service inventory created", body = ServiceInventory),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF638"
)]
pub async fn create_service_inventory(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateServiceInventoryRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_service_inventory(pool.get_ref(), body.into_inner()).await {
        Ok(inventory) => Ok(HttpResponse::Created().json(inventory)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}


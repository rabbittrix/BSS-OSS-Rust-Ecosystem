//! Request handlers for TMF639 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all resource inventories
#[utoipa::path(
    get,
    path = "/tmf-api/resourceInventoryManagement/v4/resourceInventory",
    responses(
        (status = 200, description = "List of resource inventories", body = Vec<ResourceInventory>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF639"
)]
pub async fn get_resource_inventories(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_resource_inventories(pool.get_ref()).await {
        Ok(inventories) => Ok(HttpResponse::Ok().json(inventories)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get resource inventory by ID
#[utoipa::path(
    get,
    path = "/tmf-api/resourceInventoryManagement/v4/resourceInventory/{id}",
    responses(
        (status = 200, description = "Resource inventory found", body = ResourceInventory),
        (status = 404, description = "Resource inventory not found"),
        (status = 400, description = "Invalid inventory ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Resource Inventory ID (UUID)")
    ),
    tag = "TMF639"
)]
pub async fn get_resource_inventory_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid resource inventory ID format. Expected UUID."
            })));
        }
    };

    match db::get_resource_inventory_by_id(pool.get_ref(), id).await {
        Ok(inventory) => Ok(HttpResponse::Ok().json(inventory)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new resource inventory
#[utoipa::path(
    post,
    path = "/tmf-api/resourceInventoryManagement/v4/resourceInventory",
    request_body = CreateResourceInventoryRequest,
    responses(
        (status = 201, description = "Resource inventory created", body = ResourceInventory),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF639"
)]
pub async fn create_resource_inventory(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateResourceInventoryRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_resource_inventory(pool.get_ref(), body.into_inner()).await {
        Ok(inventory) => Ok(HttpResponse::Created().json(inventory)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

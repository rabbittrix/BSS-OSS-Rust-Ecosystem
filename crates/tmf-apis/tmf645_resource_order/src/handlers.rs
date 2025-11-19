//! Request handlers for TMF645 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all resource orders
#[utoipa::path(
    get,
    path = "/tmf-api/resourceOrderingManagement/v4/resourceOrder",
    responses(
        (status = 200, description = "List of resource orders", body = Vec<ResourceOrder>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF645"
)]
pub async fn get_resource_orders(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_resource_orders(pool.get_ref()).await {
        Ok(orders) => Ok(HttpResponse::Ok().json(orders)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get resource order by ID
#[utoipa::path(
    get,
    path = "/tmf-api/resourceOrderingManagement/v4/resourceOrder/{id}",
    responses(
        (status = 200, description = "Resource order found", body = ResourceOrder),
        (status = 404, description = "Resource order not found"),
        (status = 400, description = "Invalid order ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Resource Order ID (UUID)")
    ),
    tag = "TMF645"
)]
pub async fn get_resource_order_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid resource order ID format. Expected UUID."
            })));
        }
    };

    match db::get_resource_order_by_id(pool.get_ref(), id).await {
        Ok(order) => Ok(HttpResponse::Ok().json(order)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new resource order
#[utoipa::path(
    post,
    path = "/tmf-api/resourceOrderingManagement/v4/resourceOrder",
    request_body = CreateResourceOrderRequest,
    responses(
        (status = 201, description = "Resource order created", body = ResourceOrder),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF645"
)]
pub async fn create_resource_order(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateResourceOrderRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_resource_order(pool.get_ref(), body.into_inner()).await {
        Ok(order) => Ok(HttpResponse::Created().json(order)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

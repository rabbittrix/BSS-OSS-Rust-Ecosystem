//! Request handlers for TMF641 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all service orders
#[utoipa::path(
    get,
    path = "/tmf-api/serviceOrderingManagement/v4/serviceOrder",
    responses(
        (status = 200, description = "List of service orders", body = Vec<ServiceOrder>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF641"
)]
pub async fn get_service_orders(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_service_orders(pool.get_ref()).await {
        Ok(orders) => Ok(HttpResponse::Ok().json(orders)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get service order by ID
#[utoipa::path(
    get,
    path = "/tmf-api/serviceOrderingManagement/v4/serviceOrder/{id}",
    responses(
        (status = 200, description = "Service order found", body = ServiceOrder),
        (status = 404, description = "Service order not found"),
        (status = 400, description = "Invalid order ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Service Order ID (UUID)")
    ),
    tag = "TMF641"
)]
pub async fn get_service_order_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid service order ID format. Expected UUID."
            })));
        }
    };

    match db::get_service_order_by_id(pool.get_ref(), id).await {
        Ok(order) => Ok(HttpResponse::Ok().json(order)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new service order
#[utoipa::path(
    post,
    path = "/tmf-api/serviceOrderingManagement/v4/serviceOrder",
    request_body = CreateServiceOrderRequest,
    responses(
        (status = 201, description = "Service order created", body = ServiceOrder),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF641"
)]
pub async fn create_service_order(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateServiceOrderRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_service_order(pool.get_ref(), body.into_inner()).await {
        Ok(order) => Ok(HttpResponse::Created().json(order)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

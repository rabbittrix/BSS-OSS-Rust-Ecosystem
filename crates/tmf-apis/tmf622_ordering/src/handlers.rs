//! Request handlers for TMF622 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all product orders
#[utoipa::path(
    get,
    path = "/tmf-api/productOrderingManagement/v4/productOrder",
    responses(
        (status = 200, description = "List of product orders", body = Vec<ProductOrder>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF622"
)]
pub async fn get_orders(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_orders(pool.get_ref()).await {
        Ok(orders) => Ok(HttpResponse::Ok().json(orders)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get product order by ID
#[utoipa::path(
    get,
    path = "/tmf-api/productOrderingManagement/v4/productOrder/{id}",
    responses(
        (status = 200, description = "Product order found", body = ProductOrder),
        (status = 404, description = "Product order not found"),
        (status = 400, description = "Invalid order ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Product Order ID (UUID)")
    ),
    tag = "TMF622"
)]
pub async fn get_order_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid product order ID format. Expected UUID."
            })));
        }
    };

    match db::get_order_by_id(pool.get_ref(), id).await {
        Ok(order) => Ok(HttpResponse::Ok().json(order)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new product order
#[utoipa::path(
    post,
    path = "/tmf-api/productOrderingManagement/v4/productOrder",
    request_body = CreateProductOrderRequest,
    responses(
        (status = 201, description = "Product order created", body = ProductOrder),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF622"
)]
pub async fn create_order(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateProductOrderRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_order(pool.get_ref(), body.into_inner()).await {
        Ok(order) => Ok(HttpResponse::Created().json(order)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

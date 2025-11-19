//! Request handlers for TMF635 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all usage records
#[utoipa::path(
    get,
    path = "/tmf-api/usageManagement/v4/usage",
    responses(
        (status = 200, description = "List of usage records", body = Vec<Usage>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF635"
)]
pub async fn get_usages(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_usages(pool.get_ref()).await {
        Ok(usages) => Ok(HttpResponse::Ok().json(usages)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get usage by ID
#[utoipa::path(
    get,
    path = "/tmf-api/usageManagement/v4/usage/{id}",
    responses(
        (status = 200, description = "Usage record found", body = Usage),
        (status = 404, description = "Usage record not found"),
        (status = 400, description = "Invalid usage ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Usage ID (UUID)")
    ),
    tag = "TMF635"
)]
pub async fn get_usage_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid usage ID format. Expected UUID."
            })));
        }
    };

    match db::get_usage_by_id(pool.get_ref(), id).await {
        Ok(usage) => Ok(HttpResponse::Ok().json(usage)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new usage record
#[utoipa::path(
    post,
    path = "/tmf-api/usageManagement/v4/usage",
    request_body = CreateUsageRequest,
    responses(
        (status = 201, description = "Usage record created", body = Usage),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF635"
)]
pub async fn create_usage(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateUsageRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_usage(pool.get_ref(), body.into_inner()).await {
        Ok(usage) => Ok(HttpResponse::Created().json(usage)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}


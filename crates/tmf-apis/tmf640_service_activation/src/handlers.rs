//! Request handlers for TMF640 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all service activations
#[utoipa::path(
    get,
    path = "/tmf-api/serviceActivationAndConfiguration/v4/serviceActivation",
    responses(
        (status = 200, description = "List of service activations", body = Vec<ServiceActivation>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF640"
)]
pub async fn get_service_activations(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_service_activations(pool.get_ref()).await {
        Ok(activations) => Ok(HttpResponse::Ok().json(activations)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get service activation by ID
#[utoipa::path(
    get,
    path = "/tmf-api/serviceActivationAndConfiguration/v4/serviceActivation/{id}",
    responses(
        (status = 200, description = "Service activation found", body = ServiceActivation),
        (status = 404, description = "Service activation not found"),
        (status = 400, description = "Invalid activation ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Service Activation ID (UUID)")
    ),
    tag = "TMF640"
)]
pub async fn get_service_activation_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid service activation ID format. Expected UUID."
            })));
        }
    };

    match db::get_service_activation_by_id(pool.get_ref(), id).await {
        Ok(activation) => Ok(HttpResponse::Ok().json(activation)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new service activation
#[utoipa::path(
    post,
    path = "/tmf-api/serviceActivationAndConfiguration/v4/serviceActivation",
    request_body = CreateServiceActivationRequest,
    responses(
        (status = 201, description = "Service activation created", body = ServiceActivation),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF640"
)]
pub async fn create_service_activation(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateServiceActivationRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_service_activation(pool.get_ref(), body.into_inner()).await {
        Ok(activation) => Ok(HttpResponse::Created().json(activation)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}


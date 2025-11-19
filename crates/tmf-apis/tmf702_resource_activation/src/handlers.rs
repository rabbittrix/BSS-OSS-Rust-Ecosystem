//! Request handlers for TMF702 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all resource activations
#[utoipa::path(
    get,
    path = "/tmf-api/resourceActivationAndConfiguration/v4/resourceActivation",
    responses(
        (status = 200, description = "List of resource activations", body = Vec<ResourceActivation>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF702"
)]
pub async fn get_resource_activations(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_resource_activations(pool.get_ref()).await {
        Ok(activations) => Ok(HttpResponse::Ok().json(activations)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get resource activation by ID
#[utoipa::path(
    get,
    path = "/tmf-api/resourceActivationAndConfiguration/v4/resourceActivation/{id}",
    responses(
        (status = 200, description = "Resource activation found", body = ResourceActivation),
        (status = 404, description = "Resource activation not found"),
        (status = 400, description = "Invalid activation ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Resource Activation ID (UUID)")
    ),
    tag = "TMF702"
)]
pub async fn get_resource_activation_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid resource activation ID format. Expected UUID."
            })));
        }
    };

    match db::get_resource_activation_by_id(pool.get_ref(), id).await {
        Ok(activation) => Ok(HttpResponse::Ok().json(activation)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new resource activation
#[utoipa::path(
    post,
    path = "/tmf-api/resourceActivationAndConfiguration/v4/resourceActivation",
    request_body = CreateResourceActivationRequest,
    responses(
        (status = 201, description = "Resource activation created", body = ResourceActivation),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF702"
)]
pub async fn create_resource_activation(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateResourceActivationRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_resource_activation(pool.get_ref(), body.into_inner()).await {
        Ok(activation) => Ok(HttpResponse::Created().json(activation)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}


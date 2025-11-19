//! Request handlers for TMF669 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all identities
#[utoipa::path(
    get,
    path = "/tmf-api/identityManagement/v4/identity",
    responses(
        (status = 200, description = "List of identities", body = Vec<Identity>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF669"
)]
pub async fn get_identities(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_identities(pool.get_ref()).await {
        Ok(identities) => Ok(HttpResponse::Ok().json(identities)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get identity by ID
#[utoipa::path(
    get,
    path = "/tmf-api/identityManagement/v4/identity/{id}",
    responses(
        (status = 200, description = "Identity found", body = Identity),
        (status = 404, description = "Identity not found"),
        (status = 400, description = "Invalid identity ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Identity ID (UUID)")
    ),
    tag = "TMF669"
)]
pub async fn get_identity_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid identity ID format. Expected UUID."
            })));
        }
    };

    match db::get_identity_by_id(pool.get_ref(), id).await {
        Ok(identity) => Ok(HttpResponse::Ok().json(identity)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new identity
#[utoipa::path(
    post,
    path = "/tmf-api/identityManagement/v4/identity",
    request_body = CreateIdentityRequest,
    responses(
        (status = 201, description = "Identity created", body = Identity),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF669"
)]
pub async fn create_identity(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateIdentityRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_identity(pool.get_ref(), body.into_inner()).await {
        Ok(identity) => Ok(HttpResponse::Created().json(identity)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}


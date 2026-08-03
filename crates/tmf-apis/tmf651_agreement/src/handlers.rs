//! Request handlers for TMF651 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all agreements
#[utoipa::path(
    get,
    path = "/tmf-api/agreementManagement/v4/agreement",
    responses(
        (status = 200, description = "List of agreements", body = Vec<Agreement>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF651"
)]
pub async fn get_agreements(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_agreements(pool.get_ref()).await {
        Ok(agreements) => Ok(HttpResponse::Ok().json(agreements)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get agreement by ID
#[utoipa::path(
    get,
    path = "/tmf-api/agreementManagement/v4/agreement/{id}",
    responses(
        (status = 200, description = "Agreement found", body = Agreement),
        (status = 404, description = "Agreement not found"),
        (status = 400, description = "Invalid agreement ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Agreement ID (UUID)")
    ),
    tag = "TMF651"
)]
pub async fn get_agreement_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid agreement ID format. Expected UUID."
            })));
        }
    };

    match db::get_agreement_by_id(pool.get_ref(), id).await {
        Ok(Some(agreement)) => Ok(HttpResponse::Ok().json(agreement)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Agreement with id {} not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create an agreement
#[utoipa::path(
    post,
    path = "/tmf-api/agreementManagement/v4/agreement",
    request_body = CreateAgreementRequest,
    responses(
        (status = 201, description = "Agreement created", body = Agreement),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF651"
)]
pub async fn create_agreement(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateAgreementRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_agreement(pool.get_ref(), body.into_inner()).await {
        Ok(agreement) => Ok(HttpResponse::Created().json(agreement)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Update an agreement
#[utoipa::path(
    patch,
    path = "/tmf-api/agreementManagement/v4/agreement/{id}",
    request_body = UpdateAgreementRequest,
    responses(
        (status = 200, description = "Agreement updated", body = Agreement),
        (status = 404, description = "Agreement not found"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Agreement ID (UUID)")
    ),
    tag = "TMF651"
)]
pub async fn update_agreement(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateAgreementRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid agreement ID format. Expected UUID."
            })));
        }
    };

    match db::update_agreement(pool.get_ref(), id, body.into_inner()).await {
        Ok(agreement) => Ok(HttpResponse::Ok().json(agreement)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Delete an agreement
#[utoipa::path(
    delete,
    path = "/tmf-api/agreementManagement/v4/agreement/{id}",
    responses(
        (status = 204, description = "Agreement deleted"),
        (status = 404, description = "Agreement not found"),
        (status = 400, description = "Invalid agreement ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Agreement ID (UUID)")
    ),
    tag = "TMF651"
)]
pub async fn delete_agreement(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid agreement ID format. Expected UUID."
            })));
        }
    };

    match db::delete_agreement(pool.get_ref(), id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

//! Request handlers for TMF668 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all party roles
#[utoipa::path(
    get,
    path = "/tmf-api/partyRoleManagement/v4/partyRole",
    responses(
        (status = 200, description = "List of party roles", body = Vec<PartyRole>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF668"
)]
pub async fn get_party_roles(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_party_roles(pool.get_ref()).await {
        Ok(party_roles) => Ok(HttpResponse::Ok().json(party_roles)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get party role by ID
#[utoipa::path(
    get,
    path = "/tmf-api/partyRoleManagement/v4/partyRole/{id}",
    responses(
        (status = 200, description = "Party role found", body = PartyRole),
        (status = 404, description = "Party role not found"),
        (status = 400, description = "Invalid party role ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Party Role ID (UUID)")
    ),
    tag = "TMF668"
)]
pub async fn get_party_role_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid party role ID format. Expected UUID."
            })));
        }
    };

    match db::get_party_role_by_id(pool.get_ref(), id).await {
        Ok(party_role) => Ok(HttpResponse::Ok().json(party_role)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new party role
#[utoipa::path(
    post,
    path = "/tmf-api/partyRoleManagement/v4/partyRole",
    request_body = CreatePartyRoleRequest,
    responses(
        (status = 201, description = "Party role created", body = PartyRole),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF668"
)]
pub async fn create_party_role(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreatePartyRoleRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_party_role(pool.get_ref(), body.into_inner()).await {
        Ok(party_role) => Ok(HttpResponse::Created().json(party_role)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

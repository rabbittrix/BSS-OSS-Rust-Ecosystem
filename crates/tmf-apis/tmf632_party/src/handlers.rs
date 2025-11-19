//! Request handlers for TMF632 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all parties
#[utoipa::path(
    get,
    path = "/tmf-api/partyManagement/v4/party",
    responses(
        (status = 200, description = "List of parties", body = Vec<Party>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF632"
)]
pub async fn get_parties(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_parties(pool.get_ref()).await {
        Ok(parties) => Ok(HttpResponse::Ok().json(parties)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get party by ID
#[utoipa::path(
    get,
    path = "/tmf-api/partyManagement/v4/party/{id}",
    responses(
        (status = 200, description = "Party found", body = Party),
        (status = 404, description = "Party not found"),
        (status = 400, description = "Invalid party ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Party ID (UUID)")
    ),
    tag = "TMF632"
)]
pub async fn get_party_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid party ID format. Expected UUID."
            })));
        }
    };

    match db::get_party_by_id(pool.get_ref(), id).await {
        Ok(party) => Ok(HttpResponse::Ok().json(party)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new party
#[utoipa::path(
    post,
    path = "/tmf-api/partyManagement/v4/party",
    request_body = CreatePartyRequest,
    responses(
        (status = 201, description = "Party created", body = Party),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF632"
)]
pub async fn create_party(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreatePartyRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_party(pool.get_ref(), body.into_inner()).await {
        Ok(party) => Ok(HttpResponse::Created().json(party)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

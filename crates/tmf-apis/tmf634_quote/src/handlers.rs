//! Request handlers for TMF634 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all quotes
#[utoipa::path(
    get,
    path = "/tmf-api/quoteManagement/v4/quote",
    responses(
        (status = 200, description = "List of quotes", body = Vec<Quote>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF634"
)]
pub async fn get_quotes(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_quotes(pool.get_ref()).await {
        Ok(quotes) => Ok(HttpResponse::Ok().json(quotes)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get quote by ID
#[utoipa::path(
    get,
    path = "/tmf-api/quoteManagement/v4/quote/{id}",
    responses(
        (status = 200, description = "Quote found", body = Quote),
        (status = 404, description = "Quote not found"),
        (status = 400, description = "Invalid quote ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Quote ID (UUID)")
    ),
    tag = "TMF634"
)]
pub async fn get_quote_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid quote ID format. Expected UUID."
            })));
        }
    };

    match db::get_quote_by_id(pool.get_ref(), id).await {
        Ok(Some(quote)) => Ok(HttpResponse::Ok().json(quote)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Quote with id {} not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new quote
#[utoipa::path(
    post,
    path = "/tmf-api/quoteManagement/v4/quote",
    request_body = CreateQuoteRequest,
    responses(
        (status = 201, description = "Quote created", body = Quote),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF634"
)]
pub async fn create_quote(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateQuoteRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_quote(pool.get_ref(), body.into_inner()).await {
        Ok(quote) => Ok(HttpResponse::Created().json(quote)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Update a quote
#[utoipa::path(
    patch,
    path = "/tmf-api/quoteManagement/v4/quote/{id}",
    request_body = UpdateQuoteRequest,
    responses(
        (status = 200, description = "Quote updated", body = Quote),
        (status = 404, description = "Quote not found"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Quote ID (UUID)")
    ),
    tag = "TMF634"
)]
pub async fn update_quote(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateQuoteRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid quote ID format. Expected UUID."
            })));
        }
    };

    match db::update_quote(pool.get_ref(), id, body.into_inner()).await {
        Ok(quote) => Ok(HttpResponse::Ok().json(quote)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Delete a quote
#[utoipa::path(
    delete,
    path = "/tmf-api/quoteManagement/v4/quote/{id}",
    responses(
        (status = 204, description = "Quote deleted"),
        (status = 404, description = "Quote not found"),
        (status = 400, description = "Invalid quote ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Quote ID (UUID)")
    ),
    tag = "TMF634"
)]
pub async fn delete_quote(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid quote ID format. Expected UUID."
            })));
        }
    };

    match db::delete_quote(pool.get_ref(), id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

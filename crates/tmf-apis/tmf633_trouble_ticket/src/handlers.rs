//! Request handlers for TMF633 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all trouble tickets
#[utoipa::path(
    get,
    path = "/tmf-api/troubleTicket/v4/troubleTicket",
    responses(
        (status = 200, description = "List of trouble tickets", body = Vec<TroubleTicket>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF633"
)]
pub async fn get_trouble_tickets(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_trouble_tickets(pool.get_ref()).await {
        Ok(tickets) => Ok(HttpResponse::Ok().json(tickets)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get trouble ticket by ID
#[utoipa::path(
    get,
    path = "/tmf-api/troubleTicket/v4/troubleTicket/{id}",
    responses(
        (status = 200, description = "Trouble ticket found", body = TroubleTicket),
        (status = 404, description = "Trouble ticket not found"),
        (status = 400, description = "Invalid trouble ticket ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Trouble Ticket ID (UUID)")
    ),
    tag = "TMF633"
)]
pub async fn get_trouble_ticket_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid trouble ticket ID format. Expected UUID."
            })));
        }
    };

    match db::get_trouble_ticket_by_id(pool.get_ref(), id).await {
        Ok(Some(ticket)) => Ok(HttpResponse::Ok().json(ticket)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Trouble ticket not found"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new trouble ticket
#[utoipa::path(
    post,
    path = "/tmf-api/troubleTicket/v4/troubleTicket",
    request_body = CreateTroubleTicketRequest,
    responses(
        (status = 201, description = "Trouble ticket created", body = TroubleTicket),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF633"
)]
pub async fn create_trouble_ticket(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateTroubleTicketRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_trouble_ticket(pool.get_ref(), body.into_inner()).await {
        Ok(ticket) => Ok(HttpResponse::Created().json(ticket)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Update a trouble ticket
#[utoipa::path(
    patch,
    path = "/tmf-api/troubleTicket/v4/troubleTicket/{id}",
    request_body = UpdateTroubleTicketRequest,
    responses(
        (status = 200, description = "Trouble ticket updated", body = TroubleTicket),
        (status = 404, description = "Trouble ticket not found"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Trouble Ticket ID (UUID)")
    ),
    tag = "TMF633"
)]
pub async fn update_trouble_ticket(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateTroubleTicketRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid trouble ticket ID format. Expected UUID."
            })));
        }
    };

    match db::update_trouble_ticket(pool.get_ref(), id, body.into_inner()).await {
        Ok(ticket) => Ok(HttpResponse::Ok().json(ticket)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Delete a trouble ticket
#[utoipa::path(
    delete,
    path = "/tmf-api/troubleTicket/v4/troubleTicket/{id}",
    responses(
        (status = 204, description = "Trouble ticket deleted"),
        (status = 404, description = "Trouble ticket not found"),
        (status = 400, description = "Invalid trouble ticket ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Trouble Ticket ID (UUID)")
    ),
    tag = "TMF633"
)]
pub async fn delete_trouble_ticket(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid trouble ticket ID format. Expected UUID."
            })));
        }
    };

    match db::delete_trouble_ticket(pool.get_ref(), id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

//! Request handlers for TMF688 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all appointments
#[utoipa::path(
    get,
    path = "/tmf-api/appointmentManagement/v4/appointment",
    responses(
        (status = 200, description = "List of appointments", body = Vec<Appointment>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF688"
)]
pub async fn get_appointments(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_appointments(pool.get_ref()).await {
        Ok(appointments) => Ok(HttpResponse::Ok().json(appointments)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get appointment by ID
#[utoipa::path(
    get,
    path = "/tmf-api/appointmentManagement/v4/appointment/{id}",
    responses(
        (status = 200, description = "Appointment found", body = Appointment),
        (status = 404, description = "Appointment not found"),
        (status = 400, description = "Invalid appointment ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Appointment ID (UUID)")
    ),
    tag = "TMF688"
)]
pub async fn get_appointment_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid appointment ID format. Expected UUID."
            })));
        }
    };

    match db::get_appointment_by_id(pool.get_ref(), id).await {
        Ok(appointment) => Ok(HttpResponse::Ok().json(appointment)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new appointment
#[utoipa::path(
    post,
    path = "/tmf-api/appointmentManagement/v4/appointment",
    request_body = CreateAppointmentRequest,
    responses(
        (status = 201, description = "Appointment created", body = Appointment),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF688"
)]
pub async fn create_appointment(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateAppointmentRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_appointment(pool.get_ref(), body.into_inner()).await {
        Ok(appointment) => Ok(HttpResponse::Created().json(appointment)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

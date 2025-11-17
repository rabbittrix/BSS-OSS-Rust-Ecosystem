//! Request handlers for TMF678 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all customer bills
#[utoipa::path(
    get,
    path = "/tmf-api/customerBillManagement/v4/customerBill",
    responses(
        (status = 200, description = "List of customer bills", body = Vec<CustomerBill>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF678"
)]
pub async fn get_bills(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_bills(pool.get_ref()).await {
        Ok(bills) => Ok(HttpResponse::Ok().json(bills)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get customer bill by ID
#[utoipa::path(
    get,
    path = "/tmf-api/customerBillManagement/v4/customerBill/{id}",
    responses(
        (status = 200, description = "Customer bill found", body = CustomerBill),
        (status = 404, description = "Customer bill not found"),
        (status = 400, description = "Invalid bill ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Customer Bill ID (UUID)")
    ),
    tag = "TMF678"
)]
pub async fn get_bill_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid customer bill ID format. Expected UUID."
            })));
        }
    };

    match db::get_bill_by_id(pool.get_ref(), id).await {
        Ok(bill) => Ok(HttpResponse::Ok().json(bill)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new customer bill
#[utoipa::path(
    post,
    path = "/tmf-api/customerBillManagement/v4/customerBill",
    request_body = CreateCustomerBillRequest,
    responses(
        (status = 201, description = "Customer bill created", body = CustomerBill),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF678"
)]
pub async fn create_bill(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateCustomerBillRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_bill(pool.get_ref(), body.into_inner()).await {
        Ok(bill) => Ok(HttpResponse::Created().json(bill)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}


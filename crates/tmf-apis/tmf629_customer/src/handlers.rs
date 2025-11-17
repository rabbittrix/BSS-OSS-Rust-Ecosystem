//! Request handlers for TMF629 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all customers
#[utoipa::path(
    get,
    path = "/tmf-api/customerManagement/v4/customer",
    responses(
        (status = 200, description = "List of customers", body = Vec<Customer>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF629"
)]
pub async fn get_customers(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_customers(pool.get_ref()).await {
        Ok(customers) => Ok(HttpResponse::Ok().json(customers)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get customer by ID
#[utoipa::path(
    get,
    path = "/tmf-api/customerManagement/v4/customer/{id}",
    responses(
        (status = 200, description = "Customer found", body = Customer),
        (status = 404, description = "Customer not found"),
        (status = 400, description = "Invalid customer ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Customer ID (UUID)")
    ),
    tag = "TMF629"
)]
pub async fn get_customer_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid customer ID format. Expected UUID."
            })));
        }
    };

    match db::get_customer_by_id(pool.get_ref(), id).await {
        Ok(customer) => Ok(HttpResponse::Ok().json(customer)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new customer
#[utoipa::path(
    post,
    path = "/tmf-api/customerManagement/v4/customer",
    request_body = CreateCustomerRequest,
    responses(
        (status = 201, description = "Customer created", body = Customer),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF629"
)]
pub async fn create_customer(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateCustomerRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_customer(pool.get_ref(), body.into_inner()).await {
        Ok(customer) => Ok(HttpResponse::Created().json(customer)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}


//! Request handlers for TMF666 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all billing accounts
#[utoipa::path(
    get,
    path = "/tmf-api/accountManagement/v4/billingAccount",
    responses(
        (status = 200, description = "List of billing accounts", body = Vec<BillingAccount>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF666"
)]
pub async fn get_billing_accounts(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_billing_accounts(pool.get_ref()).await {
        Ok(accounts) => Ok(HttpResponse::Ok().json(accounts)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get billing account by ID
#[utoipa::path(
    get,
    path = "/tmf-api/accountManagement/v4/billingAccount/{id}",
    responses(
        (status = 200, description = "Billing account found", body = BillingAccount),
        (status = 404, description = "Billing account not found"),
        (status = 400, description = "Invalid billing account ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Billing Account ID (UUID)")
    ),
    tag = "TMF666"
)]
pub async fn get_billing_account_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid billing account ID format. Expected UUID."
            })));
        }
    };

    match db::get_billing_account_by_id(pool.get_ref(), id).await {
        Ok(Some(account)) => Ok(HttpResponse::Ok().json(account)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Billing account with id {} not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a billing account
#[utoipa::path(
    post,
    path = "/tmf-api/accountManagement/v4/billingAccount",
    request_body = CreateBillingAccountRequest,
    responses(
        (status = 201, description = "Billing account created", body = BillingAccount),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF666"
)]
pub async fn create_billing_account(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateBillingAccountRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_billing_account(pool.get_ref(), body.into_inner()).await {
        Ok(account) => Ok(HttpResponse::Created().json(account)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Update a billing account
#[utoipa::path(
    patch,
    path = "/tmf-api/accountManagement/v4/billingAccount/{id}",
    request_body = UpdateBillingAccountRequest,
    responses(
        (status = 200, description = "Billing account updated", body = BillingAccount),
        (status = 404, description = "Billing account not found"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Billing Account ID (UUID)")
    ),
    tag = "TMF666"
)]
pub async fn update_billing_account(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateBillingAccountRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid billing account ID format. Expected UUID."
            })));
        }
    };

    match db::update_billing_account(pool.get_ref(), id, body.into_inner()).await {
        Ok(account) => Ok(HttpResponse::Ok().json(account)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Delete a billing account
#[utoipa::path(
    delete,
    path = "/tmf-api/accountManagement/v4/billingAccount/{id}",
    responses(
        (status = 204, description = "Billing account deleted"),
        (status = 404, description = "Billing account not found"),
        (status = 400, description = "Invalid billing account ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Billing Account ID (UUID)")
    ),
    tag = "TMF666"
)]
pub async fn delete_billing_account(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid billing account ID format. Expected UUID."
            })));
        }
    };

    match db::delete_billing_account(pool.get_ref(), id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get all party accounts
#[utoipa::path(
    get,
    path = "/tmf-api/accountManagement/v4/partyAccount",
    responses(
        (status = 200, description = "List of party accounts", body = Vec<PartyAccount>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF666"
)]
pub async fn get_party_accounts(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_party_accounts(pool.get_ref()).await {
        Ok(accounts) => Ok(HttpResponse::Ok().json(accounts)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get party account by ID
#[utoipa::path(
    get,
    path = "/tmf-api/accountManagement/v4/partyAccount/{id}",
    responses(
        (status = 200, description = "Party account found", body = PartyAccount),
        (status = 404, description = "Party account not found"),
        (status = 400, description = "Invalid party account ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Party Account ID (UUID)")
    ),
    tag = "TMF666"
)]
pub async fn get_party_account_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid party account ID format. Expected UUID."
            })));
        }
    };

    match db::get_party_account_by_id(pool.get_ref(), id).await {
        Ok(Some(account)) => Ok(HttpResponse::Ok().json(account)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Party account with id {} not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a party account
#[utoipa::path(
    post,
    path = "/tmf-api/accountManagement/v4/partyAccount",
    request_body = CreatePartyAccountRequest,
    responses(
        (status = 201, description = "Party account created", body = PartyAccount),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF666"
)]
pub async fn create_party_account(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreatePartyAccountRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_party_account(pool.get_ref(), body.into_inner()).await {
        Ok(account) => Ok(HttpResponse::Created().json(account)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Update a party account
#[utoipa::path(
    patch,
    path = "/tmf-api/accountManagement/v4/partyAccount/{id}",
    request_body = UpdatePartyAccountRequest,
    responses(
        (status = 200, description = "Party account updated", body = PartyAccount),
        (status = 404, description = "Party account not found"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Party Account ID (UUID)")
    ),
    tag = "TMF666"
)]
pub async fn update_party_account(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdatePartyAccountRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid party account ID format. Expected UUID."
            })));
        }
    };

    match db::update_party_account(pool.get_ref(), id, body.into_inner()).await {
        Ok(account) => Ok(HttpResponse::Ok().json(account)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Delete a party account
#[utoipa::path(
    delete,
    path = "/tmf-api/accountManagement/v4/partyAccount/{id}",
    responses(
        (status = 204, description = "Party account deleted"),
        (status = 404, description = "Party account not found"),
        (status = 400, description = "Invalid party account ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Party Account ID (UUID)")
    ),
    tag = "TMF666"
)]
pub async fn delete_party_account(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid party account ID format. Expected UUID."
            })));
        }
    };

    match db::delete_party_account(pool.get_ref(), id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

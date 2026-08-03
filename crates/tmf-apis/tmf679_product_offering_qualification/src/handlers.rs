//! Request handlers for TMF679 Product Offering Qualification

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use uuid::Uuid;

#[utoipa::path(get, path = "/tmf-api/productOfferingQualification/v4/productOfferingQualification",
    responses((status = 200, description = "List", body = Vec<ProductOfferingQualification>)), tag = "TMF679")]
pub async fn list_qualifications(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    match db::list_qualifications(pool.get_ref()).await {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(get, path = "/tmf-api/productOfferingQualification/v4/productOfferingQualification/{id}",
    params(("id" = String, Path, description = "ID")),
    responses((status = 200, description = "Found", body = ProductOfferingQualification)), tag = "TMF679")]
pub async fn get_qualification_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(u) => u,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error":"Invalid UUID"})))
        }
    };
    match db::get_qualification_by_id(pool.get_ref(), id).await {
        Ok(Some(item)) => Ok(HttpResponse::Ok().json(item)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({"error":"Not found"}))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(post, path = "/tmf-api/productOfferingQualification/v4/productOfferingQualification",
    request_body = CreateProductOfferingQualificationRequest,
    responses((status = 201, description = "Created", body = ProductOfferingQualification)), tag = "TMF679")]
pub async fn create_qualification(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateProductOfferingQualificationRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    match db::create_qualification(pool.get_ref(), body.into_inner()).await {
        Ok(item) => Ok(HttpResponse::Created().json(item)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

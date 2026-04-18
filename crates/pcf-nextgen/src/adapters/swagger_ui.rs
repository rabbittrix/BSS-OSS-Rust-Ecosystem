//! Embedded Swagger UI (no Docker). Serves the workspace OpenAPI YAML from the same origin
//! so the browser can load the spec and use "Try it out" without CORS issues.

use actix_web::web::{self, ServiceConfig};
use actix_web::{HttpRequest, HttpResponse, Responder};

const OPENAPI_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../openapi/pcf-nextgen-sba.yaml"
));

const SWAGGER_INDEX_HTML: &str = include_str!("swagger_index.html");

/// OpenAPI 3 YAML (same file as repo `openapi/pcf-nextgen-sba.yaml`).
pub async fn openapi_yaml(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/yaml; charset=utf-8")
        .body(OPENAPI_YAML)
}

/// Swagger UI (loads CDN assets; spec is same-origin).
pub async fn swagger_index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(SWAGGER_INDEX_HTML)
}

pub fn configure_swagger(cfg: &mut ServiceConfig) {
    cfg.route(
        "/openapi/pcf-nextgen-sba.yaml",
        web::get().to(openapi_yaml),
    );
    cfg.route("/swagger-ui/", web::get().to(swagger_index));
}

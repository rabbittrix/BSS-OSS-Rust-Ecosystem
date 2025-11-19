//! API route configuration for TMF635

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF635 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/usageManagement/v4")
            .service(
                web::resource("/usage")
                    .route(web::get().to(get_usages))
                    .route(web::post().to(create_usage)),
            )
            .service(web::resource("/usage/{id}").route(web::get().to(get_usage_by_id))),
    );
}

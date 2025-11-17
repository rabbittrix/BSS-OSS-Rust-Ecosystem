//! API route configuration for TMF679

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF679 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/customerUsageManagement/v4")
            .service(
                web::resource("/customerUsage")
                    .route(web::get().to(get_usages))
                    .route(web::post().to(create_usage)),
            )
            .service(web::resource("/customerUsage/{id}").route(web::get().to(get_usage_by_id))),
    );
}

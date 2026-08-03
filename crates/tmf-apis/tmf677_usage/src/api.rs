//! API route configuration for TMF677

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF677 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/usageConsumptionManagement/v4")
            .service(
                web::resource("/usageConsumption")
                    .route(web::get().to(get_usages))
                    .route(web::post().to(create_usage)),
            )
            .service(web::resource("/usageConsumption/{id}").route(web::get().to(get_usage_by_id))),
    );
}

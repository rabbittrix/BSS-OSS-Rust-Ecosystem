//! API route configuration for TMF641

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF641 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/serviceOrderingManagement/v4")
            .service(
                web::resource("/serviceOrder")
                    .route(web::get().to(get_service_orders))
                    .route(web::post().to(create_service_order)),
            )
            .service(
                web::resource("/serviceOrder/{id}").route(web::get().to(get_service_order_by_id)),
            ),
    );
}

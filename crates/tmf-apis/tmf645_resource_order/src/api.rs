//! API route configuration for TMF645

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF645 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/resourceOrderingManagement/v4")
            .service(
                web::resource("/resourceOrder")
                    .route(web::get().to(get_resource_orders))
                    .route(web::post().to(create_resource_order)),
            )
            .service(
                web::resource("/resourceOrder/{id}").route(web::get().to(get_resource_order_by_id)),
            ),
    );
}

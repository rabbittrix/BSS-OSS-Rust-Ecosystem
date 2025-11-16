//! API route configuration for TMF622

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF622 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/productOrderingManagement/v4")
            .service(
                web::resource("/productOrder")
                    .route(web::get().to(get_orders))
                    .route(web::post().to(create_order)),
            )
            .service(web::resource("/productOrder/{id}").route(web::get().to(get_order_by_id))),
    );
}

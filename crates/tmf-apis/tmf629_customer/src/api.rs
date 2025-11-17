//! API route configuration for TMF629

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF629 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/customerManagement/v4")
            .service(
                web::resource("/customer")
                    .route(web::get().to(get_customers))
                    .route(web::post().to(create_customer)),
            )
            .service(web::resource("/customer/{id}").route(web::get().to(get_customer_by_id))),
    );
}

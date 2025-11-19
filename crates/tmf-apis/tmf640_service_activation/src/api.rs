//! API route configuration for TMF640

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF640 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/serviceActivationAndConfiguration/v4")
            .service(
                web::resource("/serviceActivation")
                    .route(web::get().to(get_service_activations))
                    .route(web::post().to(create_service_activation)),
            )
            .service(
                web::resource("/serviceActivation/{id}")
                    .route(web::get().to(get_service_activation_by_id)),
            ),
    );
}

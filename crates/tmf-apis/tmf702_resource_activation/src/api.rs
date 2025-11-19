//! API route configuration for TMF702

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF702 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/resourceActivationAndConfiguration/v4")
            .service(
                web::resource("/resourceActivation")
                    .route(web::get().to(get_resource_activations))
                    .route(web::post().to(create_resource_activation)),
            )
            .service(
                web::resource("/resourceActivation/{id}")
                    .route(web::get().to(get_resource_activation_by_id)),
            ),
    );
}

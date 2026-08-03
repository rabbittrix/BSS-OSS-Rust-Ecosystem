//! API route configuration for TMF646

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF646 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/appointmentManagement/v4")
            .service(
                web::resource("/appointment")
                    .route(web::get().to(get_appointments))
                    .route(web::post().to(create_appointment)),
            )
            .service(
                web::resource("/appointment/{id}").route(web::get().to(get_appointment_by_id)),
            ),
    );
}

//! API route configuration for TMF642

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF642 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/alarmManagement/v4")
            .service(
                web::resource("/alarm")
                    .route(web::get().to(get_alarms))
                    .route(web::post().to(create_alarm)),
            )
            .service(
                web::resource("/alarm/{id}")
                    .route(web::get().to(get_alarm_by_id))
                    .route(web::patch().to(update_alarm))
                    .route(web::delete().to(delete_alarm)),
            ),
    );
}

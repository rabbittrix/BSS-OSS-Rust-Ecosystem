//! API route configuration for TMF638

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF638 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/serviceInventoryManagement/v4")
            .service(
                web::resource("/serviceInventory")
                    .route(web::get().to(get_service_inventories))
                    .route(web::post().to(create_service_inventory)),
            )
            .service(
                web::resource("/serviceInventory/{id}")
                    .route(web::get().to(get_service_inventory_by_id)),
            ),
    );
}

//! API route configuration for TMF639

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF639 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/resourceInventoryManagement/v4")
            .service(
                web::resource("/resourceInventory")
                    .route(web::get().to(get_resource_inventories))
                    .route(web::post().to(create_resource_inventory)),
            )
            .service(
                web::resource("/resourceInventory/{id}")
                    .route(web::get().to(get_resource_inventory_by_id)),
            ),
    );
}

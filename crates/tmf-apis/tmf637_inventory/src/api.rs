//! API route configuration for TMF637

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF637 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/productInventoryManagement/v4")
            .service(
                web::resource("/productInventory")
                    .route(web::get().to(get_inventories))
                    .route(web::post().to(create_inventory)),
            )
            .service(
                web::resource("/productInventory/{id}").route(web::get().to(get_inventory_by_id)),
            ),
    );
}

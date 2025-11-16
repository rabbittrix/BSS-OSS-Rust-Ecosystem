//! BSS/OSS Rust - Main Application
//!
//! This is the main entry point for the BSS/OSS Rust ecosystem.
//! It initializes the TMF620 Product Catalog Management API server
//! with OpenAPI documentation, authentication, and database connectivity.

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result as ActixResult};
use bss_oss_utils::init_logger;
use tmf620_catalog::{db::init_db, models::*};
use tmf622_ordering::models::{
    CreateOrderItemRequest, CreateProductOrderRequest, OrderItem, OrderState,
    ProductOfferingRef as Tmf622ProductOfferingRef, ProductOrder,
    ProductSpecificationRef as Tmf622ProductSpecificationRef, RelatedParty as Tmf622RelatedParty,
};
use tmf637_inventory::models::{
    CreateProductInventoryRequest, CreateRelatedPartyRequest as Tmf637CreateRelatedPartyRequest,
    InventoryState, ProductInventory, ProductOfferingRef as Tmf637ProductOfferingRef,
    ProductSpecificationRef as Tmf637ProductSpecificationRef, RelatedParty as Tmf637RelatedParty,
};
use tmf_apis_core::{BaseEntity, LifecycleStatus, TimePeriod};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // TMF620
        tmf620_catalog::handlers::get_catalogs,
        tmf620_catalog::handlers::get_catalog_by_id,
        tmf620_catalog::handlers::create_catalog,
        tmf620_catalog::handlers::get_product_offerings,
        tmf620_catalog::handlers::create_product_offering,
        // TMF622
        tmf622_ordering::handlers::get_orders,
        tmf622_ordering::handlers::get_order_by_id,
        tmf622_ordering::handlers::create_order,
        // TMF637
        tmf637_inventory::handlers::get_inventories,
        tmf637_inventory::handlers::get_inventory_by_id,
        tmf637_inventory::handlers::create_inventory,
    ),
    components(schemas(
        // TMF620
        Catalog,
        ProductOffering,
        CreateCatalogRequest,
        CreateProductOfferingRequest,
        ProductOfferingRef,
        ProductOfferingPrice,
        ProductSpecificationRef,
        PriceType,
        Money,
        // TMF622
        ProductOrder,
        CreateProductOrderRequest,
        OrderItem,
        CreateOrderItemRequest,
        OrderState,
        Tmf622ProductOfferingRef,
        Tmf622ProductSpecificationRef,
        Tmf622RelatedParty,
        // TMF637
        ProductInventory,
        CreateProductInventoryRequest,
        Tmf637CreateRelatedPartyRequest,
        InventoryState,
        Tmf637ProductOfferingRef,
        Tmf637ProductSpecificationRef,
        Tmf637RelatedParty,
        // Common
        BaseEntity,
        LifecycleStatus,
        TimePeriod,
    )),
    tags(
        (name = "TMF620", description = "Product Catalog Management API"),
        (name = "TMF622", description = "Product Ordering Management API"),
        (name = "TMF637", description = "Product Inventory Management API")
    ),
    info(
        title = "BSS/OSS Rust - TM Forum Open APIs",
        description = "TM Forum Open API implementation for BSS/OSS ecosystem (TMF620, TMF622, TMF637)",
        version = "0.1.0",
        contact(
            name = "Roberto de Souza",
            email = "rabbittrix@hotmail.com"
        )
    )
)]
struct ApiDoc;

/// Redirect /swagger-ui to /swagger-ui/
async fn redirect_to_swagger() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Found()
        .append_header(("Location", "/swagger-ui/"))
        .finish())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    init_logger();

    log::info!("🚀 Starting BSS/OSS Rust ecosystem...");

    // Initialize database connection pool
    let pool = init_db().await;
    log::info!("✅ Database connection established");

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    log::info!(
        "🌐 TM Forum APIs will be available at http://{}:{}",
        host,
        port
    );
    log::info!("   - TMF620: Product Catalog Management");
    log::info!("   - TMF622: Product Ordering Management");
    log::info!("   - TMF637: Product Inventory Management");
    log::info!(
        "📚 Swagger UI will be available at http://{}:{}/swagger-ui",
        host,
        port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .route("/swagger-ui", web::get().to(redirect_to_swagger))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .configure(tmf620_catalog::api::configure_routes)
            .configure(tmf622_ordering::api::configure_routes)
            .configure(tmf637_inventory::api::configure_routes)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

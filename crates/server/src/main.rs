//! BSS/OSS Rust - Main Application
//!
//! This is the main entry point for the BSS/OSS Rust ecosystem.
//! It initializes the TMF620 Product Catalog Management API server
//! with OpenAPI documentation, authentication, and database connectivity.

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result as ActixResult};
use bss_oss_utils::init_logger;
use tmf620_catalog::{db::init_db, handlers::*, models::*};
use tmf_apis_core::{BaseEntity, LifecycleStatus, TimePeriod};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_catalogs,
        get_catalog_by_id,
        create_catalog,
        get_product_offerings,
        create_product_offering,
    ),
    components(schemas(
        Catalog,
        ProductOffering,
        CreateCatalogRequest,
        CreateProductOfferingRequest,
        BaseEntity,
        LifecycleStatus,
        TimePeriod,
        ProductOfferingRef,
        ProductOfferingPrice,
        ProductSpecificationRef,
        PriceType,
        Money,
    )),
    tags(
        (name = "TMF620", description = "Product Catalog Management API")
    ),
    info(
        title = "BSS/OSS Rust - TMF620 Product Catalog Management API",
        description = "TM Forum Open API implementation for Product Catalog Management",
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
        "🌐 TMF620 API will be available at http://{}:{}",
        host,
        port
    );
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
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

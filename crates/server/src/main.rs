//! BSS/OSS Rust - Main Application
//!
//! This is the main entry point for the BSS/OSS Rust ecosystem.
//! It initializes the TMF620 Product Catalog Management API server
//! with OpenAPI documentation, authentication, and database connectivity.

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result as ActixResult};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use bss_oss_utils::init_logger;
use graphql_api::create_schema;
use prometheus::{Counter, Gauge, Histogram, Registry, TextEncoder};
use tmf620_catalog::{db::init_db, models::*};
use tmf622_ordering::models::{
    CreateOrderItemRequest, CreateProductOrderRequest, OrderItem, OrderState,
    ProductOfferingRef as Tmf622ProductOfferingRef, ProductOrder,
    ProductSpecificationRef as Tmf622ProductSpecificationRef, RelatedParty as Tmf622RelatedParty,
};
use tmf629_customer::models::{
    AccountRef as Tmf629AccountRef, Characteristic as Tmf629Characteristic,
    ContactMedium as Tmf629ContactMedium, CreateContactMediumRequest, CreateCustomerRequest,
    CreateRelatedPartyRequest as Tmf629CreateRelatedPartyRequest, Customer, CustomerState,
    RelatedParty as Tmf629RelatedParty,
};
use tmf632_party::models::{
    AccountRef as Tmf632AccountRef, Characteristic as Tmf632Characteristic,
    ContactMedium as Tmf632ContactMedium, CreateAccountRefRequest as Tmf632CreateAccountRefRequest,
    CreateCharacteristicRequest as Tmf632CreateCharacteristicRequest,
    CreateContactMediumRequest as Tmf632CreateContactMediumRequest, CreatePartyRequest,
    CreateRelatedPartyRequest as Tmf632CreateRelatedPartyRequest, Party, PartyState, PartyType,
    RelatedParty as Tmf632RelatedParty,
};
use tmf633_trouble_ticket::models::{
    CreateTroubleTicketRequest, TroubleTicket, TroubleTicketPriority, TroubleTicketStatus,
    TroubleTicketType, UpdateTroubleTicketRequest,
};
use tmf634_quote::models::{
    CreateQuoteRequest, Quote, QuoteItem, QuoteState, RelatedParty as Tmf634RelatedParty,
    UpdateQuoteRequest,
};
use tmf635_usage::models::{
    CreateRelatedPartyRequest as Tmf635CreateRelatedPartyRequest, CreateUsageRequest,
    ProductOfferingRef as Tmf635ProductOfferingRef, RatingRef, RelatedParty as Tmf635RelatedParty,
    Usage, UsageState as Tmf635UsageState,
};
use tmf637_inventory::models::{
    CreateProductInventoryRequest, CreateRelatedPartyRequest as Tmf637CreateRelatedPartyRequest,
    InventoryState, ProductInventory, ProductOfferingRef as Tmf637ProductOfferingRef,
    ProductSpecificationRef as Tmf637ProductSpecificationRef, RelatedParty as Tmf637RelatedParty,
};
use tmf638_service_inventory::models::{
    CreateRelatedPartyRequest as Tmf638CreateRelatedPartyRequest, CreateServiceInventoryRequest,
    RelatedParty as Tmf638RelatedParty, ServiceInventory, ServiceInventoryState,
    ServiceRef as Tmf638ServiceRef, ServiceSpecificationRef as Tmf638ServiceSpecificationRef,
};
use tmf639_resource_inventory::models::{
    CreateRelatedPartyRequest as Tmf639CreateRelatedPartyRequest, CreateResourceInventoryRequest,
    RelatedParty as Tmf639RelatedParty, ResourceInventory, ResourceInventoryState,
    ResourceRef as Tmf639ResourceRef, ResourceSpecificationRef as Tmf639ResourceSpecificationRef,
};
use tmf640_service_activation::models::{
    ConfigurationParameter as Tmf640ConfigurationParameter,
    CreateConfigurationParameterRequest as Tmf640CreateConfigurationParameterRequest,
    CreateServiceActivationRequest, ServiceActivation, ServiceActivationState,
    ServiceOrderRef as Tmf640ServiceOrderRef, ServiceRef as Tmf640ServiceRef,
};
use tmf641_service_order::models::{
    CreateRelatedPartyRequest as Tmf641CreateRelatedPartyRequest, CreateServiceOrderItemRequest,
    CreateServiceOrderRequest, RelatedParty as Tmf641RelatedParty, ServiceOrder, ServiceOrderItem,
    ServiceOrderState, ServiceRef as Tmf641ServiceRef,
    ServiceSpecificationRef as Tmf641ServiceSpecificationRef,
};
use tmf642_alarm::models::{
    Alarm, AlarmSeverity, AlarmState, AlarmType, CreateAlarmRequest,
    ResourceRef as Tmf642ResourceRef, UpdateAlarmRequest,
};
use tmf645_resource_order::models::{
    CreateRelatedPartyRequest as Tmf645CreateRelatedPartyRequest, CreateResourceOrderItemRequest,
    CreateResourceOrderRequest, RelatedParty as Tmf645RelatedParty, ResourceOrder,
    ResourceOrderItem, ResourceOrderState, ResourceRef as Tmf645ResourceRef,
    ResourceSpecificationRef as Tmf645ResourceSpecificationRef,
};
use tmf656_slice::models::{
    CreateNetworkFunctionRefRequest, CreateNetworkSliceRequest, CreateSLAParametersRequest,
    NetworkFunctionRef, NetworkSlice, SLAParameters, SliceState, SliceType,
    UpdateNetworkSliceRequest,
};
use tmf668_party_role::models::{
    ContactMedium as Tmf668ContactMedium,
    CreateContactMediumRequest as Tmf668CreateContactMediumRequest, CreatePartyRoleRequest,
    CreateRelatedPartyRequest as Tmf668CreateRelatedPartyRequest, PartyRole, PartyRoleState,
    RelatedParty as Tmf668RelatedParty,
};
use tmf669_identity::models::{
    CreateCredentialRequest, CreateIdentityRequest, Credential, CredentialType, Identity,
    IdentityState, PartyRef as Tmf669PartyRef,
};
use tmf678_billing::models::{
    BillItem, BillState, CreateBillItemRequest, CreateCustomerBillRequest,
    CreateRelatedPartyRequest as Tmf678CreateRelatedPartyRequest, CustomerBill, Money as BillMoney,
    ProductOfferingRef as Tmf678ProductOfferingRef, RelatedParty as Tmf678RelatedParty,
};
use tmf679_usage::models::{
    CreateCustomerUsageRequest, CreateRelatedPartyRequest as Tmf679CreateRelatedPartyRequest,
    CustomerUsage, RelatedParty as Tmf679RelatedParty, UsageState as Tmf679UsageState,
};
use tmf688_appointment::models::{
    Appointment, AppointmentState, ContactMedium as Tmf688ContactMedium, CreateAppointmentRequest,
    CreateContactMediumRequest as Tmf688CreateContactMediumRequest,
    CreateRelatedPartyRequest as Tmf688CreateRelatedPartyRequest,
    RelatedParty as Tmf688RelatedParty,
};
use tmf702_resource_activation::models::{
    ConfigurationParameter as Tmf702ConfigurationParameter,
    CreateConfigurationParameterRequest as Tmf702CreateConfigurationParameterRequest,
    CreateResourceActivationRequest, ResourceActivation, ResourceActivationState,
    ResourceRef as Tmf702ResourceRef, ServiceActivationRef as Tmf702ServiceActivationRef,
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
        // TMF629
        tmf629_customer::handlers::get_customers,
        tmf629_customer::handlers::get_customer_by_id,
        tmf629_customer::handlers::create_customer,
        // TMF678
        tmf678_billing::handlers::get_bills,
        tmf678_billing::handlers::get_bill_by_id,
        tmf678_billing::handlers::create_bill,
        // TMF679
        tmf679_usage::handlers::get_usages,
        tmf679_usage::handlers::get_usage_by_id,
        tmf679_usage::handlers::create_usage,
        // TMF688
        tmf688_appointment::handlers::get_appointments,
        tmf688_appointment::handlers::get_appointment_by_id,
        tmf688_appointment::handlers::create_appointment,
        // TMF641
        tmf641_service_order::handlers::get_service_orders,
        tmf641_service_order::handlers::get_service_order_by_id,
        tmf641_service_order::handlers::create_service_order,
        // TMF638
        tmf638_service_inventory::handlers::get_service_inventories,
        tmf638_service_inventory::handlers::get_service_inventory_by_id,
        tmf638_service_inventory::handlers::create_service_inventory,
        // TMF640
        tmf640_service_activation::handlers::get_service_activations,
        tmf640_service_activation::handlers::get_service_activation_by_id,
        tmf640_service_activation::handlers::create_service_activation,
        // TMF702
        tmf702_resource_activation::handlers::get_resource_activations,
        tmf702_resource_activation::handlers::get_resource_activation_by_id,
        tmf702_resource_activation::handlers::create_resource_activation,
        // TMF639
        tmf639_resource_inventory::handlers::get_resource_inventories,
        tmf639_resource_inventory::handlers::get_resource_inventory_by_id,
        tmf639_resource_inventory::handlers::create_resource_inventory,
        // TMF645
        tmf645_resource_order::handlers::get_resource_orders,
        tmf645_resource_order::handlers::get_resource_order_by_id,
        tmf645_resource_order::handlers::create_resource_order,
        // TMF635
        tmf635_usage::handlers::get_usages,
        tmf635_usage::handlers::get_usage_by_id,
        tmf635_usage::handlers::create_usage,
        // TMF668
        tmf668_party_role::handlers::get_party_roles,
        tmf668_party_role::handlers::get_party_role_by_id,
        tmf668_party_role::handlers::create_party_role,
        // TMF632
        tmf632_party::handlers::get_parties,
        tmf632_party::handlers::get_party_by_id,
        tmf632_party::handlers::create_party,
        // TMF669
        tmf669_identity::handlers::get_identities,
        tmf669_identity::handlers::get_identity_by_id,
        tmf669_identity::handlers::create_identity,
        // TMF642
        tmf642_alarm::handlers::get_alarms,
        tmf642_alarm::handlers::get_alarm_by_id,
        tmf642_alarm::handlers::create_alarm,
        tmf642_alarm::handlers::update_alarm,
        tmf642_alarm::handlers::delete_alarm,
        // TMF656
        tmf656_slice::handlers::get_network_slices,
        tmf656_slice::handlers::get_network_slice_by_id,
        tmf656_slice::handlers::create_network_slice,
        tmf656_slice::handlers::update_network_slice,
        tmf656_slice::handlers::delete_network_slice,
        // TMF633
        tmf633_trouble_ticket::handlers::get_trouble_tickets,
        tmf633_trouble_ticket::handlers::get_trouble_ticket_by_id,
        tmf633_trouble_ticket::handlers::create_trouble_ticket,
        tmf633_trouble_ticket::handlers::update_trouble_ticket,
        tmf633_trouble_ticket::handlers::delete_trouble_ticket,
        // TMF634
        tmf634_quote::handlers::get_quotes,
        tmf634_quote::handlers::get_quote_by_id,
        tmf634_quote::handlers::create_quote,
        tmf634_quote::handlers::update_quote,
        tmf634_quote::handlers::delete_quote,
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
        // TMF629
        Customer,
        CreateCustomerRequest,
        CreateContactMediumRequest,
        Tmf629CreateRelatedPartyRequest,
        CustomerState,
        Tmf629AccountRef,
        Tmf629Characteristic,
        Tmf629ContactMedium,
        Tmf629RelatedParty,
        // TMF678
        CustomerBill,
        CreateCustomerBillRequest,
        CreateBillItemRequest,
        Tmf678CreateRelatedPartyRequest,
        BillState,
        BillItem,
        BillMoney,
        Tmf678ProductOfferingRef,
        Tmf678RelatedParty,
        // TMF679
        CustomerUsage,
        CreateCustomerUsageRequest,
        Tmf679CreateRelatedPartyRequest,
        Tmf679UsageState,
        Tmf679RelatedParty,
        // TMF688
        Appointment,
        CreateAppointmentRequest,
        Tmf688CreateContactMediumRequest,
        Tmf688CreateRelatedPartyRequest,
        AppointmentState,
        Tmf688ContactMedium,
        Tmf688RelatedParty,
        // TMF641
        ServiceOrder,
        CreateServiceOrderRequest,
        ServiceOrderItem,
        CreateServiceOrderItemRequest,
        ServiceOrderState,
        Tmf641ServiceSpecificationRef,
        Tmf641ServiceRef,
        Tmf641RelatedParty,
        Tmf641CreateRelatedPartyRequest,
        // TMF638
        ServiceInventory,
        CreateServiceInventoryRequest,
        ServiceInventoryState,
        Tmf638ServiceSpecificationRef,
        Tmf638ServiceRef,
        Tmf638RelatedParty,
        Tmf638CreateRelatedPartyRequest,
        // TMF640
        ServiceActivation,
        CreateServiceActivationRequest,
        ServiceActivationState,
        Tmf640ServiceRef,
        Tmf640ServiceOrderRef,
        Tmf640ConfigurationParameter,
        Tmf640CreateConfigurationParameterRequest,
        // TMF702
        ResourceActivation,
        CreateResourceActivationRequest,
        ResourceActivationState,
        Tmf702ResourceRef,
        Tmf702ServiceActivationRef,
        Tmf702ConfigurationParameter,
        Tmf702CreateConfigurationParameterRequest,
        // TMF639
        ResourceInventory,
        CreateResourceInventoryRequest,
        ResourceInventoryState,
        Tmf639ResourceSpecificationRef,
        Tmf639ResourceRef,
        Tmf639RelatedParty,
        Tmf639CreateRelatedPartyRequest,
        // TMF645
        ResourceOrder,
        CreateResourceOrderRequest,
        ResourceOrderItem,
        CreateResourceOrderItemRequest,
        ResourceOrderState,
        Tmf645ResourceSpecificationRef,
        Tmf645ResourceRef,
        Tmf645RelatedParty,
        Tmf645CreateRelatedPartyRequest,
        // TMF635
        Usage,
        CreateUsageRequest,
        Tmf635UsageState,
        Tmf635ProductOfferingRef,
        RatingRef,
        Tmf635RelatedParty,
        Tmf635CreateRelatedPartyRequest,
        // TMF668
        PartyRole,
        CreatePartyRoleRequest,
        PartyRoleState,
        Tmf668ContactMedium,
        Tmf668CreateContactMediumRequest,
        Tmf668RelatedParty,
        Tmf668CreateRelatedPartyRequest,
        // TMF632
        Party,
        CreatePartyRequest,
        PartyState,
        PartyType,
        Tmf632ContactMedium,
        Tmf632CreateContactMediumRequest,
        Tmf632RelatedParty,
        Tmf632CreateRelatedPartyRequest,
        Tmf632AccountRef,
        Tmf632CreateAccountRefRequest,
        Tmf632Characteristic,
        Tmf632CreateCharacteristicRequest,
        // TMF669
        Identity,
        CreateIdentityRequest,
        IdentityState,
        Credential,
        CreateCredentialRequest,
        CredentialType,
        Tmf669PartyRef,
        // TMF642
        Alarm,
        CreateAlarmRequest,
        UpdateAlarmRequest,
        AlarmState,
        AlarmSeverity,
        AlarmType,
        Tmf642ResourceRef,
        // TMF656
        NetworkSlice,
        CreateNetworkSliceRequest,
        UpdateNetworkSliceRequest,
        SliceState,
        SliceType,
        SLAParameters,
        CreateSLAParametersRequest,
        NetworkFunctionRef,
        CreateNetworkFunctionRefRequest,
        // TMF633
        TroubleTicket,
        CreateTroubleTicketRequest,
        UpdateTroubleTicketRequest,
        TroubleTicketStatus,
        TroubleTicketPriority,
        TroubleTicketType,
        // TMF634
        Quote,
        CreateQuoteRequest,
        UpdateQuoteRequest,
        QuoteState,
        QuoteItem,
        Tmf634RelatedParty,
        // Common
        BaseEntity,
        LifecycleStatus,
        TimePeriod,
    )),
    tags(
        (name = "TMF620", description = "Product Catalog Management API"),
        (name = "TMF622", description = "Product Ordering Management API"),
        (name = "TMF637", description = "Product Inventory Management API"),
        (name = "TMF629", description = "Customer Management API"),
        (name = "TMF678", description = "Customer Bill Management API"),
        (name = "TMF679", description = "Customer Usage Management API"),
        (name = "TMF688", description = "Appointment Management API"),
        (name = "TMF641", description = "Service Order Management API"),
        (name = "TMF638", description = "Service Inventory Management API"),
        (name = "TMF640", description = "Service Activation & Configuration API"),
        (name = "TMF702", description = "Resource Activation & Configuration API"),
        (name = "TMF639", description = "Resource Inventory Management API"),
        (name = "TMF645", description = "Resource Order Management API"),
        (name = "TMF635", description = "Usage Management API"),
        (name = "TMF668", description = "Party Role Management API"),
        (name = "TMF632", description = "Party Management API"),
        (name = "TMF669", description = "Identity & Credential Management API"),
        (name = "TMF642", description = "Alarm Management API"),
        (name = "TMF656", description = "Slice Management API"),
        (name = "TMF633", description = "Trouble Ticket Management API"),
        (name = "TMF634", description = "Quote Management API")
    ),
    info(
        title = "BSS/OSS Rust - TM Forum Open APIs",
        description = "TM Forum Open API implementation for BSS/OSS ecosystem (TMF620, TMF622, TMF637, TMF629, TMF678, TMF679, TMF688, TMF641, TMF638, TMF640, TMF702, TMF639, TMF645, TMF635, TMF668, TMF632, TMF669, TMF642, TMF656, TMF633, TMF634)",
        version = "0.3.0",
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

/// GraphQL handler
async fn graphql_handler(
    schema: web::Data<
        async_graphql::Schema<
            graphql_api::resolvers::QueryRoot,
            async_graphql::EmptyMutation,
            async_graphql::EmptySubscription,
        >,
    >,
    pool: web::Data<sqlx::PgPool>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    req = req.data(pool.get_ref().clone());
    schema.execute(req).await.into()
}

/// Health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "bss-oss-rust",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Readiness probe endpoint - checks database connectivity
async fn readiness_check(pool: web::Data<sqlx::PgPool>) -> HttpResponse {
    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "ready",
            "database": "connected"
        })),
        Err(e) => {
            log::error!("Database health check failed: {}", e);
            HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "not_ready",
                "database": "disconnected",
                "error": e.to_string()
            }))
        }
    }
}

/// Liveness probe endpoint
async fn liveness_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "alive"
    }))
}

/// Prometheus metrics endpoint
async fn metrics_handler(registry: web::Data<Registry>) -> HttpResponse {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    match encoder.encode_to_string(&metric_families) {
        Ok(metrics) => HttpResponse::Ok()
            .content_type("text/plain; version=0.0.4")
            .body(metrics),
        Err(e) => {
            log::error!("Failed to encode metrics: {}", e);
            HttpResponse::InternalServerError().body("Failed to encode metrics")
        }
    }
}

/// GraphQL Playground handler
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            r#"
<!DOCTYPE html>
<html>
<head>
    <title>GraphQL Playground</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
    <link rel="shortcut icon" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/favicon.png" />
    <script src="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
</head>
<body>
    <div id="root">
        <style>
            body {
                margin: 0;
                background-color: rgb(23, 42, 58);
                font-family: Open Sans, sans-serif;
                overflow: hidden;
            }
            #root {
                width: 100vw;
                height: 100vh;
            }
        </style>
        <script>
            window.addEventListener('load', function (event) {
                GraphQLPlayground.init(document.getElementById('root'), {
                    endpoint: '/graphql'
                })
            });
        </script>
    </div>
</body>
</html>
            "#,
        )
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
    log::info!("   - TMF629: Customer Management");
    log::info!("   - TMF678: Customer Bill Management");
    log::info!("   - TMF679: Customer Usage Management");
    log::info!("   - TMF688: Appointment Management");
    log::info!("   - TMF641: Service Order Management");
    log::info!("   - TMF638: Service Inventory Management");
    log::info!("   - TMF640: Service Activation & Configuration");
    log::info!("   - TMF702: Resource Activation & Configuration");
    log::info!("   - TMF639: Resource Inventory Management");
    log::info!("   - TMF645: Resource Order Management");
    log::info!("   - TMF635: Usage Management");
    log::info!("   - TMF668: Party Role Management");
    log::info!("   - TMF632: Party Management");
    log::info!("   - TMF669: Identity & Credential Management");
    log::info!("   - TMF642: Alarm Management");
    log::info!("   - TMF656: Slice Management");
    log::info!("   - TMF633: Trouble Ticket Management");
    log::info!("   - TMF634: Quote Management");
    log::info!("   - GraphQL: http://{}:{}/graphql", host, port);
    log::info!(
        "📚 Swagger UI will be available at http://{}:{}/swagger-ui",
        host,
        port
    );

    // Create GraphQL schema
    let schema = create_schema();

    // Initialize Prometheus metrics registry
    let registry = Registry::new();

    // Register common metrics
    let http_requests_total = Counter::with_opts(
        prometheus::Opts::new("http_requests_total", "Total number of HTTP requests")
            .namespace("bss_oss"),
    )
    .unwrap();
    let http_request_duration_seconds = Histogram::with_opts(
        prometheus::HistogramOpts::new(
            "http_request_duration_seconds",
            "HTTP request duration in seconds",
        )
        .namespace("bss_oss")
        .buckets(vec![
            0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ]),
    )
    .unwrap();
    let active_connections = Gauge::with_opts(
        prometheus::Opts::new("active_connections", "Number of active connections")
            .namespace("bss_oss"),
    )
    .unwrap();

    registry
        .register(Box::new(http_requests_total.clone()))
        .unwrap();
    registry
        .register(Box::new(http_request_duration_seconds.clone()))
        .unwrap();
    registry
        .register(Box::new(active_connections.clone()))
        .unwrap();

    let registry_data = web::Data::new(registry.clone());

    let server = HttpServer::new(move || {
        let schema = schema.clone();
        let registry = registry_data.clone();
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(schema))
            .app_data(registry.clone())
            .wrap(Logger::default())
            .route("/health", web::get().to(health_check))
            .route("/ready", web::get().to(readiness_check))
            .route("/live", web::get().to(liveness_check))
            .route("/metrics", web::get().to(metrics_handler))
            .route("/swagger-ui", web::get().to(redirect_to_swagger))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_handler))
                    .route(web::get().to(graphql_playground)),
            )
            .configure(tmf620_catalog::api::configure_routes)
            .configure(tmf622_ordering::api::configure_routes)
            .configure(tmf637_inventory::api::configure_routes)
            .configure(tmf629_customer::api::configure_routes)
            .configure(tmf678_billing::api::configure_routes)
            .configure(tmf679_usage::api::configure_routes)
            .configure(tmf688_appointment::api::configure_routes)
            .configure(tmf641_service_order::api::configure_routes)
            .configure(tmf638_service_inventory::api::configure_routes)
            .configure(tmf640_service_activation::api::configure_routes)
            .configure(tmf702_resource_activation::api::configure_routes)
            .configure(tmf639_resource_inventory::api::configure_routes)
            .configure(tmf645_resource_order::api::configure_routes)
            .configure(tmf635_usage::api::configure_routes)
            .configure(tmf668_party_role::api::configure_routes)
            .configure(tmf632_party::api::configure_routes)
            .configure(tmf669_identity::api::configure_routes)
            .configure(tmf642_alarm::api::configure_routes)
            .configure(tmf656_slice::api::configure_routes)
            .configure(tmf633_trouble_ticket::api::configure_routes)
            .configure(tmf634_quote::api::configure_routes)
    })
    .bind((host.as_str(), port))?
    .shutdown_timeout(30); // 30 seconds for graceful shutdown

    log::info!("✅ Server started successfully");
    log::info!("   - Health: http://{}:{}/health", host, port);
    log::info!("   - Readiness: http://{}:{}/ready", host, port);
    log::info!("   - Liveness: http://{}:{}/live", host, port);
    log::info!("   - Metrics: http://{}:{}/metrics", host, port);

    // Setup graceful shutdown - Actix Web handles SIGTERM/SIGINT automatically
    // The shutdown_timeout(30) above ensures graceful shutdown with 30s timeout
    server.run().await?;
    Ok(())
}

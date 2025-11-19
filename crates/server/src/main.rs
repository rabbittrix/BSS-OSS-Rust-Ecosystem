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
use tmf645_resource_order::models::{
    CreateRelatedPartyRequest as Tmf645CreateRelatedPartyRequest, CreateResourceOrderItemRequest,
    CreateResourceOrderRequest, RelatedParty as Tmf645RelatedParty, ResourceOrder,
    ResourceOrderItem, ResourceOrderState, ResourceRef as Tmf645ResourceRef,
    ResourceSpecificationRef as Tmf645ResourceSpecificationRef,
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
        (name = "TMF669", description = "Identity & Credential Management API")
    ),
    info(
        title = "BSS/OSS Rust - TM Forum Open APIs",
        description = "TM Forum Open API implementation for BSS/OSS ecosystem (TMF620, TMF622, TMF637, TMF629, TMF678, TMF679, TMF688, TMF641, TMF638, TMF640, TMF702, TMF639, TMF645, TMF635, TMF668, TMF632, TMF669)",
        version = "0.2.0",
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
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

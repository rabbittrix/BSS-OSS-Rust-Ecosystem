//! Telecom Product Engine — orchestrates TM Forum APIs into innovative products.

pub mod error;
pub mod events;
pub mod gateway;
pub mod http_gateway;
pub mod orchestrator;
pub mod products;
pub mod service;

/// Re-exports of TMF model modules used by product adapters / HTTP gateways.
pub mod tmf {
    pub use tmf620_catalog::models as catalog;
    pub use tmf629_customer::models as customer;
    pub use tmf632_party::models as party;
    pub use tmf637_inventory::models as inventory;
    pub use tmf640_service_activation::models as activation;
    pub use tmf654_prepay_balance::models as prepay;
    pub use tmf656_slice::models as slice;
    pub use tmf666_account::models as account;
    pub use tmf669_identity::models as identity;
    pub use tmf676_payment::models as payment;
}

pub use error::{ProductError, ProductResult};
pub use events::{LogicStep, LogicStepStatus, ProductEvent, ProductEventKind};
pub use gateway::{InMemoryGateway, TmfGateway};
pub use http_gateway::HttpTmfGateway;
pub use orchestrator::ProductOrchestrator;
pub use products::*;
pub use service::ProductService;

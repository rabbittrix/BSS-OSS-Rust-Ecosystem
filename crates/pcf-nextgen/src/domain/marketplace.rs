use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceListing {
    pub listing_id: Uuid,
    pub seller_tenant_id: Uuid,
    pub title: String,
    pub base_price_minor_units: i64,
    pub currency: String,
    pub qos_bundle_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderQoSPolicyRequest {
    pub listing_id: Uuid,
    pub buyer_tenant_id: Uuid,
    pub dnn: String,
}

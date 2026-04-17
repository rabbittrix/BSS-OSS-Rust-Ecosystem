//! Marketplace-ready API stub for third-party QoS policy trading.

use std::sync::Arc;

use dashmap::DashMap;
use uuid::Uuid;

use crate::domain::{MarketplaceListing, OrderQoSPolicyRequest};

#[derive(Default)]
pub struct PolicyMarketplace {
    listings: Arc<DashMap<Uuid, MarketplaceListing>>,
}

impl PolicyMarketplace {
    pub fn new() -> Self {
        Self {
            listings: Arc::new(DashMap::new()),
        }
    }

    pub fn seed_demo(&self) {
        let listing = MarketplaceListing {
            listing_id: Uuid::parse_str("00000000-0000-4000-8000-000000000001").unwrap(),
            seller_tenant_id: Uuid::nil(),
            title: "Stadium slice URLLC booster".into(),
            base_price_minor_units: 499,
            currency: "USD".into(),
            qos_bundle_id: "BUNDLE-URLLC-STADIUM".into(),
        };
        self.listings.insert(listing.listing_id, listing);
    }

    pub fn list_open(&self) -> Vec<MarketplaceListing> {
        self.listings.iter().map(|e| e.value().clone()).collect()
    }

    /// Placeholder purchase flow — returns a token referencing the listing.
    pub fn order(&self, req: &OrderQoSPolicyRequest) -> Result<String, String> {
        self.listings
            .get(&req.listing_id)
            .map(|l| {
                format!(
                    "ORDER:{}:{}:{}",
                    l.listing_id, req.buyer_tenant_id, req.dnn
                )
            })
            .ok_or_else(|| "listing not found".into())
    }
}

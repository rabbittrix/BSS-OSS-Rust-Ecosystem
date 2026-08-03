//! Revenue Management System
//!
//! Comprehensive revenue management for charging, billing, and partner settlements:
//! - Real-time charging integration for usage events (TMF635)
//! - Usage aggregation and rating (Flat, Tiered, Volume, TimeBased)
//! - Billing cycle management with automatic bill generation
//! - Partner settlement workflows with revenue sharing

pub mod billing_cycle;
pub mod catalog;
pub mod charging;
pub mod error;
pub mod models;
pub mod rating;
pub mod settlement;

pub use billing_cycle::BillingCycleManager;
pub use catalog::{flat_rule_from_catalog, suggest_rate_type};
pub use charging::ChargingEngine;
pub use error::RevenueError;
pub use models::{
    AggregatedUsage, BillingCycle, ChargingRequest, ChargingResult, CycleStatus, CycleType, Money,
    PartnerSettlement, RateType, RatingContext, RatingRule, SettlementRule, SettlementStatus,
    TieredRate,
};
pub use rating::{ChargeAggregate, RatingEngine, RatingResult};
pub use settlement::SettlementEngine;

use sqlx::PgPool;
use std::sync::Arc;

/// Facade wiring charging, rating, billing, and settlement engines.
pub struct RevenueManager {
    pub charging: ChargingEngine,
    pub billing: Arc<BillingCycleManager>,
    pub settlement: SettlementEngine,
}

impl RevenueManager {
    pub fn new(pool: PgPool) -> Self {
        Self {
            charging: ChargingEngine::new(pool.clone()),
            billing: Arc::new(BillingCycleManager::new(pool.clone())),
            settlement: SettlementEngine::new(pool),
        }
    }

    /// Start background worker that auto-closes due billing cycles
    pub fn start_billing_worker(&self, interval_seconds: u64) -> tokio::task::JoinHandle<()> {
        self.billing.clone().start_background_worker(interval_seconds)
    }
}

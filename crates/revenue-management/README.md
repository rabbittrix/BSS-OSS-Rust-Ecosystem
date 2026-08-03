# revenue-management

[![Crates.io](https://img.shields.io/crates/v/revenue-management.svg)](https://crates.io/crates/revenue-management)
[![Documentation](https://docs.rs/revenue-management/badge.svg)](https://docs.rs/revenue-management)

**Revenue Management** for the BSS/OSS Rust ecosystem — charging, rating, billing cycles, and partner settlements.

**Status: implemented (v0.4.1)** on [crates.io](https://crates.io/crates/revenue-management).

## Features

| Area | Implementation |
|------|----------------|
| **Real-time charging** | `ChargingEngine::charge` / `charge_usage_event` (TMF635) / `charge_batch`; optional `usage.charged` events via event-bus |
| **Aggregation & rating** | Flat, Tiered (progressive), Volume, TimeBased (peak/off-peak); `aggregate_usage` + `aggregate_charges` |
| **Billing cycles** | Monthly/Quarterly/Annually/Weekly; `close_billing_cycle` builds TMF678 bills from pre-rated charges; background worker |
| **Partner settlements** | Revenue-share rules, calculate → approve/reject → paid; `run_settlement_workflow` |

Entry point: **`RevenueManager`** (or use engines directly).

## Install

```toml
revenue-management = "0.4.1"
```

```bash
cargo add revenue-management@0.4.1
```

## Quick start

```rust
use revenue_management::{ChargingEngine, ChargingRequest, RatingEngine, RateType, RatingRule};
use chrono::Utc;
use uuid::Uuid;

// Pure rating (no DB) — Flat / Tiered / Volume / TimeBased
let rule = RatingRule {
    id: Uuid::new_v4(),
    product_offering_id: Uuid::new_v4(),
    usage_type: "DATA".into(),
    unit: "MB".into(),
    rate_type: RateType::Flat,
    base_rate: 0.01,
    tiered_rates: None,
    valid_from: Utc::now(),
    valid_to: None,
};
let charge = RatingEngine::apply_rate(&rule, 100.0, None)?; // $1.00
```

With Postgres:

```rust
use revenue_management::RevenueManager;
use sqlx::PgPool;
use std::sync::Arc;

async fn boot(pool: PgPool) {
    let rm = RevenueManager::new(pool);
    let _worker = rm.start_billing_worker(60); // auto-close due cycles
}
```

## Flow

```text
TMF635 Usage event
       │
 ChargingEngine ──► rating rules ──► charging_results (+ usage.charged event)
       │
 BillingCycleManager (end of cycle)
       │
   TMF678 CustomerBill  ← sums charging_results (no double-rate)
       │
 SettlementEngine ──► partner / platform revenue share
```

## Tests

```bash
cargo test -p revenue-management
```

## Schema

See `migrations/022_revenue_management_schema.sql`.

## License

MIT

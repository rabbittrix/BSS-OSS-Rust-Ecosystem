# pcm-engine

[![Crates.io](https://img.shields.io/crates/v/pcm-engine.svg)](https://crates.io/crates/pcm-engine)
[![Documentation](https://docs.rs/pcm-engine/badge.svg)](https://docs.rs/pcm-engine)

**Product Catalog Engine (PCM)** — the catalog heart of the BSS/OSS Rust ecosystem.

**Status: implemented (v0.4.1)** on [crates.io](https://crates.io/crates/pcm-engine). All four core feature areas below are available in the library API today.

## Implemented features

| Area | What you get |
|------|----------------|
| **Pricing rules & calculations** | Base price, percentage / fixed discounts with conditions, `valid_for` windows, priority-based rule selection, complex models (tiered, volume, subscription, dynamic) |
| **Product eligibility validation** | All / Any rule sets, customer segment & attributes, `has_product` ownership checks, structured `EligibilityOutcome` with failure reasons |
| **Bundling & product relationships** | Mandatory / optional / exclusive bundles, runtime selection validation, relationships (`DependsOn`, `Excludes`, `Requires`, `MigratesTo`), bundle price calculation |
| **Catalog versioning & lifecycle** | Content snapshots of rules/bundles/relationships, create → publish → rollback, content-aware version diffs |

Entry point: **`CatalogEngine`** — register rules, call `calculate_price`, `check_eligibility` / `explain_eligibility`, `qualify_and_price`, bundle helpers, and `create_version` / `publish_version` / `rollback_to_version`.

## Install

```toml
pcm-engine = "0.4.1"
```

```bash
cargo add pcm-engine@0.4.1
```

## Quick start

```rust
use pcm_engine::{
    CatalogEngine, EligibilityContext, Money, PriceType, PricingContext, PricingRule,
};
use uuid::Uuid;

let offering = Uuid::new_v4();
let mut engine = CatalogEngine::new();

engine.add_pricing_rule(PricingRule {
    id: Uuid::new_v4(),
    product_offering_id: offering,
    price_type: PriceType::OneTime,
    base_price: Money {
        value: 29.99,
        unit: "USD".into(),
    },
    priority: 10,
    discount_rules: None,
    valid_for: None,
});

let price = engine
    .calculate_price(offering, &PricingContext::new(1))
    .expect("price");
assert!((price.value - 29.99).abs() < f64::EPSILON);

// Snapshot + publish catalog content
let catalog_id = Uuid::new_v4();
let version = engine.create_version(catalog_id, "1.0.0".into(), None, None);
engine.publish_version(version.id).unwrap();
```

### Qualify and price in one call

```rust
use pcm_engine::{CatalogEngine, EligibilityContext, PricingContext};
use uuid::Uuid;

let offering = Uuid::new_v4();
let engine = CatalogEngine::new(); // with rules already registered
let mut eligibility = EligibilityContext::new();
eligibility.customer_segment = Some("premium".into());

let result = engine.qualify_and_price(
    offering,
    &eligibility,
    &PricingContext::new(1),
    &[offering],
);
if result.eligible {
    let _money = result.price;
}
```

## Tests

```bash
cargo test -p pcm-engine
```

## Docs in this repo

- Root README — PCM section under Architecture / Core Engine Crates  
- [docs.rs/pcm-engine](https://docs.rs/pcm-engine)

## License

MIT

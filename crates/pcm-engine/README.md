# pcm-engine

[![Crates.io](https://img.shields.io/crates/v/pcm-engine.svg)](https://crates.io/crates/pcm-engine)
[![Documentation](https://docs.rs/pcm-engine/badge.svg)](https://docs.rs/pcm-engine)

**Product Catalog Engine (PCM)** — pricing, eligibility, bundling/relationships, and catalog versioning for the BSS/OSS Rust ecosystem.

## Features

| Area | Capabilities |
|------|----------------|
| **Pricing** | Base + discount rules, `valid_for` windows, priority selection, complex models (tiered / volume / subscription / dynamic) |
| **Eligibility** | All/Any rules, segment & attribute checks, `has_product` ownership, structured failure reasons |
| **Bundling** | Mandatory / optional / exclusive bundles, selection validation, product relationships (`DependsOn`, `Excludes`, `Requires`) |
| **Versioning** | Content snapshots, publish / rollback, content-aware diffs |

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
    base_price: Money { value: 29.99, unit: "USD".into() },
    priority: 10,
    discount_rules: None,
    valid_for: None,
});

let price = engine
    .calculate_price(offering, &PricingContext::new(1))
    .expect("price");
assert!((price.value - 29.99).abs() < f64::EPSILON);

let catalog_id = Uuid::new_v4();
let v = engine.create_version(catalog_id, "1.0.0".into(), None, None);
engine.publish_version(v.id).unwrap();
```

## Install

```toml
pcm-engine = "0.4.0"
```

## License

MIT

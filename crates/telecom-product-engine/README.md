# telecom-product-engine

Orchestration layer for **innovative telecom products** built on TM Forum Open API crates.

## Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│  Tauri Dashboard (React + Vite + Tailwind)                  │
│  Overview · Catalog · Customer 360 · Innovative Services    │
└────────────────────────────┬────────────────────────────────┘
                             │ invoke / events
┌────────────────────────────▼────────────────────────────────┐
│  telecom-product-engine                                      │
│  ProductService trait → TopUp · TurboBoost · DataWallet ·   │
│                         BNPL · IdentityAaaS                 │
│  TmfGateway trait → HTTP or in-process adapters             │
└─────────┬──────────┬──────────┬──────────┬──────────┬───────┘
          │          │          │          │          │
     TMF676/654  TMF656/640  TMF637/654  TMF632/666  TMF669
```

## Products

| Product | TMF APIs | Flow |
|---------|----------|------|
| **Real-Time Top-up** | TMF676 Payment, TMF654 Prepay Balance | Validate party → create payment → credit balance |
| **Turbo Boost** | TMF656 Slice, TMF640 Service Activation | Activate temporary eMBB slice / speed upgrade |
| **Data Wallet (P2P)** | TMF637 Inventory, TMF654 Prepay Balance | Debit donor bucket → credit recipient |
| **BNPL Devices** | TMF632 Party, TMF666 Account | Create / link billing account + installment plan |
| **Identity-as-a-Service** | TMF669 Identity | Issue / verify credentials |

## Defaults locked for v0.1

- Dashboard: **Tauri 2 + React + Vite + Tailwind** (not Next.js — better desktop fit)
- Balance API: thin **`tmf654-prepay-balance`** crate in this workspace
- Engine depends on workspace path crates (same versions as crates.io when published)

## Usage

```rust
use telecom_product_engine::{
    gateway::InMemoryGateway,
    products::{TopUpRequest, Money},
    ProductOrchestrator, ProductService,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let gateway = InMemoryGateway::default();
    let engine = ProductOrchestrator::new(gateway);

    let result = engine
        .real_time_topup(TopUpRequest {
            customer_id: Uuid::new_v4(),
            amount: Money { value: 10.0, unit: "EUR".into() },
            channel: "dashboard".into(),
        })
        .await?;

    println!("top-up ok: payment={:?} balance={:?}", result.payment_id, result.balance_id);
    Ok(())
}
```

## HTTP gateway (live TMF APIs)

Point at a running `bss-oss-server` with a JWT:

```bash
export BSS_OSS_BASE_URL=http://127.0.0.1:8080
export BSS_OSS_TOKEN=eyJ...   # Bearer token accepted by TMF handlers
```

```rust
use telecom_product_engine::{HttpTmfGateway, ProductOrchestrator, ProductService};

let gateway = HttpTmfGateway::from_env().expect("set BSS_OSS_TOKEN");
let engine = ProductOrchestrator::new(gateway);
// engine.turbo_boost(...).await?;
```

`HttpTmfGateway` calls TMF629, TMF632, TMF654, TMF656, TMF640, TMF666, TMF669, and TMF676 over HTTP.

## License

MIT

# Telecom Product Suite

Innovative product orchestration + Tauri dashboard on top of TM Forum Open API crates.

## Components

| Path | Role |
|------|------|
| [`crates/telecom-product-engine`](../crates/telecom-product-engine) | `ProductService` workflows (Top-up, Turbo Boost, Data Wallet, BNPL, Identity) |
| [`crates/tmf-apis/tmf654_prepay_balance`](../crates/tmf-apis/tmf654_prepay_balance) | TMF654 Prepay Balance API |
| [`apps/telecom-dashboard`](../apps/telecom-dashboard) | Tauri 2 + React + Vite + Tailwind UI |

## Product → TMF mapping

- Real-Time Top-up → TMF676 + TMF654
- Turbo Boost → TMF656 + TMF640
- Data Wallet → TMF637 + TMF654
- BNPL → TMF632 + TMF666
- Identity-as-a-Service → TMF669

## Next steps

1. ~~HTTP `TmfGateway` adapter against `bss-oss-server`~~ (`HttpTmfGateway`)
2. ~~Mount TMF654 routes in the main server OpenAPI~~
3. Live catalog/customer queries from TMF620/629 instead of demo data
4. Set `BSS_OSS_BASE_URL` + `BSS_OSS_TOKEN` so the dashboard can use `HttpTmfGateway`
5. `cargo publish -p tmf654-prepay-balance` then `telecom-product-engine`

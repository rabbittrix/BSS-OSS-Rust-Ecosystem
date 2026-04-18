# bss-oss-pcf-nextgen

[![Crates.io](https://img.shields.io/crates/v/bss-oss-pcf-nextgen.svg)](https://crates.io/crates/bss-oss-pcf-nextgen)
[![Documentation](https://docs.rs/bss-oss-pcf-nextgen/badge.svg)](https://docs.rs/bss-oss-pcf-nextgen)

Cloud-native **5G PCF** edge: SBA-style REST APIs on top of `bss-oss-pcf`, with intent translation, Policy-as-a-Service tenant overlays, monetization quotes, digital twin simulation, closed-loop suggestions, and a marketplace stub.

**Full documentation:** [docs/bss-oss-pcf-nextgen.md](https://github.com/rabbittrix/BSS-OSS-Rust-Ecosystem/blob/main/docs/bss-oss-pcf-nextgen.md) (user guide, HTTP API, metrics, embedding, deployment, limitations).

**Smoke tests (no Docker):** `cargo test -p bss-oss-pcf-nextgen --test http_smoke` — or run the server and execute [`scripts/smoke-pcf-nextgen.ps1`](../../scripts/smoke-pcf-nextgen.ps1).

## Install from crates.io

```bash
cargo install bss-oss-pcf-nextgen
```

Requires a published [`bss-oss-pcf`](https://crates.io/crates/bss-oss-pcf) compatible with the crate’s dependency version (see `Cargo.toml`).

## Run locally (from this repository)

```bash
cargo run -p bss-oss-pcf-nextgen
```

Defaults:

- Listen: `0.0.0.0:9080` (override with `PCF_BIND`)
- Optional: `KAFKA_BROKERS`, `REDIS_URL`, `DATABASE_URL`, `OTEL_EXPORTER_OTLP_ENDPOINT`
- Enforce `Authorization: Bearer` on API routes: `PCF_REQUIRE_BEARER=true`

## Key endpoints

| Path | Purpose |
|------|---------|
| `POST /npcf-sba/v1/policy/decision` | Fast policy decision (+ optional `tenant_id`) |
| `POST /npcf-sba/v1/policy/intent` | Intent → policy |
| `POST /paas/v1/tenants/{tenant_id}/policy/decision` | Multi-tenant enterprise overlay |
| `POST /npcf-sba/v1/simulation/run` | Digital twin (shadow) |
| `POST /npcf-sba/v1/closed-loop/telemetry` | Telemetry → adjustment hint |
| `POST /nchf-ready/v1/quote` | CHF-ready monetization quote |
| `GET /marketplace/v1/listings` | Marketplace listings |
| `GET /metrics` | Prometheus |

## OpenAPI & Swagger UI (no Docker)

With the server running, open in a browser:

**<http://127.0.0.1:9080/swagger-ui/>**

The UI loads the spec from **<http://127.0.0.1:9080/openapi/pcf-nextgen-sba.yaml>** (same origin, no CORS). The YAML is embedded at build time from **`crates/pcf-nextgen/openapi/pcf-nextgen-sba.yaml`** (so `cargo publish` includes it); keep that file in sync with **`openapi/pcf-nextgen-sba.yaml`** at the repository root.

Swagger assets are loaded from **unpkg.com** (needs network in the browser). Offline: use Postman with the YAML file, or `cargo test -p bss-oss-pcf-nextgen --test http_smoke`.

## Kubernetes (Helm)

In `helm/bss-oss-rust/values.yaml`, set:

```yaml
pcfNextgen:
  enabled: true
```

Then install/upgrade the chart as documented in `helm/bss-oss-rust/README.md`.

Build a container image that runs the `bss-oss-pcf-nextgen` binary and point `pcfNextgen.image.repository` / `tag` at your registry.

## Further reading

- **[docs/bss-oss-pcf-nextgen.md](../../docs/bss-oss-pcf-nextgen.md)** — manual: API, env vars, metrics, library usage, Helm
- **[docs/pcf_nextgen_architecture.md](../../docs/pcf_nextgen_architecture.md)** — architecture, HA, security, observability
- **[examples/pcf-nextgen/](../../examples/pcf-nextgen/)** — AR/VR low-latency HTTP examples

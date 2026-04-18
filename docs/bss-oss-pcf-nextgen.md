# bss-oss-pcf-nextgen — User & operator guide

This document is the **primary reference** for the [`bss-oss-pcf-nextgen`](https://crates.io/crates/bss-oss-pcf-nextgen) crate: a cloud-native **Policy Control Function (PCF)** edge that exposes HTTP APIs and a Rust library on top of [`bss-oss-pcf`](https://crates.io/crates/bss-oss-pcf).

| Resource | Location |
|----------|----------|
| Crate README (quick start) | [`crates/pcf-nextgen/README.md`](../crates/pcf-nextgen/README.md) |
| Architecture (diagrams, HA, security) | [`pcf_nextgen_architecture.md`](pcf_nextgen_architecture.md) |
| OpenAPI 3.0 | Repo: [`openapi/pcf-nextgen-sba.yaml`](../openapi/pcf-nextgen-sba.yaml) — **embedded copy for `cargo publish`:** [`crates/pcf-nextgen/openapi/pcf-nextgen-sba.yaml`](../crates/pcf-nextgen/openapi/pcf-nextgen-sba.yaml) (keep identical) |
| REST examples | [`examples/pcf-nextgen/`](../examples/pcf-nextgen/) |
| TypeScript client | [`sdk/typescript/pcf-nextgen-client.ts`](../sdk/typescript/pcf-nextgen-client.ts) |
| Rust API (docs.rs) | [docs.rs/bss-oss-pcf-nextgen](https://docs.rs/bss-oss-pcf-nextgen) |

---

## Quick testing & OpenAPI (no Docker required)

### Embedded Swagger UI (recommended for “Try it out”)

1. Start the server: `cargo run -p bss-oss-pcf-nextgen`
2. Open **http://127.0.0.1:9080/swagger-ui/** in a browser (trailing slash required).

The page loads **Swagger UI** from **unpkg.com** and the OpenAPI document from **`GET /openapi/pcf-nextgen-sba.yaml`** on the same host, so **CORS does not block** calls to your local API.

This avoids **Docker** entirely (no `dockerDesktopLinuxEngine` / API 500 issues).

### Automated HTTP smoke tests (recommended)

From the repository root:

```bash
cargo test -p bss-oss-pcf-nextgen --test http_smoke
```

This builds the Actix app **in-process** (no `docker run`, no separate server) and hits `/health/*`, `/metrics`, `/demo/ar-vr/policy`, policy decision, and closed-loop routes. Use this when **Docker Desktop is not running** or you want CI-friendly checks.

### Docker / Swagger UI troubleshooting

If `docker run ... swaggerapi/swagger-ui` fails with **`npipe:////./pipe/dockerDesktopLinuxEngine`** (Windows), the **Docker daemon is not running**. Start **Docker Desktop**, or skip Docker and use the **`cargo test`** command above, **Postman/Insomnia** (import `openapi/pcf-nextgen-sba.yaml`), or a local docs preview:

```bash
npx --yes @redocly/cli@1 preview-docs openapi/pcf-nextgen-sba.yaml
```

(`npx` requires Node.js; it opens a browser UI for the spec — point requests at your running `bss-oss-pcf-nextgen` base URL.)

### Manual smoke script (optional)

With the binary listening on `http://127.0.0.1:9080`:

```powershell
pwsh -File scripts/smoke-pcf-nextgen.ps1 -BaseUrl http://127.0.0.1:9080
```

---

## 1. What this crate provides

### 1.1 Binary: `bss-oss-pcf-nextgen`

An **Actix Web** server that listens for HTTP requests and:

- Evaluates policies using the in-process **`PcfEngine`** from `bss-oss-pcf` (QoS, charging rules, quota).
- Adds **SBA-style** route prefixes (not a full 3GPP Npcf stack; payloads are aligned for integration).
- Exposes **Prometheus** metrics, **liveness/readiness** probes, and optional **Bearer** authentication.

### 1.2 Library: `bss_oss_pcf_nextgen`

Rust modules for **clean architecture** integration in your own binary:

- **`NextGenPcfOrchestrator`** — composes fast path, tenant overlays, intent handling, and policy events.
- **`PolicyMarketplace`**, domain types, circuit breaker, Kafka-style event publisher trait.

Use the library when you embed policy logic in a larger NF or gateway; use the stock binary for a drop-in HTTP edge.

---

## 2. Configuration (environment variables)

All variables are read at process start (`RuntimeConfig::from_env`).

| Variable | Required | Default | Purpose |
|----------|----------|---------|---------|
| `PCF_BIND` | No | `0.0.0.0:9080` | HTTP listen address |
| `PCF_REQUIRE_BEARER` | No | `false` | If `true` or `1`, mutating API routes require `Authorization: Bearer <token>` |
| `KAFKA_BROKERS` | No | — | Passed to the logging event publisher (stub; replace with a real producer in production) |
| `REDIS_URL` | No | — | Reserved for future hot-state / session cache |
| `DATABASE_URL` | No | — | Reserved for persistent policy catalog / tenants |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | No | — | Logged at startup; wire OpenTelemetry in your deployment when ready |

**Logging:** use `RUST_LOG` (e.g. `info`, `debug`, or `bss_oss_pcf_nextgen=debug`).

---

## 3. HTTP API reference

Base URL examples: `http://localhost:9080` (local), or your Ingress URL in Kubernetes.

Unless noted, successful policy responses return **`200`** with a JSON **`PolicyDecision`** (see `bss_oss_pcf::PolicyDecision` and OpenAPI schema). Policy evaluation failures return **`422`** with `{ "error": "<message>" }`.

### 3.1 Authentication

When **`PCF_REQUIRE_BEARER=true`**, these routes require a header:

```http
Authorization: Bearer <your-jwt-or-api-token>
```

Affected routes: all **POST** handlers listed below except where noted, plus **GET** `/marketplace/v1/listings`. Routes that do **not** check the flag in the current implementation: **`GET /health/live`**, **`GET /health/ready`**, **`GET /metrics`**, **`GET /demo/ar-vr/policy`**.

> In production, validate the token at the gateway (OAuth2, JWT) or via service mesh identity; the server only checks for header presence and prefix.

### 3.2 Policy decision (flat session)

**`POST /npcf-sba/v1/policy/decision`**

Body: JSON merging **`PolicyRequest`** fields with an optional **`tenant_id`** (UUID string).

Example:

```json
{
  "subscriber_id": "ar-demo-001",
  "imsi": "001010987654321",
  "network_generation": "5G",
  "apn": "ims.enterprise-ar",
  "service_type": "low_latency",
  "application_id": "com.example.ar.stadium",
  "tenant_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

- **`tenant_id`**: optional; when present, enterprise overlay rules registered for that tenant may adjust QoS (see §5).

### 3.3 Policy decision (tenant path, Policy-as-a-Service)

**`POST /paas/v1/tenants/{tenant_id}/policy/decision`**

- **Path:** `tenant_id` — UUID of the B2B2X tenant.
- **Body:** full **`PolicyRequest`** JSON (no flatten).

Same response semantics as §3.2.

### 3.4 Intent-based policy

**`POST /npcf-sba/v1/policy/intent`**

Body: JSON object with:

| Field | Type | Required |
|-------|------|----------|
| `intent` | `PolicyIntent` | Yes |
| `session` | `PolicyRequest` | Yes |
| `tenant_id` | UUID string | No |

`PolicyIntent` fields:

- **`intent_id`** (string): drives mapping (e.g. `ULTRA_LOW_LATENCY_AR`, `CLOUD_GAMING`, `BULK_BACKUP`).
- **`description`**, optional numeric hints, **`slice_hint`**, **`application_id`**.

The server maps intent → service type / 5QI-style hints, then runs the same pipeline as a normal decision.

### 3.5 Digital twin (simulation)

**`POST /npcf-sba/v1/simulation/run`**

- **Body:** `PolicyRequest`.
- **Response:** `200` with `{ "mode": "SIMULATION", "decision": { ... }, "would_publish": false }`.

Uses the same evaluator as production; the wrapper marks the result as simulation-only (no SMF push in this reference build).

### 3.6 Closed-loop telemetry

**`POST /npcf-sba/v1/closed-loop/telemetry`**

- **Body:** `NetworkTelemetrySample` (`cell_id`, `congestion_score`, `mean_user_throughput_mbps`, `mean_latency_ms`, `active_ues`).
- **Response:** `PolicyAdjustmentSuggestion` (`scale_mbr_factor`, `priority_delta`, `rationale`).

Intended to be driven by RAN/CN metrics and optionally published to Kafka for automation pipelines.

### 3.7 Monetization (CHF-ready quote)

**`POST /nchf-ready/v1/quote`**

- **Body:** `MonetizationQuoteRequest` (`service_class`, `requested_downlink_mbps`, `expected_latency_ms_p99`, `duration_seconds`, `currency`).
- **Response:** `MonetizationQuoteResponse` (`price_minor_units`, `currency`, `rating_group`, `charging_key`, `explanation`).

Heuristic pricing for demos; replace with product catalog and CHF (**Nchf_ConvergedCharging**) integration in production.

### 3.8 Marketplace (stub)

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/marketplace/v1/listings` | List in-memory demo listings |
| `POST` | `/marketplace/v1/orders` | Body: `OrderQoSPolicyRequest` → `{ "order_token": "..." }` or `404` |

### 3.9 Demo, health, metrics

| Method | Path | Auth (`PCF_REQUIRE_BEARER`) | Purpose |
|--------|------|-------------------------------|---------|
| `GET` | `/demo/ar-vr/policy` | No | Runs a canned `PolicyRequest` for subscriber `ar-demo-001` (seeded at startup) |
| `GET` | `/health/live` | No | Liveness (empty `200`) |
| `GET` | `/health/ready` | No | Readiness JSON `{ "status": "READY" }` |
| `GET` | `/metrics` | No | Prometheus text exposition |

---

## 4. Prometheus metrics

Scrape **`GET /metrics`**.

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `pcf_policy_decision_seconds` | Histogram | — | Wall time for policy evaluation on instrumented routes |
| `pcf_policy_decisions_total` | Counter | `outcome` = `ok` \| `error` | Decision outcomes |

Use histogram buckets to track **p99 latency** against a sub-10 ms SLO on idle paths; under load, correlate with CPU, replicas, and downstream UDR/SMF latency.

---

## 5. Rust library: embedding

Add to your `Cargo.toml`:

```toml
bss-oss-pcf-nextgen = "0.3.1"  # use latest published patch from crates.io
bss-oss-pcf = "0.3.0"
```

Typical pattern:

```rust
use std::sync::Arc;
use bss_oss_pcf::PcfEngine;
use bss_oss_pcf_nextgen::{NextGenPcfOrchestrator, PolicyMarketplace};
use bss_oss_pcf_nextgen::application::orchestrator::seed_demo_subscriber;

let engine = Arc::new(PcfEngine::new());
seed_demo_subscriber(&engine);
let orchestrator = NextGenPcfOrchestrator::with_default_event_bus(
    engine,
    "localhost:9092".into(),
);

// Optional: enterprise overlays (not exposed via HTTP yet)
use uuid::Uuid;
use bss_oss_pcf_nextgen::domain::EnterpriseQoSRule;
let tenant = Uuid::new_v4();
orchestrator.register_tenant_rules(tenant, vec![/* EnterpriseQoSRule */]);
```

Public re-exports: see **`src/lib.rs`** on [docs.rs](https://docs.rs/bss-oss-pcf-nextgen/latest/bss_oss_pcf_nextgen/).

---

## 6. Data models (summary)

Aligned with **`openapi/pcf-nextgen-sba.yaml`** and `bss_oss_pcf` types where applicable:

- **`PolicyRequest`** / **`PolicyDecision`** — from `bss-oss-pcf`.
- **`PolicyIntent`**, **`EnterpriseQoSRule`**, **`NetworkTelemetrySample`**, **`MonetizationQuoteRequest`**, **`MarketplaceListing`**, **`OrderQoSPolicyRequest`** — in `bss_oss_pcf_nextgen::domain`.

---

## 7. Deployment

### 7.1 From crates.io

```bash
cargo install bss-oss-pcf-nextgen
bss-oss-pcf-nextgen   # runs with env as above
```

### 7.2 From source

```bash
cargo run -p bss-oss-pcf-nextgen
```

### 7.3 Kubernetes (Helm)

In **`helm/bss-oss-rust/values.yaml`**:

```yaml
pcfNextgen:
  enabled: true
  image:
    repository: <your-registry>/bss-oss-pcf-nextgen
    tag: <digest-or-tag>
```

Templates: **`pcf-nextgen-deployment.yaml`**, **`pcf-nextgen-service.yaml`**. Point probes at **`/health/live`** and **`/health/ready`**. See also [pcf_nextgen_architecture.md](pcf_nextgen_architecture.md) for HA, mesh, and observability.

---

## 8. Limitations and extension points

| Area | Current behavior | Production direction |
|------|------------------|----------------------|
| 3GPP Npcf | REST shapes inspired by SBA; not a certified N7 stack | Align JSON with TS 29.512 and SMF contract |
| Kafka | Logging stub | `rdkafka` (or cloud SDK) implementing `PolicyEventPublisher` |
| Redis / PostgreSQL | Env only | Tenant rules, sessions, catalogs |
| Tenant rules HTTP API | None | Add CRUD behind OAuth2 + tenant scope |
| mTLS / OAuth2 | Bearer presence check only | Gateway + mesh (SPIFFE, JWT validation) |
| gRPC | Not included | Add `tonic` NFs or internal admin API |

---

## 9. Related documentation

- **[pcf_nextgen_architecture.md](pcf_nextgen_architecture.md)** — system context, Mermaid diagram, microservice split, security, CI/CD.
- **Repository root [README.md](../README.md)** — overall BSS/OSS ecosystem.

---

## 10. Versioning and license

- **`bss-oss-pcf-nextgen`** uses an **explicit `version` in `crates/pcf-nextgen/Cargo.toml`** so each crates.io release can bump the patch (e.g. `0.3.0` → `0.3.1`) without changing the whole workspace meta-version.
- **License:** Apache-2.0 (see crate `Cargo.toml`).

For release and **crates.io** publishing, see **`.github/workflows/publish.yml`** and the crate README.

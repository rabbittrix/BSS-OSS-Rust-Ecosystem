# bss-oss-service-orchestrator

[![Crates.io](https://img.shields.io/crates/v/bss-oss-service-orchestrator.svg)](https://crates.io/crates/bss-oss-service-orchestrator)
[![Documentation](https://docs.rs/bss-oss-service-orchestrator/badge.svg)](https://docs.rs/bss-oss-service-orchestrator)

**Service lifecycle orchestration** for the BSS/OSS Rust ecosystem: Service Order → Service Activation → Service Inventory.

**Status: implemented (v0.4.1)** on [crates.io](https://crates.io/crates/bss-oss-service-orchestrator).

## Features

| Feature | Implementation |
|---------|----------------|
| **Workflows** | ValidateOrder → CheckDependencies → CreateActivation → ExecuteActivation → CreateInventory |
| **Dependency graph** | Shared in-memory graph + Postgres persistence; `can_provision` / `mark_active` |
| **Auto-activation** | Activates when dependencies are met (shared graph with orchestrator) |
| **Lifecycle state** | `ServiceLifecycleState` tracked per workflow and per task; DB uses SCREAMING_SNAKE_CASE |
| **Background worker** | `start_background_worker(interval_secs)` polls pending workflows |

## Install

```toml
bss-oss-service-orchestrator = "0.4.1"
```

## Quick start

```rust
use bss_oss_service_orchestrator::ServiceOrchestrator;
use sqlx::PgPool;
use std::sync::Arc;

async fn boot(pool: PgPool) -> Arc<ServiceOrchestrator> {
    let orch = Arc::new(
        ServiceOrchestrator::initialize(pool)
            .await
            .expect("orchestrator"),
    );
    // Poll incomplete workflows every 30s
    let _worker = orch.clone().start_background_worker(30);
    orch
}
```

Drive a single order:

```rust
use bss_oss_service_orchestrator::{ServiceOrchestrator, ServiceOrchestratorTrait};
// after creating a TMF641 ServiceOrder:
// orch.orchestrate(service_order).await?;
// orch.process_workflow(service_order_id).await?; // also invoked inside orchestrate
```

## Workflow graph

```text
ValidateOrder
      │
CheckDependencies  ←── waits if deps not met (background worker retries)
      │
CreateActivation   ←── auto-activate when can_provision
      │
ExecuteActivation
      │
CreateInventory    ←── mark spec Active in dependency graph
      │
   Completed
```

## Tests

```bash
cargo test -p bss-oss-service-orchestrator
```

## Schema

See `migrations/020_service_orchestration_schema.sql` for `service_workflow_contexts`, `service_dependencies`, and `service_specification_states`.

## License

MIT

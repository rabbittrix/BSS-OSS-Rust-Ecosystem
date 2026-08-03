# OpenAPI / Swagger artifacts

| File | Description |
|------|-------------|
| `tmf-apis-openapi.json` | Full BSS/OSS TMF OpenAPI document (utoipa) for the main server — version **0.3.1** |
| `pcf-nextgen-sba.yaml` | Next-gen PCF SBA OpenAPI (mirrored under `crates/pcf-nextgen/openapi/`) |

## Regenerate TMF OpenAPI

```bash
cargo test -p bss-oss-server export_openapi -- --nocapture
```

## Live Swagger UI

With the server running:

- UI: `http://localhost:8080/swagger-ui/`
- Spec: `http://localhost:8080/api-doc/openapi.json`

Corrected TMF tags in this release include **TMF621**, **TMF633**, **TMF634**, **TMF646**, **TMF648**, **TMF677**, and **TMF679**.

This OpenAPI document is **embedded** in the `bss-oss-pcf-nextgen` binary (`include_str!`) and must stay inside the crate so `cargo publish` succeeds.

When updating the API contract, edit **`pcf-nextgen-sba.yaml` here**, then copy the same file to the repo-wide path **`openapi/pcf-nextgen-sba.yaml`** at the workspace root (used by docs and external tooling), or vice versa if you prefer the root as source—just keep the two files identical before release.

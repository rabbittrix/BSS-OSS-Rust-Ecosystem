# Publish script for 0.3.1 / patch releases (PowerShell)

$ErrorActionPreference = "Stop"
Set-Location d:\bss-oss-rust

function Publish-Crate([string]$Package) {
  Write-Host "==== Publishing $Package ====" -ForegroundColor Cyan
  cargo publish -p $Package --allow-dirty 2>&1
  if ($LASTEXITCODE -ne 0) {
    Write-Host "WARN: publish $Package exit $LASTEXITCODE (may already exist or rate-limited)" -ForegroundColor Yellow
  }
  Start-Sleep -Seconds 8
}

# Layer 0
Publish-Crate tmf-apis-core

# Layer 1 — TMF APIs (depend only on core)
$tmfs = @(
  "tmf620-catalog",
  "tmf621-trouble-ticket",
  "tmf622-ordering",
  "tmf629-customer",
  "tmf632-party",
  "tmf633-service-catalog",
  "tmf634-resource-catalog",
  "tmf635-usage",
  "tmf637-inventory",
  "tmf638-service-inventory",
  "tmf639-resource-inventory",
  "tmf640-service-activation",
  "tmf641-service-order",
  "tmf642-alarm",
  "tmf645-resource-order",
  "tmf646-appointment",
  "tmf648-quote",
  "tmf656-slice",
  "tmf668-party-role",
  "tmf669-identity",
  "tmf677-usage",
  "tmf678-billing",
  "tmf679-product-offering-qualification",
  "tmf702-resource-activation"
)
foreach ($c in $tmfs) { Publish-Crate $c }

# Layer 2
Publish-Crate bss-oss-utils
Publish-Crate bss-oss-event-bus
Publish-Crate pcm-engine

# Layer 3
Publish-Crate revenue-management
Publish-Crate bss-oss-service-orchestrator
Publish-Crate bss-oss-pcf
Publish-Crate bss-oss-pcf-nextgen

Write-Host "Publish batch finished" -ForegroundColor Green

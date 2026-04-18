#Requires -Version 5.1
<#
.SYNOPSIS
    Quick HTTP checks against a running bss-oss-pcf-nextgen instance.

.DESCRIPTION
    Start the server first, for example:
      cargo run -p bss-oss-pcf-nextgen
    Default bind: http://127.0.0.1:9080 (set PCF_BIND if needed).

.PARAMETER BaseUrl
    Root URL of the service (no trailing slash).
#>
param(
    [string] $BaseUrl = "http://127.0.0.1:9080"
)

$ErrorActionPreference = "Stop"

function Invoke-PcfGetJson([string] $Path) {
    $uri = "$BaseUrl$Path"
    Write-Host "GET $uri"
    Invoke-RestMethod -Uri $uri -Method Get
}

function Invoke-PcfGetRaw([string] $Path) {
    $uri = "$BaseUrl$Path"
    Write-Host "GET $uri"
    (Invoke-WebRequest -Uri $uri -Method Get -UseBasicParsing).Content
}

Write-Host "=== bss-oss-pcf-nextgen smoke ===" -ForegroundColor Cyan
Write-Host "BaseUrl: $BaseUrl`n"

$liveUri = "$BaseUrl/health/live"
Write-Host "GET $liveUri"
$liveResp = Invoke-WebRequest -Uri $liveUri -Method Get -UseBasicParsing
if ($liveResp.StatusCode -ne 200) { throw "health/live status $($liveResp.StatusCode)" }
Write-Host "health/live: OK" -ForegroundColor Green

$r = Invoke-PcfGetJson "/health/ready"
if ($r.status -ne "READY") { throw "readiness unexpected: $r" }
Write-Host "health/ready: OK" -ForegroundColor Green

$m = Invoke-PcfGetRaw "/metrics"
if ($m -notmatch "pcf_policy_decision_seconds") {
    throw "metrics body missing pcf_policy_decision_seconds"
}
Write-Host "metrics: OK" -ForegroundColor Green

$d = Invoke-PcfGetJson "/demo/ar-vr/policy"
if ($d.subscriber_id -ne "ar-demo-001") { throw "demo subscriber mismatch" }
Write-Host "demo/ar-vr/policy: OK ($($d.policy_rule_name))" -ForegroundColor Green

Write-Host "`nAll smoke checks passed." -ForegroundColor Cyan

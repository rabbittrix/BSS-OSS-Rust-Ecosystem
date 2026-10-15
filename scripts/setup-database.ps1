# Creates bssoss role/database and applies SQL migrations.
#
# Local PostgreSQL (superuser 'postgres' — password from install / pgAdmin):
#   $env:POSTGRES_ADMIN_PASSWORD = "your-postgres-superuser-password"
#   $env:BSSOSS_APP_PASSWORD = "bssoss123"
#   .\scripts\setup-database.ps1 -Port 5433
#
# Docker dev DB (superuser is bssoss — same as app user):
#   docker compose up -d postgres
#   $env:POSTGRES_ADMIN_PASSWORD = "bssoss123"
#   .\scripts\setup-database.ps1 -Port 5432 -AdminUser bssoss -SkipRoleSetup
param(
    [string]$DbHost = "127.0.0.1",
    [int]$Port = 5433,
    [string]$AdminUser = "postgres",
    [string]$AppUser = "bssoss",
    [SecureString]$AppPassword,
    [string]$AppDatabase = "bssoss",
    [switch]$SkipRoleSetup
)

function ConvertFrom-SecureStringPlain {
    param([SecureString]$Secure)
    if (-not $Secure) {
        return $null
    }
    $bstr = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($Secure)
    try {
        return [Runtime.InteropServices.Marshal]::PtrToStringAuto($bstr)
    }
    finally {
        [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($bstr)
    }
}

$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent

function Invoke-Psql {
    param(
        [string[]]$Arguments,
        [switch]$Silent
    )
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $lines = & $psql @Arguments 2>&1
    $exitCode = $LASTEXITCODE
    $ErrorActionPreference = $prevEap

    foreach ($line in $lines) {
        $text = "$line"
        if ($text -match '(ERROR|FATAL):') {
            Write-Host $text -ForegroundColor Red
        }
        elseif (-not $Silent -and $text -notmatch 'NOTICE:') {
            if ($text) { Write-Host $text }
        }
    }

    if ($exitCode -ne 0) {
        throw "psql exited with code $exitCode"
    }

    return $lines
}

$psql = Get-ChildItem -Path "${env:ProgramFiles}\PostgreSQL" -Recurse -Filter "psql.exe" -ErrorAction SilentlyContinue |
    Sort-Object FullName -Descending |
    Select-Object -First 1 -ExpandProperty FullName

if (-not $psql) {
    Write-Error "psql.exe not found. Install PostgreSQL client tools or use Docker: docker compose up -d postgres"
}

Write-Host "Using PostgreSQL at ${DbHost}:${Port} (admin user: $AdminUser)" -ForegroundColor Cyan

$adminPassword = $env:POSTGRES_ADMIN_PASSWORD
if (-not $adminPassword) {
    Write-Host ""
    Write-Host "Enter the password for PostgreSQL superuser '$AdminUser'." -ForegroundColor Yellow
    Write-Host "  - Local install: the password you chose when installing PostgreSQL (often user 'postgres')."
    Write-Host "  - Docker (docker compose): use bssoss123 and run with -AdminUser bssoss -SkipRoleSetup."
    Write-Host ""
    $secure = Read-Host "Password for '$AdminUser'" -AsSecureString
    $adminPassword = ConvertFrom-SecureStringPlain -Secure $secure
}

$env:PGPASSWORD = $adminPassword
$adminArgs = @("-h", $DbHost, "-p", $Port, "-U", $AdminUser, "-v", "ON_ERROR_STOP=1")

Write-Host "Checking admin connection..."
try {
    Invoke-Psql -Arguments ($adminArgs + @("-d", "postgres", "-tAc", "SELECT 1")) -Silent | Out-Null
}
catch {
    Write-Host ""
    Write-Host "Admin login failed for user '$AdminUser' on ${DbHost}:${Port}." -ForegroundColor Red
    Write-Host $_.Exception.Message
    Write-Host ""
    Write-Host "Docker (known password):" -ForegroundColor Yellow
    Write-Host "  docker compose up -d postgres"
    Write-Host "  `$env:POSTGRES_ADMIN_PASSWORD='bssoss123'; .\scripts\setup-database.ps1 -Port 5432 -AdminUser bssoss -SkipRoleSetup"
    exit 1
}
Write-Host "Admin connection OK." -ForegroundColor Green

if (-not $AppPassword) {
    if ($env:BSSOSS_APP_PASSWORD) {
        $AppPassword = ConvertTo-SecureString -String $env:BSSOSS_APP_PASSWORD -AsPlainText -Force
    }
    elseif ($AdminUser -eq $AppUser -and $adminPassword) {
        $AppPassword = ConvertTo-SecureString -String $adminPassword -AsPlainText -Force
    }
    else {
        Write-Host ""
        Write-Host "Password for application role '$AppUser' (.env / DATABASE_URL). Default: bssoss123"
        $AppPassword = Read-Host "Password for role '$AppUser'" -AsSecureString
    }
}
$appPasswordPlain = ConvertFrom-SecureStringPlain -Secure $AppPassword

if (-not $SkipRoleSetup) {
    Write-Host "Creating application role (if missing)..."
    $escapedAppPassword = $appPasswordPlain -replace "'", "''"
    Invoke-Psql -Arguments ($adminArgs + @("-d", "postgres", "-c", "DO `$`$ BEGIN IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = '$AppUser') THEN CREATE ROLE $AppUser LOGIN PASSWORD '$escapedAppPassword'; ELSE ALTER ROLE $AppUser PASSWORD '$escapedAppPassword'; END IF; END `$`$;")) | Out-Null

    Write-Host "Creating database (if missing)..."
    $dbExists = (Invoke-Psql -Arguments ($adminArgs + @("-d", "postgres", "-tAc", "SELECT 1 FROM pg_database WHERE datname = '$AppDatabase'")) | Out-String).Trim()
    if ($dbExists -ne "1") {
        Invoke-Psql -Arguments ($adminArgs + @("-d", "postgres", "-c", "CREATE DATABASE $AppDatabase OWNER $AppUser;")) | Out-Null
    }
}
else {
    Write-Host "Skipping role/database creation (-SkipRoleSetup)."
}

$env:PGPASSWORD = $appPasswordPlain
$appArgs = @("-h", $DbHost, "-p", $Port, "-U", $AppUser, "-d", $AppDatabase, "-v", "ON_ERROR_STOP=1")

Write-Host "Verifying application user connection..."
try {
    Invoke-Psql -Arguments ($appArgs + @("-tAc", "SELECT 1")) -Silent | Out-Null
}
catch {
    Write-Host "Application user '$AppUser' could not connect:" -ForegroundColor Red
    exit 1
}

Write-Host "Ensuring migration history table..."
Invoke-Psql -Arguments ($appArgs + @("-q", "-c", "CREATE TABLE IF NOT EXISTS schema_migrations (filename TEXT PRIMARY KEY, applied_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP);")) | Out-Null

$appliedRows = Invoke-Psql -Arguments ($appArgs + @("-tAc", "SELECT filename FROM schema_migrations ORDER BY filename")) -Silent
$applied = @($appliedRows | ForEach-Object { "$_".Trim() } | Where-Object { $_ })

Write-Host "Applying migrations from $root\migrations ..."
Get-ChildItem "$root\migrations\*.sql" | Sort-Object Name | ForEach-Object {
    $name = $_.Name
    if ($applied -contains $name) {
        Write-Host "  -> $name (already applied, skip)" -ForegroundColor DarkGray
        return
    }
    Write-Host "  -> $name"
    try {
        Invoke-Psql -Arguments ($appArgs + @("-q", "--single-transaction", "-f", $_.FullName)) | Out-Null
    }
    catch {
        Write-Host "Migration failed: $name" -ForegroundColor Red
        exit 1
    }
    $escaped = $name -replace "'", "''"
    Invoke-Psql -Arguments ($appArgs + @("-q", "-c", "INSERT INTO schema_migrations (filename) VALUES ('$escaped');")) | Out-Null
}

Write-Host "Done. DATABASE_URL=postgresql://${AppUser}:****@${DbHost}:${Port}/${AppDatabase}" -ForegroundColor Green

# Ask Pete System Check
# "Testing for Dummies" - Vibe Coder Edition

Write-Host "🚂 Ask Pete System Check 🚂" -ForegroundColor Cyan
Write-Host "============================" -ForegroundColor Cyan

# 1. Check Rust Environment
Write-Host "`n[1] Checking Rust Environment..." -NoNewline
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host " PASS" -ForegroundColor Green
    cargo --version
} else {
    Write-Host " FAIL" -ForegroundColor Red
    Write-Host "Error: Rust (cargo) is not installed or not in PATH."
}

# 2. Check AI Models (Local GGUF)
Write-Host "`n[2] Checking AI Models..." -NoNewline
$cacheDir = "$env:LOCALAPPDATA\askpeet\models"
if (Test-Path $cacheDir) {
    $models = Get-ChildItem $cacheDir -Filter "*.gguf"
    if ($models.Count -gt 0) {
        Write-Host " PASS" -ForegroundColor Green
        Write-Host "Found $($models.Count) models in $cacheDir"
        foreach ($model in $models) {
            Write-Host "  - $($model.Name)" -ForegroundColor DarkGray
        }
    } else {
        Write-Host " WARNING" -ForegroundColor Yellow
        Write-Host "Models directory exists but is empty. Models will download on first run."
    }
} else {
    Write-Host " WARNING" -ForegroundColor Yellow
    Write-Host "Model cache directory not found ($cacheDir). It will be created on first run."
}

# 3. Check Project Compilation (Fast Check)
Write-Host "`n[3] Checking Project Compilation (Client)..." -NoNewline
# We use 'cargo check' which is faster than build
$check = Invoke-Expression "cargo check -p ask_pete_client 2>&1"
if ($LASTEXITCODE -eq 0) {
    Write-Host " PASS" -ForegroundColor Green
} else {
    Write-Host " FAIL" -ForegroundColor Red
    Write-Host "Client compilation failed. Run 'cargo check -p ask_pete_client' to see errors."
}

Write-Host "`n[4] Checking Project Compilation (Server)..." -NoNewline
$checkServer = Invoke-Expression "cargo check -p ask_pete_server 2>&1"
if ($LASTEXITCODE -eq 0) {
    Write-Host " PASS" -ForegroundColor Green
} else {
    Write-Host " FAIL" -ForegroundColor Red
    Write-Host "Server compilation failed. Run 'cargo check -p ask_pete_server' to see errors."
}

Write-Host "`n============================" -ForegroundColor Cyan
Write-Host "System Check Complete." -ForegroundColor Cyan

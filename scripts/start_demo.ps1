# Ask Pete Demo Startup Script
# One-command startup for authoring tool demo

Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "  Ask Pete Authoring Tool Demo Startup" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host ""

# Check PostgreSQL
Write-Host "[1/3] Checking PostgreSQL..." -ForegroundColor Yellow
$pgService = Get-Service -Name postgresql* -ErrorAction SilentlyContinue

if (-not $pgService) {
    Write-Host "   ⚠️  PostgreSQL service not found" -ForegroundColor Yellow
    Write-Host "   Manual Options:" -ForegroundColor Gray
    Write-Host "   - Docker: docker-compose up -d postgres" -ForegroundColor Gray
    Write-Host "   - Service: Start-Service postgresql-x64-[version]" -ForegroundColor Gray
    Write-Host ""
    $continue = Read-Host "Continue anyway? (y/n)"
    if ($continue -ne "y") {
        exit 0
    }
}
else {
    Write-Host "   ✅ PostgreSQL detected: $($pgService.Status)" -ForegroundColor Green
}

Write-Host ""

# Start Backend Server
Write-Host "[2/3] Starting Backend Server..." -ForegroundColor Yellow
Write-Host "   Port: 3000" -ForegroundColor Gray

Start-Process powershell -ArgumentList `
    "-NoExit", `
    "-Command", `
    "cd '$PSScriptRoot'; Write-Host 'Starting Ask Pete Server...' -ForegroundColor Cyan; cargo run -p ask_pete_server --bin ask_pete_server"

# Wait for server to initialize
Write-Host "   Waiting for server initialization..." -ForegroundColor Gray
Start-Sleep -Seconds 8

Write-Host ""

# Start Teacher Frontend
Write-Host "[3/3] Starting Teacher App (Authoring Tool)..." -ForegroundColor Yellow
Write-Host "   Port: 8082" -ForegroundColor Gray

Start-Process powershell -ArgumentList `
    "-NoExit", `
    "-Command", `
    "cd '$PSScriptRoot\crates\ask_pete_tools'; Write-Host 'Starting Teacher App...' -ForegroundColor Cyan; trunk serve"

Write-Host ""
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "  Demo Environment Starting..." -ForegroundColor Green
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Services:" -ForegroundColor White
Write-Host "  Backend API:  http://localhost:3000" -ForegroundColor Gray
Write-Host "  Teacher App:  http://localhost:8082" -ForegroundColor Gray
Write-Host ""
Write-Host "Wait ~30 seconds for all services to be ready." -ForegroundColor Yellow
Write-Host "Press Ctrl+C in each window to stop services." -ForegroundColor Gray
Write-Host ""

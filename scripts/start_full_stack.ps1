#!/usr/bin/env pwsh
# Full Stack Launch - Start Everything You Need
# This is THE script to run when you want to work on Ask Pete

Write-Host ""
Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  🚂 Ask Pete - Full Stack Launch" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Continue"

# Step 1: Start Backend
Write-Host "[1/2] Starting Backend Server (Port 3000)..." -ForegroundColor Yellow

Start-Process pwsh -ArgumentList "-NoExit", "-Command", `
    "cd '$PWD'; Write-Host '🔧 Backend Server Starting...' -ForegroundColor Green; cargo run --bin ask_pete_server"

Write-Host "  ✅ Backend starting in new window" -ForegroundColor Green
Start-Sleep -Seconds 3

# Step 2: Start Frontend
Write-Host ""
Write-Host "[2/2] Starting Frontend (Port 8080)..." -ForegroundColor Yellow

Start-Process pwsh -ArgumentList "-NoExit", "-Command", `
    "cd '$PWD\crates\ask_pete_node_garden'; Write-Host '🎨 Frontend Starting...' -ForegroundColor Green; trunk serve --open"

Write-Host "  ✅ Frontend starting in new window" -ForegroundColor Green

# Wait for services
Write-Host ""
Write-Host "Waiting for services to start..." -ForegroundColor Gray
Start-Sleep -Seconds 5

# Open browser
Write-Host ""
Write-Host "═══════════════════════════════════════════" -ForegroundColor Green
Write-Host "  ✅ LAUNCH COMPLETE!" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════" -ForegroundColor Green
Write-Host ""
Write-Host "Your authoring tool should open at:" -ForegroundColor Cyan
Write-Host "  http://127.0.0.1:8080/yard" -ForegroundColor White
Write-Host ""
Write-Host "If browser doesn't open, navigate manually to:" -ForegroundColor Yellow
Write-Host "  • Authoring: http://127.0.0.1:8080/yard" -ForegroundColor Gray
Write-Host "  • Student:   http://127.0.0.1:8080/play" -ForegroundColor Gray
Write-Host "  • API:       http://127.0.0.1:3000/api/health" -ForegroundColor Gray
Write-Host ""
Write-Host "Press Ctrl+C in the terminal windows to stop servers." -ForegroundColor DarkGray
Write-Host ""

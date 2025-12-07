#!/usr/bin/env pwsh
# Quick Launch Script - All Primary Components

Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  🚂 Ask Pete - MVP Quick Start" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

Write-Host "Available Commands:" -ForegroundColor Yellow
Write-Host "  1. Web Editor (Train Yard)" -ForegroundColor White
Write-Host "  2. Test Local AI" -ForegroundColor White
Write-Host "  3. Both" -ForegroundColor White
Write-Host ""

$choice = Read-Host "Select (1-3)"

switch ($choice) {
    "1" {
        Write-Host ""
        Write-Host "🌐 Launching Train Yard Web Editor..." -ForegroundColor Green
        Write-Host "   URL: http://127.0.0.1:5173" -ForegroundColor Cyan
        Write-Host ""
        cd crates\ask_pete_trainyard_web
        npm run dev
    }
    "2" {
        Write-Host ""
        Write-Host "🤖 Testing Local AI (Mistral 7B)..." -ForegroundColor Green
        Write-Host "   This will load the 4.3GB model..." -ForegroundColor Gray
        Write-Host ""
        cargo run --bin test_inference -p ask_pete_ai
    }
    "3" {
        Write-Host ""
        Write-Host "🚀 Launching both systems..." -ForegroundColor Green
        Write-Host ""
        
        # Launch web editor in background
        Start-Process pwsh -ArgumentList "-NoExit", "-Command", "cd crates\ask_pete_trainyard_web; npm run dev"
        
        # Wait a moment
        Start-Sleep -Seconds 2
        
        # Test AI in foreground
        Write-Host "🤖 Testing Local AI..." -ForegroundColor Cyan
        cargo run --bin test_inference -p ask_pete_ai
    }
    default {
        Write-Host "Invalid choice. Exiting." -ForegroundColor Red
    }
}

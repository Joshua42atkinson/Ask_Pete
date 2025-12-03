#!/usr/bin/env pwsh
# Launch Script for Ask Pete Train Yard Authoring Tool
# Usage: .\launch_trainyard.ps1

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet("web", "rust", "full")]
    [string]$Mode = "web"
)

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  🚂 ASK PETE TRAIN YARD AUTHORING TOOL              ║" -ForegroundColor Cyan
Write-Host "║  The Weigh Station for Cognitive Load Management     ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

function Launch-WebEditor {
    Write-Host "🌐 Launching Web-Based Visual Editor..." -ForegroundColor Green
    Write-Host ""
    Write-Host "Location: crates/ask_pete_trainyard_web" -ForegroundColor Gray
    Write-Host "Tech Stack: React + TypeScript + ReactFlow" -ForegroundColor Gray
    Write-Host ""
    
    Push-Location crates/ask_pete_trainyard_web
    
    if (-not (Test-Path "node_modules")) {
        Write-Host "📦 Installing dependencies..." -ForegroundColor Yellow
        npm install
    }
    
    Write-Host ""
    Write-Host "✨ Starting development server..." -ForegroundColor Green
    Write-Host "   URL: http://localhost:5173" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Features:" -ForegroundColor Yellow
    Write-Host "  ✓ Visual node-based graph editor" -ForegroundColor Gray
    Write-Host "  ✓ Drag-and-drop dialogue creation" -ForegroundColor Gray
    Write-Host "  ✓ Export to JSON" -ForegroundColor Gray
    Write-Host "  ✓ Live preview" -ForegroundColor Gray
    Write-Host ""
    
    npm run dev
    
    Pop-Location
}

function Launch-RustBackend {
    Write-Host "🦀 Launching Rust Backend Authoring Framework..." -ForegroundColor Green
    Write-Host ""
    Write-Host "Location: crates/ask_pete_trainyard" -ForegroundColor Gray
    Write-Host ""
    
    Write-Host "Available Rust Authoring Modules:" -ForegroundColor Yellow
    Write-Host "  • blueprint_station.rs - Blueprint generation" -ForegroundColor Gray
    Write-Host "  • node_canvas.rs - Visual canvas editing" -ForegroundColor Gray
    Write-Host "  • inspector.rs - Property inspector" -ForegroundColor Gray
    Write-Host "  • property_editor.rs - Edit node properties" -ForegroundColor Gray
    Write-Host "  • story_node.rs - Story data structures" -ForegroundColor Gray
    Write-Host "  • template_selector.rs - Pre-built templates" -ForegroundColor Gray
    Write-Host "  • word_smithy.rs - Vocabulary authoring" -ForegroundColor Gray
    Write-Host "  • owl_diagnostic.rs - AI diagnostics" -ForegroundColor Gray
    Write-Host ""
    
    Write-Host "📝 Running cargo check on ask_pete_trainyard..." -ForegroundColor Cyan
    cargo check -p ask_pete_trainyard
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Rust backend is ready!" -ForegroundColor Green
    } else {
        Write-Host "❌ Compilation errors detected." -ForegroundColor Red
    }
}

function Launch-FullSystem {
    Write-Host "🎯 Launching Full Integrated System..." -ForegroundColor Green
    Write-Host ""
    
    Write-Host "Step 1: Checking Rust backend..." -ForegroundColor Yellow
    Launch-RustBackend
    
    Write-Host ""
    Write-Host "Step 2: Launching Web Editor..." -ForegroundColor Yellow
    Launch-WebEditor
}

function Show-Help {
    Write-Host "Usage: .\launch_trainyard.ps1 [-Mode <mode>]" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Modes:" -ForegroundColor Yellow
    Write-Host "  web   - Launch React web editor (default)" -ForegroundColor Gray
    Write-Host "  rust  - Check Rust backend compilation" -ForegroundColor Gray
    Write-Host "  full  - Launch both web and backend" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Yellow
    Write-Host "  .\launch_trainyard.ps1" -ForegroundColor Gray
    Write-Host "  .\launch_trainyard.ps1 -Mode web" -ForegroundColor Gray
    Write-Host "  .\launch_trainyard.ps1 -Mode rust" -ForegroundColor Gray
    Write-Host ""
}

# Main execution
switch ($Mode) {
    "web" {
        Launch-WebEditor
    }
    "rust" {
        Launch-RustBackend
    }
    "full" {
        Launch-FullSystem
    }
    default {
        Show-Help
    }
}

Write-Host ""
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  Session Complete" -ForegroundColor Gray
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

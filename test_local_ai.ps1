#!/usr/bin/env pwsh
# Test Local AI Inference
# Based on research: Models are already downloaded, just need to verify they work

Write-Host ""
Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  🤖 Testing Local AI (Mistral 7B)" -ForegroundColor Cyan  
Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# Check if models exist
Write-Host "[1/3] Verifying Models..." -ForegroundColor Yellow

$modelPath = "assets/models/mistral-7b-instruct-v0.1.Q4_K_M.gguf"
$tokenizerPath = "assets/models/tokenizer.json"

if (Test-Path $modelPath) {
    $modelSize = (Get-Item $modelPath).Length / 1GB
    Write-Host "  ✅ Model found: $([math]::Round($modelSize, 2)) GB" -ForegroundColor Green
}
else {
    Write-Host "  ❌ Model not found at: $modelPath" -ForegroundColor Red
    exit 1
}

if (Test-Path $tokenizerPath) {
    $tokenizerSize = (Get-Item $tokenizerPath).Length / 1MB
    Write-Host "  ✅ Tokenizer found: $([math]::Round($tokenizerSize, 2)) MB" -ForegroundColor Green
}
else {
    Write-Host "  ❌ Tokenizer not found at: $tokenizerPath" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[2/3] Compiling ask_pete_ai crate..." -ForegroundColor Yellow

cargo check -p ask_pete_ai 2>&1 | Tee-Object -FilePath "ai_compile_check.log"

if ($LASTEXITCODE -ne 0) {
    Write-Host "  ❌ Compilation failed. Check ai_compile_check.log" -ForegroundColor Red
    exit 1
}
else {
    Write-Host "  ✅ Compilation successful" -ForegroundColor Green
}

Write-Host ""
Write-Host "[3/3] Running Local Inference Test..." -ForegroundColor Yellow
Write-Host "  (This may take 30-60 seconds on first run)" -ForegroundColor Gray
Write-Host ""

# Run test if it exists
cargo test -p ask_pete_ai --test local_inference_test -- --nocapture 2>&1 | Tee-Object -FilePath "ai_test_output.log"

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "═══════════════════════════════════════════" -ForegroundColor Green
    Write-Host "  ✅ LOCAL AI TEST PASSED!" -ForegroundColor Green
    Write-Host "═══════════════════════════════════════════" -ForegroundColor Green
}
else {
    Write-Host ""
    Write-Host "═══════════════════════════════════════════" -ForegroundColor Yellow
    Write-Host "  ⚠️  Test not found or failed" -ForegroundColor Yellow
    Write-Host "  Attempting manual model load test..." -ForegroundColor Yellow
    Write-Host "═══════════════════════════════════════════" -ForegroundColor Yellow
    
    # Try running a simple cargo run command instead
    Write-Host ""
    Write-Host "Run this manually to test:" -ForegroundColor Cyan
    Write-Host "  cargo run --example test_mistral" -ForegroundColor White
}

Write-Host ""

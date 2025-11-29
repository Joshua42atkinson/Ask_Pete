# Phase 0: Iron Chassis Stabilization - Commit Script

Write-Host "Phase 0: Committing Iron Chassis Stabilization changes..."

# Kill any hanging git processes
Get-Process | Where-Object { $_.ProcessName -like "*git*" -and $_.Id -ne $PID } | Stop-Process -Force -ErrorAction SilentlyContinue

# Wait a moment
Start-Sleep -Seconds 2

# Remove lock if it exists
if (Test-Path .git/index.lock) {
    Remove-Item -Force .git/index.lock -ErrorAction SilentlyContinue
}

# Stage the Phase 0 files
Write-Host "Staging Phase 0 files..."
git add backend/src/ai/local_inference.rs
git add backend/src/ai/mod.rs  
git add backend/src/ai/socratic_engine.rs
git add backend/src/handlers/weigh_station.rs
git add backend/src/main.rs
git add backend/Cargo.toml

# Commit
Write-Host "Committing..."
git commit -m "Phase 0: Iron Chassis Stabilization

- Created local_inference module with async GemmaModel
- Implemented spawn_blocking for non-blocking AI inference
- Refactored SocraticEngine to use local_inference::GemmaModel
- Updated WeighStation to use async generate()
- Downgraded bevy_defer to 0.12 to resolve dependency conflict
- Initialized bevy_defer::AsyncPlugin in main.rs

This establishes the foundation for the Hybrid Sovereign architecture."

# Push
Write-Host "Pushing to joshua42atkinson/ask_pete..."
git push origin HEAD

Write-Host "Done!"

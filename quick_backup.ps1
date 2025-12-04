#!/usr/bin/env pwsh
# Quick End-of-Day Backup - One command to save everything
# Usage: Just run .\quick_backup.ps1

Write-Host "💾 Quick Backup - Saving your work..." -ForegroundColor Cyan

# Stage everything
git add .

# Commit with timestamp
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm"
git commit -m "Quick backup: $timestamp"

# Try to push (but don't fail if offline)
Write-Host "Attempting GitHub push..." -ForegroundColor Yellow
git push origin main 2>&1 | Out-Null

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Backed up locally AND to GitHub!" -ForegroundColor Green
}
else {
    Write-Host "✅ Backed up locally (GitHub push failed - maybe offline?)" -ForegroundColor Yellow
}

Write-Host ""

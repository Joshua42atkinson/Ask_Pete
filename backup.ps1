#!/usr/bin/env pwsh
# Quick Backup Script - Save Your Work to Git
# Usage: .\backup.ps1 [optional message]

param(
    [string]$Message = ""
)

Write-Host ""
Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  💾 Ask Pete - Quick Backup" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# Check if we're in a git repo
if (-not (Test-Path ".git")) {
    Write-Host "❌ Error: Not in a git repository" -ForegroundColor Red
    Write-Host "   Run this script from the Ask_Pete folder" -ForegroundColor Yellow
    exit 1
}

# Show current status
Write-Host "[1/4] Checking what's changed..." -ForegroundColor Yellow
$status = git status --short
if ($status) {
    Write-Host ""
    Write-Host "Files changed:" -ForegroundColor Cyan
    git status --short | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
    Write-Host ""
}
else {
    Write-Host "  ✅ No changes to commit (everything is backed up)" -ForegroundColor Green
    Write-Host ""
    $push = Read-Host "Do you want to push existing commits to GitHub? (y/n)"
    if ($push -eq "y") {
        Write-Host ""
        Write-Host "[Pushing to GitHub...]" -ForegroundColor Yellow
        git push origin main
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Pushed to GitHub successfully!" -ForegroundColor Green
        }
        else {
            Write-Host "⚠️  Push failed. Check your internet connection or GitHub auth." -ForegroundColor Yellow
        }
    }
    exit 0
}

# Get commit message
if ($Message -eq "") {
    Write-Host "What did you work on today?" -ForegroundColor Cyan
    Write-Host "Examples:" -ForegroundColor Gray
    Write-Host "  - Added blueprint generation UI" -ForegroundColor DarkGray
    Write-Host "  - Fixed local AI inference" -ForegroundColor DarkGray
    Write-Host "  - Daily checkpoint" -ForegroundColor DarkGray
    Write-Host ""
    $Message = Read-Host "Commit message"
    
    if ($Message -eq "") {
        $Message = "Daily checkpoint: $(Get-Date -Format 'yyyy-MM-dd HH:mm')"
        Write-Host "  Using default: $Message" -ForegroundColor Yellow
    }
}

# Stage all changes
Write-Host ""
Write-Host "[2/4] Staging all changes..." -ForegroundColor Yellow
git add .

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Failed to stage files" -ForegroundColor Red
    exit 1
}
Write-Host "  ✅ Staged" -ForegroundColor Green

# Commit locally
Write-Host ""
Write-Host "[3/4] Committing locally..." -ForegroundColor Yellow
git commit -m "$Message"

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Commit failed" -ForegroundColor Red
    exit 1
}
Write-Host "  ✅ Local commit saved" -ForegroundColor Green

# Ask about pushing to GitHub
Write-Host ""
Write-Host "[4/4] Backup to GitHub?" -ForegroundColor Yellow
Write-Host "  This uploads your work to the cloud (recommended!)" -ForegroundColor Gray
Write-Host ""
$push = Read-Host "Push to GitHub now? (y/n)"

if ($push -eq "y") {
    Write-Host ""
    Write-Host "Pushing to GitHub..." -ForegroundColor Cyan
    git push origin main
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "═══════════════════════════════════════════" -ForegroundColor Green
        Write-Host "  ✅ SUCCESS! Work backed up locally AND to GitHub" -ForegroundColor Green
        Write-Host "═══════════════════════════════════════════" -ForegroundColor Green
    }
    else {
        Write-Host ""
        Write-Host "═══════════════════════════════════════════" -ForegroundColor Yellow
        Write-Host "  ⚠️  Local backup OK, but GitHub push failed" -ForegroundColor Yellow
        Write-Host "═══════════════════════════════════════════" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "Common causes:" -ForegroundColor Cyan
        Write-Host "  • Not connected to internet" -ForegroundColor Gray
        Write-Host "  • GitHub credentials expired" -ForegroundColor Gray
        Write-Host "  • Remote branch diverged" -ForegroundColor Gray
        Write-Host ""
        Write-Host "Your work is still safe locally!" -ForegroundColor Green
        Write-Host "Try: git push origin main" -ForegroundColor White
    }
}
else {
    Write-Host ""
    Write-Host "═══════════════════════════════════════════" -ForegroundColor Green
    Write-Host "  ✅ Local backup complete!" -ForegroundColor Green
    Write-Host "═══════════════════════════════════════════" -ForegroundColor Green
    Write-Host ""
    Write-Host "Your work is saved locally." -ForegroundColor White
    Write-Host "Run this script again later to push to GitHub." -ForegroundColor Gray
}

Write-Host ""
Write-Host "Tip: Run this script daily to protect your work! 💪" -ForegroundColor Cyan
Write-Host ""

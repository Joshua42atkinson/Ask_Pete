# Ticket 1: The Engine (Master Installer)
# install.ps1 - The Conductor that orchestrates the entire setup

Add-Type -AssemblyName System.Windows.Forms

function Show-Notification {
    param (
        [string]$Title,
        [string]$Message,
        [string]$Icon = "Information"
    )
    [System.Windows.Forms.MessageBox]::Show($Message, $Title, "OK", $Icon)
}

Write-Host "🚂 Ask Pete: All Aboard!" -ForegroundColor Yellow
Write-Host "Starting installation sequence..."

# 1. Check Prerequisites
if (-not (Get-Command "docker" -ErrorAction SilentlyContinue)) {
    Show-Notification -Title "Missing Ticket" -Message "Docker Desktop is not installed. Please install it and try again." -Icon "Error"
    exit 1
}

# 2. Run Setup (The Coal)
Write-Host "`n[1/3] Loading Coal (Configuration)..." -ForegroundColor Cyan
try {
    & ".\setup.ps1"
    if ($LASTEXITCODE -ne 0) { throw "Setup failed" }
}
catch {
    Show-Notification -Title "Derailment" -Message "Setup script failed. Check the console for errors." -Icon "Error"
    exit 1
}

# 3. Run Fetch Models (The Steam)
Write-Host "`n[2/3] Building Steam (Downloading AI Models)..." -ForegroundColor Cyan
try {
    & ".\fetch_models.ps1"
    if ($LASTEXITCODE -ne 0) { throw "Model download failed" }
}
catch {
    Show-Notification -Title "Derailment" -Message "Failed to download AI models. Check your internet connection." -Icon "Error"
    exit 1
}

# 4. Start Engine
Write-Host "`n[3/3] Starting Engine (Docker Containers)..." -ForegroundColor Cyan
try {
    docker-compose up -d
    if ($LASTEXITCODE -ne 0) { throw "Docker Compose failed" }
}
catch {
    Show-Notification -Title "Derailment" -Message "Failed to start the engine. Is Docker running?" -Icon "Error"
    exit 1
}

# 5. Success!
Write-Host "`n✅ TRAIN IS READY!" -ForegroundColor Green
Show-Notification -Title "All Aboard!" -Message "Ask Pete is installed and running!`n`nAccess the Train Yard at: http://localhost:8080" -Icon "Information"

# Optional: Open Browser
Start-Process "http://localhost:8080"

# Ticket 2: The Coal (Configuration & Activation)
# setup.ps1 - Prepares the environment for Ask Pete

Write-Host "🚂 Ask Pete: Station Setup" -ForegroundColor Yellow

# 1. Check for Docker
if (-not (Get-Command "docker" -ErrorAction SilentlyContinue)) {
    Write-Error "Docker is not installed. Please install Docker Desktop first."
    exit 1
}

# 2. Check for .env
if (-not (Test-Path "..\backend\.env")) {
    Write-Host "Creating .env from template..."
    Copy-Item ".env.example" "..\backend\.env"
    Write-Host "⚠️  IMPORTANT: Please edit backend/.env and add your GEMINI_API_KEY!" -ForegroundColor Red
} else {
    Write-Host "✅ .env file found." -ForegroundColor Green
}

# 3. Create Model Directories
$modelPath = "..\backend\models"
if (-not (Test-Path $modelPath)) {
    New-Item -ItemType Directory -Force -Path $modelPath | Out-Null
    Write-Host "Created models directory at $modelPath"
}

Write-Host "✅ Setup Complete. Next steps:"
Write-Host "1. Run '.\fetch_models.ps1' to download AI models."
Write-Host "2. Run 'docker-compose up' to start the train."

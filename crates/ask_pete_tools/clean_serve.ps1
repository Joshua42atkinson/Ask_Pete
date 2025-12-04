Write-Host "Stopping existing trunk processes..."
Get-Process trunk -ErrorAction SilentlyContinue | Stop-Process -Force

Write-Host "Cleaning dist directory..."
if (Test-Path "dist") {
    Remove-Item -Recurse -Force "dist"
}

Write-Host "Starting trunk serve..."
trunk serve

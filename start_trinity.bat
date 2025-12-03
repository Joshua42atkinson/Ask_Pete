@echo off
echo ===================================================
echo   THE TRINITY - DAYDREAM SYSTEM LAUNCHER
echo ===================================================
echo.
echo [1/4] Starting Iron Chassis (Backend) on Port 3000...
start "Iron Chassis (Backend)" cmd /k "cd apps\grand-central && cargo run"

echo [2/4] Starting Researcher Interface on Port 8081...
start "Researcher UI" cmd /k "cd apps\researcher && trunk serve"

echo [3/4] Starting Teacher Interface on Port 8082...
start "Teacher UI" cmd /k "cd apps\teacher && trunk serve"

echo [4/4] Starting Student Interface on Port 8083...
start "Student UI" cmd /k "cd apps\player && trunk serve"

echo.
echo ===================================================
echo   SYSTEMS ONLINE
echo   - Backend:    http://localhost:3000
echo   - Researcher: http://localhost:8081
echo   - Teacher:    http://localhost:8082
echo   - Student:    http://localhost:8083
echo ===================================================
echo.
pause

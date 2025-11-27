@echo off
title Ask Pete Engine Launcher
color 0A

echo ===================================================
echo       Ask Pete - "Socratic Engineering"
echo ===================================================
echo.
echo Checking environment...

where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Rust (cargo) is not installed!
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b
)
title Ask Pete Engine Launcher
color 0A

echo ===================================================
echo       Ask Pete Engine - "Boilermaker Approved"
echo ===================================================
echo.
echo Checking environment...

where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Rust (cargo) is not installed!
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b
)

where trunk >nul 2>nul
if %errorlevel% neq 0 (
    echo [WARNING] Trunk is not installed. Installing...
    cargo install trunk
)

echo.
echo Starting Backend Server...
start "Ask Pete Backend" cmd /k "cd backend && cargo run"

echo.
echo Starting Frontend Client...
start "Ask Pete Frontend" cmd /k "cd frontend && trunk serve --open"

echo.
echo ===================================================
echo       Ask Pete is launching!
echo       Please wait for the browser to open...
echo ===================================================
echo.
pause

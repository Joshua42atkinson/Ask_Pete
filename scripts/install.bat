@echo off
echo 🚂 Launching Ask Pete Installer...
cd deployment
powershell -ExecutionPolicy Bypass -File ".\install.ps1"
pause

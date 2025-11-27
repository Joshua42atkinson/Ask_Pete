#!/bin/bash

echo "==================================================="
echo "      Ask Pete - \"Socratic Engineering\""
echo "==================================================="
echo ""
echo "Checking environment..."

if ! command -v cargo &> /dev/null; then
    echo "[ERROR] Rust (cargo) is not installed!"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

if ! command -v trunk &> /dev/null; then
    echo "[WARNING] Trunk is not installed. Installing..."
    cargo install trunk
fi

echo ""
echo "Starting Backend Server..."
cd backend && cargo run &
BACKEND_PID=$!

echo ""
echo "Starting Frontend Client..."
cd ../frontend && trunk serve --open &
FRONTEND_PID=$!

echo ""
echo "==================================================="
echo "      Ask Pete is launching!"
echo "      Please wait for the browser to open..."
echo "==================================================="
echo ""

# Trap Ctrl+C to kill both processes
trap "kill $BACKEND_PID $FRONTEND_PID; exit" SIGINT

wait

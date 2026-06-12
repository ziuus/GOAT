#!/usr/bin/env bash
set -e

echo "======================================"
echo " GOAT Public Alpha Smoke Test"
echo "======================================"

echo "[1/7] Running cargo check..."
cargo check

echo "[2/7] Running cargo test..."
cargo test

echo "[3/7] Building the dashboard..."
cd apps/dashboard
npm install
npm run build
cd ../..

echo "[4/7] Running goat doctor..."
cargo run --release -- doctor

echo "[5/7] Verifying demo seed workflow..."
cargo run --release -- seed-demo --clear
cargo run --release -- seed-demo

echo "[6/7] Checking Git availability..."
git --version || echo "Git not found, but it is optional"

echo "[7/7] Checking mandatory docs..."
if [ ! -f "docs/README.md" ]; then
    echo "ERROR: docs/README.md missing"
    exit 1
fi
if [ ! -f "README.md" ]; then
    echo "ERROR: README.md missing"
    exit 1
fi

echo "======================================"
echo " Smoke test completed successfully!"
echo "======================================"

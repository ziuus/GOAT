#!/usr/bin/env bash
set -euo pipefail

echo "============================================================"
echo "🐐 GOAT Alpha Smoke Test 🐐"
echo "============================================================"

BIN="cargo run --quiet --bin goat --"

# Allow passing a binary path if testing a compiled release
if [ $# -eq 1 ]; then
    BIN="$1"
    echo "Testing compiled binary at: $BIN"
else
    echo "Testing via cargo run..."
fi

echo "Running checks..."

echo "1/6 Checking version..."
$BIN --version

echo "2/6 Running doctor alpha..."
$BIN doctor alpha

echo "3/6 Checking quickstart output..."
$BIN quickstart | grep "GOAT ALPHA 1 QUICKSTART"

echo "4/6 Checking tools list..."
$BIN tools list

echo "5/6 Checking tools doctor..."
$BIN tools doctor

echo "6/6 Checking root help..."
$BIN help | grep "GOAT"

echo "✅ All smoke tests passed successfully!"

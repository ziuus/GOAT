#!/usr/bin/env bash
set -e

echo "Building GOAT Release Artifacts..."

# Verify dependencies
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is required to build GOAT"
    exit 1
fi

if ! command -v npm &> /dev/null; then
    echo "Error: npm is required to build the GOAT dashboard"
    exit 1
fi

# Clean previous dist
rm -rf dist
mkdir -p dist

echo "1/2 Building Rust daemon (release profile)..."
cargo build --release

echo "2/2 Building Next.js Dashboard..."
cd apps/dashboard
npm install
npm run build
cd ../..

echo "Collecting artifacts into dist/..."
cp target/release/goat dist/
cp goat.toml dist/
cp scripts/install.sh dist/
cp scripts/uninstall.sh dist/
cp README.md dist/

# Copy dashboard static files
mkdir -p dist/dashboard
cp -r apps/dashboard/out/* dist/dashboard/

echo "Build complete. Artifacts are in dist/"

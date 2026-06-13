#!/usr/bin/env bash
set -euo pipefail

echo "============================================================"
echo "🐐 GOAT Alpha 1 Local Installer 🐐"
echo "============================================================"

# Ensure we are in the project root
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the GOAT project root."
    exit 1
fi

echo "📦 Building GOAT in release mode..."
cargo build --release --bin goat

BIN_PATH="target/release/goat"
if [ ! -f "$BIN_PATH" ]; then
    echo "❌ Error: Build failed, could not find binary at $BIN_PATH."
    exit 1
fi

DEST_DIR="$HOME/.local/bin"
DEST_PATH="$DEST_DIR/goat"

echo "📂 Destination: $DEST_DIR"

if [ ! -d "$DEST_DIR" ]; then
    read -p "Directory $DEST_DIR does not exist. Create it? [y/N]: " create_dir
    if [[ "$create_dir" =~ ^[Yy]$ ]]; then
        mkdir -p "$DEST_DIR"
    else
        echo "❌ Installation aborted."
        exit 1
    fi
fi

read -p "Install GOAT to $DEST_PATH? [y/N]: " install_bin
if [[ "$install_bin" =~ ^[Yy]$ ]]; then
    cp "$BIN_PATH" "$DEST_PATH"
    echo "✅ Successfully installed GOAT to $DEST_PATH"
    echo "💡 Make sure $DEST_DIR is in your PATH."
else
    echo "Installation skipped. You can run the binary directly from $BIN_PATH"
fi

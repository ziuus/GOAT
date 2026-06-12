#!/usr/bin/env bash
set -e

echo "Installing GOAT..."

BIN_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/goat"
DATA_DIR="$HOME/.local/share/goat"

# Create directories
mkdir -p "$BIN_DIR"
mkdir -p "$CONFIG_DIR"
mkdir -p "$DATA_DIR/dashboard"

# Install binary
if [ -f "./goat" ]; then
    echo "Installing binary to $BIN_DIR/goat"
    cp ./goat "$BIN_DIR/goat"
    chmod +x "$BIN_DIR/goat"
else
    echo "Error: goat binary not found in current directory."
    exit 1
fi

# Install config
if [ -f "./goat.toml" ]; then
    if [ ! -f "$CONFIG_DIR/goat.toml" ]; then
        echo "Installing default config to $CONFIG_DIR/goat.toml"
        cp ./goat.toml "$CONFIG_DIR/"
    else
        echo "Config already exists at $CONFIG_DIR/goat.toml. Skipping."
    fi
fi

# Install dashboard
if [ -d "./dashboard" ]; then
    echo "Installing dashboard assets to $DATA_DIR/dashboard/"
    cp -r ./dashboard/* "$DATA_DIR/dashboard/"
fi

echo "GOAT installed successfully!"
echo "Ensure $BIN_DIR is in your PATH."
echo "Run 'goat' to start the Mission Control."

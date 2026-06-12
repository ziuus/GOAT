#!/usr/bin/env bash

BIN_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/goat"
DATA_DIR="$HOME/.local/share/goat"

echo "Uninstalling GOAT..."

if [ -f "$BIN_DIR/goat" ]; then
    echo "Removing binary at $BIN_DIR/goat..."
    rm "$BIN_DIR/goat"
else
    echo "Binary not found at $BIN_DIR/goat."
fi

echo ""
read -p "Do you want to delete all configuration files in $CONFIG_DIR? [y/N] " confirm_config
if [[ "$confirm_config" =~ ^[Yy]$ ]]; then
    rm -rf "$CONFIG_DIR"
    echo "Configuration removed."
else
    echo "Keeping configuration."
fi

echo ""
read -p "Do you want to delete all data (Brain database, logs, dashboard) in $DATA_DIR? [y/N] " confirm_data
if [[ "$confirm_data" =~ ^[Yy]$ ]]; then
    rm -rf "$DATA_DIR"
    echo "Data removed."
else
    echo "Keeping data."
fi

echo "GOAT uninstalled."

#!/usr/bin/env bash
set -e

# Real Workspace Validation Suite for GOAT Alpha 1

echo "====================================================="
echo "   GOAT Alpha 1 : Real Workspace Test Suite"
echo "====================================================="

# Ensure goat is built
echo "[*] Building goat binary in debug mode..."
cargo build --bin goat -q

GOAT_BIN="$(pwd)/target/debug/goat"
FIXTURES_DIR="$(pwd)/tests/fixtures/workspaces"

if [ ! -d "$FIXTURES_DIR" ]; then
    echo "[!] Error: Fixtures directory not found at $FIXTURES_DIR"
    exit 1
fi

test_workspace() {
    local ws_name=$1
    local ws_path="$FIXTURES_DIR/$ws_name"
    
    echo ""
    echo "-----------------------------------------------------"
    echo " Testing Workspace: $ws_name"
    echo " Path: $ws_path"
    echo "-----------------------------------------------------"
    
    # Enter the workspace to test local detection
    pushd "$ws_path" > /dev/null

    echo "  [+] Testing 'goat doctor alpha'..."
    if ! "$GOAT_BIN" doctor alpha > /dev/null; then
        echo "      FAILED: doctor alpha"
        popd > /dev/null
        return 1
    fi

    echo "  [+] Testing 'goat tools doctor'..."
    if ! "$GOAT_BIN" tools doctor > /dev/null; then
        echo "      FAILED: tools doctor"
        popd > /dev/null
        return 1
    fi

    echo "  [+] Testing 'goat tools list'..."
    if ! "$GOAT_BIN" tools list > /dev/null; then
        echo "      FAILED: tools list"
        popd > /dev/null
        return 1
    fi
    
    # Test learn (safe read-only command)
    echo "  [+] Testing 'echo y | goat learn .'..."
    if ! echo "y" | "$GOAT_BIN" learn . > /dev/null; then
        echo "      FAILED: learn ."
        popd > /dev/null
        return 1
    fi

    popd > /dev/null
    echo "  [v] Workspace $ws_name passed."
    return 0
}

# Run tests on each fixture
FAILURES=0

for ws in rust_mini node_mini python_mini generic with_extension; do
    if ! test_workspace "$ws"; then
        FAILURES=$((FAILURES + 1))
    fi
done

echo ""
echo "====================================================="
echo " Global Commands Test"
echo "====================================================="
echo "  [+] Testing 'goat quickstart' (safe mode)..."
if ! "$GOAT_BIN" quickstart > /dev/null; then
    echo "      FAILED: quickstart"
    FAILURES=$((FAILURES + 1))
fi

echo ""
echo "====================================================="
if [ $FAILURES -eq 0 ]; then
    echo " ALL TESTS PASSED."
    exit 0
else
    echo " $FAILURES TEST(S) FAILED."
    exit 1
fi

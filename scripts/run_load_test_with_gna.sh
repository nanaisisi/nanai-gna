#!/usr/bin/env bash
set -euo pipefail

# Collect args passed through to the example
EXTRA_ARGS=("$@")
ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
BUILD_ROOT="$ROOT_DIR/gna/build"

# Find library file
echo "Searching for GNA library under $BUILD_ROOT"
LIB_FILE=$(find "$BUILD_ROOT" -type f \( -name 'libgna.so' -o -name 'gna.so' -o -name 'libgnad.so' -o -name 'gna.dll' -o -name 'gna.lib' \) | head -n 1 || true)
if [ -z "$LIB_FILE" ]; then
    echo "GNA library not found. Build GNA first (cargo make build-gna) or set GNA_LIB_DIR to the folder containing libgna.so / gna.lib" >&2
    exit 1
fi

LIB_DIR=$(dirname "$LIB_FILE")
echo "Found GNA library: $LIB_FILE"
echo "Using GNA_LIB_DIR=$LIB_DIR"

export GNA_LIB_DIR="$LIB_DIR"

# Run cargo with feature and forward args
echo "Running: cargo run --features link_gna --example load_test -- ${EXTRA_ARGS[*]}"
cargo run --features link_gna --example load_test -- "${EXTRA_ARGS[@]}"

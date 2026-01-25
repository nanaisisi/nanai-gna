#!/usr/bin/env bash
set -euo pipefail

CONFIG=${1:-Release}
ARCH=${2:-x64}
ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
GNA_SRC="$ROOT_DIR/gna"
BUILD_DIR="$GNA_SRC/build"

echo "Configuring GNA (src=$GNA_SRC build=$BUILD_DIR)"
cmake -S "$GNA_SRC" -B "$BUILD_DIR" -DCMAKE_BUILD_TYPE=$CONFIG

echo "Building GNA (config=$CONFIG)"
cmake --build "$BUILD_DIR" --config $CONFIG --target gna

echo "Build finished. Run ./scripts/run_load_test_with_gna.sh to set GNA_LIB_DIR and run the example."

#!/usr/bin/env sh
set -eu

mkdir -p sbom
if command -v cargo-sbom >/dev/null 2>&1; then
    cargo sbom --output-format spdx_json_2_3 > sbom/sagnir.spdx.json
else
    cargo metadata --format-version 1 > sbom/sagnir.cargo-metadata.json
    echo "cargo-sbom not installed; wrote Cargo metadata fallback" >&2
fi

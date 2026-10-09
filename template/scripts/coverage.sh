#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo-tarpaulin >/dev/null 2>&1; then
    cargo install cargo-tarpaulin --locked
fi

cargo tarpaulin \
    --workspace \
    --all-features \
    --out Lcov \
    --output-dir . \
    --skip-clean

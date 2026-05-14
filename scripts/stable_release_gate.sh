#!/usr/bin/env sh
set -eu

mode="${1:-check}"

case "$mode" in
    check | release)
        ;;
    *)
        echo "usage: scripts/stable_release_gate.sh [check|release]" >&2
        exit 2
        ;;
esac

echo "stable release gate: planning checks"
scripts/checks.sh

if [ -f Cargo.toml ]; then
    echo "stable release gate: docs"
    cargo doc --workspace --no-deps
fi

if [ "$mode" = "release" ]; then
    echo "stable release gate: provider smoke, SBOM, audit, deny, and reproducibility gates are planned after the first backend implementation"
fi

echo "stable release gate: ok ($mode)"

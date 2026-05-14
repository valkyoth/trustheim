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

if [ "$mode" = "release" ]; then
    echo "stable release gate: no implementation exists yet; v0.1 may only be tagged as a planning release"
fi

echo "stable release gate: ok ($mode)"

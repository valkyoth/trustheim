#!/usr/bin/env sh
set -eu

echo "checks: required planning files"
test -s README.md
test -s LICENSE
test -s SECURITY.md
test -s docs/architecture.md
test -s docs/backend-provider-interface.md
test -s docs/versioning-plan.md
test -s docs/openbao-operations-plan.md
test -s docs/security-and-release-gates.md
test -s docs/research-sources.md
test -s deny.toml

echo "checks: EUPL boundary"
grep -q "EUPL-1.2" LICENSE
grep -q "EUPL-1.2" README.md
grep -q "EUPL-1.2" deny.toml

echo "checks: security boundary"
grep -q "must not store CA private keys" SECURITY.md
grep -q "PKCS#11 seal alone" SECURITY.md
grep -q "runtime orchestrator token must not reach root CA paths" SECURITY.md
grep -q "provider-neutral" docs/backend-provider-interface.md
grep -q "trustheim-backend-vault" docs/backend-provider-interface.md

echo "checks: version ladder"
grep -q "v1.0: Stable Private CA Core" docs/versioning-plan.md
grep -q "v1.1: ACME And Renewal Automation" docs/versioning-plan.md
grep -q "v1.2: HA, DR, And Operations Evidence" docs/versioning-plan.md
grep -q "v1.4: Additional Backend Provider" docs/versioning-plan.md

echo "checks: source list"
grep -q "https://openbao.org/api-docs/secret/pki/" docs/research-sources.md
grep -q "https://developer.hashicorp.com/vault/docs/secrets/pki" docs/research-sources.md
grep -q "https://releases.rs/" docs/research-sources.md

if [ -f Cargo.toml ]; then
    echo "checks: formatting"
    cargo fmt --all --check

    echo "checks: clippy"
    cargo clippy --workspace --all-targets --all-features -- -D warnings

    echo "checks: tests"
    cargo test --workspace --all-targets

    if [ "${TRUSTHEIM_CHECK_API_SMOKE:-0}" = "1" ]; then
        echo "checks: local API smoke"
        scripts/smoke_api_local.sh
    else
        echo "checks: skipping local API smoke; set TRUSTHEIM_CHECK_API_SMOKE=1 to enable"
    fi

    if cargo deny --version >/dev/null 2>&1; then
        echo "checks: dependency policy"
        cargo deny check
    else
        echo "checks: skipping cargo deny; cargo-deny is not installed"
    fi

    if cargo audit --version >/dev/null 2>&1; then
        echo "checks: RustSec advisories"
        cargo audit
    else
        echo "checks: skipping cargo audit; cargo-audit is not installed"
    fi
fi

echo "checks: ok"

#!/usr/bin/env sh
set -eu

echo "checks: required planning files"
test -s README.md
test -s LICENSE
test -s SECURITY.md
test -s docs/architecture.md
test -s docs/app-boundaries.md
test -s docs/api.md
test -s docs/backend-provider-interface.md
test -s docs/versioning-plan.md
test -s docs/openbao-operations-plan.md
test -s docs/deploy/native-binaries.md
test -s docs/deploy/podman/README.md
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
grep -q "trustheim-api-server" docs/app-boundaries.md
grep -q "trustheim-web" docs/app-boundaries.md
grep -q "trustheim-cli" docs/app-boundaries.md

echo "checks: app dependency boundaries"
if rg -n "trustheim-backend|trustheim-backend-openbao|trustheim-backend-vault" crates/trustheim-web crates/trustheim-cli; then
    echo "web and CLI crates must not depend on backend provider crates" >&2
    exit 1
fi

echo "checks: version ladder"
grep -q "offline ceremony-package format" docs/versioning-plan.md
grep -q "Ceremony package golden vectors" docs/versioning-plan.md
grep -q "v0.7.0: Storage Architecture And Pending Artifacts" docs/versioning-plan.md
grep -q "Immediate physical erasure may not be" docs/versioning-plan.md
grep -q "non-cloneable redacted secret wrappers" docs/versioning-plan.md
grep -q "trust-anchor distribution" docs/versioning-plan.md
grep -q "v0.19.0: Strict PKCS#10 CSR Parser" docs/versioning-plan.md
grep -q "Verify PKCS#10 proof of possession" docs/versioning-plan.md
grep -q "v0.21.0: Certificate Profile And Local Policy Engine" docs/versioning-plan.md
grep -q "trustheim/manifest/v1" docs/versioning-plan.md
grep -q "format-evolution rules" docs/versioning-plan.md
grep -q "Each signed-object schema version defines exactly one signature construction" docs/versioning-plan.md
grep -q "unknown fields are never stripped" docs/versioning-plan.md
grep -q "trustheim/display-receipt/v1" docs/versioning-plan.md
grep -q "hard size and cardinality limits" docs/versioning-plan.md
grep -q "v0.33.0: Provider Policy Drift Detection" docs/versioning-plan.md
grep -q "dedicated read-only evidence/drift identity" docs/versioning-plan.md
grep -q "v0.37.0: Transactional Reconciliation Before Provider Effects" docs/versioning-plan.md
grep -q "Authorization intent is durably recorded before provider credential" docs/versioning-plan.md
grep -q "CRL and OCSP signing keys in the provider, HSM" docs/versioning-plan.md
grep -q "v0.47.0: Second Provider Proof" docs/versioning-plan.md
grep -q "v0.61.0: Release Candidate And Independent Assessment" docs/versioning-plan.md
grep -q "v1.0.0: Stable Custody-Free CA Coordinator" docs/versioning-plan.md

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

    echo "checks: standalone binaries"
    cargo build --workspace --bins

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

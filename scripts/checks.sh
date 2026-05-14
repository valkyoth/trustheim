#!/usr/bin/env sh
set -eu

echo "checks: required planning files"
test -s README.md
test -s LICENSE
test -s SECURITY.md
test -s docs/architecture.md
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

echo "checks: version ladder"
grep -q "v1.0: Stable Private CA Core" docs/versioning-plan.md
grep -q "v1.1: ACME And Renewal Automation" docs/versioning-plan.md
grep -q "v1.2: HA, DR, And Operations Evidence" docs/versioning-plan.md

echo "checks: source list"
grep -q "https://openbao.org/api-docs/secret/pki/" docs/research-sources.md
grep -q "https://releases.rs/" docs/research-sources.md

echo "checks: ok"

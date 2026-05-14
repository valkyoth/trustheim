# Security Policy

Trustheim is planned security-sensitive CA infrastructure. Until v1.0, treat
all releases as non-production unless the release notes say otherwise and link
to passing gate evidence.

## Reporting

Do not publish exploitable details in public issues before a fix exists. Use
GitHub private security advisories once enabled for this repository, or contact
the maintainers directly.

## Routine Checks

Planning stage:

```bash
scripts/checks.sh
scripts/stable_release_gate.sh check
```

Implementation stage must add:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo deny check
cargo audit
scripts/stable_release_gate.sh release
```

## Non-Negotiable Boundaries

- Trustheim must not store CA private keys.
- Root and intermediate CA operations require quorum or offline ceremony.
- The runtime orchestrator token must not reach root CA paths.
- OpenBao audit must be enabled before certificate operations.
- HSM-backed signing claims must be proven by tests against the deployed
  OpenBao/HSM integration, not inferred from PKCS#11 seal alone.
- EUPL-1.2 compatibility must be checked for every dependency.

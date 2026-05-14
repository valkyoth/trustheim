# Security And Release Gates

Trustheim is security infrastructure. A release is not complete until it
produces evidence.

## Baseline Rules

- Use current stable Rust at release time.
- Prefer minimal dependencies and standard library code where the behavior is
  small enough to implement and test locally.
- Use established crates for complex security protocols such as WebAuthn, TLS,
  HTTP, ASN.1/X.509 parsing, and OpenAPI generation.
- Forbid direct unsafe code in Trustheim crates unless a future design record
  proves it is unavoidable.
- Keep OpenBao root tokens and HSM PINs out of normal runtime.
- Do not accept prerelease crates for stable releases without a written
  security exception.
- Keep the Trustheim public API provider-neutral. Backend-specific mount paths,
  issuer ids, roles, and tokens belong in provider adapters and internal audit
  metadata.

## Required Local Gates

The first Rust implementation must make `scripts/checks.sh` run:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo deny check
cargo audit
scripts/generate-sbom.sh
scripts/reproducible_build_check.sh
scripts/openbao_policy_smoke.sh
scripts/api_contract_check.sh
scripts/backend_contract_check.sh
```

The stable gate must add:

```bash
scripts/podman_smoke.sh
scripts/openbao_bootstrap_smoke.sh
scripts/hsm_signing_smoke.sh
scripts/audit_sink_smoke.sh
scripts/revocation_smoke.sh
scripts/dast_local.sh
```

Optional but expected before v1.0:

- `cargo nextest`.
- `cargo geiger` inventory review.
- `cargo fuzz` target build and selected short fuzz runs.
- Miri for pure logic crates where feasible.
- Kani or proptest for policy-state invariants where useful.

## Dependency Policy

`deny.toml` must deny unknown registries and unknown git sources. License
allow-list must be compatible with EUPL-1.2 distribution and kept narrow.

Direct dependencies planned for early implementation, based on checks performed
on 2026-05-14:

- `axum` 0.8.9 for HTTP routing.
- `tokio` 1.52.3 for async runtime.
- `tower-http` 0.6.10 for HTTP middleware.
- `utoipa` 5.5.0 for OpenAPI.
- `webauthn-rs` stable 0.5.5 for WebAuthn. The registry also lists
  `0.6.1-dev`; stable releases should not use the dev line without exception.
- `rustls`: current registry maximum is `0.24.0-dev.0`; stable releases should
  pin the latest non-prerelease compatible line unless the dev line is
  explicitly accepted.

Every dependency addition must answer:

- Why is this dependency needed?
- Can the standard library or existing dependency do it safely?
- Is the crate maintained?
- Is the license compatible?
- Does it bring native code, build scripts, unsafe, crypto, parsing, or network
  exposure?
- What tests prove our usage is safe?

## CI And GitHub

Use GitHub default CodeQL setup. Do not add an advanced CodeQL workflow unless
default setup is disabled; GitHub rejects duplicate SARIF analysis for the same
language in many configurations.

CI must run:

- Formatting.
- Linting.
- Tests.
- Documentation checks.
- Dependency policy.
- Audit.
- OpenAPI generation consistency.
- Backend provider contract tests.
- Local OpenBao policy smoke when container runtime is available.

## Release Evidence

Stable release notes must include:

- Rust version.
- OpenBao version and image digest.
- HSM model or lab simulator note.
- `scripts/stable_release_gate.sh release` result.
- SBOM checksums.
- Source archive checksum.
- Binary checksum.
- Container digest.
- Signed tag verification line.
- Known residual risks and accepted exceptions.

## Security Review Checklist

Before v1.0:

- Threat model reviewed after every architecture change.
- Every API route has authorization tests.
- Every OpenBao allowed path has a corresponding denied-path test.
- WebAuthn replay and origin tests exist.
- Quorum cannot be satisfied by one identity or one authenticator cloned across
  accounts.
- CSR parser fuzz target exists.
- SAN normalization is tested against IDNA, wildcards, empty labels, trailing
  dots, IP literals, and control characters.
- Revocation and issuer rotation are covered by smoke tests.
- Audit records are append-only and externally exported.
- Backup restore has been tested in an isolated environment.
- No root token or HSM PIN appears in logs, test fixtures, or release artifacts.

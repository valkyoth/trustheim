# Versioning Plan

Trustheim uses SemVer with a conservative stability rule: a feature is not
stable until the API, threat model, tests, OpenBao automation, operational docs,
and security gates all exist.

## Stability Labels

- `research`: documented only, not suitable for real CA use.
- `experimental`: runnable in a lab with clear warnings.
- `beta`: suitable for test environments with migration risk.
- `stable`: production-supported within the documented threat model.

Default builds must include only stable modules after v1.0.

## Release Ladder

### v0.1: Repository And Security Baseline

Goal: make the project auditable before implementation begins.

Scope:

- EUPL-1.2 license boundary.
- Threat model, architecture, version plan, OpenBao operations plan, and release
  gates.
- GitHub CI placeholder for documentation checks.
- `scripts/checks.sh` and `scripts/stable_release_gate.sh`.
- Dependency policy draft for future Rust workspace.
- Native binary deployment documented as first-class; Podman remains optional
  packaging.

Exit criteria:

- Documentation gate passes.
- GitHub default CodeQL is enabled in repository settings.
- No implementation security claims are made.

### v0.2: Threat Model And HSM Feasibility Proof

Goal: remove ambiguity around the most important security claim.

Scope:

- STRIDE/LINDDUN-style threat model.
- CA key lifecycle matrix: root, intermediate, operational issuer, break-glass.
- Proof of whether OpenBao 2.5.x can perform HSM-backed PKI signing for the
  chosen algorithms.
- Decision record for OpenBao PKI managed keys, OpenBao plugin, or isolated
  PKCS#11 signer.
- SoftHSM/BouncyHSM lab scripts, clearly marked as non-production.

Exit criteria:

- A reproducible lab script proves the chosen signing path.
- A negative test proves the orchestrator token cannot reach root CA paths.
- If full HSM-backed signing is not available, the docs downgrade the security
  claim before implementation continues.

### v0.3: Rust Workspace And API Contract

Goal: create a minimal Rust service with the public API shape fixed.

Scope:

- Rust 1.95.0 or newer current stable toolchain.
- Workspace crates for API DTOs, config, backend adapters, API server, web app,
  CLI app, and auth.
- Provider boundary with OpenBao as the first `CaBackend` adapter and HashiCorp
  Vault reserved as a future adapter.
- Axum API skeleton.
- Separate placeholder web and CLI app crates that use only the public API
  contract.
- OpenAPI generation.
- Strict config parser.
- `#![forbid(unsafe_code)]` in Trustheim crates.

Exit criteria:

- `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test` pass.
- OpenAPI document is generated and checked into release evidence.
- No route exists without an authorization classification.
- Public API DTOs contain no OpenBao-specific mount, role, or token fields.
- Web and CLI crates do not depend on backend provider crates.

### v0.4: Identity And Quorum State

Goal: authenticate people before any CA operation exists.

Scope:

- WebAuthn enrollment, login, and step-up ceremony.
- Admin recovery policy.
- PostgreSQL-backed quorum workflow.
- Session security, CSRF protections for browser flows, and rate limits.
- Audit table for every API state transition.

Exit criteria:

- WebAuthn negative tests cover replay, wrong origin, wrong RP ID, and stale
  challenge.
- Quorum tests prove one person cannot approve their own critical request.
- API responses do not expose internal ids unnecessarily.

### v0.5: OpenBao Provider Bootstrap And Policy Automation

Goal: automate the first provider, OpenBao, in the style of repoheim but for
PKI.

Scope:

- OpenBao dev stack with rootless Podman.
- Bootstrap script for mounts, roles, policies, auth methods, audit devices,
  and short-lived application tokens.
- mTLS assets for local development.
- Policy tests for allowed and denied paths.
- Shared backend contract test suite for provider adapters.

Exit criteria:

- Fresh local OpenBao can be initialized, unsealed, bootstrapped, and smoke
  tested from scripts.
- Policies deny root and intermediate paths to the runtime orchestrator role.
- Audit devices are enabled before certificate operations.
- The provider contract can be run against the OpenBao adapter without changing
  Trustheim API code.

### v0.6: CSR Signing MVP

Goal: issue certificates from submitted CSRs without application-held private
keys.

Scope:

- CSR parse and validation.
- DNS/IP/URI SAN policy.
- Profile-specific TTL, EKU, key type, and key size validation.
- OpenBao sign endpoint integration.
- Certificate inventory.

Exit criteria:

- Fuzz tests cover CSR parsing and SAN normalization.
- Bad SAN, overlong TTL, wrong EKU, and disallowed issuer tests fail closed.
- Runtime memory never contains CA private key material.

### v0.7: Revocation And Audit Completeness

Goal: make certificate lifecycle management operationally safe.

Scope:

- Revocation API and UI.
- CRL/OCSP publication plan.
- Immutable audit export to separate storage.
- Correlation between Trustheim request id and OpenBao audit request id.

Exit criteria:

- Revocation requires the same or stronger authorization than issuance.
- Audit export smoke proves append-only behavior.
- OpenBao audit outage behavior is documented and tested.

### v0.8: Rootless Deployment

Goal: run the lab stack like production.

Scope:

- Rootless Podman compose.
- Native binary deployment examples.
- Hardened systemd units.
- TLS/mTLS file permission checks.
- Container image build and smoke tests.
- Backup and restore rehearsal for metadata, OpenBao, and audit logs.

Exit criteria:

- Podman smoke passes from a clean checkout.
- Restore test can recover inventory and prove audit continuity.
- Secrets are injected through files or OpenBao, not baked into images.
- Native binary users can run the same release artifacts without Podman.

### v0.9: Release Candidate Hardening

Goal: burn down stable blockers before v1.0.

Scope:

- Load and concurrency tests for approval and issuance.
- Dependency review and SBOM.
- cargo-audit, cargo-deny, cargo-geiger review.
- DAST against local API.
- HSM failure and OpenBao failover tests.
- Documentation freeze for v1.0 threat model.

Exit criteria:

- Stable release gate passes.
- All critical and high findings are fixed or the release is delayed.
- A signed release candidate tag carries evidence hashes.

### v1.0: Stable Private CA Core

Goal: production-supported private CA for controlled infrastructure.

Stable scope:

- API-first CSR signing.
- WebAuthn step-up.
- Quorum approval for critical profiles.
- OpenBao policy automation.
- HSM-backed or explicitly documented hardware-sealed CA key tier.
- mTLS to OpenBao.
- Rootless deployment.
- Revocation.
- Audit export.
- Release evidence.

Non-goals:

- Public CA/browser-trust compliance.
- ACME by default.
- Post-quantum production claims.
- Fully automated root CA ceremonies.

Exit criteria:

- `scripts/stable_release_gate.sh release` passes.
- HSM or lower-tier key-protection claim is proven by tests and docs.
- GitHub release includes SBOM, checksums, signed tag, and gate output.

### v1.1: ACME And Renewal Automation

Goal: support controlled automation without weakening identity.

Scope:

- OpenBao ACME with EAB required for exposed directories.
- Internal ACME account issuance tied to authenticated Trustheim identities.
- Renewal policy, rate limits, and service inventory.
- Optional DNS-01 integration behind strict provider allow-lists.

Exit criteria:

- ACME EAB cannot be minted without authenticated policy.
- Wildcard issuance requires explicit quorum profile.
- Abuse and rate-limit tests pass.

### v1.2: HA, DR, And Operations Evidence

Goal: make the CA resilient and measurable.

Scope:

- OpenBao HA Raft deployment runbook.
- Multi-node rootless deployment.
- Disaster recovery exercises.
- Audit sink redundancy and tamper checks.
- Prometheus/OpenTelemetry observability without leaking certificate subjects
  beyond policy.

Exit criteria:

- Failover smoke proves issuance and revocation behavior.
- DR test recovers from backup into an isolated environment.
- Audit sink loss is detected and policy behavior is documented.

### v1.3: Advanced Profiles And Research Graduation

Goal: add advanced certificate profiles only after evidence exists.

Candidates:

- SSH CA profile.
- SPIFFE/SPIRE integration.
- SCEP or EST if a real device fleet needs it.
- Post-quantum or hybrid research profile.

Exit criteria:

- Each promoted profile has its own threat model, API tests, fuzz targets, and
  release gate additions.

### v1.4: Additional Backend Provider

Goal: add a second policy-engine backend without changing Trustheim's public
API.

Likely first target:

- HashiCorp Vault PKI.

Scope:

- `trustheim-backend-vault` adapter.
- Vault bootstrap automation.
- Vault-specific policy, audit, seal, and PKI smoke tests.
- Provider capability matrix in documentation.
- Migration and interoperability notes for OpenBao and Vault deployments.

Exit criteria:

- Shared backend contract tests pass for both OpenBao and Vault.
- Public API and UI need no provider-specific DTO changes.
- Vault-specific limitations are documented and downgrade security claims where
  needed.

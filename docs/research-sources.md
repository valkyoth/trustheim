# Research Sources

Checked on 2026-05-14 from local tooling and upstream documentation.

## Current Versions

- Rust stable: 1.95.0. Local `rustc --version` reports
  `rustc 1.95.0 (59807616e 2026-04-14)`. The Rust changelog tracker lists
  stable `1.95.0`, beta `1.96.0`, and nightly `1.97.0`.
- OpenBao: upstream GitHub lists `v2.5.3` as latest on 2026-04-20. OpenBao
  docs shown for the current line are `Version 2.5.x`.
- Axum: `cargo info axum` reports 0.8.9.
- Tokio: `cargo info tokio` reports 1.52.3.
- Tower HTTP: `cargo info tower-http` reports 0.6.10.
- Serde: `cargo info serde` reports 1.0.228.
- Serde JSON: `cargo info serde_json` reports 1.0.149.
- Utoipa: `cargo info utoipa` reports 5.5.0.
- Webauthn-rs: docs.rs latest stable page is 0.5.5. `cargo info` also shows a
  newer `0.6.1-dev`; stable Trustheim releases should avoid that prerelease
  unless accepted by a written exception.
- Rustls: `cargo info` reports `0.24.0-dev.0`; stable Trustheim releases should
  pin a non-prerelease rustls line unless a written exception accepts the dev
  line.

## OpenBao Findings

- OpenBao PKI API supports roles, issue, sign, revocation, ACME, issuers, keys,
  root generation, intermediate CSR generation, and mount-level authority
  configuration.
- OpenBao recommends limiting the scope of CAs within a mount and not mixing
  different CA types.
- OpenBao PKI considerations recommend creating intermediate CSRs without
  exporting private keys.
- OpenBao PKCS#11 seal exists in the current 2.5.x docs and requires external
  key material before initialization.
- OpenBao audit devices record request and response API interactions except for
  listed system paths such as init, seal status, seal, unseal, health, and Raft
  join/bootstrap paths. Multiple audit devices are recommended.
- OpenBao ACME supports EAB and recommends EAB for public-facing deployments.

## HashiCorp Vault Findings

- HashiCorp Vault exposes HTTP APIs for secrets engines, including PKI.
- Vault secrets engines are enabled at configurable paths and then interacted
  with directly at that path.
- Vault audit devices record API requests and responses, with documented system
  endpoint exceptions and operational best-practice requirements.
- Vault is a plausible future backend provider, but it needs its own adapter,
  bootstrap automation, policy tests, audit tests, and license/deployment review
  before it can be called stable.

## Local Project Patterns Reviewed

### base64-ng

Useful patterns:

- Minimal dependency posture.
- Strict release gate with formatting, metadata validation, clippy, tests,
  cargo-deny, cargo-audit, license inventory, SBOM, reproducibility, fuzz, and
  proof hooks.
- Explicit reserved feature checks.
- Unsafe boundary validation.

### fluxheim

Useful patterns:

- EUPL-1.2 license.
- Strict `deny.toml` license and source policy.
- Rootless Podman smoke tests.
- Stable release gate and deeper optional gates.
- TLS scan, feature matrix, config validation, SBOM, reproducibility, and
  release evidence.
- Security policy that documents advisory exceptions and removal conditions.

### repoheim

Useful patterns:

- OpenBao bootstrap script with redacted output and `0600` secret files.
- Idempotent mount, transit key, secret, and policy setup.
- Separate runtime tokens per service policy.
- Rootless Podman OpenBao deployment with Raft storage and TLS.

## Source URLs

- Rust versions: https://releases.rs/
- OpenBao repository/releases: https://github.com/openbao/openbao
- OpenBao PKI API: https://openbao.org/api-docs/secret/pki/
- OpenBao PKI setup: https://openbao.org/docs/secrets/pki/setup/
- OpenBao PKI considerations: https://openbao.org/docs/secrets/pki/considerations/
- OpenBao PKCS#11 seal: https://openbao.org/docs/configuration/seal/pkcs11/
- OpenBao audit devices: https://openbao.org/docs/audit/
- HashiCorp Vault PKI: https://developer.hashicorp.com/vault/docs/secrets/pki
- HashiCorp Vault audit devices: https://developer.hashicorp.com/vault/docs/audit
- HashiCorp Vault secrets engines: https://developer.hashicorp.com/vault/docs/secrets
- webauthn-rs docs.rs: https://docs.rs/crate/webauthn-rs/latest

# OpenBao Operations Plan

OpenBao is the first policy engine and audit anchor for Trustheim. The Rust
service must treat it as a provider adapter, so another backend such as
HashiCorp Vault can be added later without changing Trustheim's public API.

## Baseline Version

As of 2026-05-14, the current OpenBao line checked for this plan is 2.5.x, with
v2.5.3 published on 2026-04-20. Pin exact container image digests in deployment
files once implementation starts; do not rely on floating tags.

## Mount Layout

Planned mounts:

- `pki_lab`: disposable lab issuer, never trusted by production clients.
- `pki_root`: offline or quorum-only root CA ceremony.
- `pki_int_ops`: operational intermediate for normal issuance.
- `pki_int_critical`: critical infrastructure intermediate, quorum required.
- `transit`: encryption and HMAC support for application secrets where needed.
- `secret`: non-CA application configuration.

Do not mix root and intermediate issuers in a shared production mount. Separate
mounts make ACLs, audit review, and rotation safer.

## Policy Model

Runtime policies:

- `trustheim-orchestrator`: read limited metadata, sign only through allowed
  operational roles, no root, no intermediate generation, no issuer deletion.
- `trustheim-quorum`: approve pending high-value operations through a narrow
  wrapped token flow.
- `trustheim-audit-reader`: read audit-related metadata only, no signing.
- `trustheim-breakglass`: disabled by default, time-bound, multi-person
  activation, fully audited.

Operator policies:

- `trustheim-openbao-bootstrap`: initial setup only.
- `trustheim-pki-root-ceremony`: root creation, rotation, and intermediate CSR
  signing only.
- `trustheim-pki-role-manager`: profile and role updates with quorum.

Every policy must have a deny-path test. Allowed-path tests are not enough.

## Bootstrap Automation

The bootstrap script should follow the repoheim pattern:

- Parse a local env file with strict permissions.
- Refuse to read secret input files that are group/world readable.
- Initialize only when explicitly requested.
- Write generated root/unseal material only to `0600` files.
- Redact secrets from stdout.
- Enable mounts idempotently.
- Create roles, policies, auth methods, and audit devices idempotently.
- Emit a machine-readable bootstrap report.
- Provide a self-test mode for parser and redaction behavior.

Required commands:

```bash
scripts/openbao_bootstrap.py status
scripts/openbao_bootstrap.py init
scripts/openbao_bootstrap.py unseal
scripts/openbao_bootstrap.py bootstrap
scripts/openbao_bootstrap.py policy-test
scripts/openbao_bootstrap.py audit-layout
```

These commands are provider-specific. The runtime server should call only the
shared backend interface described in
[Backend Provider Interface](backend-provider-interface.md).

## Audit

OpenBao audit devices must be enabled before certificate operations. Use at
least two independent audit sinks:

- Local file or socket device on append-only storage for node-local evidence.
- Remote HTTP/socket/syslog collector on a separate network segment.

Trustheim must store OpenBao audit correlation ids with local API audit rows.
Large or sensitive values should be strings so OpenBao audit HMAC handling can
protect them consistently.

## Seal And HSM

Production OpenBao must use hardware-backed seal where possible:

- PKCS#11 seal with a supported HSM.
- AEAD-capable mechanism preferred.
- Key labels versioned for rotation.
- Old seal keys retained until all protected data is rewrapped or no longer
  needed.

Important boundary: PKCS#11 seal protects OpenBao storage and unseal material.
It does not automatically prove that CA signing keys are generated and used
inside an HSM. That must be tested separately in v0.2.

## Local Lab Stack

The local stack should use rootless Podman:

- OpenBao 2.5.x pinned by digest.
- PostgreSQL for Trustheim metadata.
- Valkey only if short-lived distributed state is needed.
- SoftHSM or BouncyHSM for lab-only PKCS#11 testing.
- Generated development mTLS certificates.

Lab HSMs are never production evidence. They prove automation and failure
handling, not tamper resistance.

## Failure Tests

Release gates must eventually test:

- OpenBao sealed state.
- OpenBao unavailable.
- Expired orchestrator token.
- Wrong mTLS client certificate.
- Audit sink blocked.
- HSM unavailable.
- HSM wrong PIN or missing key label.
- Policy denies root path to orchestrator.
- Issuer role rejects forbidden SAN.
- Revocation path requires stronger authorization.

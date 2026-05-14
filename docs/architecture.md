# Architecture

Trustheim uses a split-control architecture: a Rust API service orchestrates
certificate ceremonies, a pluggable backend provider enforces PKI policy and
audit, and hardware-backed cryptographic material sits outside the application
trust boundary. OpenBao is the first provider target. The web interface and CLI
are separate API clients, not part of the backend trust boundary.

## Primary Goal

Build a private CA where compromise of the web UI is insufficient to export CA
keys, mint arbitrary certificates, rotate trust anchors, or hide audit evidence.

The design goal is defense in depth, not a marketing phrase. A release may only
claim a security level that the automated gates, operational runbooks, and
hardware integration prove.

## Trust Tiers

### Tier 1: Rust Orchestrator

Responsibilities:

- API-first certificate request, approval, revocation, inventory, and audit
  workflow.
- Axum HTTP API with OpenAPI generated from typed request and response models.
- Optional web UI, backed only by the public API and deployed as a separate app.
- Optional CLI, backed only by the public API and deployed as a separate app or
  operator binary.
- WebAuthn registration and authentication using hardware-backed authenticators.
- Typed validation for common names, DNS SANs, IP SANs, URI SANs, requested
  TTL, certificate profile, and requester identity.
- Quorum state machine for sensitive operations.
- Short-lived OpenBao credentials, ideally AppRole or OIDC-derived wrapped
  tokens.
- mTLS client authentication to OpenBao.

Non-responsibilities:

- No CA private key storage.
- No private key export path.
- No direct root CA signing endpoint in the normal application runtime.
- No unaudited administrative bypass.

### Tier 2: Backend Policy Engine

Responsibilities:

- PKI secrets engine mounts, issuers, roles, and path ACLs.
- Root, intermediate, operational, and break-glass policy separation.
- Audit devices with at least two independent sinks.
- Token lifecycle, response wrapping, lease limits, and revocation.
- Raft-backed HA storage for production clusters.
- PKCS#11 seal or another approved hardware-backed seal for OpenBao master-key
  protection.

OpenBao PKI mounts must be separated by CA purpose. Do not mix roots,
intermediates, lab CAs, and production issuers in one mount just because the
engine supports multiple issuers.

The Rust domain layer must talk to a `CaBackend` capability interface, not to
OpenBao route strings directly. Provider-specific code belongs in adapter crates
such as `trustheim-backend-openbao` and future `trustheim-backend-vault`.
Provider-specific differences are allowed in deployment configuration and
bootstrap tooling, but not in public certificate-request DTOs.

### Tier 3: Cryptographic Vault

The target security tier requires non-exportable CA private keys. OpenBao
PKCS#11 seal protects OpenBao storage and master-key unseal material; it is not
by itself proof that every PKI signing key never enters OpenBao memory.

Therefore v0.2 must explicitly prove one of these paths before v1.0 can claim
hardware-rooted CA-key protection:

- OpenBao PKI can use HSM-backed managed keys for the selected root and
  intermediate signing operations in the deployed OpenBao version.
- A reviewed OpenBao plugin or isolated signing service performs PKCS#11-backed
  signing while OpenBao remains the policy and audit control point.
- The release is documented as a lower security tier where OpenBao internal
  non-exportable keys are encrypted by hardware-sealed storage, but signing
  operations are not claimed to occur wholly inside an HSM.

## API-First Surface

Every workflow starts as an API route with typed DTOs and OpenAPI coverage. The
UI consumes the same API. This keeps authorization, input validation, audit
fields, and release tests focused on one contract.

Initial API groups:

- `/api/v1/auth/webauthn/*`
- `/api/v1/profiles`
- `/api/v1/certificate-requests`
- `/api/v1/certificate-requests/{id}/approvals`
- `/api/v1/certificates`
- `/api/v1/certificates/{serial}/revoke`
- `/api/v1/ca/intermediates`
- `/api/v1/audit/events`
- `/api/v1/admin/openbao/health`

Response DTOs must avoid internal database ids unless the id is part of the
public route model. Audit and manager routes may expose operational identifiers
only when they are needed for action and are covered by explicit authorization.

## Ceremony Workflow

1. A requester submits a CSR or requests a keypair profile that allows server
   side key generation.
2. Trustheim validates the typed request against local policy before OpenBao is
   contacted.
3. The requester performs WebAuthn step-up.
4. Low-risk requests can proceed immediately when profile policy allows it.
5. Critical requests enter quorum review. Approvers must authenticate with
   separate hardware-backed WebAuthn credentials.
6. Trustheim obtains a short-lived OpenBao token scoped to the exact signing
   path.
7. OpenBao signs through the allowed issuer and role.
8. Trustheim returns certificate material and stores only metadata, audit
   linkage, and public certificate data.
9. Every state transition is written to append-only audit storage and correlated
   with OpenBao audit request ids.

## Cryptographic Profiles

Default profiles should be conservative and interoperable:

- `server-modern`: ECDSA P-256 or P-384 leaf keys, short TTL, serverAuth EKU.
- `service-ed25519`: Ed25519 for controlled internal clients that are verified
  to support it.
- `client-device`: clientAuth EKU, device-bound SAN rules, short TTL.
- `critical-infra`: quorum required, narrow SAN allow-list, one-year maximum
  only after explicit policy approval.

Post-quantum or hybrid X.509 signing must stay `research` until the selected
algorithms, HSM support, OpenBao support, certificate path validation, and
client compatibility are production-proven. Do not ship a "quantum-ready"
production claim from experimental crates alone.

## Storage Boundary

Trustheim stores:

- User, role, authenticator, and quorum metadata.
- Certificate request metadata.
- Public certificates and chains.
- Revocation records.
- Audit correlation ids.

Trustheim does not store:

- CA private keys.
- OpenBao root tokens.
- Long-lived administrator tokens.
- WebAuthn private key material.
- Raw HSM PINs in the database.

Long-lived secrets belong in OpenBao or in rootless container/systemd secret
injection with file permissions checked before startup.

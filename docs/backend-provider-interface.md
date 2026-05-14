# Backend Provider Interface

Trustheim should support more than one policy engine over time. OpenBao is the
first backend because it is open-source and has a strong PKI/audit model, but
the architecture must not make OpenBao a permanent assumption in the public API.

HashiCorp Vault is the obvious future compatibility target because it also has
HTTP APIs, PKI secrets engine mounts, token-based auth, audit devices, and
similar policy concepts. The adapter still needs careful testing because API
details, enterprise features, licensing, managed-key behavior, seal behavior,
and operational defaults differ.

## Design Rule

Trustheim owns the domain model. Backend providers own the translation to their
API.

The public API should say:

- certificate profile
- issuer reference
- CSR
- SAN policy
- TTL
- approval policy
- revocation reason
- audit correlation id

The public API should not say:

- OpenBao mount path
- Vault mount path
- OpenBao issuer id
- Vault issuer id
- raw backend token
- backend-specific policy name

Those values belong in provider configuration and internal audit metadata.

## Crate Boundary

Planned Rust workspace split:

- `trustheim-domain`: provider-neutral request, approval, certificate, and
  policy types.
- `trustheim-api`: HTTP DTOs and OpenAPI schema.
- `trustheim-backend`: provider trait, capability model, and shared test suite.
- `trustheim-backend-openbao`: OpenBao implementation.
- `trustheim-backend-vault`: future HashiCorp Vault implementation.
- `trustheim-server`: Axum server wiring, auth, state, and routing.
- `trustheim-bootstrap-openbao`: OpenBao bootstrap automation.
- `trustheim-bootstrap-vault`: future Vault bootstrap automation.

`trustheim-server` may depend on `trustheim-backend`, but must not import
OpenBao or Vault HTTP route constants directly.

## Capability Interface

The backend trait should be narrow and explicit:

```rust
pub trait CaBackend {
    fn provider_info(&self) -> ProviderInfo;
    async fn health(&self) -> Result<BackendHealth, BackendError>;
    async fn capabilities(&self) -> Result<BackendCapabilities, BackendError>;
    async fn sign_csr(&self, request: SignCsrRequest) -> Result<SignedCertificate, BackendError>;
    async fn revoke_certificate(&self, request: RevokeCertificateRequest) -> Result<RevocationResult, BackendError>;
    async fn read_issuer(&self, issuer: IssuerRef) -> Result<IssuerInfo, BackendError>;
    async fn audit_status(&self) -> Result<AuditStatus, BackendError>;
}
```

Bootstrap and ceremony traits should be separate:

- Runtime signing must not need root or bootstrap permissions.
- Root and intermediate ceremony operations must not be reachable through the
  normal runtime backend client.
- Provider bootstrap code can have provider-specific commands and config.

## Capability Flags

Providers must declare capabilities at startup:

- `csr_signing`
- `backend_key_generation`
- `issuer_listing`
- `revocation`
- `crl_publication`
- `ocsp`
- `acme`
- `audit_status`
- `response_wrapping`
- `approle_or_equivalent`
- `mtls_client_auth`
- `hardware_seal`
- `hsm_backed_pki_signing`

Trustheim should refuse to enable a certificate profile if the selected backend
lacks a required capability.

## Configuration Shape

Provider-neutral profile example:

```toml
[[profiles]]
name = "server-modern"
issuer = "ops-intermediate"
max_ttl = "720h"
allow_dns_sans = ["*.svc.example.internal", "*.corp.example.internal"]
require_quorum = false
```

Provider mapping example:

```toml
[backend]
provider = "openbao"

[backend.openbao]
addr = "https://openbao.example.internal:8200"
ca_cert = "/etc/trustheim/openbao-ca.crt"
client_cert = "/etc/trustheim/trustheim.crt"
client_key = "/etc/trustheim/trustheim.key"

[[backend.openbao.issuers]]
name = "ops-intermediate"
mount = "pki_int_ops"
role = "server-modern"
```

A future Vault provider should use the same `profiles` block and a separate
`[backend.vault]` mapping block.

## Test Contract

Every provider must pass the shared backend contract tests:

- Health reports sealed/unavailable states safely.
- Disallowed profile refuses before backend call.
- Sign CSR preserves subject and allowed SANs.
- Sign CSR rejects forbidden SANs and overlong TTLs.
- Runtime token cannot create root issuers.
- Runtime token cannot generate intermediates.
- Revocation is audited and idempotent.
- Audit device status is checked before signing if the provider exposes it.
- Backend error bodies are redacted before logs and API responses.

The OpenBao provider may have additional tests for OpenBao-specific policy and
audit behavior. The future Vault provider must add equivalent tests instead of
weakening the shared contract.

## Operational Boundary

OpenBao and HashiCorp Vault can both be supported, but they are not identical
security products. Provider documentation must state:

- supported versions;
- license and deployment implications;
- seal and HSM support;
- PKI managed-key support;
- audit-device behavior;
- API differences that affect Trustheim guarantees;
- missing capabilities that downgrade a security claim.

No provider can claim `stable` support until it passes the full release gate
with its own bootstrap and smoke tests.

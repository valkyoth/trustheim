# trustheim

Trustheim is planned as an API-first private certificate authority orchestrator.
The core design is intentionally split: Rust validates identity and intent, a
backend provider owns policy and audit, and an HSM or equivalent
hardware-backed signer owns the CA private keys.

Trustheim is licensed under the European Union Public Licence 1.2
(`EUPL-1.2`).

## Current Status

This repository is in planning stage. No CA implementation is shipped yet, and
no production security claim is made until the release gates in
[docs/security-and-release-gates.md](docs/security-and-release-gates.md) exist
as executable checks and pass on tagged source.

The starting architecture and roadmap are:

- [Architecture](docs/architecture.md)
- [Application Boundaries](docs/app-boundaries.md)
- [API](docs/api.md)
- [Backend Provider Interface](docs/backend-provider-interface.md)
- [Native Binary Deployment](docs/deploy/native-binaries.md)
- [Versioning Plan](docs/versioning-plan.md)
- [OpenBao Operations Plan](docs/openbao-operations-plan.md)
- [Security And Release Gates](docs/security-and-release-gates.md)
- [Research Sources](docs/research-sources.md)

## Security Boundary

Trustheim must never become a private-key storage product. The Rust service is a
ceremony orchestrator and typed API gateway. It may hold short-lived request
state, WebAuthn challenges, OpenBao client credentials, and certificate request
metadata, but it must not store CA private keys or exportable intermediate keys.

OpenBao is the first supported backend provider, not a permanent hardcoded
dependency. The public Trustheim API must stay provider-neutral so future
support for HashiCorp Vault or another compatible policy engine can be added
behind a narrow provider interface.

High-value signing operations require:

- WebAuthn step-up with hardware-backed authenticators.
- Policy and role checks before any OpenBao call.
- Quorum approval for root, intermediate, revocation, policy, and critical
  infrastructure actions.
- mTLS between Trustheim and OpenBao.
- OpenBao ACLs that make root and intermediate CA paths unreachable to the
  normal orchestrator token.
- Non-exportable CA keys in an HSM or a formally accepted fallback for a lower
  security tier.

## Development Checks

Planning-stage checks:

```bash
scripts/checks.sh
scripts/stable_release_gate.sh check
```

The first Rust milestone includes a provider-neutral workspace, a separate Axum
API server app, placeholder CLI and web app crates, OpenAPI output, and a
rejecting backend placeholder. Later milestones will add SBOM, reproducibility,
fuzz, OpenBao bootstrap, Podman smoke, and provider contract checks.

Build all standalone binaries:

```bash
cargo build --bins
```

Run the API server locally:

```bash
TRUSTHEIM_BIND_ADDR=127.0.0.1:8787 cargo run -p trustheim-api-server
```

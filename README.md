# trustheim

Trustheim is planned as an API-first private certificate authority orchestrator
for European sovereign infrastructure. The core design is intentionally split:
Rust validates identity and intent, OpenBao owns policy and audit, and an HSM or
equivalent hardware-backed signer owns the CA private keys.

Trustheim is licensed under the European Union Public Licence 1.2
(`EUPL-1.2`).

## Current Status

This repository is in planning stage. No CA implementation is shipped yet, and
no production security claim is made until the release gates in
[docs/security-and-release-gates.md](docs/security-and-release-gates.md) exist
as executable checks and pass on tagged source.

The starting architecture and roadmap are:

- [Architecture](docs/architecture.md)
- [Backend Provider Interface](docs/backend-provider-interface.md)
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

The first Rust milestone will replace these documentation-only checks with
`cargo fmt`, `cargo clippy`, `cargo test`, `cargo deny`, `cargo audit`, SBOM,
reproducibility, fuzz, OpenBao bootstrap, Podman smoke, and API contract checks.

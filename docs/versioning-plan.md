# Versioning Plan

Trustheim uses SemVer with a conservative stability rule: a feature is not
stable because it compiles. A feature becomes stable only after its API,
authorization model, provider behavior, tests, operational docs, and release
evidence all exist.

This plan is intentionally front-loaded before `v1.0.0`. Anything needed for the
stable custody-free CA coordinator must land in a `0.x` milestone. New blockers
found during review create new pre-1.0 milestones; they are not silently deferred
to a later minor release.

## Current Baseline

As of 2026-07-17, the repository is an auditable prototype. It has useful
foundations: EUPL-1.2 licensing, a Rust workspace, app separation, provider
adapter placeholders, OpenAPI generation, dependency checks, audit checks, and a
provider-neutral sign-CSR route that rejects before any real backend exists.

It is not yet a high-assurance CA coordinator. The current implementation maps
to the `v0.1.x` planning/prototype line until the gates below are implemented.

Version research checked on 2026-07-17:

- Rust stable has moved past the pinned `1.95.0` baseline. Official Rust release
  announcements list `1.97.0` on 2026-07-09, with `1.96.1` on 2026-06-30.
- OpenBao v2.5.4 was published on 2026-05-20 and supersedes the earlier v2.5.3
  planning baseline. Its release notes include security fixes for audit header
  handling and token creation behavior.

Before any release work, re-check current Rust, crate, OpenBao, Vault, and HSM
vendor versions from primary sources.

## Stability Labels

- `research`: documented or lab-only. No production support.
- `experimental`: runnable in a controlled lab. Behavior and APIs may change.
- `beta`: usable for test environments with explicit migration risk.
- `stable`: production-supported within the documented threat model.

Default production builds after `v1.0.0` must include only stable modules.

## Non-Negotiable Stable Boundaries

- Trustheim is custody-free: it must not generate, receive, deliver, store, log,
  retry, or cache CA private keys or subscriber private keys.
- Stable issuance is CSR-only. Server-side subscriber key generation is excluded
  from `v1.0.0`.
- The public API, web app, and CLI are provider-neutral clients of Trustheim
  policy. Provider mount paths, role names, raw tokens, and provider issuer IDs
  stay in internal configuration and audit metadata.
- Web and CLI must not link provider adapters or receive provider credentials.
- Public DTO crates must not depend on backend adapter crates or backend
  implementation types.
- Runtime provider adapters must not accept public API DTOs directly. They
  operate on validated, authorized, manifest-bound operation types.
- Runtime, bootstrap, evidence inspection, and offline ceremony interfaces are
  separate. The normal runtime binary must not link bootstrap/admin path
  constants.
- Provider capabilities are verified evidence, not self-declared booleans.
- Rootless Podman is supported, but native binaries are first-class.
- Release gates must fail closed. In release mode, security tools and required
  smoke tests may not be skipped.

## Required Architecture Before v1.0.0

The high-assurance target is separate trust domains, not merely separate crates:

- API edge: HTTP/TLS, body limits, generic JSON syntax, no provider credentials.
- Coordinator: identity, policy, quorum, audit, manifest state, no raw CSR
  parser beyond trusted validated outputs.
- CSR parser worker: constrained ASN.1/X.509 parsing, no provider credentials,
  no database write authority beyond controlled parser results.
- Provider credential broker: mTLS identity, short-lived provider credential,
  egress only to configured provider.

Where local development keeps these in one process temporarily, the docs and
release gates must clearly mark that build as non-production.

## Release Ladder To v1.0.0

### v0.1.x: Repository And Prototype Baseline

Goal: keep the repository buildable, auditable, and honest about its prototype
status.

Scope:

- EUPL-1.2 license boundary.
- Rust workspace with app separation: API server, web app, CLI.
- Provider adapter placeholders.
- Provider-neutral API route skeleton and OpenAPI output.
- Native binary deployment documented as first-class.
- Rootless Podman documented as optional packaging.
- `scripts/checks.sh`, dependency policy, cargo-audit, and cargo-deny.

Exit criteria:

- Local checks pass.
- No production security claim is made.
- Documentation says the current implementation is not a CA-ready release.

### v0.2.0: Formal Threat And Privacy Model

Goal: define exactly what Trustheim protects and what it does not.

Scope:

- Trust-boundary diagrams for API edge, coordinator, parser worker, provider
  broker, database, audit sinks, and provider.
- STRIDE and LINDDUN analysis.
- Asset inventory and attacker capabilities.
- Misuse cases and abuse flows.
- Residual-risk register.
- Precise definition of "custody-free" and "restart-safe/stateless".

Exit criteria:

- Threat model reviewed and checked into docs.
- Server-side subscriber-key generation is removed from stable architecture.
- New high/critical threats create pre-1.0 milestones.

### v0.3.0: HSM And Key-Custody Feasibility Decision

Goal: choose the stable key-custody tier before backend implementation.

Scope:

- Reproducible lab proof for the chosen signing path.
- Decision between provider external key, reviewed plugin, isolated signer, or
  explicitly lower-tier provider-protected key custody.
- Negative tests for HSM unavailable and key export attempts.
- Signed ADR defining supported custody claims and downgrade rules.
- SoftHSM/BouncyHSM allowed only as lab automation, not production evidence.

Exit criteria:

- Hardware seal is not treated as proof of HSM PKI signing.
- Stable profile claims match proven custody evidence.
- No fallback to software signing satisfies a hardware profile.

### v0.4.0: Cryptographic And CA-Key Lifecycle Policy

Goal: define CA behavior before writing real issuance code.

Scope:

- Root, intermediate, leaf, SSH, ACME, and emergency issuer lifecycle matrix.
- Algorithm policy and compatibility matrix.
- Entropy requirements.
- Rotation, cross-signing, issuer retirement, and revocation implications.
- Post-quantum and hybrid profiles classified as research unless proven.

Exit criteria:

- Every profile has an allowed key type, algorithm, EKU, KU, TTL, issuer class,
  and custody requirement.
- Unsupported or research profiles cannot be enabled by default.

### v0.5.0: Secret, Logging, Crash, And Temporary-File Hygiene

Goal: prevent sensitive data from becoming ordinary cloneable/debuggable text.

Scope:

- Redacted `Debug` for request, CSR, provider, token, and credential types.
- Non-cloneable, zeroizing wrappers for secret-bearing values.
- No request-body, cookie, authorization-header, WebAuthn, provider-response, or
  CSR PEM logging.
- Safe temp directories using `mktemp -d`, `umask 077`, cleanup traps, and
  collision-resistant names.
- Core-dump and process-dump policy.
- Secret scanning tests and canaries.

Exit criteria:

- Raw CSR and backend error bodies cannot cross public API/log boundaries
  accidentally.
- Smoke scripts leave no predictable `/tmp/trustheim-*` residue.

### v0.6.0: Runtime Process And Sandbox Boundaries

Goal: stop treating crate boundaries as isolation boundaries.

Scope:

- ADR for API edge, coordinator, CSR parser worker, and provider broker.
- Separate runtime users/UIDs where supported.
- Local authenticated RPC between trust domains.
- Seccomp, namespace, mount, and egress policy plans.
- Parser compromise test proving no provider credential access.

Exit criteria:

- Production topology separates untrusted parsing from provider credentials.
- Single-process mode is marked development-only.

### v0.7.0: Strict Configuration And Secret Injection

Goal: make insecure runtime configuration hard to express.

Scope:

- `deny_unknown_fields` configuration parsing.
- HTTPS-only provider endpoints.
- Explicit private trust roots.
- Owner/mode checks for mTLS and secret files.
- Descriptor-based secret loading where possible.
- No secret-bearing CLI arguments or environment variables.
- Configuration provenance digest for operation manifests.

Exit criteria:

- Insecure provider transport is unrepresentable in production config.
- Secret files fail closed on unsafe ownership or permissions.

### v0.8.0: Public API/Domain/Backend Dependency Inversion

Goal: make provider neutrality real in the dependency graph.

Scope:

- Public API DTOs moved out of backend implementation concepts.
- `trustheim-api` no longer depends on `trustheim-backend`.
- Provider identity and security topology removed from unauthenticated public
  DTOs.
- Logical issuer/profile identifiers remain stable across providers.
- Automated dependency-direction gate using cargo metadata.

Exit criteria:

- Desired direction holds: provider adapters depend on domain/backend contracts;
  public API does not depend on provider or backend implementation types.
- Web and CLI dependency deny-list remains enforced.

### v0.9.0: Provider-Neutral Backend Trait v2

Goal: redesign the backend API around authorized operations and evidence.

Scope:

- Private-constructor `ValidatedCsr`.
- Private-constructor `AuthorizedOperation`.
- Runtime issuance/revocation trait separated from health/evidence,
  bootstrap/configuration, and offline ceremony traits.
- Verified assurance model replacing self-declared capability booleans.
- Non-serializable internal backend errors.
- Public API errors use fixed client-safe messages plus correlation IDs.

Exit criteria:

- Provider adapter cannot sign using only a public request DTO.
- Backend errors cannot leak raw provider details to clients.

### v0.10.0: Provider Conformance Harness

Goal: prove the backend contract without trusting a real provider first.

Scope:

- Strict in-memory conformance provider with no OpenBao or Vault concepts.
- Adversarial provider exercising unsupported audit, missing custody evidence,
  malformed responses, timeouts, and misleading readiness.
- Shared provider contract tests.
- Identical public API contract snapshots across providers.

Exit criteria:

- The same public API and policy code passes with the in-memory provider and
  fails closed with the adversarial provider.

### v0.11.0: OpenAPI Contract Validation

Goal: make the API testable by clients before backend behavior stabilizes.

Scope:

- Generated OpenAPI snapshot.
- Schema constraints and examples.
- Security schemes.
- Route coverage check.
- OpenAPI linter.
- Backward-compatibility diff gate.
- Negative client/server conformance tests.

Exit criteria:

- Every public route appears in OpenAPI.
- Breaking API changes are intentional and documented.

### v0.12.0: HTTP Transport And Request Hardening

Goal: harden Axum ingress before identity and issuance.

Scope:

- Explicit body, header, nesting, array-cardinality, and method limits.
- Request, response, idle, and shutdown timeouts.
- Concurrency and rate limits.
- Decompression policy; ceremony endpoints do not accept compressed bodies.
- Generic rejection bodies.
- Security headers where applicable.
- Parser-exhaustion tests.

Exit criteria:

- Oversized and malformed traffic fails before domain or backend code.
- Resource exhaustion tests are part of the gate.

### v0.13.0: Identity, Sessions, Recovery, And Route Authorization

Goal: authorize every route before WebAuthn step-up exists.

Scope:

- Identity model and role separation.
- Opaque Secure/HttpOnly/SameSite session cookies.
- CSRF defense for cookie-authenticated routes.
- Recovery and break-glass governance.
- Route authorization classification.
- Authorization tests for every route.

Exit criteria:

- No route lacks an authorization class and test.
- Browser sessions do not imply approval authority.

### v0.14.0: WebAuthn Enrollment Correctness

Goal: enroll authenticators safely.

Scope:

- RP ID and origin validation.
- Secure registration state.
- Duplicate credential/public-key prevention.
- Supported algorithm policy.
- Credential lifecycle and replacement workflow.
- Negative registration tests.

Exit criteria:

- Credential IDs and public keys cannot enroll under multiple guardians.
- Registration state is single-use and expires.

### v0.15.0: Hardware Authenticator Attestation

Goal: make high-value guardian enrollment fail closed.

Scope:

- Attestation format, signature, and chain verification.
- AAGUID and metadata policy.
- Firmware/status handling.
- User-verification and backup-state policy.
- Guardian replacement ceremony.

Exit criteria:

- Hardware-bound guardian profiles reject synced/backup credentials unless an
  explicit policy permits them.
- Attestation policy changes require controlled administration.

### v0.16.0: WebAuthn Step-Up And Replay Resistance

Goal: bind approvals to one exact operation.

Scope:

- Operation-bound challenges with at least 32 random bytes.
- Exact challenge, origin, RP ID, UV, signature, credential, and expiry checks.
- Atomic single-use challenge consumption.
- Sign-counter anomaly handling.
- Parallel replay tests.

Exit criteria:

- No session-wide approval token exists.
- Replayed, cross-window, and cross-account assertions fail.

### v0.17.0: Canonical Operation Manifest

Goal: make approvers sign the exact operation, not a mutable row.

Scope:

- Domain-separated manifest schema.
- CSR digest, SPKI fingerprint, subject, SANs, profile, issuer, TTL, EKU, KU,
  guardian set, custody tier, config digest, nonce, and expiry fields.
- Deterministic canonical encoding.
- Golden vectors.
- Canonicalization fuzz/property tests.
- Amendment invalidation.

Exit criteria:

- Any change to request, policy, issuer, profile, or guardian set creates a new
  manifest and invalidates old approvals.

### v0.18.0: Multi-Party Quorum State Machine

Goal: enforce quorum over humans and immutable manifests.

Scope:

- Durable states: pending, quorum reached, signing leased, issued, expired,
  rejected.
- Unique guardian counting.
- Requester exclusion where required.
- Threshold evaluation.
- Guardian-set epochs.
- Database invariants.

Exit criteria:

- One person, one credential clone, or one account cannot satisfy quorum alone.
- Pending approvals are invalidated by guardian-policy changes.

### v0.19.0: Quorum Substitution And Concurrency Defense

Goal: close race and substitution paths.

Scope:

- Token/credential substitution tests.
- Duplicate and race tests.
- Policy-change invalidation.
- Distinct-human enforcement.
- Fencing leases and idempotency keys.
- Model/property checking of state transitions.

Exit criteria:

- Only one worker can enter signing for a given operation.
- Ambiguous retries reconcile before a second signing attempt.

### v0.20.0: Tamper-Evident Audit Ledger

Goal: make authorization evidence independently checkable.

Scope:

- Append-only transition events.
- Per-event hash chain or Merkle accumulator.
- Signed checkpoints.
- External anchoring to WORM/append-only storage.
- Provider request correlation.
- Certificate digest correlation.
- Rollback detection and reconciliation tooling.

Exit criteria:

- Every issued certificate has a completed authorization chain.
- Database rollback alone cannot hide or rewrite issuance history.

### v0.21.0: Backend mTLS Transport

Goal: make backend transport identity mandatory and provider-neutral.

Scope:

- rustls client configuration with explicit protocol policy.
- Private trust store; no silent host root-store fallback.
- Mandatory client certificate and key descriptor.
- Provider DNS/SPIFFE identity validation.
- Certificate rotation checks.
- Connection, handshake, request, response-body, and idle timeouts.
- No environment proxy discovery.
- Startup and periodic TLS revalidation.

Exit criteria:

- Plain HTTP and optional client-auth backends are rejected in production.
- Negative TLS matrix is part of the gate.

### v0.22.0: Provider Credential Broker And Lifecycle

Goal: isolate provider credentials from parser and API handling.

Scope:

- Non-renewable or strictly bounded short-lived credentials.
- Immutable deployment identity to runtime role mapping.
- Redacted, non-cloneable, zeroizing token storage.
- One-operation or very low-use credential scope where supported.
- Token accessor or opaque handle for correlation, never token value.
- Explicit revocation/drop after terminal operation.
- Egress only to configured provider.
- Startup negative probe proving privileged paths are denied.

Exit criteria:

- Runtime identity cannot access root, intermediate ceremony, auth, token,
  policy, audit, rekey, plugin, mount, or system-administration paths.
- Negative probes use the exact runtime identity.

### v0.23.0: First Provider Bootstrap

Goal: automate the first real provider without hardcoding provider assumptions
into the public API.

Scope:

- OpenBao is the first target unless a later ADR chooses otherwise.
- Idempotent init/unseal/bootstrap tooling.
- Strict secret-file handling.
- Pinned provider image digest or binary checksum.
- Machine-readable bootstrap report.
- Externalized recovery material.
- Clean-checkout bootstrap smoke.

Exit criteria:

- Provider bootstrap produces no public API changes.
- Runtime role is least privilege and separate from bootstrap/ceremony roles.

### v0.24.0: First Provider ACL And Declarative Audit Policy

Goal: prove least privilege and audit behavior for the first provider.

Scope:

- Exact runtime allow-list.
- Deny tests for every privileged path.
- Issuer override and verbatim signing denial.
- Declarative dual audit devices where supported.
- Audit-failure behavior.
- Runtime identity proof.

Exit criteria:

- Runtime identity cannot mutate provider policy, roles, mounts, issuers, audit
  devices, auth methods, or root/intermediate material.
- Audit devices are treated as infrastructure configuration, not routine runtime
  mutation.

### v0.25.0: First Provider Runtime Adapter

Goal: connect Trustheim to the first provider through the hardened contract.

Scope:

- Health/readiness.
- Role-constrained CSR signing plumbing.
- Audit status and request correlation.
- Typed redaction.
- Bounded provider response parsing.
- Contract-test compliance.

Exit criteria:

- Provider adapter passes shared conformance tests.
- Public API does not change for provider-specific details.

### v0.26.0: Strict PKCS#10 Parser

Goal: validate CSRs structurally before policy or provider use.

Scope:

- Single PEM block only.
- Strict base64 and bounded DER.
- PKCS#10 signature verification.
- SPKI and extension validation.
- Duplicate/unknown attribute policy.
- No trailing blocks or smuggled data.
- Malformed corpus tests.

Exit criteria:

- Trustheim records digest, SPKI fingerprint, and normalized public metadata.
- Raw CSR bytes are discarded unless explicit retention policy requires them.

### v0.27.0: Fuzzed CSR And Name Parsers

Goal: harden all untrusted certificate parsing.

Scope:

- libFuzzer targets for PEM, DER, SAN, DNS, IDNA, URI, IP, extension
  duplication, and normalization.
- Seed corpus.
- Sanitizer runs where supported.
- Timeout and memory budgets.
- Crash triage gate.

Exit criteria:

- Parser fuzz target build and representative runs are part of release gates.

### v0.28.0: Certificate Profile And Local Policy Engine

Goal: deny unsafe issuance before provider calls.

Scope:

- Immutable versioned profiles.
- DNS/IP/URI/EKU/KU/key constraints.
- Issuer binding.
- TTL and not-before rules.
- Profile and policy digesting.
- Conflict detection.
- Deny-by-default evaluation.

Exit criteria:

- Requested SANs are compared against CSR SANs and profile policy.
- Disallowed profiles fail before backend access.

### v0.29.0: End-To-End CSR Signing MVP

Goal: issue a certificate through the full custody-free path.

Scope:

- Authenticated request to manifest.
- Step-up/quorum to authorized operation.
- Credential broker to provider sign.
- Provider response verification.
- Idempotent issuance smoke.
- No key-generation endpoint.

Exit criteria:

- All failure paths fail closed.
- Trustheim never receives subscriber private keys.

### v0.30.0: Certificate Output Verification And Inventory

Goal: verify provider output before returning it.

Scope:

- Signature and chain verification.
- SPKI, SAN, EKU, KU, validity, issuer fingerprint, serial, and profile checks.
- Public certificate inventory.
- Provider reconciliation.

Exit criteria:

- Provider output that does not match the authorized manifest is rejected.

### v0.31.0: Revocation Workflow

Goal: make revocation at least as controlled as issuance.

Scope:

- Revocation reasons and semantics.
- Stronger authorization policy for compromised-key and critical profiles.
- Idempotency and already-revoked handling.
- Audit linkage.
- Provider contract.
- Emergency compromised-key procedure.

Exit criteria:

- Revocation is audited, idempotent, and fail-closed.

### v0.32.0: CRL, OCSP, And ACME Policy

Goal: finish lifecycle publication and automation policy before v1.

Scope:

- CRL publication ownership, freshness, overlap, partitioning, and outage
  behavior.
- OCSP responder authorization and client validation tests.
- ACME profile policy, if enabled.
- EAB or equivalent external-account binding where exposed.
- DNS-01 and wildcard issuance behind explicit quorum policy.

Exit criteria:

- Automated renewal cannot bypass identity, profile, quorum, or audit policy.
- ACME remains disabled unless its provider/profile gates pass.

### v0.33.0: Root, Intermediate, And Issuer-Rotation Ceremonies

Goal: isolate high-value CA ceremonies from runtime issuance.

Scope:

- Offline packages.
- Air-gap transfer format.
- Quorum receipts.
- Intermediate CSR/signing workflow.
- Cross-sign and rotation sequencing.
- Runtime credential denial proof.

Exit criteria:

- Runtime binaries cannot invoke root/intermediate ceremony operations.

### v0.34.0: Privacy-Preserving Telemetry

Goal: observe operations without leaking certificate inventory or secrets.

Scope:

- Allow-listed structured events.
- Cardinality controls.
- No subjects, SANs, tokens, WebAuthn values, or provider response bodies in
  metrics.
- Protected diagnostic mode.
- OpenTelemetry propagation policy.
- Leakage canaries.

Exit criteria:

- Telemetry tests fail if sensitive fields enter logs, metrics, or traces.

### v0.35.0: Concurrency, Idempotency, And Ambiguous-Outcome Recovery

Goal: survive retries, crashes, and provider timeouts.

Scope:

- Load tests.
- Database isolation proof.
- Provider-timeout reconciliation by serial/request correlation.
- Fencing and duplicate suppression.
- Crash/restart recovery.
- Exactly-once externally observable behavior.

Exit criteria:

- Ambiguous provider outcomes reconcile before retry.
- Duplicate issuance is prevented or detected and blocked.

### v0.36.0: Real HSM-Backed Signing Evidence

Goal: prove the production custody tier.

Scope:

- Supported production HSM integration.
- Issuer-key fingerprint proof.
- Vendor, platform, or external-key attestation where available.
- Non-export tests.
- HSM loss and recovery behavior.
- No software fallback for hardware profiles.

Exit criteria:

- HSM-backed profile cannot sign when the HSM/key is unavailable.
- Evidence is rechecked at startup and before high-value signing.

### v0.37.0: Explicit Lower-Tier Overlays And Break-Glass

Goal: make downgrades visible, narrow, and hard to misuse.

Scope:

- Signed or quorum-approved exception format.
- Scope limited by issuer, profile, environment, time, and incident/change ID.
- Manifest inclusion.
- Visible inventory and audit labels.
- Expiry and rollback behavior.
- Tests rejecting implicit or environment-driven downgrades.

Exit criteria:

- A lower-tier overlay cannot authorize root/intermediate operations or silently
  satisfy a hardware profile.

### v0.38.0: Second Provider Proof

Goal: prove provider neutrality before v1.0.0.

Scope:

- Vault preview adapter or another independent provider adapter.
- Provider bootstrap/policy tests for the second provider.
- Semantic capability/evidence matrix.
- Public API, profile, manifest, quorum, and client code unchanged.
- Identical OpenAPI contract snapshot.

Exit criteria:

- Shared provider contract tests pass for first and second provider.
- Provider-specific limitations downgrade claims instead of changing public API.

### v0.39.0: Web Client

Goal: ship the optional browser interface as a normal API client.

Scope:

- Public-API-only client.
- CSP and frontend dependency policy.
- No provider network access.
- WebAuthn UX showing exact manifest details.
- CSRF and XSS tests.
- Disposable deployment proof.

Exit criteria:

- Web compromise exposes only the public API surface and browser-session
  authority, not provider credentials.

### v0.40.0: CLI Client

Goal: ship operator automation without hidden privilege.

Scope:

- Public-API-only commands.
- Secure credential storage.
- No secrets in argv, shell history, logs, or machine-readable output.
- Manifest display and confirmation.
- No hidden privileged route.

Exit criteria:

- CLI cannot do anything unavailable through documented API authorization.

### v0.41.0: Rootless Podman Profiles

Goal: provide hardened container packaging without making containers mandatory.

Scope:

- Separate images, networks, UIDs, and volumes.
- Read-only filesystems.
- Dropped capabilities and no-new-privileges.
- Seccomp and egress policy.
- Secret injection.
- Health checks.
- Pinned base image digests.
- Clean smoke.

Exit criteria:

- Web and CLI containers receive no provider credentials.
- Provider network is not reachable from web/CLI containers.

### v0.42.0: Native/Systemd Hardening

Goal: make native binaries as first-class as containers.

Scope:

- Non-root or dynamic users.
- Filesystem protections.
- Core-dump prohibition.
- Syscall restrictions where supported.
- Credential files.
- Graceful shutdown.
- Socket and egress policy.
- Parity with container security claims.

Exit criteria:

- Native deployment can run without Podman and without weaker defaults.

### v0.43.0: Backup, Restore, And Audit Continuity

Goal: prove recoverability without losing evidence.

Scope:

- Encrypted metadata and provider backups.
- Isolated restore rehearsal.
- Key-share handling.
- Point-in-time recovery.
- Inventory reconciliation.
- Audit checkpoint continuity.
- Recovery objectives.

Exit criteria:

- Restored environment proves certificate inventory and audit continuity.

### v0.44.0: HA, Failover, And Split-Brain Behavior

Goal: make distributed operation safe before stable.

Scope:

- Multi-node coordinator and provider tests.
- Challenge and quorum consistency.
- Leader fencing.
- Clock skew handling.
- Partition behavior.
- Audit sink failure behavior.
- Duplicate issuance prevention under failover.

Exit criteria:

- Failover cannot create duplicate or unaudited issuance.

### v0.45.0: Advanced Profile Gate

Goal: decide which optional protocols can be stable in v1.

Candidates:

- SSH CA.
- SPIFFE/SPIRE.
- SCEP.
- EST.
- Post-quantum or hybrid research profiles.

Exit criteria:

- Any profile promoted before v1 has its own threat model, API tests, fuzz
  targets, provider support matrix, and release-gate additions.
- Profiles not meeting the gate remain research/experimental and disabled.

### v0.46.0: SBOM, Provenance, And Artifact Inventory

Goal: produce release evidence for every artifact.

Scope:

- SPDX and CycloneDX SBOMs for source, binaries, and images.
- License inventory.
- Dependency evidence.
- Source, binary, and image checksums.
- SLSA-style provenance where practical.
- Verification scripts.

Exit criteria:

- Release evidence can be reproduced and verified from a clean checkout.

### v0.47.0: Reproducible Build Gate

Goal: make build drift visible.

Scope:

- Two isolated builders.
- Pinned toolchain and dependencies.
- Path/time normalization.
- diffoscope or equivalent evidence.
- Documented normalized-equivalence exception only where bit-for-bit equality is
  not practical.

Exit criteria:

- Reproducibility failure blocks release.

### v0.48.0: Broad Verification Suite

Goal: expand beyond unit tests.

Scope:

- Property tests.
- Parser and state-machine fuzzing.
- Miri for pure logic.
- Kani/model checking where useful.
- Mutation testing of policy decisions.
- Minimum coverage thresholds.

Exit criteria:

- Verification suite runs in the release gate with documented budgets.

### v0.49.0: DAST, Fault Injection, And Adversarial Testing

Goal: attack the whole system before stable.

Scope:

- Auth bypass.
- Request smuggling.
- SSRF and egress bypass.
- Replay.
- Provider malformation.
- Network partitions.
- Disk-full behavior.
- Clock skew.
- HSM/provider failure.
- Resource exhaustion.

Exit criteria:

- High and critical findings block release.

### v0.50.0: Supply Chain And Hermetic CI

Goal: make CI and dependencies defensible.

Scope:

- GitHub Actions pinned by commit.
- Minimal workflow permissions.
- Protected environments.
- Offline locked builds where practical.
- Mandatory audit/deny tools with no skips in release mode.
- Signed artifacts.
- Cache isolation.

Exit criteria:

- CI can produce and verify the full release evidence bundle.

### v0.51.0: Capacity And Operational Readiness

Goal: prove the system can be operated.

Scope:

- Load and capacity limits.
- HSM latency and failure behavior.
- Audit backpressure.
- Rate-limit tuning.
- SLOs and alerts.
- Upgrade and rollback drills.
- Certificate-expiry monitoring.
- Operator training exercises.

Exit criteria:

- Operations runbooks and alerts are tested, not just documented.

### v0.52.0: Release Candidate And Independent Assessment

Goal: freeze and assess the stable release candidate.

Scope:

- Frozen v1 API.
- Frozen threat model.
- External cryptographic/security review.
- Red-team ceremony.
- Migration and support policy.
- Signed evidence bundle.

Exit criteria:

- All high and critical findings are closed or v1.0.0 is delayed.
- Independent assessment findings are linked from release evidence.

### v1.0.0: Stable Custody-Free CA Coordinator

Goal: production-supported CA coordinator within the documented threat model.

Stable scope:

- CSR-only issuance.
- Public API, web, and CLI as provider-neutral clients.
- At least two backend provider implementations or one real provider plus a
  fully specified independent conformance provider accepted by review.
- WebAuthn enrollment, attestation policy, step-up, and quorum.
- Immutable operation manifests.
- Tamper-evident audit ledger.
- Provider mTLS and credential broker.
- Least-privilege runtime identity.
- First real provider runtime adapter.
- HSM-backed or explicitly lower-tier key-custody evidence.
- Revocation, CRL/OCSP, inventory, backup, restore, HA, native deployment, and
  rootless Podman profiles.
- SBOM, provenance, reproducibility, DAST, fuzzing, and independent assessment.

Non-goals:

- Public CA/browser-trust compliance.
- Silent fallback from hardware-backed to software-backed signing.
- Server-side subscriber-key generation.
- Post-quantum production claims unless the pre-v1 advanced-profile gate has
  promoted a specific profile.

Exit criteria:

- `scripts/stable_release_gate.sh release` and all provider/profile gates run
  without skips.
- Two-provider neutrality evidence or accepted equivalent conformance evidence
  is published.
- The release evidence bundle includes SBOMs, checksums, signatures,
  reproducibility results, provider evidence, HSM/custody evidence, residual
  risks, and supported version matrix.
- GitHub release is signed and references the exact evidence artifacts.

## Immediate Priority Order

Before implementing real signing, complete these decisions and refactors:

1. Remove server-side subscriber-key generation from stable architecture and
   code contracts.
2. Define custody-free/restart-safe semantics and operation manifests.
3. Decide the real HSM/key-custody signing path.
4. Invert API/backend dependencies and redesign provider traits.
5. Split parsing and provider credentials into separate memory domains.
6. Make release-gate skips fatal in release mode.
7. Design WebAuthn/quorum storage around atomic one-time consumption.
8. Implement mTLS and runtime ACL negative tests before real issuance.
9. Prove provider neutrality before v1.0.0.
10. Treat v1.0.0 as blocked until external assessment and reproducible evidence
    pass.

# Trustheim Versioning Plan

Trustheim is versioned as a security product, not as a feature demo. Every item
below is pre-1.0 unless explicitly listed under `v1.0.0`. The purpose of the
pre-1.0 line is to make unsafe shortcuts visible early, keep the public API
provider-neutral, and prevent a working prototype from becoming a release
candidate before custody, audit, identity, and recovery properties are proven.

## Current Research Baseline

These versions were re-checked on 2026-07-17 before updating this plan:

- Rust stable is 1.97.0, released on 2026-07-09.
- OpenBao latest release is v2.5.4, released on 2026-05-20.
- HashiCorp Vault latest observed release line is 1.21.x, with 1.21.4 announced
  in March 2026 and later 1.21.x patch releases visible in upstream release
  listings.

Before any implementation milestone that pins toolchain or provider behavior,
re-check Rust, crate versions, provider releases, HSM/PKCS#11 libraries, WebAuthn
libraries, container base images, and advisory databases. Trustheim may support
OpenBao, HashiCorp Vault, another CA backend, or a future in-house provider, but
no public API may expose a provider-specific concept as an authorization boundary.

## Non-Negotiable Boundaries

- Trustheim is a CA coordinator. It must not store exportable CA private keys.
- The API server is the only component allowed to coordinate signing decisions.
- Web and CLI clients call the API. They must not call CA providers directly.
- Provider adapters implement Trustheim-owned traits. Providers do not define
  Trustheim's public model.
- Storage is backend-neutral. The roadmap may require transactional semantics,
  migrations, locks, uniqueness, optimistic versions, encryption, and backup
  consistency, but it must not make one database product mandatory.
- One administrative trust domain per deployment is the default for v1. Tenant or
  deployment IDs must still be included in persistent keys, manifests, queries,
  audit entries, and provider mappings where future multi-tenancy would otherwise
  create ambiguity.
- Internal database IDs are never bearer authorization tokens.
- The parser worker has no provider credentials and no direct database
  credentials. It receives bounded input and returns authenticated parse results
  to the coordinator.
- A provider credential broker may receive only an authorized operation and a
  digest-matched artifact. It must never be a general-purpose signing oracle.
- WebAuthn proves possession of an enrolled authenticator, not that a human read
  the screen. High-value operations need a trusted transaction display path.
- Hardware-required signing must produce provider or HSM evidence for each
  operation, or under a bounded freshness interval approved by policy.

## Version Gates

Each version has a narrow implementation target and a verification gate. If a
gate cannot be automated, the plan must add a manual ceremony checklist before
the milestone can be considered done.

### v0.1.0: Repository And Prototype Baseline

Goal:
- Keep the initial API, web, CLI, and provider crates separated in one workspace.
- Preserve EUPL-1.2 licensing and project boundary documents.
- Keep OpenAPI generation, smoke tests, and the generic provider trait visible.

Exit criteria:
- `scripts/checks.sh` passes.
- API, web, and CLI crates compile as separate binaries.
- Web and CLI crates have no provider crate dependency.

### v0.2.0: Formal Threat And Privacy Model

Goal:
- Document assets, attacker classes, trust boundaries, and non-goals.
- Define what Trustheim protects from the UI, API, provider, storage, operator,
  network, and host compromise.
- Classify request, CSR, certificate, audit, identity, and provider metadata.

Exit criteria:
- Threat model contains STRIDE-style abuse cases and mitigations.
- Privacy model defines retention and redaction defaults.
- Release gate fails when threat model sections are missing.

### v0.3.0: Key Custody And HSM Boundary Model

Goal:
- Define supported custody tiers: software-dev, provider-managed, TPM-assisted,
  PKCS#11/HSM, and split offline root.
- Define what evidence each tier must provide.
- Make lower-tier modes explicit overlays that cannot be mistaken for the
  highest-security profile.

Exit criteria:
- Custody tier appears in policy decisions, manifests, audit events, and provider
  capability records.
- Hardware-required profiles fail closed without acceptable evidence.

### v0.4.0: CA Lifecycle And Ceremony Model

Goal:
- Define root creation, intermediate issuance, issuer activation, rollover,
  revocation, decommissioning, and disaster-recovery ceremonies.
- Keep root operations outside normal runtime issuance paths.

Exit criteria:
- Runtime orchestrator credentials cannot reach root CA paths.
- Ceremony steps are documented with required quorum, evidence, and audit
  artifacts.

### v0.5.0: Secret, Log, Crash, And Temporary-Data Hygiene

Goal:
- Define data classes for secrets, sensitive request data, public certificates,
  and operational metadata.
- Define logging, panic, crash dump, temporary file, and memory-scrubbing rules.

Exit criteria:
- Logs and errors are redacted by default.
- CSR and provider responses have retention classes.
- Tests cover accidental secret formatting for core request types.

### v0.6.0: Runtime Process And Sandbox Boundaries

Goal:
- Make API, web, CLI, parser worker, provider broker, and background workers
  independent binaries where useful.
- Support native execution and rootless Podman without making containers
  mandatory.

Exit criteria:
- Deployment docs cover native binaries and rootless Podman.
- Process boundary docs state allowed callers and denied callers.
- Parser and broker can be run with smaller privileges than the API server.

### v0.7.0: Storage Architecture And Pending Artifacts

Goal:
- Define the storage abstraction without binding Trustheim to one database
  product.
- Specify required semantics: migrations, schema ownership, least-privilege
  roles, transaction isolation, row locks or equivalent leases, uniqueness,
  optimistic generations, backup consistency, and audit checkpoint alignment.
- Decide pending CSR handling: encrypted canonical CSR DER until terminal state,
  requester resubmission by digest, or an encrypted pending-artifact store.
- Define retention and secure deletion behavior for pending and terminal data.

Exit criteria:
- Storage trait documents required consistency and failure semantics.
- Parser worker has no database credentials.
- Coordinator persists parser results and pending artifact references.
- Tests cover duplicate request IDs, state-generation conflicts, and retention
  transitions using an in-memory conformance store.

### v0.8.0: Inter-Process Protocol Contract

Goal:
- Specify API-to-parser and API-to-broker protocol methods, authentication,
  message limits, concurrency, timeout, cancellation, backpressure, version
  negotiation, downgrade rejection, replay protection, and correlation IDs.
- Bind parser results to the submitted CSR digest so they cannot be substituted.
- Make broker authorization capability-bearing and operation-scoped.

Exit criteria:
- Schemas reject oversize, deeply nested, stale, replayed, or downgraded
  messages.
- RPC errors are redacted but traceable through correlation IDs.
- Broker refuses arbitrary sign-bytes requests.

### v0.9.0: Strict Configuration And Secret Injection

Goal:
- Replace permissive environment parsing with typed configuration files and
  explicit secret references.
- Validate file permissions, unknown keys, default-deny toggles, clock settings,
  provider identities, and storage settings at startup.

Exit criteria:
- Invalid or unknown configuration fails closed.
- Secret values are never printed through debug output.
- Configuration schema is versioned and covered by negative tests.

### v0.10.0: Service-Key Lifecycle

Goal:
- Define lifecycle rules for session-cookie keys, local RPC auth keys, audit
  checkpoint keys, database field-encryption keys, API TLS keys, provider mTLS
  client keys, overlay verification keys, and challenge-MAC keys.
- Define custody, generation, injection, key IDs, rotation overlap, revocation,
  backup, disaster recovery, missing-key behavior, and historical verification.

Exit criteria:
- Signed records include key IDs.
- Rotation tests verify old audit checkpoints and reject expired signing keys.
- Missing or expired keys fail closed with redacted errors.

### v0.11.0: Public API/Domain/Backend Dependency Inversion

Goal:
- Keep public request and response types provider-neutral.
- Move provider-specific configuration behind adapters.
- Keep backend path, mount, role, token, and issuer details out of public API
  authorization models.

Exit criteria:
- Web and CLI compile without provider crates.
- Public API tests pass against a fake provider.
- Provider concepts appear only in provider crates and internal mapping tables.

### v0.12.0: Provider-Neutral Backend Trait v2

Goal:
- Expand the provider trait around capabilities, health, issuer mapping,
  signing, revocation, audit evidence, drift detection, and hardware evidence.
- Model feature absence explicitly instead of with provider names.

Exit criteria:
- Unsupported capabilities downgrade policy claims or fail closed.
- Conformance tests cover at least the fake provider.
- Provider adapters cannot sign using only a public request DTO.

### v0.13.0: Provider Conformance Harness

Goal:
- Build a reusable harness that every provider adapter must pass.
- Include success, denial, timeout, malformed output, ambiguous completion, and
  reconciliation cases.

Exit criteria:
- Harness can run without a real provider.
- Harness proves providers cannot bypass local policy, manifest, audit, quorum,
  or output verification requirements.

### v0.14.0: OpenAPI Contract Validation

Goal:
- Generate and validate a stable OpenAPI document from the API server.
- Add negative contract tests for request sizes, malformed JSON, denied fields,
  content types, idempotency keys, and redacted errors.

Exit criteria:
- OpenAPI diff is reviewed intentionally.
- CLI smoke tests use the published API shape.

### v0.15.0: HTTP Transport And Request Hardening

Goal:
- Add strict request limits, timeout layers, trace IDs, content-type checks,
  body limits, compression policy, CORS policy, and security headers.

Exit criteria:
- Oversized and slow requests are rejected.
- Errors remain provider-neutral and redact sensitive input.
- DAST smoke tests cover malformed HTTP cases.

### v0.16.0: Identity, Sessions, Recovery, And Route Authorization

Goal:
- Define the primary identity source for v1.
- Implement route authorization, session lifecycle, enrollment authorization,
  deprovisioning, suspension, recovery, and genesis guardian ceremony.
- Separate requester, approver, security admin, provider admin, auditor, and
  break-glass roles.

Exit criteria:
- Route tests cover every privileged endpoint.
- Suspended or deprovisioned guardians cannot approve.
- Minimum guardian set and recovery rules are documented and tested.

### v0.17.0: WebAuthn Enrollment And Credential Lifecycle

Goal:
- Add WebAuthn registration and authentication for administrative identities.
- Bind credentials to identity records, authenticator metadata, backup state,
  and revocation state.

Exit criteria:
- Enrollment requires an authorized ceremony.
- Credential revocation takes effect before new approvals.
- Tests cover replayed and cross-user assertions.

### v0.18.0: Hardware Attestation And Assurance Tiers

Goal:
- Record authenticator attestation where available.
- Express policy in assurance tiers, not vendor names.

Exit criteria:
- Profiles can require attested hardware-backed credentials.
- Missing attestation downgrades or denies according to policy.

### v0.19.0: Certificate Profile And Local Policy Engine

Goal:
- Move local policy before operation manifests, WebAuthn challenges, quorum, and
  provider calls.
- Evaluate profile schema, subject and SAN normalization, issuer mapping, TTL,
  KU/EKU, key type, algorithm, custody tier, quorum, authenticator assurance,
  overlays, and decision facts.
- Produce an immutable policy decision with profile versions and digests.

Exit criteria:
- Mutable facts are rechecked immediately before signing.
- Policy result is included in the canonical operation manifest.
- Denial tests cover subject, SAN, TTL, issuer, algorithm, and custody mismatch.

### v0.20.0: Canonical Operation Manifest

Goal:
- Define canonical bytes for all high-value operations.
- Include request digest, normalized subject/SAN, issuer fingerprint, policy
  decision digest, custody tier, requester, approvers, time bounds, provider
  mapping, artifact references, and expected output constraints.

Exit criteria:
- Same logical request produces the same manifest bytes.
- Non-canonical encodings are rejected.
- Manifest digest appears in WebAuthn challenges, quorum approvals, provider
  calls, audit events, and output records.

### v0.21.0: Trusted Transaction Display

Goal:
- Prevent a compromised web UI from silently changing high-value operations.
- Return a signed human-readable receipt derived from the canonical manifest
  bytes for web, CLI, and future dedicated clients to verify.
- Define short authentication strings or digest displays for independent review.

Exit criteria:
- Highest-security profiles require independent client or workstation display.
- Tests prove display receipts fail if manifest bytes are changed.

### v0.22.0: WebAuthn Step-Up And Replay Resistance

Goal:
- Require step-up authentication for certificate issuance, revocation, provider
  mapping changes, policy changes, and guardian changes.
- Bind challenge bytes to canonical manifest digests, route, identity, session,
  time window, and nonce.

Exit criteria:
- Captured assertions cannot be replayed across routes or manifests.
- Expired challenges fail closed.

### v0.23.0: Multi-Party Quorum State Machine

Goal:
- Implement pending, approved, rejected, vetoed, withdrawn, expired, completed,
  and reconciled states.
- Support approval withdrawal before signing and veto/rejection semantics.

Exit criteria:
- Quorum approvals are bound to the same canonical manifest bytes.
- The requester cannot satisfy approver requirements when policy forbids it.
- Guardian unavailability and minimum guardian count are modeled explicitly.

### v0.24.0: Quorum Substitution, Concurrency, And Recovery

Goal:
- Handle concurrent approvals, duplicate submissions, guardian replacement,
  suspension, expiration, stale manifests, and recovery ceremonies.

Exit criteria:
- State-generation tests prevent lost approvals and double completion.
- Recovery requires a documented ceremony and audit event.

### v0.25.0: Secure Time Semantics

Goal:
- Define trusted wall-clock sources, maximum skew, monotonic local deadlines,
  lease fencing independent of wall clock, canonical UTC encoding, provider time
  comparison, database time comparison, and behavior on time jumps.

Exit criteria:
- Signing fails closed when time trust is lost.
- Certificate `notBefore` backdating, expiry, and provider clock drift are
  bounded by policy.
- Tests simulate clock rollback, jump forward, and provider/database skew.

### v0.26.0: Tamper-Evident Audit Ledger

Goal:
- Implement append-only audit records with hash chaining or equivalent external
  checkpointing.
- Cover intent, policy decision, manifest creation, challenge, approval, veto,
  withdrawal, provider call, output verification, drift, reconciliation, and
  access events.
- Define audit confidentiality, HMAC/redaction, query authorization, retention,
  legal hold, deletion constraints, and independent verification tooling.

Exit criteria:
- Tampering with an audit record is detected.
- Audit checkpoints survive service-key rotation.
- Database rollback alone cannot hide or rewrite issuance history.

### v0.27.0: Backend mTLS Transport

Goal:
- Implement provider transport with mTLS, pinned trust roots, explicit SNI/SPIFFE
  or equivalent identity checks, certificate rotation, and denied fallback.

Exit criteria:
- Plain HTTP provider connections are impossible outside explicit dev mode.
- Provider identity mismatch fails closed.

### v0.28.0: Provider Credential Broker And Lifecycle

Goal:
- Issue short-lived provider credentials only for authorized operations.
- Scope credentials to provider, issuer, manifest digest, operation type, and
  time window.

Exit criteria:
- Credentials cannot be reused for a different operation.
- Broker logs are redacted and independently auditable.

### v0.29.0: First Provider Bootstrap

Goal:
- Implement the first real provider bootstrap selected by ADR and conformance
  readiness, without exposing provider-specific public API.
- Automate dev bootstrap for local testing and document production boundaries.

Exit criteria:
- Bootstrap creates least-privilege paths, roles, issuers, audit sinks, and
  health checks for the selected provider.
- Provider bootstrap produces no public API changes.

### v0.30.0: First Provider ACL And Declarative Audit Policy

Goal:
- Codify provider-side least privilege and audit settings as declarative policy.
- Ensure runtime credentials cannot reach root, broad admin, raw key export, or
  unrelated secret paths.

Exit criteria:
- Tests prove denied provider paths remain denied.
- Provider audit settings are verified at startup.

### v0.31.0: Provider Policy Drift Detection

Goal:
- Store canonical expected provider configuration for roles, issuer mappings,
  ACLs, audit settings, key references, transport identity, and hardware evidence
  requirements.
- Check drift at startup, periodically, and before signing.
- Disable signing on unapproved drift and require an explicit reconciliation
  ceremony. Runtime credentials must not auto-repair provider policy.

Exit criteria:
- Drift events are audited.
- Expected issuer public-key fingerprint is included in manifests.
- Signing is denied when provider state differs from the approved model.

### v0.32.0: First Provider Runtime Adapter

Goal:
- Implement health, capability discovery, issuer resolution, signing, evidence
  extraction, revocation support, and reconciliation hooks for the first provider.

Exit criteria:
- Adapter passes provider conformance tests.
- Adapter cannot bypass local policy, manifest, quorum, audit, or drift checks.

### v0.33.0: Strict PKCS#10 CSR Parser

Goal:
- Parse CSR DER in an isolated worker with strict size, algorithm, attribute,
  extension, and string handling.
- Normalize names and SANs before policy evaluation.

Exit criteria:
- Malformed, oversized, ambiguous, and duplicate CSR fields are rejected.
- Parser result is bound to the CSR digest and returned to the coordinator.

### v0.34.0: X.509 Semantic Linting

Goal:
- Validate BasicConstraints, CA bit rejection, pathLenConstraint, name
  constraints, subject/CN policy, SKI/AKI, AIA, CRL DP, Certificate Policies,
  critical extension allowlist, serial entropy and uniqueness, validity windows,
  issuer lifetime, signature algorithm compatibility, IDNA, URI SAN including
  SPIFFE, IP SAN constraints, and unknown critical extensions.
- Add external linting plus Trustheim-specific lint rules.

Exit criteria:
- Provider output that violates the authorized manifest or semantic lint rules is
  rejected.
- Interop vectors cover OpenSSL, rustls, webpki, Java, and Go where practical.

### v0.35.0: Fuzzed CSR And Name Parsers

Goal:
- Fuzz CSR parsing, name normalization, IDNA, URI SAN, IP SAN, profile parsing,
  and manifest canonicalization.

Exit criteria:
- Fuzz corpus is checked in with regression seeds.
- Sanitizer or fuzz jobs are documented in the release gate.

### v0.36.0: Dry-Run CSR Signing MVP

Goal:
- Wire request creation, parsing, policy, manifest, WebAuthn step-up, quorum,
  audit intent, provider drift check, and provider signing in a dry-run or
  internal-only mode.

Exit criteria:
- No externally supported signing release is allowed at this milestone.
- Dry-run proves the full authorization path without publishing a production
  signing claim.

### v0.37.0: Certificate Output Verification And Inventory

Goal:
- Verify provider output against the manifest and policy decision before
  exposing certificates.
- Persist certificate inventory, issuer fingerprint, serial, validity, subject,
  SAN digest, policy digest, manifest digest, and provider evidence reference.

Exit criteria:
- First externally usable signing endpoint is blocked until this gate passes.
- Mismatched provider output is rejected and audited.

### v0.38.0: Revocation Workflow

Goal:
- Implement provider-neutral revocation requests, approval policy, provider call,
  output verification, audit records, and inventory state transitions.

Exit criteria:
- Revocation cannot bypass local policy and quorum.
- Repeated revocation requests are idempotent.

### v0.39.0: CRL, OCSP, And ACME Policy

Goal:
- Define publication and freshness policy for CRL and OCSP behavior.
- Decide whether ACME requests must flow through Trustheim policy, manifest,
  quorum, and audit, or whether direct provider ACME is explicitly out of scope.

Exit criteria:
- Direct provider ACME cannot silently bypass Trustheim claims.
- Freshness and outage behavior are documented and tested.

### v0.40.0: Root, Intermediate, Issuer Rotation, And Rollover

Goal:
- Implement controlled issuer rollover and decommissioning for runtime-managed
  intermediates while keeping root ceremonies separate.

Exit criteria:
- Leaf certificates cannot outlive allowed issuer windows.
- Rollover emits signed manifests and audit records.

### v0.41.0: Privacy-Preserving Telemetry

Goal:
- Add metrics that expose operational health without leaking subject names, SANs,
  CSR data, serial inventory, provider tokens, or identity secrets.

Exit criteria:
- Metrics are reviewed against the data classification table.
- Sensitive labels are denied by tests.

### v0.42.0: Concurrency, Idempotency, And Ambiguous-Recovery

Goal:
- Make provider effects and audit/storage state transactionally coherent:
  persist authorized intent, anchor audit, acquire fenced signing lease, call
  provider, verify output, persist result, anchor completion, and expose output.
- Add outbox and reconciliation handling for crashes after provider signing.
- Prefer at-most-one active authorization lease and deterministic
  reconciliation. Where a provider cannot support lookup or idempotency, detect
  and quarantine possible duplicate issuance.

Exit criteria:
- Crash tests cover every transition around provider calls.
- Ambiguous completions require reconciliation before retry.

### v0.43.0: Real HSM-Backed Signing Evidence

Goal:
- Integrate real HSM, PKCS#11, KMS, or provider-backed hardware evidence for the
  highest custody profile.

Exit criteria:
- Hardware-required profiles fail without current evidence.
- Evidence is bound to manifest, issuer, and provider output.

### v0.44.0: Explicit Lower-Tier Overlays And Break-Glass

Goal:
- Implement development, lab, and emergency overlays that downgrade claims
  visibly and require approval.
- Define break-glass scope, expiry, auditing, and post-event review.

Exit criteria:
- Lower-tier overlays cannot be mistaken for the highest-security profile.
- Break-glass leaves signed audit evidence and expires automatically.

### v0.45.0: Second Provider Proof

Goal:
- Add a second real provider adapter or a high-fidelity external provider
  simulator to prove Trustheim's API, policy, storage, audit, and lifecycle model
  are not bound to the first provider.

Exit criteria:
- Provider conformance harness passes for two implementations.
- Public API and storage schema do not gain provider-specific authorization
  concepts.

### v0.46.0: Web Client

Goal:
- Build the separate web application as an API client.
- Include secure session handling, role-aware UI, manifest receipt display,
  approval workflows, revocation workflows, and audit views.

Exit criteria:
- Web app has no provider credentials or provider crate dependencies.
- High-value operations display signed manifest receipts.

### v0.47.0: CLI Client

Goal:
- Build the separate CLI as an API client for automation and ceremonies.
- Support request creation, status checks, manifest verification, approvals,
  revocation, inventory queries, and audit export verification.

Exit criteria:
- CLI can operate without the web app.
- CLI verifies signed receipts before approval commands.

### v0.48.0: Rootless Podman Profiles

Goal:
- Provide rootless Podman deployment profiles for API, web, parser, broker,
  storage, provider integration, and support services.

Exit criteria:
- Containers run without root privileges.
- Provider network is isolated from web and CLI containers.

### v0.49.0: Native Binary And systemd Hardening

Goal:
- Provide native binary deployment with systemd units, sandboxing, read-only
  paths, capability drops, private tmp, restart policy, and secret injection.

Exit criteria:
- Native deployment reaches feature parity with container deployment.
- Hardening settings are tested by a smoke script.

### v0.50.0: Backup, Restore, And Audit Continuity

Goal:
- Verify backup and restore for storage, configuration, service keys, audit
  checkpoints, provider mappings, and certificate inventory.

Exit criteria:
- Restore drills prove audit continuity and key availability.
- Restored systems reject stale or conflicting provider state.

### v0.51.0: HA, Failover, And Split-Brain Behavior

Goal:
- Define leader election, lease fencing, storage isolation, provider locks,
  audit ordering, and failover recovery for HA deployments.

Exit criteria:
- Split-brain tests cannot produce two active signing leases for one manifest.
- Failover preserves audit continuity.

### v0.52.0: Advanced Profile Gate

Goal:
- Lock the strongest supported profile end to end: hardware custody, trusted
  display, WebAuthn assurance, quorum, drift detection, tamper-evident audit,
  strict X.509 linting, and recovery ceremonies.

Exit criteria:
- Claim matrix lists exactly which properties are proven and which are not.
- Unsupported environments are denied or explicitly downgraded.

### v0.53.0: SBOM, Provenance, And Artifact Inventory

Goal:
- Generate SBOMs, build provenance, checksums, signatures where available, and
  artifact inventories for binaries, containers, schemas, and generated OpenAPI.

Exit criteria:
- Release artifacts can be independently matched to source and dependency graph.

### v0.54.0: Reproducible Build Gate

Goal:
- Make release builds deterministic where practical and document remaining
  nondeterminism.

Exit criteria:
- Rebuild comparison script passes for supported targets or records accepted
  exceptions.

### v0.55.0: Broad Verification Suite

Goal:
- Run formatting, clippy, unit tests, integration tests, property tests, fuzz
  regressions, provider conformance, OpenAPI checks, cargo-deny, cargo-audit,
  SBOM generation, and deployment smoke tests.

Exit criteria:
- A single release gate script verifies the full suite.
- Skipped checks require explicit documented justification.

### v0.56.0: DAST, Fault Injection, And Adversarial Testing

Goal:
- Add dynamic HTTP tests, malformed provider responses, storage faults, network
  partitions, clock faults, replay attempts, privilege confusion attempts, and
  audit tampering.

Exit criteria:
- Adversarial tests are part of release qualification.

### v0.57.0: Supply Chain And Hermetic CI

Goal:
- Pin toolchains, lock dependencies, verify licenses, reject vulnerable
  dependencies, verify generated artifacts, and restrict CI permissions.

Exit criteria:
- CI can build from a clean checkout with minimal permissions.
- Dependency updates require advisory and license checks.

### v0.58.0: Capacity And Operational Readiness

Goal:
- Benchmark issuance, approval, audit, storage, provider latency, and failure
  recovery under realistic load.
- Define SLOs, alerts, runbooks, upgrade process, downgrade policy, and operator
  handoff material.

Exit criteria:
- Load tests meet documented limits.
- Runbooks cover common and security-critical incidents.

### v0.59.0: Release Candidate And Independent Assessment

Goal:
- Freeze public API, run all release gates, complete internal security review,
  resolve high and critical findings, and prepare an external review package.

Exit criteria:
- Independent assessment findings are tracked.
- No known critical or high security issue remains open.
- Documentation claims match tested behavior.

### v1.0.0: Stable Custody-Free CA Coordinator

Goal:
- Release Trustheim as a stable API-first CA coordinator with separate API, web,
  and CLI applications; provider-neutral backend support; storage-neutral
  persistence requirements; strong custody boundaries; quorum approvals;
  tamper-evident audit; trusted transaction display; strict X.509 validation;
  reproducible release artifacts; native and rootless Podman deployment paths;
  and EUPL-1.2 licensing.

Exit criteria:
- Public APIs are versioned and documented.
- At least two provider implementations or one provider plus a high-fidelity
  external simulator pass the conformance harness.
- Release gates pass from a clean checkout.
- Security, operations, backup, restore, recovery, and incident documentation are
  complete enough for a production operator to run without source-code knowledge.

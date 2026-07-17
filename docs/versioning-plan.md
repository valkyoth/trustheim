# Trustheim Versioning Plan

Trustheim is versioned as a security product, not as a feature demo. Every item
below is pre-1.0 unless explicitly listed under `v1.0.0`. The purpose of the
pre-1.0 line is to make unsafe shortcuts visible early, keep the public API
provider-neutral, keep storage replaceable by semantics rather than by product
name, and prevent a working prototype from becoming a release candidate before
custody, audit, identity, storage, and recovery properties are proven.

## Current Research Baseline

These versions were re-checked on 2026-07-17 before updating this plan:

- Rust stable is 1.97.0, released on 2026-07-09.
- OpenBao latest release is v2.5.4, released on 2026-05-20.
- HashiCorp Vault latest observed release line is 1.21.x.

Before any implementation milestone that pins toolchain or provider behavior,
re-check Rust, crate versions, provider releases, HSM/PKCS#11 libraries, WebAuthn
libraries, container base images, and advisory databases. Trustheim may support
OpenBao, HashiCorp Vault, another CA backend, or a future in-house provider, but
no public API may expose a provider-specific concept as an authorization boundary.

## Non-Negotiable Boundaries

- Trustheim must never generate, receive, deliver, persist, log, retry, or cache
  CA or subscriber private keys. Stable issuance is CSR-only. Trustheim may
  retain opaque provider/HSM key references and public-key evidence, but never
  private-key bytes.
- The API server is the only component allowed to coordinate signing decisions.
- Web and CLI clients call the API. They must not call CA providers directly.
- Provider adapters implement Trustheim-owned traits. Providers do not define
  Trustheim's public model.
- Storage is backend-neutral. Trustheim requires explicit transactional,
  durability, locking, migration, and backup semantics; unsupported stores cannot
  be enabled merely because they implement a partial trait.
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

### v0.3.0: Key Custody, HSM Boundary, And Cryptographic Policy

Goal:
- Define supported custody tiers: software-dev, provider-managed, TPM-assisted,
  PKCS#11/HSM, and split offline root.
- Define allowed CA and subscriber key algorithms, RSA sizes and exponents,
  approved curves, signature hash policy, entropy and random-number requirements,
  serial-number entropy, client compatibility, provider/HSM compatibility, FIPS
  claim requirements, deprecation, and algorithm migration.
- Explicitly reject SHA-1, weak RSA, unsupported curves, ambiguous parameters,
  and production claims for post-quantum or hybrid algorithms until promoted by a
  dedicated advanced-profile gate.

Exit criteria:
- Custody tier and cryptographic-policy digest appear in policy decisions,
  manifests, audit events, and provider capability records.
- Hardware-required profiles fail closed without acceptable evidence.
- Profiles reference a versioned cryptographic policy.

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
- Implement non-cloneable redacted secret wrappers for provider tokens, private
  credentials, HSM PIN material, session keys, broker grants, and service keys.
- Use zeroization where copies can be controlled, and document memory-zeroization
  limits honestly.
- Remove predictable `/tmp/trustheim-*` behavior; use `umask 077`, exclusive
  temporary creation, private temporary directories, and cleanup traps.

Exit criteria:
- Logs and errors are redacted by default.
- CSR and provider responses have retention classes.
- Tests cover accidental secret formatting for core request types.
- Provider tokens and private credentials do not implement `Serialize` or
  unredacted `Debug`.
- Production profiles disable core dumps where the platform supports it.
- Panic and error tests prove request bodies, CSR bodies, WebAuthn values, and
  provider response bodies do not appear in logs or client errors.
- Provider credentials, HSM PINs, and authority-bearing secrets cannot be passed
  through CLI arguments or environment variables.

### v0.6.0: Runtime Process And Sandbox Boundaries

Goal:
- Make API, web, CLI, parser worker, provider broker, and background workers
  mandatory separate binaries for the high-assurance profile. Consolidation is
  allowed only in development or explicitly lower-tier profiles.
- Support native execution and rootless Podman without making containers
  mandatory.
- Define high-assurance isolation: parser and broker are separate processes with
  different runtime identities; parser has no DB, provider, or signing authority;
  broker has no session, WebAuthn, or general database credentials.

Exit criteria:
- Deployment docs cover native binaries and rootless Podman.
- Web and CLI cannot reach parser or broker directly.
- Broker egress is limited to configured providers.
- Parser has no network egress unless explicitly required by a documented
  profile.
- A compromise simulation proves the parser cannot invoke signing.
- Single-process mode is explicitly development-only.

### v0.7.0: Storage Architecture And Pending Artifacts

Goal:
- Define the storage abstraction without binding Trustheim to one database
  product.
- Specify required semantics: migrations, schema ownership, least-privilege
  roles, transaction isolation, row locks or equivalent leases, uniqueness,
  optimistic generations, backup consistency, and audit checkpoint alignment.
- Decide and implement pending CSR handling: encrypted canonical CSR DER until
  terminal state, requester resubmission by digest, or an encrypted
  pending-artifact store.
- Define retention and secure deletion behavior for pending and terminal data.

Exit criteria:
- Storage trait documents required consistency and failure semantics.
- Parser worker has no database credentials.
- Coordinator persists parser results and pending artifact references.
- A production storage adapter selected by ADR passes migrations from every
  supported schema version, concurrency tests, crash/restart tests, durable
  fencing tests, and backup-consistency tests.
- The exact isolation level and lock/lease assumptions for each supported store
  are documented.

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
- Enforce owner/mode checks for secret and mTLS files, HTTPS-only production
  provider endpoints, private trust roots, and no provider credentials or HSM
  PINs in CLI arguments or environment variables.
- Define schema-version downgrade rejection, configuration/policy epoch,
  approved provider-state digest, and atomic reload or explicit restart-only
  behavior.

Exit criteria:
- Invalid or unknown configuration fails closed.
- Secret values are never printed through debug output.
- Configuration schema is versioned and covered by negative tests.
- Security-relevant reload invalidates pending operations instead of silently
  preserving old approvals.
- Configuration digest and approved provider-state digest are included in the
  operation manifest.

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
- Use private constructors for `ValidatedCsr` and `AuthorizedOperation`.
- Separate runtime, bootstrap, evidence-inspection, and offline-ceremony traits.
- Represent readiness and signer evidence through `VerifiedProviderReadiness`
  and `VerifiedSignerEvidence`, not self-declared security booleans.
- Keep internal backend errors non-serializable; public errors use fixed safe
  messages and opaque correlation IDs.

Exit criteria:
- Unsupported capabilities downgrade policy claims or fail closed.
- Conformance tests cover at least the fake provider.
- Provider adapters cannot sign using only a public request DTO.
- Provider response bodies are bounded and never directly forwarded to clients or
  logs.

### v0.13.0: Provider Conformance Harness

Goal:
- Build a reusable harness that every provider adapter must pass.
- Include success, denial, timeout, malformed output, ambiguous completion, and
  reconciliation cases.

Exit criteria:
- Harness can run without a real provider.
- Harness proves providers cannot bypass local policy, manifest, audit, quorum,
  drift detection, reconciliation, or output verification requirements.

### v0.14.0: Public API Resource Lifecycle And OpenAPI Validation

Goal:
- Generate and validate a stable OpenAPI document from the API server.
- Define resource lifecycle for create certificate request, retrieve status,
  bounded list/pagination, cancel before signing, approve, reject, veto,
  withdraw, expire, retrieve issued certificate, revoke, and inventory queries.
- Define idempotency-key scope and expiry, conditional updates/state generation,
  enumeration-resistant public identifiers, retention and tombstone semantics,
  and requester visibility into guardian identities.

Exit criteria:
- OpenAPI diff is reviewed intentionally.
- CLI smoke tests use the published API shape.
- Negative contract tests cover request sizes, malformed JSON, denied fields,
  content types, idempotency, conditional updates, and redacted errors.

### v0.15.0: Inbound HTTP/TLS Transport And Request Hardening

Goal:
- Add strict request limits, timeout layers, trace IDs, content-type checks,
  body limits, compression policy, CORS policy, and security headers.
- Define inbound TLS or explicitly trusted terminating proxy behavior, trusted
  proxy allow-list, forwarded header handling, host validation, cookie Secure,
  HttpOnly, SameSite, session fixation prevention, absolute and idle session
  expiry, CSRF handling, liveness/readiness/authenticated diagnostics, and
  graceful shutdown of in-flight approval and signing leases.

Exit criteria:
- Oversized and slow requests are rejected.
- Errors remain provider-neutral and redact sensitive input.
- DAST smoke tests cover malformed HTTP and proxy-header cases.

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
- Distinct approvers mean distinct active guardian identities established through
  the documented identity-proofing procedure, not a cryptographic claim about
  distinct humans.

### v0.17.0: WebAuthn Enrollment And Credential Lifecycle

Goal:
- Add WebAuthn registration and authentication for administrative identities.
- Bind credentials to identity records, authenticator metadata, backup state,
  guardian-set epoch, credential epoch, and revocation state.

Exit criteria:
- Enrollment requires an authorized ceremony.
- Credential ID and public-key uniqueness are enforced across identities.
- Credential revocation takes effect before new approvals.
- Tests cover replayed and cross-user assertions.

### v0.18.0: WebAuthn Verification, Attestation, And Assurance Tiers

Goal:
- Verify exact RP ID hash, allowed origin and top-origin policy, challenge,
  ceremony type, user presence, required user verification, credential public-key
  algorithm, backup eligibility and backup state, signature-counter anomalies,
  attestation format and chain, AAGUID policy, FIDO metadata trust roots,
  metadata freshness, status, revocation, and outage behavior.
- Express policy in assurance tiers, not vendor names.

Exit criteria:
- Missing, stale, revoked, or unverifiable attestation denies a
  hardware-required profile.
- Selecting a lower assurance tier requires an explicit policy decision before
  manifest creation; it is never an authentication-time fallback.
- Metadata-service outage behavior is explicit per profile: fail closed for
  hardware-required profiles unless a documented bounded cache remains valid.

### v0.19.0: Strict PKCS#10 CSR Parser

Goal:
- Parse CSR DER in an isolated worker with strict size, algorithm, attribute,
  extension, and string handling.
- Normalize names and SANs before policy evaluation.
- Define `ValidatedCsr` and parser-result vectors before policy code depends on
  them.
- Verify PKCS#10 proof of possession by checking the CSR signature with the CSR
  public key, rejecting unsupported signature algorithms, malformed parameters,
  algorithm confusion, and key/signature mismatches.
- Reject trailing or multiple PEM/DER objects and duplicate `extensionRequest`
  attributes.

Exit criteria:
- Malformed, oversized, ambiguous, and duplicate CSR fields are rejected.
- Parser result is authenticated, bound to the CSR digest, and returned to the
  coordinator.
- The parsed SPKI is proven to be exactly the key covered by the CSR signature.

### v0.20.0: CSR Request Semantic Validation

Goal:
- Validate CSR semantics before profile policy: requested subject, CN behavior,
  DNS/IP/URI/email SAN normalization, IDNA handling, SPIFFE URI rules where
  enabled, requested extensions, public-key algorithm, and key-usage intent.

Exit criteria:
- Policy receives only normalized, validated CSR facts.
- Tests cover forbidden CA requests, duplicate names, unknown critical CSR
  extensions, unsupported algorithms, and ambiguous encodings.

### v0.21.0: Certificate Profile And Local Policy Engine

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
- Pending operations are invalidated or returned for fresh approval when
  requester or guardian status, credential revocation, attestation status,
  guardian-set epoch, quorum policy, profile version, cryptographic-policy
  version, issuer state, provider mapping, provider-state digest, custody
  evidence, overlay state, artifact digest, or trust-domain configuration
  changes.
- Changed mutable facts are never silently re-evaluated while preserving
  approvals created under the old facts.

### v0.22.0: Canonical Operation Manifest

Goal:
- Define canonical bytes for all high-value operations.
- Include protocol/domain separator `trustheim/manifest/v1`, manifest schema
  version, deployment/trust-domain ID, operation type, operation ID, nonce,
  request digest, normalized subject/SAN facts, issuer fingerprint, policy
  decision digest, cryptographic-policy digest, approved configuration digest,
  approved provider-state digest, guardian approval-policy digest, hash/signature
  algorithm identifiers, canonical encoding identifier, custody tier, requester
  identity, eligible guardian-set digest and epoch, quorum threshold, required
  roles, separation-of-duty constraints, approval-window ID and expiry, time
  bounds, provider mapping, artifact references, and expected output constraints.
- Store actual approvers in separate approval records that reference the
  immutable manifest digest.
- Use separate signed-object domains for manifests, approval receipts, audit
  checkpoints, provider grants, and future protocol objects.

Exit criteria:
- The same fully specified manifest field set has exactly one canonical byte
  encoding.
- Every operation contains a fresh operation ID and nonce, so separate operations
  remain cryptographically distinct.
- Manifest digest appears in WebAuthn challenges, quorum approvals, provider
  calls, audit events, and output records.

### v0.23.0: Trusted Transaction Display

Goal:
- Prevent a compromised web UI from silently changing high-value operations.
- Return a signed human-readable receipt derived from the canonical manifest
  bytes for web, CLI, and future dedicated clients to verify.
- Define short authentication strings or digest displays for independent review.

Exit criteria:
- Highest-security profiles require independent client or workstation display.
- Tests prove display receipts fail if manifest bytes are changed.

### v0.24.0: WebAuthn Step-Up And Replay Resistance

Goal:
- Require step-up authentication for certificate issuance, provider mapping
  changes, policy changes, guardian changes, and high-risk revocation paths.
- Bind challenge bytes to canonical manifest digests, route, identity, session,
  time window, and nonce.
- Use at least 32 random challenge bytes, store durable challenge state for
  restart/HA safety, and prefer a keyed digest over storing raw challenges as
  authority-bearing database values.

Exit criteria:
- Captured assertions cannot be replayed across routes or manifests.
- Expired challenges fail closed.
- Challenges are consumed by atomic compare-and-swap with a uniqueness guarantee;
  there is no verify-then-delete sequence.
- Challenge state is invalidated on manifest, policy, credential, guardian-set,
  or session changes.
- Parallel replay tests prove double consumption fails.

### v0.25.0: Multi-Party Quorum State Machine

Goal:
- Implement pending, approved, rejected, vetoed, withdrawn, expired, completed,
  signing-leased, reconciliation-required, and reconciled states.
- Support approval withdrawal before signing and veto/rejection semantics.
- Retain approval-record transcript fields needed for later independent review
  without unnecessarily retaining raw sensitive WebAuthn data.

Exit criteria:
- Quorum approvals are bound to the same canonical manifest bytes.
- The requester cannot satisfy approver requirements when policy forbids it.
- Guardian unavailability and minimum guardian count are modeled explicitly.

### v0.26.0: Quorum Substitution, Concurrency, And Recovery

Goal:
- Handle concurrent approvals, duplicate submissions, guardian replacement,
  suspension, expiration, stale manifests, and recovery ceremonies.

Exit criteria:
- State-generation tests prevent lost approvals and double completion.
- Recovery requires a documented ceremony and audit event.

### v0.27.0: Secure Time Semantics

Goal:
- Define trusted wall-clock sources, maximum skew, monotonic local deadlines,
  lease fencing independent of wall clock, canonical UTC encoding, provider time
  comparison, database time comparison, and behavior on time jumps.

Exit criteria:
- Signing fails closed when time trust is lost.
- Certificate `notBefore` backdating, expiry, and provider clock drift are
  bounded by policy.
- Tests simulate clock rollback, jump forward, and provider/database skew.

### v0.28.0: Tamper-Evident Audit Ledger And Independent Anchoring

Goal:
- Implement canonical hash-linked or Merkle-accumulated local events plus signed
  checkpoints persisted to an independent append-only or WORM destination.
- Cover intent, policy decision, manifest creation, challenge, approval, veto,
  withdrawal, provider call, output verification, drift, reconciliation, and
  access events.
- Define audit confidentiality, HMAC/redaction, query authorization, retention,
  legal hold, deletion constraints, and independent verification tooling.
- Define audit schema versioning, canonical serialization, checkpoint sequence,
  anti-rollback counters, anchoring timeout, remote backpressure, and behavior
  when completion anchoring fails after provider success.
- Specify whether provider access requires synchronous external checkpoint
  completion or whether a durable local outbox plus independently replicated
  checkpoint is sufficient for each profile.

Exit criteria:
- Tampering with an audit record is detected.
- Database rollback alone cannot hide or rewrite issuance history.
- Audit checkpoints survive service-key rotation.
- Provider audit correlation includes authorized intent before provider access,
  provider request identifier, provider audit-event confirmation where available,
  output digest, and reconciliation result.
- Authorization intent is durably recorded before provider credential
  acquisition.
- Signing does not start when the required audit or checkpoint destination is
  unavailable for the active profile.
- Independent audit verification does not require access to the primary
  database.
- Reconciliation operations themselves require audit intent.

### v0.29.0: Backend mTLS Transport

Goal:
- Implement provider transport with mTLS, pinned private trust roots, explicit
  SNI/SPIFFE or equivalent identity checks, TLS 1.3 by default, justified TLS 1.2
  compatibility only where required, allowed cipher suites, signature algorithms,
  key-exchange groups, client certificate EKU and identity checks, redirect
  denial, no environment proxy discovery, timeouts, response-size limits,
  rotation overlap, and expiry handling.

Exit criteria:
- Plain HTTP provider connections are impossible outside explicit dev mode.
- Host root-store fallback is impossible unless explicitly configured for a lower
  assurance profile.
- Provider identity mismatch fails closed.

### v0.30.0: Provider Credential Broker And Lifecycle

Goal:
- Issue opaque single-use broker grants only for authorized operations.
- Distinguish provider-enforced restrictions, such as issuer path, role,
  operation type, TTL, and use count, from broker-enforced restrictions, such as
  manifest binding, single-use handle, operation state, and caller identity.
- Attach manifest digest as safe metadata or header where the provider supports
  it without claiming cross-provider cryptographic token binding.

Exit criteria:
- Broker grants cannot be reused for a different operation.
- Underlying provider tokens are never exposed to web, CLI, parser, or public API
  callers.
- Broker logs are redacted and independently auditable.

### v0.31.0: First Provider Bootstrap

Goal:
- Implement the first real provider bootstrap selected by ADR and conformance
  readiness, without exposing provider-specific public API.
- Automate disposable dev bootstrap for local testing and document production
  boundaries.
- Add a separate production bootstrap/ceremony mode that is not linked into the
  runtime server, uses explicit administrative credentials supplied only for the
  ceremony, supports plan/dry-run and idempotent apply, and creates or verifies
  mounts, roles, issuer mappings, auth methods, and runtime policies.

Exit criteria:
- Dev bootstrap may create local disposable audit sinks.
- Production audit sinks are provisioned declaratively or out of band.
- Runtime and routine bootstrap credentials cannot mutate production audit sinks.
- Trustheim verifies audit readiness but cannot auto-repair it with runtime
  credentials.
- Production bootstrap never creates production roots outside the root ceremony.
- Root tokens, recovery shares, HSM PINs, and unseal material are never written
  to ordinary output.
- Bootstrap produces a configuration/evidence digest consumed by drift detection,
  revokes or removes bootstrap credentials afterward, and refuses to operate with
  runtime credentials.

### v0.32.0: First Provider ACL And Declarative Audit Policy

Goal:
- Codify provider-side least privilege and audit settings as declarative policy.
- Ensure runtime credentials cannot reach root, broad admin, raw key export, or
  unrelated secret paths.

Exit criteria:
- Tests prove denied provider paths remain denied.
- Provider audit settings are verified at startup.

### v0.33.0: Provider Policy Drift Detection

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
- Detailed provider readiness and capability information is authenticated-only;
  public health endpoints expose only coarse profile availability.

### v0.34.0: First Provider Runtime Adapter

Goal:
- Implement health, capability discovery, issuer resolution, signing, evidence
  extraction, revocation support, audit correlation, and reconciliation hooks for
  the first provider.

Exit criteria:
- Adapter passes provider conformance tests.
- Adapter cannot bypass local policy, manifest, quorum, audit, drift checks,
  reconciliation, or output verification.

### v0.35.0: X.509 Semantic Linting

Goal:
- Validate BasicConstraints, CA bit rejection, pathLenConstraint, name
  constraints, subject/CN policy, SKI/AKI, AIA, CRL DP, Certificate Policies,
  critical extension allowlist, serial entropy and uniqueness, validity windows,
  issuer lifetime, signature algorithm compatibility, IDNA, URI SAN including
  SPIFFE, IP SAN constraints, and unknown critical extensions.
- Add external linting plus Trustheim-specific lint rules.
- Define cryptographic output checks: leaf SPKI must equal the approved CSR
  SPKI, leaf signature verifies under the expected issuer, chain verifies to the
  configured trust anchor under Trustheim policy, issuer fingerprint matches the
  manifest, authorized subject/SAN set matches exactly, KU/EKU/BasicConstraints/
  policies/validity/critical extensions match, returned serial is valid and
  unique, returned chain contains no unexpected certificates, and the leaf does
  not outlive the issuer.

Exit criteria:
- Provider output that violates the authorized manifest or semantic lint rules is
  rejected.
- Interop vectors cover OpenSSL, rustls, webpki, Java, and Go where practical.
- Provider-supplied private-key fields are never accepted or deserialized.

### v0.36.0: Fuzzed CSR, Name, Policy, And Manifest Parsers

Goal:
- Fuzz CSR parsing, name normalization, IDNA, URI SAN, IP SAN, profile parsing,
  policy decisions, and manifest canonicalization.

Exit criteria:
- Fuzz corpus is checked in with regression seeds.
- Sanitizer or fuzz jobs are documented in the release gate.

### v0.37.0: Transactional Reconciliation Before Provider Effects

Goal:
- Make provider effects and audit/storage state transactionally coherent before
  any externally usable signing or revocation endpoint exists: persist
  authorized intent, anchor audit, acquire fenced signing lease, call provider,
  verify output, persist result, anchor completion, and expose output.
- Add outbox and reconciliation handling for crashes after provider signing or
  revocation.
- Prefer at-most-one active authorization lease and deterministic
  reconciliation. Where a provider cannot support lookup or idempotency, detect
  and quarantine possible duplicate issuance.
- Define graceful shutdown so a worker cannot abandon a provider call while
  immediately releasing its fencing lease.

Exit criteria:
- Crash tests cover every transition around provider calls.
- Ambiguous completions require reconciliation before retry.
- v0.38 and later provider-effect milestones are lab-only unless this gate
  passes.

### v0.38.0: Real HSM-Backed Signing Evidence

Goal:
- Integrate real HSM, PKCS#11, KMS, or provider-backed hardware evidence for the
  highest custody profile before production intermediate creation, rollover, or
  externally supported issuance claims.

Exit criteria:
- Hardware-required profiles fail without current evidence.
- Evidence is bound to manifest, issuer, and provider output.
- Earlier SoftHSM or provider-key paths remain lab or lower-tier profiles only.

### v0.39.0: Dry-Run CSR Signing MVP

Goal:
- Wire request creation, parsing, policy, manifest, trusted display, WebAuthn
  step-up, quorum, audit intent, provider drift check, reconciliation, and
  provider signing in a dry-run or internal-only mode.

Exit criteria:
- No externally supported signing release is allowed at this milestone.
- Dry-run proves the full authorization path without publishing a production
  signing claim.

### v0.40.0: Certificate Output Verification And Inventory

Goal:
- Verify provider output against the manifest and policy decision before
  exposing certificates.
- Persist certificate inventory, issuer fingerprint, serial, validity, subject,
  SAN digest, policy digest, manifest digest, and provider evidence reference.
- Re-run the v0.35 cryptographic output checks on the exact certificate and
  chain returned by the provider before any client exposure.

Exit criteria:
- First externally usable signing endpoint is blocked until reconciliation,
  hardware-evidence requirements, and this gate pass.
- Mismatched provider output is rejected and audited.
- Any output mismatch quarantines the result and triggers reconciliation instead
  of retrying issuance blindly.

### v0.41.0: Revocation Workflow

Goal:
- Implement provider-neutral revocation requests, approval policy, provider call,
  reconciliation, output verification, audit records, and inventory state
  transitions.
- Define a risk-based authorization matrix: owner/requester rapid
  self-revocation, security-operator emergency revocation with strong
  authentication and immediate audit, stronger authorization for unrelated
  certificate revocation, and isolated CA/intermediate revocation ceremonies.

Exit criteria:
- Revocation cannot bypass its configured authorization matrix.
- Repeated revocation requests are idempotent or reconciled before retry.

### v0.42.0: CRL And OCSP Publication

Goal:
- Implement mandatory CRL/OCSP freshness monitoring, publication behavior,
  delegated responder or CRL-signing key custody, rotation, outage policy, and
  audit records.
- Cryptographically verify CRL signatures, CRL issuer, CRL number, validity,
  `thisUpdate`, `nextUpdate`, locally known revoked serial inclusion, and
  partitioned or delta CRL reconciliation where supported.
- Verify delegated OCSP responder certificate, EKU, issuer, lifetime, response
  signature, and good/revoked/unknown/stale/malformed response handling.

Exit criteria:
- Freshness failures are visible and policy-bound.
- Responder and CRL-signing keys follow the custody and service-key lifecycle.
- Retired issuers continue publishing status until all relevant certificates
  expire.
- Monitoring failure is loud and policy-bound without necessarily taking down
  unrelated issuance unless the active profile requires it.

### v0.43.0: Optional ACME Policy Gate

Goal:
- Decide whether ACME is supported through Trustheim-controlled policy,
  manifest, inventory, and audit paths.
- Keep direct provider ACME explicitly out of v1 unless Trustheim can prove it
  cannot bypass local policy, inventory, and audit.

Exit criteria:
- ACME remains disabled or experimental unless it passes the same policy,
  provider, audit, and inventory gates as other issuance paths.

### v0.44.0: Root, Intermediate, Issuer Rotation, And Rollover

Goal:
- Implement controlled issuer rollover and decommissioning for runtime-managed
  intermediates while keeping root ceremonies separate.

Exit criteria:
- Leaf certificates cannot outlive allowed issuer windows.
- Production rollover requires the applicable hardware-evidence profile.
- Rollover emits signed manifests and audit records.

### v0.45.0: Privacy-Preserving Telemetry

Goal:
- Add metrics that expose operational health without leaking subject names, SANs,
  CSR data, serial inventory, provider tokens, or identity secrets.

Exit criteria:
- Metrics are reviewed against the data classification table.
- Sensitive labels are denied by tests.

### v0.46.0: Explicit Lower-Tier Overlays And Break-Glass

Goal:
- Implement development, lab, and emergency overlays that downgrade claims
  visibly and require approval.
- Define break-glass scope, expiry, auditing, and post-event review.
- Define an invariant floor: no overlay or break-glass path may disable CSR-only
  custody, private-key prohibition, provider identity verification, audit intent
  recording, manifest integrity, output verification, root/runtime credential
  separation, or public API/provider isolation.

Exit criteria:
- Lower-tier overlays cannot be mistaken for the highest-security profile.
- Break-glass leaves signed audit evidence and expires automatically.
- Break-glass may alter quorum, availability, custody claims, or profile
  constraints only within explicitly permitted bounds; it is never "skip
  authorization and call provider."

### v0.47.0: Second Provider Proof

Goal:
- Prove Trustheim's API, policy, storage, audit, credential, issuer, capability,
  and lifecycle model are not bound to the first provider.
- Require either two real provider adapters, or one real provider adapter plus an
  independently specified adapter using meaningfully different credential, audit,
  issuer, and capability semantics.

Exit criteria:
- Provider conformance harness passes for the required independent providers.
- A fake provider remains useful for tests but is not accepted as the sole
  provider-neutrality proof.
- Public API and storage schema do not gain provider-specific authorization
  concepts.

### v0.48.0: Web Client

Goal:
- Build the separate web application as an API client.
- Include secure session handling, role-aware UI, manifest receipt display,
  approval workflows, revocation workflows, and audit views.

Exit criteria:
- Web app has no provider credentials or provider crate dependencies.
- High-value operations display signed manifest receipts.

### v0.49.0: CLI Client

Goal:
- Build the separate CLI as an API client for automation and ceremonies.
- Support request creation, status checks, manifest verification, approvals,
  revocation, inventory queries, and audit export verification.

Exit criteria:
- CLI can operate without the web app.
- CLI verifies signed receipts before approval commands.

### v0.50.0: Rootless Podman Profiles

Goal:
- Provide rootless Podman deployment profiles for API, web, parser, broker,
  storage, provider integration, and support services.

Exit criteria:
- Containers run without root privileges.
- Provider network is isolated from web and CLI containers.

### v0.51.0: Native Binary And systemd Hardening

Goal:
- Provide native binary deployment with systemd units, sandboxing, read-only
  paths, capability drops, private tmp, restart policy, and secret injection.

Exit criteria:
- Native deployment reaches feature parity with container deployment.
- Hardening settings are tested by a smoke script.

### v0.52.0: Backup, Restore, And Audit Continuity

Goal:
- Verify backup and restore for storage, configuration, service keys, audit
  checkpoints, provider mappings, and certificate inventory.

Exit criteria:
- Restore drills prove audit continuity and key availability.
- Restored systems reject stale or conflicting provider state.

### v0.53.0: HA, Failover, And Split-Brain Behavior

Goal:
- Define leader election, lease fencing, storage isolation, provider locks,
  audit ordering, and failover recovery for HA deployments.

Exit criteria:
- Split-brain tests cannot produce two active signing leases for one manifest.
- Failover preserves audit continuity.

### v0.54.0: Advanced Profile Gate

Goal:
- Lock the strongest supported profile end to end: hardware custody, trusted
  display, WebAuthn assurance, quorum, drift detection, tamper-evident audit,
  strict X.509 linting, and recovery ceremonies.

Exit criteria:
- Claim matrix lists exactly which properties are proven and which are not.
- Unsupported environments are denied or explicitly downgraded.

### v0.55.0: SBOM, Provenance, And Artifact Inventory

Goal:
- Generate SBOMs, build provenance, checksums, signatures where available, and
  artifact inventories for binaries, containers, schemas, and generated OpenAPI.

Exit criteria:
- Release artifacts can be independently matched to source and dependency graph.

### v0.56.0: Reproducible Build Gate

Goal:
- Make release builds deterministic where practical and document remaining
  nondeterminism.

Exit criteria:
- Rebuild comparison script passes for supported targets or records accepted
  exceptions.

### v0.57.0: Broad Verification Suite

Goal:
- Run formatting, clippy, unit tests, integration tests, property tests, fuzz
  regressions, provider conformance, OpenAPI checks, cargo-deny, cargo-audit,
  SBOM generation, and deployment smoke tests.

Exit criteria:
- A single release gate script verifies the full suite.
- Skipped checks require explicit documented justification.

### v0.58.0: DAST, Fault Injection, And Adversarial Testing

Goal:
- Add dynamic HTTP tests, malformed provider responses, storage faults, network
  partitions, clock faults, replay attempts, privilege confusion attempts, and
  audit tampering.

Exit criteria:
- Adversarial tests are part of release qualification.

### v0.59.0: Supply Chain And Hermetic CI

Goal:
- Pin toolchains, lock dependencies, verify licenses, reject vulnerable
  dependencies, verify generated artifacts, and restrict CI permissions.

Exit criteria:
- CI can build from a clean checkout with minimal permissions.
- Dependency updates require advisory and license checks.

### v0.60.0: Capacity And Operational Readiness

Goal:
- Benchmark issuance, approval, audit, storage, provider latency, and failure
  recovery under realistic load.
- Define SLOs, alerts, runbooks, upgrade process, downgrade policy, and operator
  handoff material.

Exit criteria:
- Load tests meet documented limits.
- Runbooks cover common and security-critical incidents.

### v0.61.0: Release Candidate And Independent Assessment

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
  persistence requirements with at least one production storage adapter; strong
  custody boundaries; quorum approvals; tamper-evident audit with independent
  anchoring; trusted transaction display; strict X.509 validation; real
  hardware-backed evidence for hardware-required profiles; reproducible release
  artifacts; native and rootless Podman deployment paths; and EUPL-1.2 licensing.

Exit criteria:
- Public APIs are versioned and documented.
- Two real provider adapters, or one real provider adapter plus an independently
  specified adapter with meaningfully different semantics, pass the conformance
  harness.
- Release gates pass from a clean checkout.
- Security, operations, backup, restore, recovery, and incident documentation are
  complete enough for a production operator to run without source-code knowledge.

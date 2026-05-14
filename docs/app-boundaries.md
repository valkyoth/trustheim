# Application Boundaries

Trustheim should remain usable entirely through its API. The web interface and
CLI are separate clients, not privileged paths into the CA.

## Runnable Apps

The monorepo currently contains three runnable app crates:

- `trustheim-api-server`: the API service and only app allowed to talk to a CA
  backend provider.
- `trustheim-web`: optional web interface app. It must call the public API and
  must not import backend provider crates.
- `trustheim-cli`: optional command-line client. It must call the public API and
  must not import backend provider crates.

The reusable crates are:

- `trustheim-domain`: provider-neutral domain types and validation.
- `trustheim-api`: public DTOs and OpenAPI schema.
- `trustheim-backend`: backend provider trait and shared contract.
- `trustheim-backend-openbao`: first provider adapter.

## Deployment Boundary

Production deployment should use separate processes. Rootless Podman is the
recommended hardened deployment path for operators who want containers, but it
must not be required. Every runnable app should also work as a normal native
binary under a service manager such as systemd, s6, runit, OpenRC, or a simple
operator shell during testing.

- API process or container: `trustheim-api-server`.
- Web process or container: `trustheim-web`, when enabled.
- CLI binary or utility container: `trustheim-cli`, when useful for operators.
- Backend provider process, container, or external service: OpenBao first, Vault
  later.

The web app should be disposable from a security point of view. If the web app
process or container is compromised, the attacker still has only the public API
surface, not direct OpenBao/Vault access and not CA private key custody.

## Repository Boundary

Keeping the API server, web interface, CLI, and provider adapters in this
repository is useful while the public contract is still moving. Before a stable
1.x release, the project should re-evaluate whether to split clients into their
own repositories:

- `trustheim-web` can move when the OpenAPI contract is stable enough for it to
  version independently.
- `trustheim-cli` can move when API authentication and certificate workflows are
  stable.
- Provider adapters should stay in the main repository until the backend
  contract has more than one stable implementation.

Splitting repositories must not create a hidden privileged API for the web or
CLI. They remain normal API clients.

## Dependency Rule

Allowed dependency direction:

- API server may depend on `trustheim-api`, `trustheim-domain`,
  `trustheim-backend`, and selected backend adapters.
- Web may depend on `trustheim-api` and client-side UI dependencies only.
- CLI may depend on `trustheim-api` and an HTTP client only.
- Web and CLI must not depend on `trustheim-backend-openbao`,
  `trustheim-backend-vault`, or any provider bootstrap crate.

## Native Binary Rule

Container packaging must wrap the same binaries that operators can run
directly. Do not put required runtime behavior only in a container entrypoint.

Required standalone binaries:

- `trustheim-api-server`
- `trustheim-web`
- `trustheim-cli`

The release gate must build all of them outside a container before any Podman
image build is considered valid.

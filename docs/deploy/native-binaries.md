# Native Binary Deployment

Trustheim must be usable without Podman. Rootless containers are a supported
hardening and packaging option, not a requirement.

## Binaries

The workspace builds separate native binaries:

- `trustheim-api-server`: API service.
- `trustheim-web`: optional web interface.
- `trustheim-cli`: optional CLI client.

Build them locally:

```bash
cargo build --release --bins
```

Run the API server locally:

```bash
TRUSTHEIM_BIND_ADDR=127.0.0.1:8787 cargo run -p trustheim-api-server
```

The API server currently starts with a rejecting backend placeholder. Real
OpenBao/Vault configuration will be added in later milestones.

## Operational Requirements

Native deployments must support:

- non-root runtime user;
- explicit bind address;
- configuration from files and environment;
- mTLS file permission checks before backend use;
- no root token or HSM PIN in process arguments;
- structured logs without secret values;
- graceful shutdown on `SIGTERM`.

## Packaging Rule

Podman images, systemd units, and future package formats must all run the same
compiled binaries. Container entrypoints may prepare filesystem permissions or
load environment files, but they must not contain business logic that native
binary users cannot run.

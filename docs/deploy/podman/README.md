# Podman Deployment Boundary

Trustheim should ship separate rootless container images for separate trust
roles:

- `trustheim-api-server`: public API, authentication, authorization, audit
  correlation, and backend-provider calls.
- `trustheim-web`: optional browser interface. It talks only to
  `trustheim-api-server`.
- `trustheim-cli`: optional operator utility image. It talks only to
  `trustheim-api-server`.
- `openbao` or another backend provider: policy engine and PKI backend.

The web and CLI containers must not receive OpenBao or Vault credentials. They
should receive only the API base URL and their own client configuration.

The first real Podman milestone should add image builds and smoke tests for the
API server only. Web and CLI images should be added when those apps have real
runtime behavior.

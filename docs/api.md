# API

Trustheim is API-first. The web interface and CLI are clients of this API, not
separate privileged control paths.

The API server exposes the OpenAPI document at:

```text
GET /api/openapi.json
```

Current implemented endpoints:

- `GET /health`
- `GET /api/v1/backend/capabilities`
- `POST /api/v1/certificates/sign-csr`

`POST /api/v1/certificates/sign-csr` accepts provider-neutral certificate
request fields:

- `profile`
- `issuer`
- `csr_pem`
- `dns_sans`
- `ttl`

OpenBao or Vault mount paths, role names, raw backend tokens, and provider
issuer ids are not part of the public request body. Those remain provider
configuration and internal audit metadata.

Until a backend is configured, the sign-CSR endpoint validates the request body
and then returns `503` with `unsupported`. Invalid JSON or invalid typed fields
return `invalid_request` before any backend is called.

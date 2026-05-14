#!/usr/bin/env sh
set -eu

addr="${TRUSTHEIM_SMOKE_ADDR:-127.0.0.1:18787}"
base_url="http://${addr}"

TRUSTHEIM_BIND_ADDR="$addr" cargo run -p trustheim-api-server >/tmp/trustheim-smoke-api.log 2>&1 &
server_pid="$!"

cleanup() {
    kill "$server_pid" >/dev/null 2>&1 || true
}
trap cleanup EXIT INT TERM

ready=0
for _ in 1 2 3 4 5 6 7 8 9 10; do
    if curl -sSf "${base_url}/health" >/tmp/trustheim-smoke-health.json 2>/dev/null; then
        ready=1
        break
    fi
    sleep 1
done

if [ "$ready" -ne 1 ]; then
    echo "smoke api: server did not become ready" >&2
    cat /tmp/trustheim-smoke-api.log >&2 || true
    exit 1
fi

grep -q '"status":"degraded"' /tmp/trustheim-smoke-health.json

curl -sSf "${base_url}/api/v1/backend/capabilities" >/tmp/trustheim-smoke-capabilities.json
grep -q '"provider":"not_configured"' /tmp/trustheim-smoke-capabilities.json
grep -q '"csr_signing":false' /tmp/trustheim-smoke-capabilities.json

curl -sSf "${base_url}/api/openapi.json" >/tmp/trustheim-smoke-openapi.json
grep -q '"/health"' /tmp/trustheim-smoke-openapi.json
grep -q '"/api/v1/backend/capabilities"' /tmp/trustheim-smoke-openapi.json
grep -q '"/api/v1/certificates/sign-csr"' /tmp/trustheim-smoke-openapi.json

cat >/tmp/trustheim-smoke-sign-csr.json <<'JSON'
{
  "profile": "server-modern",
  "issuer": "ops-intermediate",
  "csr_pem": "-----BEGIN CERTIFICATE REQUEST-----\nMIIB\n-----END CERTIFICATE REQUEST-----\n",
  "dns_sans": ["api.example.internal"],
  "ttl": 3600
}
JSON

status="$(curl -sS -o /tmp/trustheim-smoke-sign-csr-response.json -w "%{http_code}" \
    -H "content-type: application/json" \
    --data @/tmp/trustheim-smoke-sign-csr.json \
    "${base_url}/api/v1/certificates/sign-csr")"
test "$status" = "503"
grep -q '"code":"unsupported"' /tmp/trustheim-smoke-sign-csr-response.json

cat >/tmp/trustheim-smoke-invalid-sign-csr.json <<'JSON'
{
  "profile": "Server Modern",
  "issuer": "ops-intermediate",
  "csr_pem": "-----BEGIN CERTIFICATE REQUEST-----\nMIIB\n-----END CERTIFICATE REQUEST-----\n",
  "dns_sans": ["*.example.internal"],
  "ttl": 0
}
JSON

status="$(curl -sS -o /tmp/trustheim-smoke-invalid-sign-csr-response.txt -w "%{http_code}" \
    -H "content-type: application/json" \
    --data @/tmp/trustheim-smoke-invalid-sign-csr.json \
    "${base_url}/api/v1/certificates/sign-csr")"
test "$status" = "422"
grep -q '"code":"invalid_request"' /tmp/trustheim-smoke-invalid-sign-csr-response.txt

echo "smoke api: ok"

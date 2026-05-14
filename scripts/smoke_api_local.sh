#!/usr/bin/env sh
set -eu

addr="${TRUSTHEIM_SMOKE_ADDR:-127.0.0.1:18787}"
base_url="http://${addr}"

TRUSTHEIM_BIND_ADDR="$addr" cargo run -p trustheim-server >/tmp/trustheim-smoke-api.log 2>&1 &
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

echo "smoke api: ok"

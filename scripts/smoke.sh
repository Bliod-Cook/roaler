#!/usr/bin/env bash
set -euo pipefail

base_url="${ROALER_SMOKE_BASE_URL:-http://localhost:8080}"

echo "[smoke] checking health endpoint"
curl --fail --silent "${base_url}/api/system/health" >/dev/null

echo "[smoke] checking openapi endpoint"
curl --fail --silent "${base_url}/api/system/openapi.json" >/dev/null

echo "[smoke] ok"


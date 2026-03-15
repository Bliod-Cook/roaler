#!/usr/bin/env bash
set -euo pipefail

python3 -m http.server 9324 --bind 127.0.0.1 --directory tests/fixtures >/tmp/roaler-feed-server.log 2>&1 &
FEED_SERVER_PID=$!

cleanup() {
  kill "$FEED_SERVER_PID" >/dev/null 2>&1 || true
}

trap cleanup EXIT INT TERM

exec pnpm dev --host 127.0.0.1 --port 4173

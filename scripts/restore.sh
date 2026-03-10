#!/usr/bin/env bash
set -euo pipefail

sql_dump="${1:?usage: scripts/restore.sh <postgres.sql> [files.tar.gz]}"
files_archive="${2:-}"

echo "[restore] loading postgres dump"
cat "${sql_dump}" | docker compose exec -T postgres psql -U roaler roaler

if [[ -n "${files_archive}" ]]; then
  echo "[restore] restoring local files"
  docker run --rm -v roaler_roaler-files:/data -v "$(pwd):/workspace" alpine \
    sh -c "rm -rf /data/* && tar -xzf /workspace/${files_archive} -C /data"
fi

echo "[restore] done"


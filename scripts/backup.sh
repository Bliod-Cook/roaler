#!/usr/bin/env bash
set -euo pipefail

backup_dir="${1:-./backups}"
timestamp="$(date -u '+%Y%m%d-%H%M%S')"
mkdir -p "${backup_dir}"

echo "[backup] dumping postgres"
docker compose exec -T postgres pg_dump -U roaler roaler > "${backup_dir}/postgres-${timestamp}.sql"

echo "[backup] archiving local files"
docker run --rm -v roaler_roaler-files:/data -v "$(pwd)/${backup_dir}:/backup" alpine \
  tar -czf "/backup/files-${timestamp}.tar.gz" -C /data .

echo "[backup] done -> ${backup_dir}"


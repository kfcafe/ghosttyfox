#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
MANIFEST_DIR="${HOME}/Library/Application Support/Mozilla/NativeMessagingHosts"
MANIFEST_PATH="${MANIFEST_DIR}/ghosttyfox.json"
HOST_PATH="${PROJECT_DIR}/native-host/target/debug/ghosttyfox-host"

mkdir -p "${MANIFEST_DIR}"

cargo build --manifest-path "${PROJECT_DIR}/native-host/Cargo.toml"
npm --prefix "${PROJECT_DIR}" run build

cat > "${MANIFEST_PATH}" <<EOF
{
  "name": "ghosttyfox",
  "description": "Ghosttyfox native messaging host",
  "path": "${HOST_PATH}",
  "type": "stdio",
  "allowed_extensions": ["ghosttyfox@tower.dev"]
}
EOF

cd "${PROJECT_DIR}"
npx web-ext run --source-dir extension

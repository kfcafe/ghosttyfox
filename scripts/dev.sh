#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
HOST_PATH="${PROJECT_DIR}/native-host/ghosttyfox-host"

# Detect platform for manifest directory
case "$(uname -s)" in
  Darwin)
    MANIFEST_DIR="${HOME}/Library/Application Support/Mozilla/NativeMessagingHosts"
    ;;
  Linux)
    MANIFEST_DIR="${HOME}/.mozilla/native-messaging-hosts"
    ;;
  *)
    echo "Unsupported platform: $(uname -s)" >&2
    exit 1
    ;;
esac

MANIFEST_PATH="${MANIFEST_DIR}/ghosttyfox.json"

mkdir -p "${MANIFEST_DIR}"

# Bundle extension
npm --prefix "${PROJECT_DIR}" run build

# Write native messaging manifest
cat > "${MANIFEST_PATH}" <<EOF
{
  "name": "ghosttyfox",
  "description": "Ghosttyfox native messaging host",
  "path": "${HOST_PATH}",
  "type": "stdio",
  "allowed_extensions": ["ghosttyfox@tower.dev"]
}
EOF

# Launch Firefox with web-ext
cd "${PROJECT_DIR}"
npx web-ext run --source-dir extension

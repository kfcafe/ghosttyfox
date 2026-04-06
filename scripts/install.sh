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

# Verify Python 3 is available
if ! command -v python3 &>/dev/null; then
  echo "Error: python3 is required but not found in PATH." >&2
  exit 1
fi

mkdir -p "${MANIFEST_DIR}"

# Bundle the extension
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

cat <<EOF

✓ Installed Ghosttyfox native host manifest:
  ${MANIFEST_PATH}

Next steps:
  1. Open about:debugging#/runtime/this-firefox in Firefox.
  2. Click "Load Temporary Add-on...".
  3. Select ${PROJECT_DIR}/extension/manifest.json.
EOF

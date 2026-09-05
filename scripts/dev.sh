#!/usr/bin/env bash
# Build, serve and reload. Options pass through to serve.py.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec python3 "$ROOT/scripts/serve.py" --watch "$@"

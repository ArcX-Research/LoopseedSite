#!/usr/bin/env bash
# Save an explicit Loopseed connection for this website; no model process is started.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec python3 "$ROOT/scripts/loopseed_connection.py" connect "$@"

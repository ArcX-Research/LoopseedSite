#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "▸ format"
cargo fmt --all -- --check

echo "▸ script syntax"
for script in scripts/*.sh; do
  bash -n "$script"
done
python3 -c 'import ast, pathlib; [ast.parse(path.read_text(), filename=str(path)) for path in pathlib.Path("scripts").glob("*.py")]'
if command -v shellcheck >/dev/null 2>&1; then
  shellcheck scripts/*.sh
fi

echo "▸ connection tests"
python3 -m unittest discover -s scripts -p '*_test.py'

echo "▸ clippy (record, native)"
cargo clippy -p loopseed-record --all-targets -- -D warnings

echo "▸ tests (record, native)"
cargo test -p loopseed-record

echo "▸ clippy (site, wasm32)"
cargo clippy -p loopseed-site --target wasm32-unknown-unknown -- -D warnings

echo "▸ check (site, wasm32)"
cargo check -p loopseed-site --target wasm32-unknown-unknown

echo "✓ all checks passed"

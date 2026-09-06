SHELL := /bin/bash

CARGO ?= cargo
PYTHON ?= python3
HOST ?= 127.0.0.1
PORT ?= 8790
# Optional command-line/environment override for the saved connection.
export LOOPSEED

.DEFAULT_GOAL := help

.PHONY: help fmt fmt-check lint test check build build-dev dev serve skin clean

help:
	@printf '%s\n' \
		'loopseed.io development commands:' \
		'  make dev          Build, serve, watch, and reload (http://127.0.0.1:8790/)' \
		'  make build        Build the release site in dist/' \
		'  make build-dev    Build the development site in dist/' \
		'  make serve        Serve the existing dist/ directory' \
		'  make check        Format, lint, test, and wasm checks' \
		'  make test         Run record and connection tests' \
		'  make skin         Export chart data from the connected Loopseed (read-only)' \
		'                    Connect first: scripts/connect-loopseed.sh /path/to/Loopseed' \
		'  make clean        Remove generated build output'

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

lint:
	$(CARGO) clippy -p loopseed-record --all-targets -- -D warnings
	$(CARGO) clippy -p loopseed-site --target wasm32-unknown-unknown -- -D warnings

test:
	$(PYTHON) -m unittest discover -s scripts -p '*_test.py'
	$(CARGO) test -p loopseed-record

check:
	./scripts/check.sh

build:
	./scripts/build.sh release

build-dev:
	./scripts/build.sh dev

dev:
	./scripts/dev.sh --host "$(HOST)" --port "$(PORT)"

serve:
	$(PYTHON) scripts/serve.py --host "$(HOST)" --port "$(PORT)" --no-build

skin:
	$(PYTHON) scripts/export_skin.py

clean:
	$(CARGO) clean
	rm -rf "$(CURDIR)/dist"

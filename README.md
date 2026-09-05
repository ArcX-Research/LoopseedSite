# Loopseed

The Loopseed research program, its measured results, and its promise: a static site compiled
from Rust to WebAssembly and rendered in the browser. Every number on the site is typed in
`crates/record`, quoted from a frozen ruling in the loopseed repository, and the statistics that
rule on those numbers are reimplemented there so a reader can recompute them.

## Run it locally

```bash
make dev                          # builds, serves at http://127.0.0.1:8790/, watches and live-reloads
```

Other commands:

```bash
make help                         # list the development commands
make build                        # optimised release build into ./dist
make build-dev                    # fast debug build into ./dist
make serve                        # serve ./dist without rebuilding
make test                         # the record tests: statistics reproduce the frozen values, data is consistent
make check                        # format, lint, tests and wasm checks
make skin LOOPSEED=../loopseed    # re-export the skin figure data, read-only, from a loopseed checkout
```

Requirements: Rust ≥ 1.85 with the `wasm32-unknown-unknown` target, the `wasm-bindgen` CLI at
the version pinned in `crates/site/Cargo.toml` and `Cargo.lock` (0.2.127), Python 3 for the dev
server. `wasm-opt` is used when present but not required.

## Layout

```
loopseed-site/
  crates/record        the record: experiments, laws, rulings, stages, measures, skin data, exact statistics (native tests)
  crates/site          the Leptos client-side application: pages and figures
  static/              index.html, styles.css, favicon.svg
  scripts/             build.sh, dev.sh, serve.py, check.sh, export_skin.py
  dist/                build output (generated)
```

## Editing the record

- A result lives in one place: `crates/record/src/experiments.rs`, `measures.rs`, `laws.rs`,
  `rulings.rs` or `stages.rs`. Each item names its source path in the loopseed repository.
- `make test` refuses a comparison whose frozen statistic does not reproduce from its counts.
- `REPOSITORY_URL` and `CONTACT_EMAIL` in `crates/record/src/lib.rs` are `None` until the study
  publishes its repository; the site then says evidence is available on request.

## Design

Light theme only, in the Dilate visual language: ink `#000020`, indigo `#4946ff`, warm
off-whites, hairlines; Geist for text, Adamina for the equation, DM Mono for numbers and paths.
No gradients, no dark mode, no tracking.

## Deploying

The build output is static (`dist/`). The app uses client-side routing, so the host must serve
`index.html` for unknown paths; `dist/404.html` is a copy of the shell for hosts that use that
convention. `amplify.yml` and `customHttp.yml` configure AWS Amplify Hosting.

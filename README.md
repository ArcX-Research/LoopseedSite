# Loopseed

Loopseed is a framework for systems that learn from interaction. Built around Dynamical
Synthesis, it connects prediction, feedback, memory and action in a repeating cycle.

The Dynamical Synthesis equation, `I = W(I) + you`, describes a system combining its evolving
state with external input. The framework is intended for any system whose state, observations
and actions can be defined and measured. Current experiments use language models.

The formal-reasoning lab takes a neurosymbolic approach: a neural model proposes calculations
in a typed domain-specific language, a deterministic compiler translates them, and a symbolic
checker evaluates them against an exact task generator. Independent AI review checks the full
reply against the task. These checks do not yet provide general formal proofs of correctness.

The Becoming track aims to test the framework in other systems, improve transfer across tasks,
and extend symbolic checking toward formal verification. This is an effort toward artificial
general intelligence (AGI); current findings establish narrower results in language-model tests.

This repository contains the research website, built in Rust and compiled to WebAssembly.
`crates/record` stores the reported measurements and their source references. The browser
recalculates paired comparisons from the original counts.

## Run it locally
```bash
make dev    # preview with live reload
```
Open http://127.0.0.1:8790/. Other commands:

```bash
make help                      # list commands
make build                     # release build
make build-dev                 # debug build
make serve                     # serve dist/
make test                      # verify recorded statistics and local connections
make check                     # format, lint, tests, Wasm
make skin                      # export chart data from the connected Loopseed
```

Requirements: Rust ≥ 1.85, the `wasm32-unknown-unknown` target, Python 3, and `wasm-bindgen`
CLI 0.2.127 (matching `crates/site/Cargo.toml` and `Cargo.lock`). `wasm-opt` is optional.

## Connect a Loopseed checkout

Loopseed and this website can live in separate folders at any depth. Register the
checkout explicitly before exporting chart data:

```bash
./scripts/connect-loopseed.sh "/absolute/path/to/Loopseed"
make skin
```

The helper validates the checkout and saves its absolute path in this website’s
ignored `.loopseedsite/config.json`. It does not change Loopseed or start a model.
The chart exporter opens `fish/sediment.db` in read-only mode and reads `SOUL.md`
from that checkout. Reconnect after moving Loopseed; previewing and building the
website use the committed chart data and do not require a connection.

For one export, pass a Make variable or environment variable without changing the
saved connection:

```bash
make skin LOOPSEED="/another/location/Loopseed"
LOOPSEED="/another/location/Loopseed" make skin
python3 scripts/export_skin.py --loopseed "/another/location/Loopseed"
```

Resolution order is the exporter’s `--loopseed` argument, `LOOPSEED`, then the
saved connection. An unset connection or invalid explicit path produces a setup
error. No sibling directory is assumed. `LOOPSEEDSITE_CONFIG` can select another
settings file, for example when testing. Inspect the selected path with
`python3 scripts/loopseed_connection.py path`.

This connection belongs to LoopseedSite. Meridian’s adapter and workspace settings
remain in its own `.meridian/` folder; connecting the website does not alter them.

## Layout

```
LoopseedSite/
  crates/record/   results, source references and statistics
  crates/site/     pages and figures (Leptos)
  static/         HTML, CSS and icons
  scripts/        build, preview, checks and data export
  dist/           generated site
```

## Editing the record

- Edit results in `crates/record/src/`. Keep original counts, source paths, hashes and decisions.
- Use plain language and no periods in headings. Define terms and separate findings from goals.
- Run `make test` to check that recorded statistics still reproduce from their counts.
- Set `REPOSITORY_URL` and `CONTACT_EMAIL` in `crates/record/src/lib.rs` when available.
  Until then, the site directs evidence requests to Dilate Technologies.

## Design

Light theme with Dilate colours: ink `#000020`, indigo `#4946ff` and warm off-whites.
Geist for text, Adamina for equations, DM Mono for numbers and paths. No tracking.

## Deploying

Deploy `dist/` to a static host. Serve `index.html` for unknown paths so browser routing works.
`dist/404.html` provides the same fallback. AWS Amplify settings are in `amplify.yml` and
`customHttp.yml`.

## License

MIT, © 2026 Dilate Technologies. See `LICENSE`. The license covers this site and its code;
the rulings it quotes remain the loopseed repository's record.

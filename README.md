# Loopseed

Loopseed researches whether verified experience can produce lasting improvements, transfer
to new situations and preserve earlier abilities. Fish, the current testbed, combines a
language model with prediction, memory, tools and separate adapters for prediction and replies.

Dynamical Synthesis is the project's organising idea for a system combining its activity
with external input. `I = W(I) + you` is design shorthand, not a complete learning algorithm
or a proven law of intelligence. The Method page defines the implemented state recurrence,
prediction measurements, memory rules and gradient-based adapter training separately.

The formal-reasoning lab takes a neurosymbolic approach: a neural model proposes calculations
in a typed domain-specific language, a deterministic compiler translates them, and a symbolic
checker evaluates them against an exact task generator. A separate AI reviewer with condition
labels withheld checks the full reply. This assessment is distinct from independent replication
and does not provide a general formal proof of correctness.

The strongest completed adapter result is 43 accepted answers out of 64 new parameter
instances, with gains within eleven learned task families. Both principal controls scored zero.
The result does not establish transfer beyond those families or long-term retention. The
Becoming track proposes broader applications and formal verification; AGI remains an ambition.

This repository contains the research website, built in Rust and compiled to WebAssembly.
`crates/record` stores the reported measurements and their source references. The browser
recalculates paired comparisons from the original counts.

The scientific content was reviewed on 11 September 2026, using experiment records through
7 September. The studies are dated snapshots, not a live status feed. The paper,
eight scientific figures, LaTeX sources and aggregate data are available from `/record#paper-h`.
The mathematics evidence package contains the full 1,024-cell comparison, 155 AI
reviews and selected training splits, with a standalone count check. Original
records for the other studies remain private and can be requested for review.

The paper has a plain HTML abstract and downloads at `/papers/dynamical-synthesis.html`.
This page provides Google Scholar citation metadata without JavaScript; the homepage
links to it in its HTML source. `/robots.txt` permits access and `/sitemap.xml` lists
the page. The files must be deployed to loopseed.io before they can be discovered.
Scholar indexing and manually adding a profile entry are separate steps; neither
is evidence of peer review. Publication-page checks run with `make test`.

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
- Distinguish observations, controlled comparisons, exploratory work and independent replication.
  New parameter instances within trained families are not new-family generalisation.
- Describe the exact instrument. Prediction loss, embedding discrepancy, word-based association,
  task correctness and retention measure different things. A descriptive band is not a confidence interval.
- Preserve frozen task-level statistics and explain their independence assumptions. Shared templates
  can correlate errors; the current bounds do not adjust for family clustering.
- Identify AI review as AI review and local protocol freezing as local protocol freezing. Hashes
  verify artifact identity, not truth, public preregistration or complete absence of leakage.
- Update dated study summaries only from saved outcomes. Training completion, validation-loss
  reduction and probe eligibility are not interchangeable with a measured behavioural gain.
- Run `make test` to check that recorded statistics still reproduce from their counts.
- Set `REPOSITORY_URL` and `CONTACT_EMAIL` in `crates/record/src/lib.rs` when available.
  Until then, the site directs evidence requests to Dilate Technologies.

The expanded record is `static/data/research-record.json`. `crates/record/src/research.rs`
reads this same downloadable JSON; figures under `static/figures/dynamical-synthesis/` are
generated from it in the paper workspace. The typed record tests preserve denominators and
distinguish completed pilots from successful allocation. Paper assets can be refreshed from
the Loopseed checkout using `docs/papers/dynamical-synthesis/prepare_delivery.py --site`
with this checkout's explicit path. That operation copies reviewed local files and does not deploy.

Featured livestreams are selected in `crates/record/src/livestreams.rs`; their source
metadata and selection rationale are in `docs/livestream-selection.json`. The home page uses
local thumbnail files and loads a YouTube player only after a play button is pressed. Each
card also links directly to the original recording, and the section links to the full playlist.
These historical recordings are not scored efficacy observations.

## Design

Light theme with Dilate colours: ink `#000020`, indigo `#4946ff` and warm off-whites.
Geist for text, Adamina for equations, DM Mono for numbers and paths. No site analytics.
The optional livestream players load third-party YouTube content when activated.

## Deploying

Deploy `dist/` to a static host. Serve `index.html` for unknown paths so browser routing works.
`dist/404.html` provides the same fallback. AWS Amplify settings are in `amplify.yml` and
`customHttp.yml`.

## License

MIT, © 2026 Dilate Technologies. See `LICENSE`. The license covers this site and its code;
the rulings it quotes remain the loopseed repository's record.

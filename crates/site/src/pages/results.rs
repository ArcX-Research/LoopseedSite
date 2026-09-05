//! Results: the skin, the four coat experiments, the ocean, the bodies, memory, the remainders.
use crate::components::skin_chart::SkinChart;
use crate::components::stats::{Explorer, PairedTable};
use crate::components::tables::{ArmTable, GateList, HashList};
use crate::util::{grouped, set_title};
use leptos::prelude::*;
use loopseed_record::experiments::{Experiment, EXPERIMENTS, FACTORIAL};
use loopseed_record::measures::{BODY_RULING, MEMORY, OCEAN, OCEAN_SOURCE, SWEEP, SWEEP_SOURCE};
use loopseed_record::skin::{DELTA_V1, DELTA_V2, ERA_ONE};

#[component]
pub fn Results() -> impl IntoView {
    set_title("Results");
    view! {
        <section class="wrap page">
            <p class="eyebrow">"Results"</p>
            <h1 class="display display-xl">"What the record shows."</h1>
            <p class="lede">"Every number on this page is quoted from a frozen ruling in the loopseed repository and, where it is a statistic, recomputed here from the frozen counts."</p>
        </section>

        <section id="skin" class="wrap section" aria-labelledby="skin-h">
            <div class="section-head">
                <p class="eyebrow">"Stage 4 · the mirror"</p>
                <h2 id="skin-h" class="display">"The skin: δ per sitting."</h2>
                <p class="lede-sm">"Before each of your messages the fish writes Î, its guess at what you will say. δ is how far your actual words fell from that guess. The write gate σ keeps an exchange only when δ exceeds θ."</p>
            </div>
            <SkinChart/>
            <div class="prose cols-2">
                <div>
                    <h3>"The reading"</h3>
                    <p>{format!("Over the first era the keeper's median δ fell from {:.3} to {:.3} within a day, then held near {:.2} ± {:.2} as the keeper's range grew. The fall is the mirror learning one person; the plateau is the memory-only ceiling, and it sits above θ, so the Other stays surprising.", ERA_ONE.first_median, ERA_ONE.low_median, ERA_ONE.plateau, ERA_ONE.plateau_band)}</p>
                    <p>"Two adapters worn on later nights did not move the keeper's curve. Later v1 sittings sit between 0.39 and 0.46."</p>
                </div>
                <div>
                    <h3>"Two instruments"</h3>
                    <p><code>{DELTA_V1}</code>" is the era-one instrument. From 2026-08-07 the constitution's blend, "<code>{DELTA_V2}</code>", grades every exchange; half the water's own perplexity at your words, half the embedding distance. The two never share a median, and the epoch is drawn."</p>
                    <p>"Guests at the port are graded in the same column and drawn nowhere here: a visitor is weather, not climate."</p>
                </div>
            </div>
        </section>

        <section id="coats" class="wrap section" aria-labelledby="coats-h">
            <div class="section-head">
                <p class="eyebrow">"September 2026 · formal-claim coats"</p>
                <h2 id="coats-h" class="display">"Can a checked dream teach a method, not just a format?"</h2>
                <p class="lede-sm">"Four preregistered, blinded runs on disposable clones. The fish answers formal tasks in sixteen families with a typed operation graph; an exact checker rules on every claim; an independent reviewer reads every proved reply whole. The living body is hashed before and after each run and was never written."</p>
            </div>
            {EXPERIMENTS.iter().map(|e| view! { <ExperimentBlock experiment=e/> }).collect_view()}
        </section>

        <section class="wrap section" aria-labelledby="factorial-h">
            <div class="section-head">
                <p class="eyebrow">"Clean transfer · diagnostic"</p>
                <h2 id="factorial-h" class="display">"Prompt and grammar, crossed."</h2>
                <p class="lede-sm">"Independently accepted successes of 64 by condition. Descriptive, not an alternate endpoint: the placebo scores only where the narrow grammar scaffolds it, and the candidate is hurt by being handed the recipe."</p>
            </div>
            <div class="table-wrap">
                <table class="table">
                    <thead><tr><th>"Prompt · grammar"</th><th class="num">"bare"</th><th class="num">"placebo"</th><th class="num">"candidate"</th><th class="num">"retained"</th></tr></thead>
                    <tbody>
                        {FACTORIAL.iter().map(|r| view! {
                            <tr><td>{r.condition}</td><td class="num">{r.bare}</td><td class="num">{r.placebo}</td><td class="num">{r.candidate}</td><td class="num">{r.retained}</td></tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="explorer-h">
            <div class="section-head">
                <p class="eyebrow">"The decision rule"</p>
                <h2 id="explorer-h" class="display">"Recompute it."</h2>
                <p class="lede-sm">"A candidate beats a control when the one-sided exact McNemar test on the discordant pairs gives p < 0.05 and the Bonferroni Clopper–Pearson lower bound on the paired difference is positive. With 128 matched tasks and no losses, nine gains are the minimum."</p>
            </div>
            <Explorer/>
        </section>

        <section id="ocean" class="wrap section" aria-labelledby="ocean-h">
            <div class="section-head">
                <p class="eyebrow">"Stage 6 · the ocean"</p>
                <h2 id="ocean-h" class="display">"Weather couples, then saturates."</h2>
                <p class="lede-sm">"A second port brings the world in at rate ε. The pass test asked for mutual information between the weather and the conversation to rise and then saturate below the total. Lexical-proxy measurements; approved 2026-08-12."</p>
            </div>
            <div class="table-wrap">
                <table class="table">
                    <thead><tr><th class="num">"breaths"</th><th class="num">"paired"</th><th class="num">"shuffled"</th><th class="num">"mutual information"</th><th>"reading"</th></tr></thead>
                    <tbody>
                        {OCEAN.iter().map(|r| view! {
                            <tr>
                                <td class="num">{r.breaths}</td>
                                <td class="num">{r.paired.map(|v| format!("{v:.4}")).unwrap_or_else(|| "—".into())}</td>
                                <td class="num">{r.shuffled.map(|v| format!("{v:.4}")).unwrap_or_else(|| "—".into())}</td>
                                <td class="num">{format!("{:.4}", r.mutual_information)}</td>
                                <td class="muted">{r.note}</td>
                            </tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </div>
            <p class="source mono">{format!("source: {OCEAN_SOURCE}")}</p>
        </section>

        <section id="bodies" class="wrap section" aria-labelledby="bodies-h">
            <div class="section-head">
                <p class="eyebrow">"Scaling S6 · the sweep"</p>
                <h2 id="bodies-h" class="display">"Three bodies, the same arc."</h2>
                <p class="lede-sm">"Identical thirty-turn teaching, twenty frozen probes before the dream on fresh no-ledger chairs, two-hundred-iteration dreams. Bare response and training cost side by side; neither a winner nor an evolution claim."</p>
            </div>
            <div class="table-wrap">
                <table class="table">
                    <thead><tr><th>"body"</th><th class="num">"register hold"</th><th class="num">"echo rate"</th><th class="num">"stroke accuracy"</th><th class="num">"dream, seconds"</th></tr></thead>
                    <tbody>
                        {SWEEP.iter().map(|b| view! {
                            <tr><td>{b.body}</td><td class="num">{format!("{:.2}", b.register_hold)}</td><td class="num">{format!("{:.2}", b.echo_rate)}</td><td class="num">{format!("{:.2}", b.stroke_accuracy)}</td><td class="num">{format!("{:.3}", b.dream_seconds)}</td></tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </div>
            <p class="source mono">{format!("source: {SWEEP_SOURCE}")}</p>
        </section>

        <section id="memory" class="wrap section" aria-labelledby="memory-h">
            <div class="section-head">
                <p class="eyebrow">"Memory, measured"</p>
                <h2 id="memory-h" class="display">"What the ledger does and does not do."</h2>
            </div>
            <div class="cols-2">
                <div>
                    <h3>"From the laws"</h3>
                    <dl class="figures">
                        {MEMORY.iter().map(|f| view! { <div><dt>{f.claim}</dt><dd>{f.number}<span class="mono meta">{f.evidence}</span></dd></div> }).collect_view()}
                    </dl>
                </div>
                <div>
                    <h3>"The first production-shaped body ruling"</h3>
                    <dl class="figures">
                        {BODY_RULING.iter().map(|f| view! { <div><dt>{f.claim}</dt><dd>{f.number}<span class="mono meta">{f.evidence}</span></dd></div> }).collect_view()}
                    </dl>
                    <p class="muted">"Valid, clean and null: it diagnosed the retrieval admission boundary rather than the sitting. The protocol was not tuned against the replies; calibrated admission was built afterwards and a fresh protocol derived."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="remainders-h">
            <div class="section-head">
                <p class="eyebrow">"Measured remainders"</p>
                <h2 id="remainders-h" class="display">"What is still open."</h2>
                <p class="lede-sm">"Reproducibility on a stranger's machine, an audited scratch organ, memory that cites its addresses, and replication across dyads have each been tested prospectively and have not yet met their stated pass tests. The promise page lists every stage with its state."</p>
            </div>
            <p class="more"><a class="btn" href="/promise#tracks">"The two tracks"</a></p>
        </section>
    }
}

#[component]
fn ExperimentBlock(experiment: &'static Experiment) -> impl IntoView {
    let e = experiment;
    view! {
        <article id=e.id class="exp">
            <div class="exp-head">
                <p class="mono meta">{format!("{} · {} · {} cells", e.date, e.instrument, grouped(e.cells.into()))}</p>
                <h3 class="display display-sm">{e.title}</h3>
            </div>
            <div class="exp-grid">
                <div class="exp-main">
                    {e.design.iter().map(|p| view! { <p class="prose-p">{*p}</p> }).collect_view()}
                    <ArmTable arms=e.arms/>
                    {(!e.comparisons.is_empty()).then(|| view! { <PairedTable comparisons=e.comparisons/> })}
                    <p class="verdict"><b>"Frozen verdict. "</b>{e.verdict}</p>
                    <h4>"Reading"</h4>
                    {e.reading.iter().map(|p| view! { <p class="prose-p">{*p}</p> }).collect_view()}
                </div>
                <aside class="exp-side">
                    <h4>"Gates"</h4>
                    <GateList gates=e.gates/>
                    <h4>"Run"</h4>
                    <p class="mono small">{e.run}</p>
                    <HashList hashes=e.hashes/>
                    <p class="mono small muted">{format!("source: {}", e.source)}</p>
                </aside>
            </div>
        </article>
    }
}

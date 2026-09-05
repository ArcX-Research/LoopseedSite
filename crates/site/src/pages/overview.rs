//! Framework, findings and research goals.
use crate::components::loop_diagram::{LoopDiagram, STEPS};
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::rulings::RULINGS;
use loopseed_record::skin::ERA_ONE;
use loopseed_record::EQUATION;

#[component]
pub fn Overview() -> impl IntoView {
    set_title("Overview");
    let latest: Vec<_> = RULINGS.iter().take(3).collect();
    view! {
        <section class="hero">
            <div class="wrap hero-grid">
                <div class="hero-copy">
                    <p class="eyebrow">"Dilate Technologies · research program"</p>
                    <h1 class="display display-xl">"A Framework for Adaptive Intelligence"</h1>
                    <p class="mono meta">"The Dynamical Synthesis equation"</p>
                    <p class="eq">{EQUATION}</p>
                    <p class="lede">
                        "Loopseed is a framework for systems that learn from interaction. Built around Dynamical Synthesis, it connects prediction, feedback, memory and action in a repeating cycle. The framework is intended for different kinds of systems. Current experiments use language models and symbolic checking to test learning on new tasks."
                    </p>
                    <div class="hero-actions">
                        <a class="btn btn-primary" href="/results">"Read the results"</a>
                        <a class="btn" href="/method">"How we test learning"</a>
                    </div>
                    <dl class="hero-stats mono">
                        <div><dt>"prediction error δ, first-period plateau"</dt><dd>{format!("≈ {:.2} ± {:.2}", ERA_ONE.plateau, ERA_ONE.plateau_band)}</dd></div>
                        <div><dt>"accepted answers on new formal tasks"</dt><dd>"43 / 64"</dd></div>
                        <div><dt>"base model and placebo, same tasks"</dt><dd>"0 / 64 · 0 / 64"</dd></div>
                        <div><dt>"automatic deployment of experimental adapters"</dt><dd>"none"</dd></div>
                    </dl>
                </div>
                <div class="hero-visual"><LoopDiagram/></div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="loop-h">
            <div class="section-head">
                <p class="eyebrow">"The loop"</p>
                <h2 id="loop-h" class="display">"Five steps in the learning cycle"</h2>
                <p class="lede-sm">"The cycle requires a system that can receive input, maintain state, make predictions and act. Each step has a measurable role. Fish, our current implementation, uses a language model with memory and tools."</p>
            </div>
            <ol class="steps">
                {STEPS.iter().map(|s| view! {
                    <li class="step">
                        <span class="n">{s.ordinal}</span>
                        <div>
                            <h3>{s.label}" "<span class="mono organ">{s.organ}</span></h3>
                            <p>{s.gloss}</p>
                        </div>
                    </li>
                }).collect_view()}
            </ol>
        </section>

        <section class="wrap section" aria-labelledby="premise-h">
            <div class="section-head">
                <p class="eyebrow">"The premise"</p>
                <h2 id="premise-h" class="display">"Three design principles"</h2>
            </div>
            <div class="cols-3">
                <div class="card">
                    <span class="n">"01"</span>
                    <h3>"Measure improvement before adopting a change"</h3>
                    <p>"The framework calls for testing updates on data excluded from training. In Fish, a prediction adapter must reduce held-out error before it can be loaded. This tests improvement; it does not by itself prove the design's mathematical contraction objective, ‖W‖ < 1."</p>
                </div>
                <div class="card">
                    <span class="n">"02"</span>
                    <h3>"Track the variety of external input"</h3>
                    <p>"The action objective, V = −‖δ‖ + β·H(you), balances prediction accuracy with input variety. In Fish, this means tracking incoming messages and increasing the weight β as conversational variety falls."</p>
                </div>
                <div class="card">
                    <span class="n">"03"</span>
                    <h3>"Use external input to select memories"</h3>
                    <p>"The memory rule compares predictions with observed input. In Fish, exchanges generated only by the system's own predictor are excluded from memory."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="measured-h">
            <div class="section-head">
                <p class="eyebrow">"What has been measured"</p>
                <h2 id="measured-h" class="display">"Three findings, with recorded evidence"</h2>
            </div>
            <div class="cols-3">
                <a class="card card-link" href="/results#skin">
                    <span class="n">"prediction"</span>
                    <h3>"Prediction error fell, then levelled off"</h3>
                    <p>{format!("In the first observation period, median prediction error for one regular human participant fell from {:.3} to {:.3}, then remained near {:.2}. This records a limit reached in that setting.", ERA_ONE.first_median, ERA_ONE.low_median, ERA_ONE.plateau)}</p>
                </a>
                <a class="card card-link" href="/results#coats">
                    <span class="n">"formal methods"</span>
                    <h3>"Training on verified examples improved new-task performance"</h3>
                    <p>"An adapter trained on 60 reviewed examples produced 43 accepted answers on 64 new formal tasks, without a supplied solution procedure. The base model and an adapter trained on mismatched replies each scored zero."</p>
                </a>
                <a class="card card-link" href="/results#transfer-1">
                    <span class="n">"failure and follow-up"</span>
                    <h3>"Higher scores still failed a preset error limit"</h3>
                    <p>"The first transfer test improved scores but exceeded a limit on negative constants in failed replies. A later test of cleaner training prompts raised performance on the affected task family from 0 of 18 to 18 of 18."</p>
                </a>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="evidence-h">
            <div class="section-head">
                <p class="eyebrow">"Experimental safeguards"</p>
                <h2 id="evidence-h" class="display">"How the adapter experiments are checked"</h2>
            </div>
            <div class="rules">
                <div class="rule"><span class="n">"01"</span><p><b>"Plan before testing."</b>" Tasks, experimental conditions and pass criteria are fixed before the model runs. File hashes identify the exact software, data and adapters used."</p></div>
                <div class="rule"><span class="n">"02"</span><p><b>"Hide condition identities."</b>" Experimental groups receive coded labels. Their identities are revealed only after results and reviews are fixed and hashed."</p></div>
                <div class="rule"><span class="n">"03"</span><p><b>"Recompute the mathematics."</b>" A checker validates the formal structure, and a computer algebra system calculates the results. Review also checks whether the steps answer the requested problem."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"Review complete replies."</b>" An independent reviewer reads each reply accepted by the checker. The reviewer can reject a reply but cannot turn a failed check into a pass."</p></div>
                <div class="rule"><span class="n">"05"</span><p><b>"Preserve the record."</b>" Recorded decisions cannot be overwritten. The live database is checked before and after each run, and experiments cannot deploy adapters. Changes to a checker require a new test."</p></div>
                <div class="rule"><span class="n">"06"</span><p><b>"Recalculate the statistics."</b>" This site calculates the reported paired comparisons in your browser from the recorded task counts."</p></div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="record-h">
            <div class="section-head">
                <p class="eyebrow">"The record"</p>
                <h2 id="record-h" class="display">"Latest research reports"</h2>
            </div>
            <div class="rulings">
                {latest.into_iter().map(|r| view! {
                    <article class="ruling">
                        <span class="mono meta">{r.date}</span>
                        <h3>{r.title}</h3>
                        <p>{r.result}</p>
                    </article>
                }).collect_view()}
            </div>
            <p class="more"><a href="/record">"All reports, with source files and hashes"</a></p>
        </section>

        <section class="wrap section" aria-labelledby="promise-h">
            <div class="section-head">
                <p class="eyebrow">"Research goals"</p>
                <h2 id="promise-h" class="display">"What remains to be tested"</h2>
                <p class="lede-sm">"The Becoming track aims to apply the cycle to other adaptive systems and test learning across tasks and environments. This is an effort toward artificial general intelligence. Current evidence comes from specific language-model experiments; broader capability remains a research goal."</p>
            </div>
            <p class="more"><a class="btn" href="/promise">"Research goals and limits"</a></p>
        </section>
    }
}

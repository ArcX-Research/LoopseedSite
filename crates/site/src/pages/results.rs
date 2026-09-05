//! Experimental results and their limits.
use crate::components::skin_chart::{GuestChart, SkinChart};
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
            <h1 class="display display-xl">"What the record shows"</h1>
            <p class="lede">"This page summarises the recorded experiments and their limits. Source files accompany each result. The paired comparison tables recalculate the reported statistics from the original counts in your browser."</p>
        </section>

        <section id="skin" class="wrap section" aria-labelledby="skin-h">
            <div class="section-head">
                <p class="eyebrow">"Stage 4 · prediction"</p>
                <h2 id="skin-h" class="display">"Prediction error across sessions"</h2>
                <p class="lede-sm">"Before each incoming message, Fish records a prediction, Î. The score δ measures the difference between that prediction and the actual message. The memory rule σ stores an exchange when δ exceeds the threshold θ."</p>
            </div>

            <h3 id="participant" class="figure-h">"Regular participant exchanges"</h3>
            <p class="lede-sm figure-lede">"One human participant across the whole period. Each dot is an exchange; each outlined marker is a session median, calculated separately for each version of the measure."</p>
            <SkinChart/>
            <div class="prose cols-2">
                <div>
                    <h3>"Observed change"</h3>
                    <p>{format!("In the first observation period, median δ for the regular human participant fell from {:.3} to {:.3} within a day, then remained near {:.2} ± {:.2} as the conversation became more varied. This is consistent with improved prediction of one participant, followed by a limit in that setting. The plateau remained above the memory threshold.", ERA_ONE.first_median, ERA_ONE.low_median, ERA_ONE.plateau, ERA_ONE.plateau_band)}</p>
                    <p>"Loading two later adapters did not visibly shift this curve. Later sessions measured with v1 had medians between 0.39 and 0.46."</p>
                </div>
                <div>
                    <h3>"A change in measurement"</h3>
                    <p><code>{DELTA_V1}</code>" is the original measure. E converts text into vectors, and cosine distance measures their difference. From 2026-08-07, "<code>{DELTA_V2}</code>" adds an equal contribution from normalised perplexity: how unexpected the message is to the model. The chart marks this change and calculates medians separately for each version."</p>
                    <p>"Guest exchanges are recorded but excluded from this chart, which follows the same regular participant."</p>
                </div>
            </div>

            <h3 id="guests" class="figure-h">"Guest exchanges"</h3>
            <p class="lede-sm figure-lede">"The same prediction error measure is used for AI teachers, nine automated teaching programs and external input. Each teaching program is shown as a separate series. Outlined markers show daily medians and dots show session medians. Bands span the middle 50% of scores for the AI teacher and external input groups. Guest results are reported separately and never pooled with the regular participant's results."</p>
            <GuestChart/>
            <div class="prose cols-2">
                <div>
                    <h3>"Observed pattern"</h3>
                    <p>"External input has the highest overall prediction error: these messages come from outside the conversation and are the least predictable. Its daily medians increase over the period. Daily medians for the teaching programs are close to the overall median for that group. Daily teacher medians vary most, from highly predictable exchanges to scores above the early medians for external input."</p>
                </div>
                <div>
                    <h3>"Comparing measurement versions"</h3>
                    <p>"The teachers' daily v1 medians are close to the regular participant's v1 plateau. Overall v2 scores for all three guest groups are higher than that plateau. The dashed v1 and solid v2 teacher lines use different measures, so their separation cannot establish a change in the teachers."</p>
                </div>
            </div>
        </section>

        <section id="coats" class="wrap section" aria-labelledby="coats-h">
            <div class="section-head">
                <p class="eyebrow">"September 2026 · adapter experiments"</p>
                <h2 id="coats-h" class="display">"Does training teach a method that works on new tasks?"</h2>
                <p class="lede-sm">"Four preregistered, blinded experiments used isolated copies of the system. Fish answered tasks from sixteen mathematical families as graphs of calculation steps. An exact checker evaluated each graph, then an independent reviewer assessed complete replies that passed. File hashes confirmed that the live database remained unchanged."</p>
                <p class="prose-p">"The candidate is the adapter being tested. The base model runs without it. The placebo is an adapter trained on replies assigned to the wrong tasks. The retained prediction adapter is an additional control, used experimentally to generate replies. Each experimental condition is also called an arm."</p>
                <p class="prose-p">"‘Names-only’ prompts give the problem and required result names without a solution procedure. ‘Node-specified’ prompts also provide the calculation steps. A strict grammar limits names and argument counts for each task; a loose grammar limits only the general output structure. A negative literal is a negative number written directly in the answer; its preset limit is specific to these tests."</p>
            </div>
            {EXPERIMENTS.iter().map(|e| view! { <ExperimentBlock experiment=e/> }).collect_view()}
        </section>

        <section class="wrap section" aria-labelledby="factorial-h">
            <div class="section-head">
                <p class="eyebrow">"Clean transfer · additional comparisons"</p>
                <h2 id="factorial-h" class="display">"How prompts and output rules affected scores"</h2>
                <p class="lede-sm">"Each score counts accepted answers out of 64 tasks. The placebo succeeded only with the strict grammar; the candidate scored lower when given the calculation steps. These comparisons help explain the result. The primary test remains the condition selected before the run."</p>
            </div>
            <div class="table-wrap">
                <table class="table">
                    <thead><tr><th>"Prompt · grammar"</th><th class="num">"base model"</th><th class="num">"placebo"</th><th class="num">"candidate"</th><th class="num">"prediction adapter"</th></tr></thead>
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
                <h2 id="explorer-h" class="display">"Recalculate the statistical test"</h2>
                <p class="lede-sm">"A gain means the candidate passed a task that the control failed. A loss means the reverse. The test compares these outcomes on the same tasks. The statistical criterion requires p < 0.05 and a positive lower confidence bound on the difference in success rates. With 128 matched tasks and no losses, at least nine gains are needed."</p>
                <p class="prose-p">"The calculation uses a one-sided exact McNemar test and a Bonferroni-adjusted Clopper–Pearson bound. Differences are shown as fractions: 0.10 means 10 percentage points. Passing this statistical test is one requirement; the experiment's other pass criteria still apply."</p>
            </div>
            <Explorer/>
        </section>

        <section id="ocean" class="wrap section" aria-labelledby="ocean-h">
            <div class="section-head">
                <p class="eyebrow">"Stage 6 · external input"</p>
                <h2 id="ocean-h" class="display">"External input became more related to the conversation, then levelled off"</h2>
                <p class="lede-sm">"A second input channel supplies outside material at rate ε. This test tracked its relationship with the conversation using word-based estimates of overlap and mutual information. The criterion required the estimate to rise and then level off below its maximum. The recorded result was approved on 2026-08-12."</p>
                <p class="prose-p">"These are approximate measures based on words. Later changes to input handling started a separate measurement period that cannot be compared directly with this one."</p>
            </div>
            <div class="table-wrap">
                <table class="table">
                    <thead><tr><th class="num">"input cycles"</th><th class="num">"paired overlap"</th><th class="num">"shuffled overlap"</th><th class="num">"estimated mutual information"</th><th>"Observation"</th></tr></thead>
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
                <p class="eyebrow">"Scaling S6 · model comparison"</p>
                <h2 id="bodies-h" class="display">"Three models tested with the same procedure"</h2>
                <p class="lede-sm">"Each model received 30 teaching turns, 20 fixed test questions and 200 training iterations. Testing took place before training, in fresh sessions with memory retrieval disabled. These measurements compare baseline responses and training time; they do not measure learning gains."</p>
            </div>
            <div class="table-wrap">
                <table class="table">
                    <thead><tr><th>"Model"</th><th class="num">"style consistency"</th><th class="num">"repetition rate"</th><th class="num">"task accuracy"</th><th class="num">"training time (s)"</th></tr></thead>
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
                <h2 id="memory-h" class="display">"How stored records affect answers"</h2>
            </div>
            <div class="cols-2">
                <div>
                    <h3>"Recorded memory tests"</h3>
                    <dl class="figures">
                        {MEMORY.iter().map(|f| view! { <div><dt>{f.claim}</dt><dd>{f.number}<span class="mono meta">{f.evidence}</span></dd></div> }).collect_view()}
                    </dl>
                </div>
                <div>
                    <h3>"An early test of the full system"</h3>
                    <dl class="figures">
                        {BODY_RULING.iter().map(|f| view! { <div><dt>{f.claim}</dt><dd>{f.number}<span class="mono meta">{f.evidence}</span></dd></div> }).collect_view()}
                    </dl>
                    <p class="muted">"The run completed without a measured gain or loss. No memories passed the retrieval threshold, even with retrieval enabled. A calibrated retrieval process was developed afterwards for a new protocol; the original result remains unchanged."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="remainders-h">
            <div class="section-head">
                <p class="eyebrow">"Unresolved questions"</p>
                <h2 id="remainders-h" class="display">"What is still open"</h2>
                <p class="lede-sm">"Independent setup on another machine, reliable use of a working area, memory answers with source references, and replication across model–supervisor pairs have not yet met their stated criteria. The research goals page lists each stage and its status."</p>
            </div>
            <p class="more"><a class="btn" href="/promise#tracks">"Research progress by stage"</a></p>
        </section>
    }
}

#[component]
fn ExperimentBlock(experiment: &'static Experiment) -> impl IntoView {
    let e = experiment;
    view! {
        <article id=e.id class="exp">
            <div class="exp-head">
                <p class="mono meta">{format!("{} · {} · {} test requests", e.date, e.instrument, grouped(e.cells.into()))}</p>
                <h3 class="display display-sm">{e.title}</h3>
            </div>
            <div class="exp-grid">
                <div class="exp-main">
                    {e.design.iter().map(|p| view! { <p class="prose-p">{*p}</p> }).collect_view()}
                    <ArmTable arms=e.arms/>
                    {(!e.comparisons.is_empty()).then(|| view! { <PairedTable comparisons=e.comparisons/> })}
                    <p class="verdict"><b>"Recorded decision. "</b>{e.verdict}</p>
                    <h4>"Interpretation and limits"</h4>
                    {e.reading.iter().map(|p| view! { <p class="prose-p">{*p}</p> }).collect_view()}
                </div>
                <aside class="exp-side">
                    <h4>"Pass criteria"</h4>
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

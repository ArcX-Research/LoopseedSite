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
            <p class="lede">"The strongest completed adapter comparison found 43 accepted answers out of 64 new parameter instances, versus zero for both the base model and a shuffled-reply control. Gains occurred within eleven learned task families. The studies below show the design, failed criteria and limits of that finding."</p>
            <p class="prose-p">"This is a dated research summary, reviewed on 7 September 2026. It separates controlled tests, a private continuation study and observational measurements. The source repository is private; "<a href="/record#where-h">"evidence is available on request"</a>". Recalculating the statistics checks the arithmetic, not the underlying observations."</p>
            <p class="more"><a href="#coats">"Controlled adapter tests"</a>" · "<a href="#current">"Current private study"</a>" · "<a href="#skin">"Prediction observations"</a>" · "<a href="#memory">"Memory tests"</a></p>
        </section>

        <section id="coats" class="wrap section" aria-labelledby="coats-h">
            <div class="section-head">
                <p class="eyebrow">"September 2026 · adapter experiments"</p>
                <h2 id="coats-h" class="display">"Do verified examples improve answers on new problem instances?"</h2>
                <p class="lede-sm">"Four experiments fixed their protocols locally before evaluation and used coded adapter conditions on isolated copies. Fish answered tasks from sixteen mathematical families as graphs of calculation steps. An exact checker evaluated each graph, then a separate AI reviewer assessed complete replies that passed, with condition identities withheld."</p>
                <p class="prose-p">"These are successive studies by the project team, not four independent replications. The protocols and source artifacts are held in the private repository; ‘preregistered’ in the original reports refers to those frozen local plans. Matching before-and-after hashes verify the checked database files, rather than every aspect of the live system."</p>
                <p class="prose-p">"The candidate is the adapter being tested. The base model runs without it. The placebo is an adapter trained on replies assigned to the wrong tasks. The retained prediction adapter is an additional control, used experimentally to generate replies. Each experimental condition is also called an arm."</p>
                <p class="prose-p">"‘Names-only’ prompts give the problem and required result names without a solution procedure. ‘Node-specified’ prompts also provide the calculation steps. A strict grammar limits names and argument counts for each task; a loose grammar limits the general output structure. Both constrain generation. A negative literal is a negative number written directly in the answer; the preset limit is a task-specific contamination check, not a general mathematical rule or safety measure."</p>
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
                <p class="lede-sm">"A gain means the candidate passed a task that the control failed. A loss means the reverse. The recorded one-sided exact McNemar test asks whether gains exceed losses among these paired outcomes. The preset criterion also requires a positive lower bound on the success-rate difference. With 128 matched tasks and no losses, at least nine gains meet both requirements."</p>
                <p class="prose-p">"The lower bound subtracts a Clopper–Pearson upper bound on the loss rate from a lower bound on the gain rate. Two 97.5% one-sided bounds provide a conservative 95% joint bound through Bonferroni. This adjustment covers those two bounds; it is not a correction for every comparison across the research programme. Differences are fractions: 0.10 means 10 percentage points."</p>
                <p class="prose-p">"These calculations assume independent task pairs drawn from a common evaluation distribution. The clean study has four related instances per family, which can correlate errors. Its frozen p-values and bounds do not adjust for family clustering, model-training variability or selection of task families. Read them as task-level calculations under those assumptions, alongside the family counts. Broader claims require new families and independent training and evaluation runs. "<a href="https://www.itl.nist.gov/div898/software/dataplot/refman1/auxillar/mcnemar.htm">"Statistical background: NIST"</a>"."</p>
            </div>
            <Explorer/>
        </section>

        <section id="current" class="wrap section" aria-labelledby="current-h">
            <div class="section-head">
                <p class="eyebrow">"Private continuation study · completed 7 September 2026"</p>
                <h2 id="current-h" class="display">"Lower validation loss did not pass the answer test"</h2>
                <p class="lede-sm">"A continuing private clone preserves its history and trains prediction and speaking adapters separately. Its first guided continuation study completed all 168 comparison replies. Both candidates failed their preset adoption criteria and remain unworn. The clone retains its previous configuration and all study artifacts."</p>
            </div>
            <div class="cols-3">
                <div class="card">
                    <h3>"Prediction continuation"</h3>
                    <p>"Training replayed 85 admitted pairs and added 11 more. Loss on three new held-out pairs rose from 1.967 to 2.201; loss on 16 historical held-out pairs rose from 1.977 to 2.676. The criterion required a decrease of at least 0.01 on the new set with no historical increase. The candidate failed this small continuation check."</p>
                </div>
                <div class="card">
                    <h3>"Spoken answers"</h3>
                    <p>"The study used 84 teacher-derived, exactly checked solutions: 63 for training and 21 for validation, grouped by shared computation. Validation loss fell from 1.919 to 0.035. Direct evaluation then compared 42 tasks at two seeds across retained and candidate conditions. It recorded 12 paired gains and 17 losses."</p>
                </div>
                <div class="card">
                    <h3>"Decide within the clone"</h3>
                    <p>"The speaking candidate met the minimum of four gains on new cases and answered every unavailable-information check correctly. Its 17 paired losses failed the zero-loss criterion, so it was not activated. Prediction and speech were judged separately; neither changed the main live system."</p>
                </div>
            </div>
            <h3 class="figure-h">"Paired task–seed outcomes"</h3>
            <div class="table-wrap">
                <table class="table">
                    <thead><tr><th>"Evaluation group"</th><th class="num">"pairs"</th><th class="num">"gains"</th><th class="num">"losses"</th><th class="num">"ties"</th></tr></thead>
                    <tbody>
                        <tr><td>"New parameter cases"</td><td class="num">"60"</td><td class="num">"9"</td><td class="num">"10"</td><td class="num">"41"</td></tr>
                        <tr><td>"Earlier-skill controls"</td><td class="num">"18"</td><td class="num">"3"</td><td class="num">"7"</td><td class="num">"8"</td></tr>
                        <tr><td>"Unavailable-information controls"</td><td class="num">"6"</td><td class="num">"0"</td><td class="num">"0"</td><td class="num">"6"</td></tr>
                    </tbody>
                </table>
            </div>
            <p class="prose-p">"These are 84 task–seed pairs from 42 tasks, with shared operation families and repeated seeds. The counts are descriptive, not independent replications or a new statistical efficacy claim. This result shows why an improvement in the training objective needs a separate behavioural test, including earlier abilities."</p>
            <p class="prose-p">"A separate controlled learning study is still qualifying eligible probes before its causal comparison. Screening and pilot replies measure whether the experiment can run as specified; they are not evidence of a learning benefit. Failed eligibility attempts remain in the record."</p>
            <p class="source mono">"result: fish/album/2026-09-07T0335-EAT-private-continuing-learning-result.md · plan: fish/album/2026-09-07T0048-EAT-private-continuing-learning.md"</p>
            <p class="source mono">"result.json SHA-256: e7154d70e19b88564b8248ac40c329f7d41591ff64a2044019d22000640e8ebc"</p>
        </section>

        <section id="skin" class="wrap section" aria-labelledby="skin-h">
            <div class="section-head">
                <p class="eyebrow">"Stage 4 · prediction"</p>
                <h2 id="skin-h" class="display">"Prediction error across sessions"</h2>
                <p class="lede-sm">"Fish records a next-message prediction, Î, before observing the incoming message. The score δ measures their discrepancy. It helps select exchanges for active memory, subject to admission rules. These logs describe ordinary interaction; they are not a randomised trial of training."</p>
            </div>

            <h3 id="participant" class="figure-h">"Regular participant exchanges"</h3>
            <p class="lede-sm figure-lede">"One human participant across the whole period. Each dot is an exchange; each outlined marker is a session median, calculated separately for each version of the measure."</p>
            <SkinChart/>
            <div class="prose cols-2">
                <div>
                    <h3>"Observed change"</h3>
                    <p>{format!("In the first observation period, session medians for one participant fell from {:.3} to {:.3} within a day, then stayed near {:.2}. The original report described the plateau as ± {:.2}; that is a descriptive band, not a standard error or confidence interval. The plateau was above the reference memory threshold.", ERA_ONE.first_median, ERA_ONE.low_median, ERA_ONE.plateau, ERA_ONE.plateau_band)}</p>
                    <p>"The report found no visible shift after two later adapter changes; later v1 session medians ranged from 0.39 to 0.46. Conversation content, context and system changes were not held constant. Neither the decline nor the later plateau isolates a training effect or establishes a permanent performance limit."</p>
                </div>
                <div>
                    <h3>"A change in measurement"</h3>
                    <p><code>{DELTA_V1}</code>" is the original measure. E converts text into vectors. From 2026-08-07, "<code>{DELTA_V2}</code>" adds an equal contribution from transformed token loss. Here ppl_norm = 1 − exp(−L), where L is mean token negative log probability from the prior prediction context. The score is dimensionless; 0.40 does not mean 40% incorrect answers."</p>
                    <p>"The chart separates measurement versions, including later fallback to v1 when token scoring is unavailable. A v1-to-v2 difference cannot establish a behavioural change. Guest exchanges are shown separately below."</p>
                </div>
            </div>

            <h3 id="guests" class="figure-h">"Guest exchanges"</h3>
            <p class="lede-sm figure-lede">"The same prediction error measure is used for AI teachers, nine automated teaching programs and external input. Each teaching program is shown as a separate series. Outlined markers show daily medians and dots show session medians. Bands span the middle 50% of scores for the AI teacher and external input groups. Guest results are reported separately and never pooled with the regular participant's results."</p>
            <GuestChart/>
            <div class="prose cols-2">
                <div>
                    <h3>"Observed pattern"</h3>
                    <p>"The stored export shows higher discrepancy for the external-input group and substantial variation across teachers and sessions. The sources differ in topic, format and timing. Their rankings describe this dataset; they do not measure the intelligence of a source or demonstrate why a group is harder to predict."</p>
                </div>
                <div>
                    <h3>"Comparing measurement versions"</h3>
                    <p>"Read each version separately. Group totals may include both versions and should not be compared with a single-version plateau. The dashed v1 and solid v2 lines use different instruments; neither their separation nor pooled group medians identify a learning effect."</p>
                </div>
            </div>
        </section>

        <section id="ocean" class="wrap section" aria-labelledby="ocean-h">
            <div class="section-head">
                <p class="eyebrow">"Stage 6 · external input"</p>
                <h2 id="ocean-h" class="display">"A word-based association score rose, then levelled off"</h2>
                <p class="lede-sm">"A second input channel supplies outside material at rate ε. This test tracked its relationship with the conversation using word-based estimates of overlap and mutual information. The criterion required the estimate to rise and then level off below its maximum. The recorded result was approved on 2026-08-12."</p>
                <p class="prose-p">"These are implementation-specific word-based estimates. Their scale is not a direct measure of semantic understanding, causal influence or general intelligence. This descriptive trajectory met the project's stage criterion. Later changes to input handling started a separate measurement period; direct before-and-after comparisons would confound those changes."</p>
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
                <p class="lede-sm">"Each model received 30 teaching turns, 20 fixed test questions and 200 training iterations. Testing took place before training, in fresh sessions with memory retrieval disabled. These measurements compare baseline responses and training time on the recorded setup. The small task set and single recorded timing per model do not establish learning gains or a general model ranking."</p>
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
                <p class="lede-sm">"These early local tests used small samples and, in some cases, repeated prompts. Counts refer to answer attempts, not independent people or replicated studies. Recall of stored text, correctness of a judgement and learning in model parameters are different outcomes."</p>
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

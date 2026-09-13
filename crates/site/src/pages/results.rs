//! Experimental results and their limits.
use crate::components::page_nav::PageNav;
use crate::components::research_figure::ResearchFigure;
use crate::components::research_results::{DecisiveResults, WorldResults};
use crate::components::skin_chart::{GuestChart, SkinChart};
use crate::components::stats::{Explorer, PairedTable};
use crate::components::tables::{ArmTable, GateList, HashList, ScrollTable};
use crate::util::{grouped, set_title};
use leptos::prelude::*;
use loopseed_record::experiments::{Experiment, EXPERIMENTS, FACTORIAL};
use loopseed_record::measures::{BODY_RULING, MEMORY, OCEAN, OCEAN_SOURCE, SWEEP, SWEEP_SOURCE};
use loopseed_record::skin::{DELTA_V1, DELTA_V2};

const SECTIONS: &[(&str, &str)] = &[
    ("#coats", "Training studies"),
    ("#harder", "Harder problems"),
    ("#explorer-h", "Statistical checks"),
    ("#current", "Further training"),
    ("#worlds", "Learning rules"),
    ("#decisive", "Experiment preparation"),
    ("#skin", "Prediction"),
    ("#ocean", "External input"),
    ("#bodies", "Model comparisons"),
    ("#memory", "Memory"),
    ("#remainders-h", "Open questions"),
];

#[component]
pub fn Results() -> impl IntoView {
    set_title("Results");
    view! {
        <div class="results-page research-page">
        <section class="page-intro page-intro-results">
            <div class="wrap">
                <p class="eyebrow">"Results"</p>
                <h1 class="display display-xl">"Learning, transfer and retention"</h1>
                <p class="lede">"These studies examine how further training and stored experience affect answers and prediction. Mathematics tests found improvements within trained problem types, while subsequent experiments showed limited transfer and losses on earlier tasks."</p>
                <p class="prose-p">"The findings cover experiments reported by 11 September 2026. Each study has its own tasks and scoring rules, so the results cannot be combined into one overall success rate. The "<a href="/record#paper-h">"paper and supporting data"</a>" provide the records behind these comparisons. Independent replication is still needed."</p>
            </div>
        </section>
        <div class="research-layout">
        <PageNav items=SECTIONS show_label=false sidebar=true/>
        <div class="research-sections">
        <section id="coats" class="wrap section" aria-labelledby="coats-h">
            <div class="section-head">
                <p class="eyebrow">"2–4 September 2026"</p>
                <h2 id="coats-h" class="display">"Mathematics training studies"</h2>
                <p class="lede-sm">"Four related studies tested how training examples and answer requirements affected mathematical performance. The sequence began with calculation steps supplied in the prompt, then tested answers without those steps and revised training prompts that contained correction text. Each evaluation used new numerical values within problem types represented in training."</p>
                <p class="prose-p">"Accepted answers passed calculation checks and a separate AI review, except in the focused place-value study, which used calculation checks alone. Two studies met all their criteria; the other two improved scores but failed additional checks set before testing."</p>
            </div>
            <details class="study-details">
                <summary>"Model versions, answer formats and review procedure"</summary>
                <div class="study-details-content">
                    <p class="prose-p">"Training changed additional model weights, called adapters, while keeping the base model’s original weights fixed. Response adapters were trained to answer questions; prediction adapters were trained to predict incoming messages. The candidate is the response adapter under evaluation. The comparisons used the base model without an adapter, an earlier prediction adapter and a shuffled-answer control trained on the same questions with answers taken from other questions."</p>
                    <p class="prose-p">"The model returned a structured description of calculation steps. Our software checked that structure, and the Wolfram kernel, run locally through WolframScript, evaluated the calculations. In the main comparisons, an AI reviewer then read complete answers that passed, with model identities hidden. Review could reject an answer but could not reverse a failed calculation check."</p>
                    <p class="prose-p">"Prompts either supplied the calculation steps or asked the model to produce them. The reports call these ‘node-specified’ and ‘names-only’. Both required structured JSON output. Task-specific output rules restricted names and the number of inputs to each operation; general output rules restricted the overall structure. These are called strict and loose grammars in the reports."</p>
                    <p class="prose-p">"The studies also counted negative numbers written directly in replies. Their limits were chosen to detect particular errors associated with the training material. Negative numbers are valid mathematics; this check was specific to these tasks."</p>
                    <p class="prose-p">"The team recorded tasks, settings and decision criteria locally before evaluation, without public preregistration. Experiments used isolated system copies and checked for changes to the live database. AI review was an internal check and could itself make errors. The reports use ‘family’ for problems that share a mathematical procedure."</p>
                </div>
            </details>
            <ResearchFigure name="adapter-transfer"
                alt="Training study: response adapter 43 of 64 accepted answers, base model 0, shuffled-answer control 0, earlier prediction adapter 5. Harder problems: response adapter 2 of 81, shuffled-answer control 2, prediction adapter 3."
                caption="Panel A tests new numerical values in 16 problem types represented in training. Panel B tests 27 harder problems, each answered three times per model version. Both the tasks and the required answer format changed between studies, so the score difference cannot be attributed to difficulty alone. The figure labels the tested response adapter ‘Candidate’."/>
            <nav class="study-index" aria-label="Mathematics training studies">
                {EXPERIMENTS.iter().enumerate().map(|(i, e)| view! {
                    <a href=format!("#{}", e.id)>
                        <span class="n">{format!("{:02}", i + 1)}</span>
                        <span class="study-index-title">{e.title}</span>
                        <span aria-hidden="true" class="study-index-arrow">"↗"</span>
                    </a>
                }).collect_view()}
            </nav>
            {EXPERIMENTS.iter().map(|e| view! { <ExperimentBlock experiment=e/> }).collect_view()}
        </section>

        <section class="wrap section" aria-labelledby="factorial-h">
            <div class="section-head">
                <p class="eyebrow">"4 September study · additional comparisons"</p>
                <h2 id="factorial-h" class="display">"Effects of prompts and output rules"</h2>
                <p class="lede-sm">"The 4 September study also tested the same problems under four combinations of prompts and output rules. Supplying calculation steps reduced the response adapter’s score, while the shuffled-answer control succeeded only with task-specific output rules. The main comparison, selected before testing, used no supplied steps and general output rules."</p>
                <p class="prose-p">"Each cell counts accepted answers out of 64. Differences from the training format and more replies reaching the length limit may explain the lower scores with supplied steps, but their separate effects were not tested."</p>
            </div>
            <ScrollTable label="Prompt and grammar comparisons">
                <table class="table">
                    <thead><tr><th>"Prompt and output rules"</th><th class="num">"Base model"</th><th class="num">"Shuffled-answer control"</th><th class="num">"Response adapter"</th><th class="num">"Earlier prediction adapter"</th></tr></thead>
                    <tbody>
                        {FACTORIAL.iter().map(|r| view! {
                            <tr><td>{r.condition}</td><td class="num">{r.bare}</td><td class="num">{r.placebo}</td><td class="num">{r.candidate}</td><td class="num">{r.retained}</td></tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </ScrollTable>
        </section>

        <section id="harder" class="wrap section" aria-labelledby="harder-h">
            <div class="section-head">
                <p class="eyebrow">"Harder mathematics problems · 5 September 2026"</p>
                <h2 id="harder-h" class="display">"Performance on harder problems"</h2>
                <p class="lede-sm">"The response adapter and shuffled-answer control each produced 2 accepted answers out of 81 on harder problems. The earlier prediction adapter produced 3. This study did not meet its criterion for improved performance."</p>
            </div>
            <p class="prose-p">"Each model version answered 27 mathematics problems at three random seeds. Five memory questions and four questions with unavailable answers were tested in the same way, giving 324 replies overall. Each version failed all 15 memory-test attempts, leaving no successful baseline for assessing retention."</p>
            <p class="prose-p">"Across the 27 problems, the response adapter scored higher than the shuffled-answer control on two, lower on two and tied on 23 (one-sided sign test, p = 0.6875). This result does not establish equivalent performance. Plain-text requirements and reply-length limits also contributed to failures."</p>
            <h3 class="figure-h">"Answer-format comparison"</h3>
            <p class="prose-p">"Eight selected problems were tested with two adapters and three answer formats, producing 48 replies. Structured answers described calculations for the evaluator to execute; direct answers required the model to provide the numerical results."</p>
            <ScrollTable label="Accepted answers in the answer-format study">
                <table class="table">
                    <thead><tr><th>"Answer format"</th><th class="num">"Response adapter"</th><th class="num">"Shuffled-answer control"</th></tr></thead>
                    <tbody>
                        <tr><td>"Structured calculations, with output rules"</td><td class="num">"4 / 8"</td><td class="num">"0 / 8"</td></tr>
                        <tr><td>"Structured calculations, without output rules"</td><td class="num">"4 / 8"</td><td class="num">"0 / 8"</td></tr>
                        <tr><td>"Direct numerical answers"</td><td class="num">"3 / 8"</td><td class="num">"2 / 8"</td></tr>
                    </tbody>
                </table>
            </ScrollTable>
            <p class="prose-p">"The problems were selected using earlier results, and one direct-answer task required less than its structured counterpart. The comparison therefore cannot isolate the effect of answer format. An earlier attempt at the harder study was excluded because the requested adapter was not active."</p>
            <p class="source">"Study identifiers: decisive-formal-2026-09-05T101400Z and formal-interface-2026-09-05T150825Z. "<a href="/record#paper-h">"Methods and supporting records"</a>"."</p>
        </section>

        <section class="wrap section" aria-labelledby="explorer-h">
            <div class="section-head">
                <p class="eyebrow">"Statistical methods"</p>
                <h2 id="explorer-h" class="display">"Paired performance comparisons"</h2>
                <p class="lede-sm">"Comparing the same tasks across two model versions shows where performance improved or declined. A gain means only the response adapter passed, a loss means only the control passed, and a tie means both passed or both failed. The calculator applies the statistical rule used in the mathematics studies."</p>
                <p class="prose-p">"That rule requires a one-sided exact McNemar p-value below 0.05 and a positive lower confidence bound on the difference in success rates. With 128 matched tasks and no losses, at least nine gains meet both requirements. Other study criteria still apply."</p>
                <p class="prose-p">"The calculation assumes independent task pairs. In the 4 September study, four tasks shared each problem type, so their errors may be related. The statistics do not account for that grouping or for variation between independently trained models."</p>
            </div>
            <details class="study-details">
                <summary>"How the p-value and confidence bound are calculated"</summary>
                <div class="study-details-content">
                    <p class="prose-p">"Under the null hypothesis, gains and losses are equally likely among tasks where the two models differ. The exact one-sided p-value is the probability of observing at least this many gains among those differing outcomes. It is not the probability that the null hypothesis is true. "<a href="https://www.itl.nist.gov/div898/software/dataplot/refman1/auxillar/mcnemar.htm">"NIST describes the McNemar test and its assumptions"</a>"."</p>
                    <p class="prose-p">"The lower bound subtracts a Clopper–Pearson upper bound on the loss rate from a lower bound on the gain rate. Two 97.5% one-sided bounds give a conservative 95% joint bound using the Bonferroni method. This adjustment covers those two bounds, not all comparisons across the programme. Differences are shown as fractions: 0.10 means 10 percentage points."</p>
                    <p class="prose-p">"Recalculating the statistics checks that the arithmetic follows from the recorded counts. It does not independently check the original answers or validate how they were scored."</p>
                </div>
            </details>
            <Explorer/>
        </section>

        <section id="current" class="wrap section" aria-labelledby="current-h">
            <div class="section-head">
                <p class="eyebrow">"Further training · 7 September 2026"</p>
                <h2 id="current-h" class="display">"Performance after further training"</h2>
                <p class="lede-sm">"Separate updates for prediction and answering questions were tested on an isolated system copy. Prediction error increased on new and older test examples. The response update lowered validation loss from 1.919 to 0.035, but direct tests of its answers found more losses than gains. Neither update met the criteria for adoption."</p>
            </div>
            <ResearchFigure name="continuation"
                alt="Prediction loss increased from 1.967 to 2.201 on new test examples and 1.977 to 2.676 on older test examples. The response update produced 12 improvements, 17 declines and 55 unchanged outcomes."
                caption="Panel A compares prediction loss before and after training; lower values indicate more accurate prediction. ‘Witness’ denotes examples excluded from training. Panel B compares answers from the response update with the previous version: gains are newly passed tasks, and losses are previously passed tasks that now fail."/>
            <h3 class="figure-h">"Answer accuracy and retention"</h3>
            <p class="prose-p">"Each of 42 tasks was answered twice by each model version, using matching random seeds. The 168 replies therefore form 84 comparisons. ‘Unchanged’ includes cases where both versions passed and cases where both failed."</p>
            <ScrollTable label="Answer outcomes for the updated and previous model versions">
                <table class="table">
                    <thead><tr><th>"Questions tested"</th><th class="num">"Comparisons"</th><th class="num">"Improved"</th><th class="num">"Declined"</th><th class="num">"Unchanged"</th></tr></thead>
                    <tbody>
                        <tr><td>"Problems with new numerical values"</td><td class="num">"60"</td><td class="num">"9"</td><td class="num">"10"</td><td class="num">"41"</td></tr>
                        <tr><td>"Previously demonstrated abilities"</td><td class="num">"18"</td><td class="num">"3"</td><td class="num">"7"</td><td class="num">"8"</td></tr>
                        <tr><td>"Questions with unavailable answers"</td><td class="num">"6"</td><td class="num">"0"</td><td class="num">"0"</td><td class="num">"6"</td></tr>
                    </tbody>
                    <tfoot><tr><th>"Total"</th><td class="num">"84"</td><td class="num">"12"</td><td class="num">"17"</td><td class="num">"55"</td></tr></tfoot>
                </table>
            </ScrollTable>
            <p class="prose-p">"The losses included seven comparisons involving earlier abilities. Both versions correctly acknowledged unavailable information in all six checks of that behaviour. Repeated questions and related problem types limit the independence of these observations."</p>
            <details class="study-details">
                <summary>"Training examples and adoption criteria"</summary>
                <div class="study-details-content">
                    <p class="prose-p">"Prediction training used 85 previously selected examples and 11 additional examples. Evaluation reserved three new examples and 16 older ones. Adoption required a loss reduction of at least 0.01 on the new set without an increase on the older set."</p>
                    <p class="prose-p">"Response training used 84 solutions supplied by an AI teacher and checked by exact calculation: 63 for training and 21 for validation, split by shared computation. Adoption required at least four gains on new cases without losing any previously successful answers. The isolated copy retained its previous configuration."</p>
                </div>
            </details>
            <details class="study-details">
                <summary>"Study records and result file"</summary>
                <div class="study-details-content">
                    <p class="source mono">"Report: fish/album/2026-09-07T0335-EAT-private-continuing-learning-result.md · plan: fish/album/2026-09-07T0048-EAT-private-continuing-learning.md"</p>
                    <p class="source mono">"Result file: data/fish/labs/private-learning/dreams/20260906T214327762030Z/result.json"</p>
                    <p class="source mono">"Result file SHA-256: e7154d70e19b88564b8248ac40c329f7d41591ff64a2044019d22000640e8ebc"</p>
                    <p class="prose-p"><a href="/record#paper-h">"Paper, data and supporting methods"</a></p>
                </div>
            </details>
        </section>

        <WorldResults/>
        <DecisiveResults/>

        <section id="skin" class="wrap section" aria-labelledby="skin-h">
            <div class="section-head">
                <p class="eyebrow">"Prediction during interaction · August 2026"</p>
                <h2 id="skin-h" class="display">"Prediction error during interaction"</h2>
                <p class="lede-sm">"Before each incoming message, the system recorded a prediction. The score δ measures how much that prediction differed from the observed message, with lower values indicating a closer match. Topics, context and system settings changed during collection, so the records do not isolate an effect of training."</p>
                <p class="prose-p">"Two versions of the score were used. They are plotted separately because a change in the measure can change the score even without a change in performance. The "<a href="/method#symbols">"symbol guide"</a>" introduces the notation used in the figures."</p>
            </div>

            <h3 id="participant" class="figure-h">"Interactions with one regular participant"</h3>
            <p class="prose-p">"The export contains 717 scored exchanges with one participant: 652 under version 1 and 65 under version 2, grouped into 15 and seven sessions respectively. The version-1 session median fell from 0.4795 on 2 August to 0.3694 on 3 August; subsequent medians ranged from 0.3912 to 0.5497. Dates are in UTC."</p>
            <SkinChart/>

            <details id="prediction-settings" class="study-details">
                <summary>"Score definitions and encoder settings"</summary>
                <div class="study-details-content">
                    <p class="prose-p">"Version 1 compares representations of predicted and observed text using cosine distance. Version 2, introduced on 7 August, gives equal weight to that distance and a measure of how unlikely the observed text was under the prediction model. Version 1 was also used later when the additional scoring was unavailable. Neither score is an answer-error rate."</p>
                    <p class="prose-p">"The recorded encoder was "<code>"bge-small-en-v1.5"</code>", with d = 384. In the formulas, E converts text to a vector of dimension d, Î is the recorded prediction and ‘you’ is the observed message."</p>
                    <p class="prose-p"><code>{DELTA_V1}</code></p>
                    <p class="prose-p"><code>{DELTA_V2}</code></p>
                    <p class="prose-p">"In version 2, ppl_norm = 1 − exp(−L). L is the mean negative log probability of the observed tokens, evaluated using the context available before the message arrived. Both scores are dimensionless."</p>
                </div>
            </details>

            <h3 id="guests" class="figure-h">"Interactions with other input sources"</h3>
            <p class="prose-p">"AI teachers, nine automated teaching programmes and a stream of external text supplied additional inputs. Prediction error was higher for the external-text group and varied across teachers and sessions. Because the sources supplied different topics and formats at different times, these scores cannot rank their learning effectiveness."</p>
            <GuestChart/>
        </section>

        <section id="ocean" class="wrap section" aria-labelledby="ocean-h">
            <div class="section-head">
                <p class="eyebrow">"External input · 12 August 2026"</p>
                <h2 id="ocean-h" class="display">"Word overlap between input and replies"</h2>
                <p class="lede-sm">"This study compared shared words in actual input–reply pairs with pairs formed by reassigning replies to other inputs. The resulting association estimate increased and then levelled off, meeting the study’s criterion. It describes a pattern in word overlap; it does not measure learning, understanding or causal influence."</p>
                <p class="prose-p">"The table reports mean word overlap and the association estimate in bits. A dash indicates an unreported value. Later changes to input processing began a separate measurement period."</p>
            </div>
            <ScrollTable label="External-input measurements">
                <table class="table">
                    <thead><tr><th class="num">"Input cycles"</th><th class="num">"Paired word overlap"</th><th class="num">"Shuffled word overlap"</th><th class="num">"Association estimate (bits)"</th><th>"Observation"</th></tr></thead>
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
            </ScrollTable>
            <details class="study-details">
                <summary>"How word overlap and association were measured"</summary>
                <div class="study-details-content">
                    <p class="prose-p">"Word overlap is Jaccard similarity: the number of distinct words shared by two texts divided by the number present in either. The software compares actual input–reply pairs with a reproducibly shuffled set of replies. It then groups the pooled overlap values into four bins using quartile boundaries."</p>
                    <p class="prose-p">"The reported statistic is a plug-in estimate of mutual information between the pairing label (actual or shuffled) and the overlap bin. It is measured in bits, with a maximum of one bit for two equally sized groups. The estimate depends on the observations and binning, and the reports provide no uncertainty interval for this trajectory."</p>
                    <p class="source mono">{format!("Report: {OCEAN_SOURCE} · calculation: fish/lab/ocean_mi.py")}</p>
                </div>
            </details>
        </section>

        <section id="bodies" class="wrap section" aria-labelledby="bodies-h">
            <div class="section-head">
                <p class="eyebrow">"Model comparison · 14 August 2026"</p>
                <h2 id="bodies-h" class="display">"Baseline performance and training time"</h2>
                <p class="lede-sm">"Three models answered the same 20 test questions before training, with accuracy ranging from 45% to 70%. Each received 30 teaching messages, and testing used fresh sessions with memory retrieval disabled. Training time was recorded separately for 200 iterations per model."</p>
                <p class="prose-p">"The accuracy and format scores describe this small set of questions. Runtime comes from one training run per model, so neither result establishes a broader model ranking."</p>
            </div>
            <ScrollTable label="Model baseline comparisons">
                <table class="table">
                    <thead><tr><th>"Model"</th><th class="num">"Format compliance"</th><th class="num">"Repeated replies"</th><th class="num">"Task accuracy"</th><th class="num">"Training time (seconds)"</th></tr></thead>
                    <tbody>
                        {SWEEP.iter().map(|b| view! {
                            <tr><td>{b.body}</td><td class="num">{format!("{:.2}", b.register_hold)}</td><td class="num">{format!("{:.2}", b.echo_rate)}</td><td class="num">{format!("{:.2}", b.stroke_accuracy)}</td><td class="num">{format!("{:.3}", b.dream_seconds)}</td></tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </ScrollTable>
            <details class="study-details">
                <summary>"Measures and model sizes"</summary>
                <div class="study-details-content">
                    <p class="prose-p">"The first three numeric columns are proportions of the 20 replies. Format compliance records whether a reply passed the output checks; repetition counts replies identical to an earlier test reply after whitespace is normalised; accuracy follows the study’s answer checker."</p>
                    <p class="prose-p">"B denotes billions of parameters. The mixture-of-experts model has 30 billion parameters in total, with about 3 billion active for each token."</p>
                </div>
            </details>
            <p class="source mono">{format!("source: {SWEEP_SOURCE}")}</p>
        </section>

        <section id="memory" class="wrap section" aria-labelledby="memory-h">
            <div class="section-head">
                <p class="eyebrow">"Memory studies"</p>
                <h2 id="memory-h" class="display">"Use of stored experience"</h2>
                <p class="lede-sm">"Exploratory tests examined whether supplied or retrieved records helped the system answer questions. Their outcomes concern access to information during a task, rather than learning retained through model training."</p>
                <p class="prose-p">"Some tests repeated prompts or changed the questions between evaluations, and several reports lack complete counts. The observations below retain those limits."</p>
            </div>
            <div class="cols-2">
                <div>
                    <h3>"Recorded memory tests"</h3>
                    <dl class="figures">
                        {MEMORY.iter().map(|f| view! { <div><dt>{f.claim}</dt><dd>{f.number}<span class="mono meta">{f.evidence}</span></dd></div> }).collect_view()}
                    </dl>
                </div>
                <div>
                    <h3>"Memory enabled and disabled"</h3>
                    <dl class="figures">
                        {BODY_RULING.iter().map(|f| view! { <div><dt>{f.claim}</dt><dd>{f.number}<span class="mono meta">{f.evidence}</span></dd></div> }).collect_view()}
                    </dl>
                    <p class="prose-p">"Because retrieval returned no records, this comparison left the effect of successful retrieval untested. The procedure was revised for a later experiment."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="remainders-h">
            <div class="section-head">
                <p class="eyebrow">"Unresolved questions"</p>
                <h2 id="remainders-h" class="display">"Questions for further study"</h2>
                <p class="lede-sm">"The next phase will test whether repeated training adds a lasting benefit beyond access to stored examples, and whether that benefit can coexist with retention of earlier abilities. Further work will examine unfamiliar problem types and repeat the comparisons with other models and participants."</p>
                <p class="prose-p">"Reliable use of working memory, accurate citation of stored sources and reproduction on another machine also remain open. The "<a href="/goals#tracks">"research goals"</a>" describe these tests and the separate questions of human control and oversight, which the results here do not resolve."</p>
            </div>
        </section>
        </div>
        </div>
        </div>
    }
}

#[component]
fn ExperimentBlock(experiment: &'static Experiment) -> impl IntoView {
    let e = experiment;
    let met = e.gates.iter().all(|g| g.passed);
    view! {
        <article id=e.id class="exp">
            <div class="exp-head">
                <div class="exp-meta">
                    <p class="mono meta">{format!("{} · {} replies across all conditions", e.date, grouped(e.cells.into()))}</p>
                    <span class=if met { "status-badge status-pass" } else { "status-badge status-fail" }>{if met { "Criteria met" } else { "Criteria not met" }}</span>
                </div>
                <h3 class="display display-sm">{e.title}</h3>
            </div>
            <div class="exp-main">
                <ArmTable arms=e.arms/>
                <p class="verdict">{e.verdict}</p>
                <h4>"Interpretation and limitations"</h4>
                {e.reading.iter().map(|p| view! { <p class="prose-p">{*p}</p> }).collect_view()}
                {(!e.comparisons.is_empty()).then(|| view! {
                    <details class="study-details">
                        <summary>"Paired counts and statistics"</summary>
                        <div class="study-details-content"><PairedTable comparisons=e.comparisons/></div>
                    </details>
                })}
                <details class="study-details">
                    <summary>"Study design, criteria and source records"</summary>
                    <div class="study-details-content">
                    {e.design.iter().map(|p| view! { <p class="prose-p">{*p}</p> }).collect_view()}
                    <h4>"Criteria set before testing"</h4>
                    <GateList gates=e.gates/>
                    <h4>"Study identifiers"</h4>
                    <p class="mono small muted">{e.instrument}</p>
                    <p class="mono small">{e.run}</p>
                    <HashList hashes=e.hashes/>
                    <p class="mono small muted">{format!("source: {}", e.source)}</p>
                    </div>
                </details>
            </div>
        </article>
    }
}

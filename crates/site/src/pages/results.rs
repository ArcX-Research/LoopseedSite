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
        <section class="page-intro page-intro-results">
            <div class="wrap">
                <p class="eyebrow">"Results"</p>
                <h1 class="display display-xl">"Results from learning experiments"</h1>
                <p class="lede">"One controlled mathematics study found 43 accepted answers out of 64 after training, compared with zero from each of two control models. The problems used new numbers in problem types included in training. Follow-up studies found limited success on harder problems and losses in some earlier abilities."</p>
                <p class="prose-p">"This page brings together the findings available on 11 September 2026. The studies used different tasks and scoring rules, so their scores cannot be combined into one overall success rate. The "<a href="/record#paper-h">"paper, summary data and analysis code"</a>" are available for review; detailed experiment records are available on request. Independent replication is still needed."</p>
            </div>
        </section>
        <PageNav items=SECTIONS/>

        <section id="coats" class="wrap section" aria-labelledby="coats-h">
            <div class="section-head">
                <p class="eyebrow">"Mathematics studies · September 2026"</p>
                <h2 id="coats-h" class="display">"Training improved answers within familiar problem types"</h2>
                <p class="lede-sm">"The response adapters improved mathematical answers in four related studies, although two failed additional criteria set before testing. The gains concerned new numerical examples within problem types represented in training. The studies below show how the training material and answer requirements affected those results."</p>
                <p class="prose-p">"The training updates are called adapters: small sets of additional weights trained while the base model’s original weights remain fixed. Response adapters were trained to answer questions; prediction adapters were trained to predict incoming messages."</p>
                <p class="prose-p">"In the main comparisons, accepted answers passed both calculation checks and AI review. The smaller place-value follow-up reports exact-check results, as noted in its methods. A problem type, called a ‘family’ in the reports, groups tasks that use the same mathematical procedure with different numbers. These studies test new examples from types already included in training."</p>
            </div>
            <details class="study-details">
                <summary>"Model versions, answer formats and review procedure"</summary>
                <div class="study-details-content">
                    <p class="prose-p">"The model returned a structured description of calculation steps. Our software checked that structure, and the Wolfram kernel, run locally through WolframScript, evaluated the calculations. In the main comparisons, a separate AI reviewer assessed complete answers that passed, without knowing which model version produced them."</p>
                    <p class="prose-p">"The candidate is the response adapter being tested. The base-model control uses no adapter. The shuffled-answer control uses an adapter trained with answers reassigned to different questions. An earlier prediction adapter provides another comparison; it was used here to generate answers."</p>
                    <p class="prose-p">"Prompts either supplied the calculation steps or asked the model to produce them. The reports call these ‘node-specified’ and ‘names-only’. Both required structured JSON output. Task-specific output rules restricted names and the number of inputs to each operation; general output rules restricted the overall structure. These are called strict and loose grammars in the reports."</p>
                    <p class="prose-p">"The studies also counted negative numbers written directly in replies. Their limits were chosen to detect particular errors associated with the training material. Negative numbers are valid mathematics; this check was specific to these tasks."</p>
                    <p class="prose-p">"The project team recorded the tasks, settings and decision criteria locally before evaluation. These plans were not publicly preregistered, and the four studies are successive experiments by the same team. Experiments used isolated copies and checked for changes to the live database file. The AI review is an additional check within the project; it is fallible and does not constitute independent replication."</p>
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
                <h2 id="factorial-h" class="display">"How prompts and output rules affected scores"</h2>
                <p class="lede-sm">"The same 64 problems were tested with four combinations of prompts and output rules. The response adapter scored 43 when it had to produce the calculation steps and 4 when the steps were supplied. The shuffled-answer control had accepted answers only under the task-specific output rules."</p>
                <p class="prose-p">"The main comparison was selected before testing: no supplied steps, with general output rules. Each cell counts accepted answers out of 64. Differences from the training format and more replies reaching the length limit are possible explanations for the lower scores with supplied steps; the study did not isolate their separate effects."</p>
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
                <h2 id="harder-h" class="display">"The harder test found no advantage over the control"</h2>
                <p class="lede-sm">"The response adapter and shuffled-answer control each produced 2 accepted answers out of 81 on harder problems. The earlier prediction adapter produced 3. This study did not meet its criterion for improved performance."</p>
            </div>
            <p class="prose-p">"Each model version answered 27 mathematics problems three times, using different random seeds to control generation randomness. Five memory questions and four questions whose answers were unavailable were tested in the same way, giving 324 replies overall. All three versions failed all 15 memory-test replies, so those questions could not establish whether earlier abilities were preserved."</p>
            <p class="prose-p">"Across the 27 problems, the response adapter outscored the shuffled-answer control on two, scored lower on two and tied on 23. The one-sided sign test gave p = 0.6875. Equal overall scores do not establish that the models perform equivalently. Plain-text requirements and reply-length limits contributed to failures, and both the problems and answer requirements differed from the preceding study."</p>
            <h3 class="figure-h">"A separate test of answer format"</h3>
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
            <p class="prose-p">"The problems were selected using earlier results, and one direct-answer task required less than its structured counterpart. These differences limit what the comparison can tell us about the effect of answer format. An earlier attempt at the harder study was invalid because the requested adapter was not active; its results remain excluded."</p>
            <p class="source">"Study identifiers: decisive-formal-2026-09-05T101400Z and formal-interface-2026-09-05T150825Z. "<a href="/record#paper-h">"Methods and supporting records"</a>"."</p>
        </section>

        <section class="wrap section" aria-labelledby="explorer-h">
            <div class="section-head">
                <p class="eyebrow">"Statistical methods"</p>
                <h2 id="explorer-h" class="display">"Compare gains and losses on the same tasks"</h2>
                <p class="lede-sm">"A gain is a task the response adapter passed and the control failed. A loss is a task the control passed and the response adapter failed. A tie means both passed or both failed. The calculator below reproduces the statistical rule used in the mathematics studies."</p>
                <p class="prose-p">"The rule requires a one-sided exact McNemar p-value below 0.05 and a positive lower confidence bound on the difference in success rates. With 128 matched tasks and no losses, at least nine gains meet both requirements. Passing this statistical rule is only one of the study’s success criteria."</p>
                <p class="prose-p">"The calculation assumes independent task pairs. In the 4 September study, four tasks shared each problem type, so their errors may be related. The reported statistics do not account for this grouping or variation across separately trained models. They describe this evaluation under the stated assumptions; stronger conclusions require new problem types and independent repetitions."</p>
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
                <h2 id="current-h" class="display">"Two training updates failed the study’s criteria"</h2>
                <p class="lede-sm">"An isolated copy of the system was used to train separate updates for prediction and answering questions. Prediction became less accurate on test examples. The response update improved some answers but caused more previously successful answers to fail. Neither update was adopted."</p>
            </div>
            <div class="cols-3">
                <div class="card">
                    <h3>"Predicting incoming messages"</h3>
                    <p>"Training used 85 previously selected examples and 11 additional examples. On three new test examples, prediction loss rose from 1.967 to 2.201; on 16 older test examples, it rose from 1.977 to 2.676. Lower loss means more accurate prediction. These test examples were excluded from training."</p>
                </div>
                <div class="card">
                    <h3>"Answering questions"</h3>
                    <p>"The study used 84 solutions supplied by an AI teacher and checked by exact calculation: 63 for training and 21 for validation, split by shared computation. Validation loss, an error measure on the reserved examples, fell from 1.919 to 0.035. A separate evaluation of generated answers found 12 improvements, 17 declines and 55 unchanged outcomes."</p>
                </div>
                <div class="card">
                    <h3>"Why the updates were not adopted"</h3>
                    <p>"The prediction update had to reduce loss by at least 0.01 on the new test set without increasing it on the older set. The response update had to improve at least four new cases without losing any previously successful answers. Both failed their criteria. The isolated copy kept its previous configuration."</p>
                </div>
            </div>
            <ResearchFigure name="continuation"
                alt="Prediction loss increased from 1.967 to 2.201 on new test examples and 1.977 to 2.676 on older test examples. The response update produced 12 improvements, 17 declines and 55 unchanged outcomes."
                caption="Panel A compares prediction loss before and after training. ‘Witness’ denotes examples excluded from training. Panel B compares answers from the response update with the previous version. A gain means only the update passed; a loss means only the previous version passed. Lower validation loss during training did not ensure better performance on these tasks."/>
            <h3 class="figure-h">"Answer comparisons, including earlier abilities"</h3>
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
            <p class="prose-p">"Seven declines involved earlier abilities. Both versions correctly acknowledged unavailable information in all six of those checks. Repeated questions and related problem types mean these 84 comparisons are not independent replications. They show why generated answers and earlier abilities need direct testing even when validation loss falls."</p>
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
                <h2 id="skin-h" class="display">"How prediction error changed across sessions"</h2>
                <p class="lede-sm">"Prediction error decreased between early sessions and varied across later ones. Topics, context and system settings changed during the observation period, so these records do not isolate an effect of training."</p>
                <p class="prose-p">"Before each incoming message, the system recorded a prediction. The score δ measures the difference between the prediction and observation. Two versions were used and are shown in separate panels; lower values mean a closer match under that version’s measure. The dashed line marked θₘ is the reference threshold for selecting memories. These are the same figures shown in the paper. See the "<a href="/method#symbols">"symbol guide"</a>" for the notation."</p>
                <p id="prediction-settings" class="prose-p">"The recorded text encoder for these interaction measurements is "<code>"bge-small-en-v1.5"</code>", with d = 384. The same setting is reported in Section 5.8 of the paper."</p>
            </div>

            <h3 id="participant" class="figure-h">"Interactions with one regular participant"</h3>
            <p class="lede-sm figure-lede">"All scores in this chart come from the same human participant. Dots show individual exchanges; outlined markers show session medians. Medians are calculated separately for each measurement version."</p>
            <SkinChart/>
            <div class="prose cols-2">
                <div>
                    <h3>"What changed"</h3>
                    <p>"The first session median was 0.4795 on 2 August and a later median was 0.3694 on 3 August, using UTC dates. Subsequent version-1 session medians ranged from 0.3912 to 0.5497. All sessions are included in the plot, including those with few exchanges."</p>
                    <p>"The export contains 717 scored exchanges: 652 under version 1 and 65 under version 2, grouped into 15 and seven sessions respectively. A gap of at least 30 minutes starts a new session within each measurement version. These are repeated observations from one participant, not independent participants or a controlled training comparison."</p>
                </div>
                <div>
                    <h3>"Why the two score versions differ"</h3>
                    <p>"Version 1 compares numerical representations of the predicted and observed text using cosine distance. Version 2, introduced on 7 August, combines that distance with a measure of how unlikely the observed text was under the prediction model. Each contributes half of the combined score."</p>
                    <p>"The original measure was also used later when the additional scoring was unavailable. A change between versions can therefore reflect a change in measurement. Neither score is a percentage of wrong answers: a value of 0.40 does not mean 40% incorrect."</p>
                </div>
            </div>

            <details class="study-details">
                <summary>"Prediction-error formulas and definitions"</summary>
                <div class="study-details-content">
                    <p class="prose-p">"In these formulas, E converts text to a vector of dimension d, Î is the recorded prediction and ‘you’ is the observed message. Cosine distance compares the directions of the two vectors."</p>
                    <p class="prose-p"><code>{DELTA_V1}</code></p>
                    <p class="prose-p"><code>{DELTA_V2}</code></p>
                    <p class="prose-p">"In version 2, ppl_norm = 1 − exp(−L). L is the mean negative log probability of the observed tokens, evaluated using the context available before the message arrived. Both scores are dimensionless."</p>
                </div>
            </details>

            <h3 id="guests" class="figure-h">"Interactions with other input sources"</h3>
            <p class="lede-sm figure-lede">"These scores come from AI teachers, nine automated teaching programmes and a separate stream of external text. The figures below separate the source groups and measurement versions; each teaching programme has its own panel. These records are separate from the regular participant’s results."</p>
            <GuestChart/>
            <div class="prose cols-2">
                <div>
                    <h3>"What the differences mean"</h3>
                    <p>"Prediction error was higher for the external-text group, with substantial variation across teachers and sessions. The sources supplied different topics and formats at different times. These observations cannot establish why a source was harder to predict, and they do not rank the sources’ intelligence."</p>
                </div>
                <div>
                    <h3>"Compare like measurements"</h3>
                    <p>"Every panel identifies its score version. Daily medians and quartiles are calculated separately for each version, and no line crosses a version boundary or a missing observation date. Differences between groups or score versions do not establish a learning effect."</p>
                </div>
            </div>
        </section>

        <section id="ocean" class="wrap section" aria-labelledby="ocean-h">
            <div class="section-head">
                <p class="eyebrow">"External input · 12 August 2026"</p>
                <h2 id="ocean-h" class="display">"Word-based association increased, then levelled off"</h2>
                <p class="lede-sm">"The word-based association estimate rose from 0.0244 bits after eight input cycles to 0.0762 after 63, then fell slightly to 0.0712 after 128. It compared overlap between external text and the model’s replies with overlap after the replies were shuffled."</p>
                <p class="prose-p">"This pattern met the project’s criterion that the estimate should rise and then level off below its maximum of one bit. The statistic describes how word overlap distinguishes actual from shuffled pairings. It does not measure how much the system learned, or establish understanding or causal influence. Later changes to input processing began a separate measurement period."</p>
                <p class="prose-p">"The overlap columns report mean scores for actual and shuffled input–reply pairs. A dash means no value is reported for that entry."</p>
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
                <h2 id="bodies-h" class="display">"Baseline answers and training time for three models"</h2>
                <p class="lede-sm">"The three models answered between 45% and 70% of the 20 test questions correctly. Testing preceded training, so these scores describe baseline performance. Training time was measured separately."</p>
                <p class="prose-p">"Each model received the same 30 teaching messages, 20 fixed test questions and 200 training iterations. The questions were answered in fresh sessions with memory retrieval disabled."</p>
                <p class="prose-p">"The first three columns are proportions: 1.00 means all replies, and 0.05 means one of 20. Format compliance records whether replies passed the study’s output checks. Repetition counts replies identical to an earlier test reply after whitespace is normalised. Task accuracy follows the study’s answer checker. Training time is one recorded run per model on the tested setup."</p>
                <p class="prose-p">"B denotes billions of model parameters. The mixture-of-experts model has 30 billion parameters in total, with about 3 billion active for each token. This small comparison cannot establish a general ranking of models."</p>
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
            <p class="source mono">{format!("source: {SWEEP_SOURCE}")}</p>
        </section>

        <section id="memory" class="wrap section" aria-labelledby="memory-h">
            <div class="section-head">
                <p class="eyebrow">"Memory studies"</p>
                <h2 id="memory-h" class="display">"How stored records affect answers"</h2>
                <p class="lede-sm">"Small exploratory tests found that stored records could improve recall while worsening some judgements. These observations concern the use of supplied or retrieved information. They do not by themselves show that model training produced lasting learning."</p>
                <p class="prose-p">"The counts describe answer attempts. Some tests repeated prompts or used different questions before and after an intervention. Those designs limit comparisons, and the original reports do not always provide complete denominators."</p>
            </div>
            <div class="cols-2">
                <div>
                    <h3>"Recorded memory tests"</h3>
                    <dl class="figures">
                        {MEMORY.iter().map(|f| view! { <div><dt>{f.claim}</dt><dd>{f.number}<span class="mono meta">{f.evidence}</span></dd></div> }).collect_view()}
                    </dl>
                </div>
                <div>
                    <h3>"A comparison with memory enabled and disabled"</h3>
                    <dl class="figures">
                        {BODY_RULING.iter().map(|f| view! { <div><dt>{f.claim}</dt><dd>{f.number}<span class="mono meta">{f.evidence}</span></dd></div> }).collect_view()}
                    </dl>
                    <p class="muted">"No records were retrieved in this test, including when memory was enabled. The run therefore left the effect of successful retrieval untested. The retrieval procedure was later revised for a separate experiment; the original outcome remains in the record."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="remainders-h">
            <div class="section-head">
                <p class="eyebrow">"Unresolved questions"</p>
                <h2 id="remainders-h" class="display">"What the next studies need to establish"</h2>
                <p class="lede-sm">"The central questions are whether improvements persist, extend to unfamiliar problem types and preserve earlier abilities. The programme also needs independent replication. The experiments on this page do not establish general intelligence or reliable alignment with human goals."</p>
                <p class="prose-p">"Other open tests concern installation on another machine, reliable use of an intermediate working area, answers that cite the correct memory sources, and repeated studies with different models and human participants. The research goals page records their criteria and progress."</p>
            </div>
            <p class="more"><a class="btn" href="/goals#tracks">"Research goals and progress"</a></p>
        </section>
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
            <div class="exp-grid">
                <div class="exp-main">
                    <details class="study-details">
                        <summary>"Study design and training setup"</summary>
                        <div class="study-details-content">
                            {e.design.iter().map(|p| view! { <p class="prose-p">{*p}</p> }).collect_view()}
                        </div>
                    </details>
                    <ArmTable arms=e.arms/>
                    {(!e.comparisons.is_empty()).then(|| view! { <PairedTable comparisons=e.comparisons/> })}
                    <p class="verdict"><b>"Study decision. "</b>{e.verdict}</p>
                    <h4>"What this result supports"</h4>
                    {e.reading.iter().map(|p| view! { <p class="prose-p">{*p}</p> }).collect_view()}
                </div>
                <aside class="exp-side">
                    <h4>"Criteria set before testing"</h4>
                    <GateList gates=e.gates/>
                    <h4>"Study identifiers"</h4>
                    <p class="mono small muted">{e.instrument}</p>
                    <p class="mono small">{e.run}</p>
                    <HashList hashes=e.hashes/>
                    <p class="mono small muted">{format!("source: {}", e.source)}</p>
                </aside>
            </div>
        </article>
    }
}

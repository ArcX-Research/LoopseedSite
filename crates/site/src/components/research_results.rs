use super::{research_figure::ResearchFigure, tables::ScrollTable};
use leptos::prelude::*;
use loopseed_record::research::research_record;

#[component]
pub fn WorldResults() -> impl IntoView {
    let record = research_record();
    view! {
        <section id="worlds" class="wrap section" aria-labelledby="worlds-h">
            <div class="section-head">
                <p class="eyebrow">"Learning rules from observations · 5–6 September 2026"</p>
                <h2 id="worlds-h" class="display">"Identification and reuse of rules"</h2>
                <p class="lede-sm">"These studies tested whether the model could infer a hidden rule from observations and use it to predict the outcome of new action sequences. Rule identification remained unreliable, although supplying a correct rule sometimes improved later answers."</p>
                <p class="prose-p">"Model weights stayed fixed, with saved rule descriptions supplied in the prompts. The results therefore concern finding and using rules in context; they do not establish retention through training."</p>
                <p class="prose-p">"The Meridian engine generated and checked the simulated worlds "<a href="/papers/dynamical-synthesis.html" rel="external">"(Dilate Technologies, 2026)"</a>"."</p>
            </div>
            <ResearchFigure name="world-learning"
                alt="Separate studies of rule use and discovery: 9 of 12 correct answers with a supplied rule in one generation setting; 2 of 6 hidden rules identified correctly; an earlier eight-environment test produced 0 of 16 correct answers using its own records. Full comparisons follow."
                caption="Panel A tests supplied rules, panel B tests later questions after rule identification, and panel C reports the eight-environment study. Each panel shows a separate experiment. Retained denotes the earlier prediction adapter; candidate denotes the response adapter. Thinking on allows additional generation before the final answer."/>

            <h3 class="figure-h">"Use of supplied rules"</h3>
            <p class="prose-p">{format!("Two adapters answered 12 questions with thinking mode enabled and disabled. Including 12 repeated requests, the study recorded {} replies. Enabling thinking mode improved accuracy for both adapters but required substantially more generated text. The earlier prediction adapter met the criterion for proceeding with 9 correct answers.", record.rule_use.calls)}</p>
            <p class="prose-p">"All repeated outputs matched their originals. Token totals include all text generated across the 12 questions in each setting."</p>
            <ScrollTable label="Supplied-rule use and generation counts">
                <table class="table">
                    <thead><tr><th>"Model and generation setting"</th><th class="num">"Correct answers / questions"</th><th class="num">"Total generated tokens"</th></tr></thead>
                    <tbody>
                        {[ ("retained-off", "Earlier prediction adapter · thinking off"), ("retained-on", "Earlier prediction adapter · thinking on"), ("candidate-off", "Response adapter · thinking off"), ("candidate-on", "Response adapter · thinking on") ].into_iter().map(|(key, label)| {
                            let row = &record.rule_use.conditions[key];
                            view! { <tr><td>{label}</td><td class="num">{format!("{} / {}", row.exact, row.n)}</td><td class="num">{row.total_generated_tokens}</td></tr> }
                        }).collect_view()}
                    </tbody>
                </table>
            </ScrollTable>

            <h3 class="figure-h">"Rule identification from observations"</h3>
            <p class="prose-p">{format!("A separate study collected {} replies across {} environments. The model identified {} rules correctly; its other four proposals fell outside the allowed rule set. Using its own descriptions, it answered 3 of 12 later questions correctly and failed the study’s criteria for finding and reusing the rules.", record.acquisition.calls, record.acquisition.worlds, record.acquisition.rules_correct)}</p>
            <p class="prose-p">"All three correct answers came from the four questions in environments where the rule was identified. The totals include the unsuccessful attempts and replies that reached the length limit. Each table row tests one question in each of six environments."</p>
            <ScrollTable label="Acquisition and query conditions, including unsuccessful acquisitions">
                <table class="table">
                    <thead><tr><th>"Information supplied with the question"</th><th class="num">"Correct answers / questions"</th><th class="num">"Replies reaching length limit"</th></tr></thead>
                    <tbody>
                        {[ ("own-p1", "Model’s rule · first question"), ("own-p2", "Model’s rule · second question"), ("supplied-p1", "Correct rule · first question"), ("supplied-p2", "Correct rule · second question"), ("raw-p1", "Original observations · first question"), ("none-p1", "No record · first question"), ("foreign-p1", "Unrelated environment’s record · first question") ].into_iter().map(|(key, label)| {
                            let row = &record.acquisition.conditions[key];
                            view! { <tr><td>{label}</td><td class="num">{format!("{} / {}", row.exact, row.n)}</td><td class="num">{row.capped}</td></tr> }
                        }).collect_view()}
                    </tbody>
                </table>
            </ScrollTable>
            <p class="prose-p">"The model appropriately acknowledged uncertainty in 5 of 6 cases without records and all 6 cases with unrelated records. These outcomes were assessed separately from correct predictions of the environment’s behaviour."</p>

            <h3 class="figure-h">"Eight-environment study"</h3>
            <p class="prose-p">{format!("An earlier study collected {} replies across {} environments, with {} correct initial rules and {} after feedback and revision. Later answers scored {}/{} with the model’s records, {}/{} with the original observations and {}/{} without records. Seven failed attempts left no usable rule for the later questions. The planned delayed test was not run.", record.worlds.calls, record.worlds.worlds, record.worlds.initial_rules_correct, record.worlds.repaired_rules_correct, record.worlds.own, record.worlds.queries_per_condition, record.worlds.raw, record.worlds.queries_per_condition, record.worlds.none, record.worlds.queries_per_condition)}</p>
            <h3 class="figure-h">"Feedback and rule correction"</h3>
            <p class="prose-p">"A later study collected 70 replies across six environments. Success on the rule and both later questions occurred in 1 of 6 cases with checked feedback, 0 of 6 with a general request to check again, and 2 of 6 with a supplied correct rule. None of four initially incorrect or invalid rules was corrected."</p>
            <p class="prose-p">"Two replies reached the length limit. A software conflict during final record verification prevented the planned study decision, so these counts describe saved observations from an incomplete evaluation. One reply from an earlier attempt was recorded separately and excluded from the 70 replies."</p>
            <p class="source">"Studies: decisive-worlds, decisive-ruleuse, decisive-acquisition and decisive-repair-context. "<a href="/data/research-record.json" download="research-record.json">"Download counts and source references"</a>" or read the "<a href="/record#paper-h">"paper and methods"</a>"."</p>
        </section>
    }
}

#[component]
pub fn DecisiveResults() -> impl IntoView {
    let record = research_record();
    view! {
        <section id="decisive" class="wrap section" aria-labelledby="decisive-h">
            <div class="section-head">
                <p class="eyebrow">"Preparatory experiments"</p>
                <h2 id="decisive-h" class="display">"Preparation for a controlled learning comparison"</h2>
                <p class="lede-sm">"The planned experiment required questions with suitable difficulty and reliable scoring. Preliminary runs tested whether enough questions met those requirements. Two completed collection but lacked enough qualifying questions; the others ended before collection was complete. None reached the main comparison of learning outcomes."</p>
            </div>
            <p class="prose-p">"The checker labelled each reply SOUND if its answer and derivation passed, FAIL if an answer requirement failed or the reply was unfinished, and INVALID if it violated output or information-leak rules. These criteria differ from the accepted-answer checks in the mathematics studies."</p>
            <ResearchFigure name="decisive-pilots"
                alt="Saved preliminary measurements out of 336 planned per attempt: version 11, 15; version 12, 130; versions 13 and 14, 336 each; version 15, 175 before pausing. No attempt completed the main learning comparison."
                caption="The bars show collected replies and their recorded labels. Hatched areas are measurements that were not collected. ‘Allocation refused’ means there were too few qualifying questions to form the required comparison. Attempts differed in random seeds, question selection or reply limits, so differences between these bars are not a measure of learning progress."/>
            <ScrollTable label="Decisive pilot collection, grader labels and final status">
                <table class="table">
                    <thead><tr><th>"Attempt"</th><th class="num">"Recorded / planned"</th><th class="num">"Passed (SOUND)"</th><th class="num">"Failed (FAIL)"</th><th class="num">"Invalid (INVALID)"</th><th>"Outcome of preparation"</th></tr></thead>
                    <tbody>
                        {record.decisive_pilots.iter().map(|run| {
                            let counts = run.counts();
                            let status = match run.version {
                                11 => "Closed: incorrect random-seed schedule",
                                12 => "Closed before the reply limit changed",
                                13 | 14 => "Too few qualifying questions",
                                15 => "Paused during collection",
                                _ => run.status.as_str(),
                            };
                            view! { <tr><td>{format!("v{}", run.version)}</td><td class="num">{format!("{} / {}", run.saved, run.scheduled)}</td><td class="num">{counts.sound}</td><td class="num">{counts.fail}</td><td class="num">{counts.invalid}</td><td>{status.to_owned()}</td></tr> }
                        }).collect_view()}
                    </tbody>
                </table>
            </ScrollTable>
            <details class="study-details">
                <summary>"Run conditions and reasons for stopping"</summary>
                <div class="study-details-content">
                    <p class="prose-p">"Version 11 used the wrong random-seed schedule. Version 12 ended when the reply limit was increased from 2,048 to 4,096 tokens. Versions 13 and 14 each lacked eight required questions in one problem type, identified in the records as class 07 and class 06 respectively."</p>
                    <p class="prose-p">"Version 15 saved 168 initial and seven repeated measurements before a requested pause to conserve compute. No outcome is assigned to uncollected measurements."</p>
                    <p class="prose-p">"The archive also includes five August attempts invalidated by protocol or measurement problems and two later attempts stopped before model evaluation. These records document preparation and its limitations."</p>
                </div>
            </details>
            <p class="source">"Experiment records through "{record.latest_experiment_date.clone()}"; summary compiled "{record.reviewed_on.clone()}". "<a href="/data/research-record.json" download="research-record.json">"Download the counts and source references"</a>"."</p>
        </section>
    }
}

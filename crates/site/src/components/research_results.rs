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
                <h2 id="worlds-h" class="display">"Identifying and reusing rules remained unreliable"</h2>
                <p class="lede-sm">"In the six-environment study, the model identified two hidden rules correctly and answered 3 of 12 later questions using its own rule descriptions. Separate tests showed that supplying a correct rule could support better answers, depending on the generation settings."</p>
                <p class="prose-p">"The environments followed hidden rules that the model tried to infer from observations. Later questions asked it to predict the outcome of new action sequences. Model weights stayed fixed, and saved descriptions were supplied in each prompt. These tests assess finding and using rules in context; they do not establish long-term retention."</p>
            </div>
            <ResearchFigure name="world-learning"
                alt="Separate studies of rule use and discovery: 9 of 12 correct answers with a supplied rule in one generation setting; 2 of 6 hidden rules identified correctly; an earlier eight-environment test produced 0 of 16 correct answers using its own records. Full comparisons follow."
                caption="Panel A tests use of supplied rules, panel B tests later questions after an attempt to identify each rule, and panel C reports an earlier eight-environment test. These are separate studies, not successive points on a learning curve. Retained denotes the earlier prediction adapter; candidate denotes the response adapter. Thinking on allows additional intermediate generation before the final answer. Failed attempts to identify rules remain in the totals."/>

            <h3 class="figure-h">"Applying a rule supplied in the question"</h3>
            <p class="prose-p">{format!("This study recorded {} replies: 12 questions in each of four settings, plus 12 repeated requests. Two adapters were compared with thinking mode enabled and disabled. The earlier prediction adapter answered 9 of 12 correctly with it enabled and 2 of 12 with it disabled.", record.rule_use.calls)}</p>
            <p class="prose-p">"All 12 repeated outputs matched their originals. The setting with 9 correct answers met the study’s criterion for proceeding. It also generated substantially more text. These results compare generation settings; no new training took place. Token totals include all generated text across the 12 questions in each setting."</p>
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

            <h3 class="figure-h">"Identifying rules from observations"</h3>
            <p class="prose-p">{format!("A separate study recorded {} replies across {} simulated environments. The model identified {} rules correctly; its other four proposals fell outside the set of allowed rules. With its own rule descriptions supplied, it answered 3 of 12 later questions correctly. The study did not meet its criteria for finding and then using the rules.", record.acquisition.calls, record.acquisition.worlds, record.acquisition.rules_correct)}</p>
            <p class="prose-p">"The model answered 3 of 4 questions correctly in the two environments where it identified the rule. That subset excludes four unsuccessful rule-discovery attempts; the full result remains 3 of 12. Each row below tests one question in each of six environments. Replies reaching the length limit are included in the denominator."</p>
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
            <p class="prose-p">"When information was missing or unrelated, the model was also assessed on whether it acknowledged uncertainty. It did so appropriately in 5 of 6 no-record cases and all 6 unrelated-record cases. Those are successes in expressing uncertainty, reported separately from correct predictions of the environment’s behaviour."</p>

            <div class="cols-2">
                <div class="card">
                    <h3>"An earlier test in eight environments"</h3>
                    <p>{format!("{} replies across {} environments produced {} correct initial rules and {} after feedback and revision. Later answers scored {}/{} with the model’s records, {}/{} with the original observations and {}/{} without records. Seven failed attempts left no usable rule to supply, so their later questions had no rule record. The planned delayed test was not run.", record.worlds.calls, record.worlds.worlds, record.worlds.initial_rules_correct, record.worlds.repaired_rules_correct, record.worlds.own, record.worlds.queries_per_condition, record.worlds.raw, record.worlds.queries_per_condition, record.worlds.none, record.worlds.queries_per_condition)}</p>
                </div>
                <div class="card">
                    <h3>"Feedback and rule correction"</h3>
                    <p>"A later study collected 70 replies across six environments; two reached the length limit. A software conflict during final record verification prevented the planned study decision. The saved observations show success on the rule and both later questions in 1 of 6 cases with checked feedback, 0 of 6 with a general request to check again, and 2 of 6 with a supplied correct rule. None of four initially incorrect or invalid rules was corrected."</p>
                </div>
            </div>
            <p class="prose-p">"The feedback study’s counts describe the saved replies; they do not replace its incomplete final evaluation. One reply from an earlier attempt is recorded separately and was not scored with those 70 replies."</p>
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
                <p class="eyebrow">"Preparation for the decisive-learning experiment"</p>
                <h2 id="decisive-h" class="display">"The planned learning comparison remains incomplete"</h2>
                <p class="lede-sm">"Versions 13 and 14 completed their preliminary measurements but lacked enough qualifying questions in required problem types. Version 15 was paused after 175 of 336 planned measurements. None reached the main comparison of learning outcomes."</p>
                <p class="prose-p">"Preparation had to identify enough questions with suitable difficulty and reliable scoring before the learning comparison could begin. These questions are called probes in the records. The counts below describe preparation, rather than a measured effect of learning."</p>
            </div>
            <p class="prose-p">"The recorded labels describe individual preliminary replies: SOUND means the reply passed the checker’s answer and derivation requirements; FAIL means it failed an answer requirement, including unfinished replies; INVALID means it violated the study’s output or information-leak rules. These labels use different criteria from the accepted-answer counts in the mathematics studies."</p>
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
            <div class="cols-3">
                <div class="card"><h3>"Changes to generation settings"</h3><p>"Version 11 used a different random-seed schedule from the one specified for preparation. Version 12 ended before completing collection, when the maximum reply length was increased from 2,048 to 4,096 tokens. Both attempts remain incomplete and excluded from completed learning comparisons."</p></div>
                <div class="card"><h3>"Not enough qualifying questions"</h3><p>"Version 13 had no qualifying questions in one required problem type, where eight were needed. Version 14 had the same shortfall in a different type. The records identify these as class 07 and class 06 respectively. Completing all 336 measurements did not satisfy the requirements for the next stage."</p></div>
                <div class="card"><h3>"Collection paused"</h3><p>"Version 15 saved 168 initial measurements and seven repeated measurements before the requested pause. The pause was an instruction to stop work, rather than a failure of a scientific test. The uncollected measurements have no reported outcomes."</p></div>
            </div>
            <p class="prose-p">"The archive also preserves five August attempts invalidated by protocol or measurement problems, and two later attempts stopped during preparation before model evaluation began. These records document difficulties in constructing the experiment. They are not completed tests of whether learning worked."</p>
            <p class="source">"Experiment records through "{record.latest_experiment_date.clone()}"; summary compiled "{record.reviewed_on.clone()}". "<a href="/data/research-record.json" download="research-record.json">"Download the counts and source references"</a>"."</p>
        </section>
    }
}

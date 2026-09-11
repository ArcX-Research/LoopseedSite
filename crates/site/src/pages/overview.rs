//! Framework, findings and research goals.
use crate::components::livestreams::FeaturedLivestreams;
use crate::components::loop_diagram::{LoopDiagram, STEPS};
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::rulings::RULINGS;
use loopseed_record::EQUATION;

#[component]
pub fn Overview() -> impl IntoView {
    set_title("Mission");
    let latest: Vec<_> = RULINGS.iter().take(3).collect();
    view! {
        <section class="hero">
            <div class="wrap hero-grid">
                <div class="hero-copy">
                    <p class="eyebrow">"Dilate Technologies · research program"</p>
                    <h1 class="display display-xl">"Toward Intelligence that keeps Learning"</h1>
                    <p class="lede">
                        "Our mission is to develop intelligence that learns from verified experience, adapts to unfamiliar situations and preserves useful knowledge and abilities over time."
                    </p>
                    <p class="prose-p">"Loopseed is a research programme investigating how computational systems learn through interaction with their environments. Fish, its current experimental platform, combines a language model with memory, prediction and tool use for controlled studies of learning, transfer and retention."</p>
                    <div class="hero-actions">
                        <a class="btn btn-primary" href="/results">"Read the results"</a>
                        <a class="btn" href="/method">"How we test learning"</a>
                        <a class="text-link" href="/goals">"Research goals and next tests "<span aria-hidden="true">"↗"</span></a>
                    </div>
                </div>
                <figure class="hero-visual">
                    <div class="visual-label"><span>"Research framework"</span><span>"01 — 05"</span></div>
                    <LoopDiagram/>
                    <figcaption>"The Fish learning cycle"</figcaption>
                </figure>
                <div class="research-summary">
                    <div class="summary-heading"><p class="eyebrow">"Results from one controlled mathematics study"</p><a class="text-link" href="/results#transfer-clean">"Read the study "<span aria-hidden="true">"↗"</span></a></div>
                    <dl class="hero-stats">
                        <div><dt>"Answers accepted after training"</dt><dd>"43"<span>" / 64"</span></dd></div>
                        <div class="stat-controls"><dt>"Accepted answers from the two control models"</dt><dd>"0 / 64"<span>" · "</span>"0 / 64"</dd></div>
                        <div><dt>"Problem types with at least one accepted answer"</dt><dd>"11"<span>" / 16"</span></dd></div>
                        <div class="stat-status"><dt>"Replication by an independent research team"</dt><dd>"Pending"</dd></div>
                    </dl>
                    <p class="source">"4 September 2026. Answers were accepted only if they passed exact mathematical checks and a separate AI review. The controls were the base model and a model trained with answers reassigned to different questions. "<a href="/results#transfer-clean">"Methods and limitations"</a>"."</p>
                    <p class="prose-p summary-context">"The test changed the numbers in problems from 16 types included in training. Later studies found limited success on harder problems and reduced performance on some earlier tasks. The figures above describe this study alone."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="paper-preview-h">
            <div class="paper-callout">
                <p class="eyebrow">"Research paper · 11 September 2026"</p>
                <h2 id="paper-preview-h" class="display">"Dynamical Synthesis: Learning through Interaction"</h2>
                <p class="prose-p">"The paper presents the Dynamical Synthesis framework and experiments with Fish. It examines whether learning improves performance on new problems and preserves earlier abilities, reporting both improvements and failed tests. It also discusses how feedback can repeat a system’s own claims without providing new evidence, and identifies gaps for further research."</p>
                <p class="prose-p">"A second paper will report follow-up studies aimed at closing gaps in the current findings. We plan to publish the research in phases, with each paper reporting new findings, unsuccessful tests and remaining gaps. The "<a href="/goals#next-h">"next research priorities"</a>" guide this work."</p>
                <div class="paper-downloads">
                    <a class="btn btn-primary" href="/papers/dynamical-synthesis.html" rel="external">"Read the paper"</a>
                    <a class="btn" href="/results">"View experimental results"</a>
                    <a class="text-link" href="/papers/dynamical-synthesis.html#materials" rel="external">"Data and sources ↗"</a>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="loop-h">
            <div class="section-head">
                <p class="eyebrow">"The approach"</p>
                <h2 id="loop-h" class="display">"Five steps in the learning cycle"</h2>
                <p class="lede-sm">"Dynamical Synthesis is Loopseed’s proposed framework for learning through interaction. It describes how a system combines its own computation with input from its environment as its state changes over time."</p>
                <p class="eq">{EQUATION}</p>
                <p class="prose-p">"Here, "<em>"I"</em>" represents the system’s state, "<em>"W(I)"</em>" the result of its internal computation, and "<em>"you"</em>" input from the environment, people or other systems. This equation is shorthand; an implementation must define these terms and how they change over time."</p>
                <p class="prose-p">"A further research goal is to test whether external observations help the system generate useful new hypotheses, methods or designs. "<a href="/goals#novelty-h">"How we would evaluate new ideas"</a>"."</p>
                <p class="prose-p">"If one system produces an incorrect answer and another copies it, the first system may receive its own error back as feedback. We propose testing whether the original system corrects the error when given new observations or verified calculations. "<a href="/method#boundary-h">"Read the computational model and proposed tests"</a>"."</p>
                <p class="prose-p">"The symbols and abbreviations beside each step are explained in the "<a href="/method#symbols">"symbol guide"</a>". Select any symbol to open the guide."</p>
            </div>
            <ol class="steps">
                {STEPS.iter().map(|s| view! {
                    <li class="step">
                        <span class="n">{s.ordinal}</span>
                        <div>
                            <h3>{s.label}" "<a class="mono organ" href="/method#symbols" aria-label=format!("Symbol definitions for {}: {}", s.label, s.organ)>{s.organ}</a></h3>
                            <p>{s.gloss}</p>
                        </div>
                    </li>
                }).collect_view()}
            </ol>
        </section>

        <section class="wrap section" aria-labelledby="premise-h">
            <div class="section-head">
                <p class="eyebrow">"Research principles"</p>
                <h2 id="premise-h" class="display">"How we evaluate learning"</h2>
            </div>
            <div class="cols-3">
                <div class="card">
                    <span class="n">"01"</span>
                    <h3>"Test prediction and task performance separately"</h3>
                    <p>"Predicting an observation and completing a task correctly are different abilities. Evaluate both directly, using separate measures of prediction error and task performance."</p>
                </div>
                <div class="card">
                    <span class="n">"02"</span>
                    <h3>"Test new learning and earlier abilities"</h3>
                    <p>"Compare an updated system with a control on the same tasks, using examples excluded from training. Test abilities demonstrated before the update as well as new learning, and report both improvements and declines."</p>
                </div>
                <div class="card">
                    <span class="n">"03"</span>
                    <h3>"Document sources and checks"</h3>
                    <p>"Preserve original observations and record how each training example was produced. Document the checks applied and their results, including failures. Keep unchecked examples clearly identified."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="measured-h">
            <div class="section-head">
                <p class="eyebrow">"Further experiments"</p>
                <h2 id="measured-h" class="display">"Results beyond the initial mathematics study"</h2>
                <p class="lede-sm">"These studies tested harder problems, the effects of further training and learning rules from observations. Each used different tasks and evaluation criteria."</p>
            </div>
            <div class="cols-3">
                <a class="card card-link" href="/results#harder">
                    <span class="n">"harder mathematics"</span>
                    <h3>"2 of 81 answers accepted after training"</h3>
                    <p>"Each model answered 27 problems three times. A control trained with answers paired to different questions also scored 2 of 81; a model using the earlier prediction update scored 3 of 81. The study did not establish improved performance over the control."</p>
                </a>
                <a class="card card-link" href="/results#current">
                    <span class="n">"further training"</span>
                    <h3>"More declines than improvements"</h3>
                    <p>"The update passed 12 cases the previous version failed, but failed 17 the previous version passed. Another 55 outcomes matched. The 84 comparisons covered 42 tasks tested twice. Seven declines involved earlier abilities, and the update was not adopted."</p>
                </a>
                <a class="card card-link" href="/results#worlds">
                    <span class="n">"learning rules"</span>
                    <h3>"2 of 6 rules identified correctly"</h3>
                    <p>"The model proposed rules after observing six simulated environments. With its saved rule descriptions supplied alongside new questions, it answered 3 of 12 correctly. No additional training was used. The study did not meet its criteria for learning and using the rules."</p>
                </a>
            </div>
        </section>

        <FeaturedLivestreams/>

        <section class="wrap section" aria-labelledby="evidence-h">
            <div class="section-head">
                <p class="eyebrow">"Study methods"</p>
                <h2 id="evidence-h" class="display">"How the mathematics studies were checked"</h2>
            </div>
            <div class="rules">
                <div class="rule"><span class="n">"01"</span><p><b>"Set the test criteria in advance."</b>" Tasks, model settings and success criteria were recorded before evaluation. The records identify the software, data and model versions used."</p></div>
                <div class="rule"><span class="n">"02"</span><p><b>"Use coded labels during review."</b>" The AI reviewer received answers with coded model labels. The mapping to model versions was revealed after the scores and review decisions were recorded."</p></div>
                <div class="rule"><span class="n">"03"</span><p><b>"Check the calculations."</b>" Software checked each proposed solution’s structure. The Wolfram kernel, run locally through WolframScript, evaluated its calculations using exact arithmetic. The results were compared with the expected answers."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"Review the complete answer."</b>" A separate AI reviewer examined complete answers that passed the calculation checks and could reject them. Replication by another research team is still needed."</p></div>
                <div class="rule"><span class="n">"05"</span><p><b>"Keep failures in the record."</b>" Unsuccessful and invalid runs retain their original reports and decisions. Experiments used separate system copies, with checks for changes to the live database file."</p></div>
                <div class="rule"><span class="n">"06"</span><p><b>"Provide data and methods."</b>" The "<a href="/papers/dynamical-synthesis.html#materials" rel="external">"paper, mathematics records and analysis code"</a>" can be downloaded. The mathematics package includes full replies, review decisions and a standalone count check. Reports state the statistical assumptions and access limits for other studies."</p></div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="record-h">
            <div class="section-head">
                <p class="eyebrow">"Research updates"</p>
                <h2 id="record-h" class="display">"Latest research reports"</h2>
            </div>
            <div class="rulings">
                {latest.into_iter().map(|r| view! {
                    <article class="ruling">
                        <span class="mono meta">{r.date}</span>
                        <div class="ruling-body"><h3>{r.title}</h3><p>{r.result}</p></div>
                    </article>
                }).collect_view()}
            </div>
            <p class="more"><a href="/record">"Read all reports and supporting evidence"</a></p>
        </section>

        <section class="section section-feature" aria-labelledby="goals-h">
            <div class="wrap">
                <div class="section-head">
                    <p class="eyebrow">"Research goals"</p>
                    <h2 id="goals-h" class="display">"Next studies and broader research goals"</h2>
                    <p class="lede-sm">"We intend to continue the research to close the gaps identified in the paper. The next studies will examine learning on unfamiliar tasks, retention of earlier abilities, rule learning and the effects of memory and training. They will also address feedback, human control and independent replication."</p>
                    <p class="prose-p">"The broader goals are lasting learning, useful new ideas and an understanding of learning across computational systems. These goals support the long-term aim of general intelligence that preserves human agency and wellbeing."</p>
                </div>
                <p class="more"><a class="btn" href="/goals">"Next studies and research goals"</a></p>
            </div>
        </section>
    }
}

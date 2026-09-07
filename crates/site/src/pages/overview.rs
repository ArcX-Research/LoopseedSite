//! Framework, findings and research goals.
use crate::components::loop_diagram::{LoopDiagram, STEPS};
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::rulings::RULINGS;
use loopseed_record::skin::ERA_ONE;
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
                    <h1 class="display display-xl">"Toward intelligence that keeps learning"</h1>
                    <p class="lede">
                        "Our mission is to build systems that turn verified experience into lasting improvements, apply what they learn to new situations, and retain earlier abilities. Loopseed is a research programme and experimental testbed for that question. Fish, its current implementation, combines a language model, memory, prediction and tools."
                    </p>
                    <p class="prose-p">"Early controlled tests show improved answers on new instances of specific mathematical tasks. Reliable learning across unfamiliar tasks and sustained interaction remains the goal."</p>
                    <div class="hero-actions">
                        <a class="btn btn-primary" href="/results">"Read the results"</a>
                        <a class="btn" href="/method">"How we test learning"</a>
                        <a class="btn" href="/promise">"The promise and next tests"</a>
                    </div>
                    <dl class="hero-stats mono">
                        <div><dt>"accepted answers · new parameter instances"</dt><dd>"43 / 64"</dd></div>
                        <div><dt>"base model and placebo, same tasks"</dt><dd>"0 / 64 · 0 / 64"</dd></div>
                        <div><dt>"task families with at least one success"</dt><dd>"11 / 16"</dd></div>
                        <div><dt>"independent replication of this result"</dt><dd>"pending"</dd></div>
                    </dl>
                    <p class="source">"Clean transfer study · 4 September 2026. Exact calculation plus a separate AI review of complete answers. "<a href="/results#transfer-clean">"Design, counts and limitations"</a>"."</p>
                </div>
                <div class="hero-visual"><LoopDiagram/></div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="loop-h">
            <div class="section-head">
                <p class="eyebrow">"The approach"</p>
                <h2 id="loop-h" class="display">"Five steps in the learning cycle"</h2>
                <p class="lede-sm">"The project calls its organising idea Dynamical Synthesis: a system repeatedly combines its own activity with external input. This is a design framework. Its value must be established through defined implementations and controlled tests."</p>
                <p class="eq">{EQUATION}</p>
                <p class="prose-p">"This shorthand names the idea; it is not a complete learning algorithm or an established law of intelligence. The Method page distinguishes the implemented state update, measured signals and offline training procedure."</p>
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
                    <h3>"Test prediction and answers separately"</h3>
                    <p>"Predicting a participant's next message and answering a problem correctly are different abilities. They use separate adapters and evaluation criteria. Lower prediction loss alone does not establish better answers."</p>
                </div>
                <div class="card">
                    <span class="n">"02"</span>
                    <h3>"Measure gains and losses"</h3>
                    <p>"Compare the same tasks across candidate and control conditions. Reserve examples from training, test earlier abilities, and record failures alongside gains. A successful task comparison is one step toward demonstrating lasting learning."</p>
                </div>
                <div class="card">
                    <span class="n">"03"</span>
                    <h3>"Keep experience traceable"</h3>
                    <p>"Retain the original exchanges, identify who supplied a training target, and distinguish verified solutions from model attempts. Prediction error helps select memories; correctness requires its own checks."</p>
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
                    <p>{format!("In the first observation period, session medians for one participant fell from {:.3} to {:.3}, then stayed near {:.2}. This is an observational trend: changing topics, context and system settings prevent attributing it to training alone.", ERA_ONE.first_median, ERA_ONE.low_median, ERA_ONE.plateau)}</p>
                </a>
                <a class="card card-link" href="/results#coats">
                    <span class="n">"controlled comparison"</span>
                    <h3>"Verified examples improved answers within tested families"</h3>
                    <p>"A corpus of 60 reviewed examples supplied 48 training examples, six validation examples and six test examples. The resulting adapter produced 43 accepted answers on 64 further parameter instances; both controls scored zero. Gains occurred within eleven learned families. Broader transfer remains open."</p>
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
                <div class="rule"><span class="n">"01"</span><p><b>"Plan before testing."</b>" The formal adapter studies freeze tasks, conditions and pass criteria in local protocol files before evaluation. Hashes identify the software, data and adapters used."</p></div>
                <div class="rule"><span class="n">"02"</span><p><b>"Hide condition identities."</b>" Experimental groups receive coded labels. Their identities are revealed only after results and reviews are fixed and hashed."</p></div>
                <div class="rule"><span class="n">"03"</span><p><b>"Recompute the mathematics."</b>" A checker validates the formal structure, and a computer algebra system calculates the results. Review also checks whether the steps answer the requested problem."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"Review complete replies."</b>" A separate AI reviewer reads each reply accepted by the checker, with condition identities withheld. It may reject an answer. This is an additional assessment within the project; independent replication is still needed."</p></div>
                <div class="rule"><span class="n">"05"</span><p><b>"Preserve the record."</b>" Failed runs retain their original decisions. These studies use isolated copies and compare live-database hashes before and after testing. A later private learning study can activate a passing adapter only on its clone."</p></div>
                <div class="rule"><span class="n">"06"</span><p><b>"Make the evidence assessable."</b>" This site recalculates paired statistics from recorded counts and states their assumptions. Reports identify source files; the underlying repository is currently private, with evidence available on request."</p></div>
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
                <p class="eyebrow">"The promise"</p>
                <h2 id="promise-h" class="display">"Useful experience that survives the next conversation"</h2>
                <p class="lede-sm">"If verified experience can improve future behaviour without erasing earlier abilities, adaptive systems could become more reliable through use. Our next tests examine retention, unfamiliar task families and independent replication. Broader adaptive intelligence, including AGI, is a long-term ambition whose required evidence extends well beyond these initial results."</p>
            </div>
            <p class="more"><a class="btn" href="/promise">"Research goals and limits"</a></p>
        </section>
    }
}

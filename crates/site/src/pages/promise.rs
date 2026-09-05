//! Research goals, limits and planned tests.
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::stages::{Stage, BECOMING_CLOSED, ORGANISM, SCALING};

#[component]
pub fn Promise() -> impl IntoView {
    set_title("Research goals");
    view! {
        <section class="wrap page">
            <p class="eyebrow">"Research goals"</p>
            <h1 class="display display-xl">"Can verified experience produce reliable learning?"</h1>
            <p class="lede">"Through Dynamical Synthesis, Loopseed aims to guide learning in any system whose inputs, state, predictions and actions can be defined and measured. Language models are the current testbed. The broader effort is toward artificial general intelligence (AGI): systems that can learn and apply knowledge across a wide range of tasks and environments."</p>
            <p class="prose-p">"The central question is whether learning transfers to new situations while preserving existing abilities. Each step toward that goal needs a stated test and a recorded result."</p>
        </section>

        <section class="wrap section" aria-labelledby="claim-h">
            <div class="section-head">
                <p class="eyebrow">"What we are trying to show"</p>
                <h2 id="claim-h" class="display">"Three questions, in order"</h2>
            </div>
            <div class="cols-3">
                <div class="card">
                    <span class="n">"01"</span>
                    <h3>"Does interaction improve prediction?"</h3>
                    <p>"Observed: prediction error for one regular participant fell, then levelled off above the memory threshold θ. The adapters tested so far have not visibly shifted that participant's curve."</p>
                </div>
                <div class="card">
                    <span class="n">"02"</span>
                    <h3>"Does verified training improve new-task performance?"</h3>
                    <p>"Observed on isolated copies: an adapter trained on reviewed examples succeeded on new instances in eleven task families. It had no losses against the base model or shuffled-reply placebo, both of which scored zero. This adapter has not been deployed to the live system."</p>
                </div>
                <div class="card">
                    <span class="n">"03"</span>
                    <h3>"Does it replicate?"</h3>
                    <p>"Unresolved. Replication uses model–supervisor pairs, called dyads. In each of two prospective protocols, only one of three pairs confirmed the predicted pattern. An unattended setup on an independent machine is also still required."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="not-h">
            <div class="section-head">
                <p class="eyebrow">"Limits of the evidence"</p>
                <h2 id="not-h" class="display">"What these results establish"</h2>
            </div>
            <ul class="plain">
                <li>"The measurements describe system behaviour. They provide no evidence of consciousness or subjective experience."</li>
                <li>"The transfer test used new parameter values within a fixed formal language covering sixteen task families. It does not establish general theorem proving, code generation or reasoning outside that language."</li>
                <li>"The September adapter experiments used disposable copies. Matching database hashes before and after each run confirm that the live system was unchanged."</li>
                <li>"Memory retrieval improved recall in some tests and reduced judgement accuracy in others. Reliable answers with references to their source records remain an open goal."</li>
                <li>"The external-input study used approximate word-based measures. Later safeguards changed the conditions, so the two measurement periods cannot be compared directly."</li>
            </ul>
        </section>

        <section id="becoming" class="wrap section" aria-labelledby="becoming-h">
            <div class="section-head">
                <p class="eyebrow">"Becoming track · research ambition"</p>
                <h2 id="becoming-h" class="display">"A learning cycle for different kinds of systems"</h2>
                <p class="lede-sm">"The aim is to use the same cycle in adaptive software, control systems, embodied agents and language models. Each application would define its own observations, actions, memory and measures of success. Evidence from one implementation would guide the next experiment."</p>
            </div>
            <div class="cols-3">
                <div class="card">
                    <span class="n">"01"</span>
                    <h3>"Apply the cycle beyond language"</h3>
                    <p>"Test whether prediction, feedback, memory and action improve a system working with signals or observations other than text. Compare it with the same system operating without the cycle."</p>
                </div>
                <div class="card">
                    <span class="n">"02"</span>
                    <h3>"Transfer learning across tasks"</h3>
                    <p>"Test whether experience in one setting helps on unfamiliar tasks, changing conditions and new environments. Measure what transfers and what is lost."</p>
                </div>
                <div class="card">
                    <span class="n">"03"</span>
                    <h3>"Work toward general intelligence"</h3>
                    <p>"Study whether these abilities can combine into broader, sustained learning. AGI is the long-term research goal. The results reported here establish narrower findings from language-model experiments."</p>
                </div>
            </div>
            <div class="prose">
                <h3>"Extend symbolic checking toward formal verification"</h3>
                <p>"The current neurosymbolic lab gives this work a starting point: structured proposals, a deterministic compiler, exact calculation and independent review. The next goal is to verify reusable methods against explicit specifications on a wider range of tasks."</p>
                <p>"A later direction is to generate an implementation and a machine-checkable formal proof from the same specification. That would require proving that the implementation satisfies the specification, as well as checking that the specification represents the intended task. This capability remains a research goal."</p>
            </div>
            <p class="prose-p">"These are proposed directions. They require new protocols and independent tests. The current roadmap's unfinished criteria remain prerequisites for opening the Becoming experiments."</p>
        </section>

        <section id="tracks" class="wrap section" aria-labelledby="tracks-h">
            <div class="section-head">
                <p class="eyebrow">"Research progress"</p>
                <h2 id="tracks-h" class="display">"System development and replication"</h2>
                <p class="lede-sm">{BECOMING_CLOSED}</p>
            </div>
            <p class="prose-p">"Stage identifiers match the project roadmap. A closed stage may include an explicitly recorded waiver; the status column states where that occurred."</p>
            <h3 class="track-h">"System development"</h3>
            <StageTable stages=ORGANISM/>
            <h3 class="track-h">"Scaling and replication"</h3>
            <StageTable stages=SCALING/>
        </section>

        <section class="wrap section" aria-labelledby="next-h">
            <div class="section-head">
                <p class="eyebrow">"Next tests"</p>
                <h2 id="next-h" class="display">"What the latest result supports testing next"</h2>
            </div>
            <div class="rules">
                <div class="rule"><span class="n">"01"</span><p><b>"Verify reusable methods."</b>" Extract a general procedure from an answer produced without supplied steps. Check it on newly generated tasks and compare it with the base model in a blinded test. If a later task disproves it, deactivate the method and retain its history. Only methods that remain verified would be eligible for reply training."</p></div>
                <div class="rule"><span class="n">"02"</span><p><b>"Test harder tasks."</b>" Compare the candidate, placebo and retained prediction adapter on tasks outside the sixteen templates. Check that the calculation steps match the requested problem before independent review."</p></div>
                <div class="rule"><span class="n">"03"</span><p><b>"Check for losses before deployment."</b>" Test memory, uncertainty and performance within each family. Require a candidate advantage over the placebo and a separate test of reply generation before considering deployment. Experiments cannot automatically train or load a new adapter into the live system."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"Repeat the setup independently."</b>" Run the packaged system unattended on another machine and record a signed result. Local setup tests alone do not meet the reproducibility criterion."</p></div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="falsify-h">
            <div class="section-head">
                <p class="eyebrow">"Interpreting future results"</p>
                <h2 id="falsify-h" class="display">"Findings that would change the conclusion"</h2>
            </div>
            <ul class="plain">
                <li>"Failure on tasks outside the trained families would limit the transfer claim to the tested formal language."</li>
                <li>"A shuffled-reply placebo matching the candidate under the loose grammar would weaken the claim that training taught a method beyond the output format."</li>
                <li>"Lower prediction error after an approved deployment would support a benefit in live interaction. An unchanged curve would show that the isolated task gains did not improve that measure."</li>
                <li>"A predicted pattern that fails its preset test across three model–supervisor pairs is rejected under that protocol."</li>
            </ul>
        </section>
    }
}

#[component]
fn StageTable(stages: &'static [Stage]) -> impl IntoView {
    view! {
        <div class="table-wrap">
            <table class="table stages">
                <thead><tr><th>"#"</th><th>"Stage"</th><th>"Purpose"</th><th>"Pass criterion"</th><th>"Status"</th></tr></thead>
                <tbody>
                    {stages.iter().map(|s| view! {
                        <tr>
                            <td class="mono muted">{s.id}</td>
                            <td><b>{s.name}</b></td>
                            <td>{s.what}</td>
                            <td class="muted">{s.pass_test}</td>
                            <td><span class=format!("st {}", s.status.class())>{s.status.label()}</span><span class="state">{s.state}</span></td>
                        </tr>
                    }).collect_view()}
                </tbody>
            </table>
        </div>
    }
}

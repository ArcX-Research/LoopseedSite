//! Research goals, limits and planned tests.
use crate::components::page_nav::PageNav;
use crate::components::tables::ScrollTable;
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::stages::{Stage, BECOMING_CLOSED, ORGANISM, SCALING};

const SECTIONS: &[(&str, &str)] = &[
    ("#claim-h", "Research questions"),
    ("#not-h", "Evidence limits"),
    ("#becoming", "Ambition"),
    ("#tracks", "Roadmap"),
    ("#next-h", "Next tests"),
];

#[component]
pub fn Goals() -> impl IntoView {
    set_title("Research goals");
    view! {
        <section class="page-intro page-intro-goals">
            <div class="wrap">
                <p class="eyebrow">"Research goals"</p>
                <h1 class="display display-xl">"Systems that improve through experience and keep what they learn"</h1>
                <p class="lede">"Loopseed aims to develop cumulative learning: useful exchanges become improvements that persist, transfer to new situations and preserve earlier abilities. If demonstrated, this could support assistants that benefit from verified corrections and adaptive software that improves as its environment changes."</p>
                <p class="prose-p">"Fish is our current language-model testbed. The longer-term ambition is broader adaptive intelligence, including artificial general intelligence (AGI). Here that means learning and applying knowledge across a wide range of tasks and environments. It is a research direction, not a capability established by the results on this site."</p>
            </div>
        </section>

        <PageNav items=SECTIONS/>
        <section class="wrap section" aria-labelledby="claim-h">
            <div class="section-head">
                <p class="eyebrow">"What we are trying to show"</p>
                <h2 id="claim-h" class="display">"Three questions, in order"</h2>
            </div>
            <div class="cols-3">
                <div class="card">
                    <span class="n">"01"</span>
                    <h3>"Does interaction improve prediction?"</h3>
                    <p>"An observational record shows lower prediction discrepancy followed by a plateau for one participant. It does not isolate the effect of training. The next requirement is improvement on excluded future messages under controlled comparisons, while retaining performance on earlier material."</p>
                </div>
                <div class="card">
                    <span class="n">"02"</span>
                    <h3>"Does verified training improve useful answers?"</h3>
                    <p>"The clean formal study accepted 43 of 64 answers on new parameter instances, with gains within eleven learned families. Both principal controls scored zero. That supports an effect in the tested setting; zero paired losses against zero-scoring controls provides little evidence about preserving other abilities."</p>
                </div>
                <div class="card">
                    <span class="n">"03"</span>
                    <h3>"Does the improvement persist and replicate?"</h3>
                    <p>"Independent replication of the clean formal result is pending. A separate roadmap test of predicted patterns across model–supervisor pairs confirmed only one of three pairs in each of two protocols. Those tests concern a different endpoint; they do not replicate the 43-of-64 result. Sustained retention and independent setup also remain open."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="not-h">
            <div class="section-head">
                <p class="eyebrow">"Limits of the evidence"</p>
                <h2 id="not-h" class="display">"The scope of the present evidence"</h2>
            </div>
            <ul class="plain">
                <li>"The measurements describe system behaviour. They provide no evidence of consciousness or subjective experience."</li>
                <li>"The transfer test used new parameter values within a fixed formal language covering sixteen task families. It does not establish general theorem proving, code generation or reasoning outside that language."</li>
                <li>"The formal adapter studies used disposable copies and verified that the checked live database file was unchanged. A later continuing study may activate passing adapters on its private clone. These are distinct from qualification for the main live system."</li>
                <li>"A separate AI review strengthens answer assessment but can still make mistakes. The studies are internally conducted, and the underlying evidence is available on request from a private repository."</li>
                <li>"Memory retrieval improved recall in some tests and reduced judgement accuracy in others. Reliable answers with references to their source records remain an open goal."</li>
                <li>"The external-input study used approximate word-based measures. Later safeguards changed the conditions, so the two measurement periods cannot be compared directly."</li>
            </ul>
        </section>

        <section id="becoming" class="wrap section" aria-labelledby="becoming-h">
            <div class="section-head">
                <p class="eyebrow">"Becoming track · research ambition"</p>
                <h2 id="becoming-h" class="display">"A learning cycle for different kinds of systems"</h2>
                <p class="lede-sm">"We would like to test the approach in adaptive software, control systems and embodied agents as well as language models. Each would need its own operational definitions, suitable baselines and criteria for improvement. The language-model results do not establish that the same approach will work in those settings."</p>
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
                <p>"The current neurosymbolic lab provides structured proposals, a deterministic compiler, exact calculation and separate AI review. The next goal is to verify reusable methods against explicit specifications on a wider range of tasks. Exact arithmetic on a generated instance is a useful check; proving correctness for all permitted inputs requires additional machinery."</p>
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
            <p class="prose-p">"Stage identifiers and status labels reproduce the project roadmap as reviewed on 7 September 2026. ‘Closed’ is a project decision, not independent scientific validation. A stage may close through a recorded supervisor waiver even when its original empirical criterion was not met."</p>
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
                <div class="rule"><span class="n">"03"</span><p><b>"Test retention through successive updates."</b>" Replay admitted earlier experience while preserving held-out sets. Compare prediction and spoken answers separately, including uncertainty and memory controls. The continuing private study takes an initial step; longer runs and independent test sets are needed before broader deployment claims."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"Repeat the setup independently."</b>" Run the packaged system unattended on another machine and record a signed result. Local setup tests alone do not meet the reproducibility criterion."</p></div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="falsify-h">
            <div class="section-head">
                <p class="eyebrow">"Interpreting future results"</p>
                <h2 id="falsify-h" class="display">"Findings that would change the conclusion"</h2>
            </div>
            <ul class="plain">
                <li>"Repeated failure on unfamiliar task families would weaken the case for broader transfer; the current within-family finding would remain bounded by its original test."</li>
                <li>"A matched shuffled-reply control reaching the candidate's score in new loose-grammar tests would weaken the explanation that correct example pairings produce the advantage."</li>
                <li>"Improvement on a new held-out task set, compared with a concurrent control, would support a benefit from the update. A before-and-after curve alone would still be vulnerable to changes in topics and interaction."</li>
                <li>"Losses on earlier abilities across successive updates would challenge the claim of cumulative learning, even if each update improved its immediate training objective."</li>
                <li>"A predicted pattern that fails its preset test across three model–supervisor pairs is rejected under that protocol."</li>
            </ul>
        </section>
    }
}

#[component]
fn StageTable(stages: &'static [Stage]) -> impl IntoView {
    view! {
        <ScrollTable label="Research roadmap and stage status">
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
        </ScrollTable>
    }
}

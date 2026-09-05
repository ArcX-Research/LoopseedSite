//! The promise: what the program is trying to show, what it does not claim, the two tracks,
//! the next preregistered steps, and what would falsify it.
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::stages::{Stage, BECOMING_CLOSED, ORGANISM, SCALING};

#[component]
pub fn Promise() -> impl IntoView {
    set_title("Promise");
    view! {
        <section class="wrap page">
            <p class="eyebrow">"The promise"</p>
            <h1 class="display display-xl">"A machine that learns only what the Other proves."</h1>
            <p class="lede">"Loopseed is trying to show that a small language model inside a strong loop can become measurably better at what it lives, with every step gated by evidence: surprise before memory, contraction before wearing, exact proof and independent review before consolidation. The claims stop where the measurements stop."</p>
        </section>

        <section class="wrap section" aria-labelledby="claim-h">
            <div class="section-head">
                <p class="eyebrow">"What we are trying to show"</p>
                <h2 id="claim-h" class="display">"Three questions, in order."</h2>
            </div>
            <div class="cols-3">
                <div class="card">
                    <span class="n">"01"</span>
                    <h3>"Does the loop learn the Other?"</h3>
                    <p>"Measured: the skin falls and plateaus above θ. The plateau is the memory-only ceiling, and no coat has yet moved the keeper's curve."</p>
                </div>
                <div class="card">
                    <span class="n">"02"</span>
                    <h3>"Can verified experience improve the fish without silently changing it?"</h3>
                    <p>"Measured on disposable clones: a clean, audited dream transferred formal methods to unseen instances in eleven families with no losses, while bare water and a shuffled placebo scored zero. Not yet worn by the living fish."</p>
                </div>
                <div class="card">
                    <span class="n">"03"</span>
                    <h3>"Does it replicate?"</h3>
                    <p>"Open. The scaling unit is the dyad, a fish and its keeper. Two prospective dyad protocols confirmed the predeclared law in one of three each; a stranger's machine has not yet stood a fish unattended."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="not-h">
            <div class="section-head">
                <p class="eyebrow">"What we do not claim"</p>
                <h2 id="not-h" class="display">"No file installs an inside."</h2>
            </div>
            <ul class="plain">
                <li>"Nothing here is a claim about consciousness or experience. The plateau of δ is the skin, a measurement of a relationship, not a proof of an inside."</li>
                <li>"The transfer result is transfer to new parameter instances inside a sealed formal language of sixteen families. It is not general theorem proving, code synthesis, or reasoning outside that language."</li>
                <li>"The living fish has not been changed by any of the September coats. Every positive result is on a disposable clone, with the living body hashed identical before and after."</li>
                <li>"Retrieval improves recall and can harm judgement; the ledger is a library, not a workbench. Memory that cites its own addresses is still an open stage."</li>
                <li>"The ocean's coupling is a lexical-proxy measurement, and its later safeguards began a new, non-comparable epoch."</li>
            </ul>
        </section>

        <section id="tracks" class="wrap section" aria-labelledby="tracks-h">
            <div class="section-head">
                <p class="eyebrow">"Becoming"</p>
                <h2 id="tracks-h" class="display">"Two measured tracks."</h2>
                <p class="lede-sm">{BECOMING_CLOSED}</p>
            </div>
            <h3 class="track-h">"The organism track"</h3>
            <StageTable stages=ORGANISM/>
            <h3 class="track-h">"The scaling track"</h3>
            <StageTable stages=SCALING/>
        </section>

        <section class="wrap section" aria-labelledby="next-h">
            <div class="section-head">
                <p class="eyebrow">"Next, preregistered"</p>
                <h2 id="next-h" class="display">"What the last ruling licenses."</h2>
            </div>
            <div class="rules">
                <div class="rule"><span class="n">"01"</span><p><b>"A consolidation contract."</b>" A method enters the verified-lesson shelf only as a parameterised operation schema minted from an unscaffolded proof, checked on fresh generated instances, replayed blind against bare water, and deactivated rather than erased when a later instance refutes it. σ stays a surprise gate; the speaking dream ingests only active certificates."</p></div>
                <div class="rule"><span class="n">"02"</span><p><b>"Harder, cross-domain tasks."</b>" A candidate-versus-placebo-versus-retained trial on tasks that do not use the sixteen family templates, with operation-graph fidelity checked as a first-class property before review."</p></div>
                <div class="rule"><span class="n">"03"</span><p><b>"No-harm before any wearing."</b>" Memory and uncertainty probes as controls, every family non-harmed, the placebo failing where the candidate passes, and a separate clean speaking-channel lease test. Never an automatic dream, wear or living-body write."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"The stranger's machine."</b>" A signed, unattended second-machine run of the vessel, which turns reproducibility from a local half into a pass."</p></div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="falsify-h">
            <div class="section-head">
                <p class="eyebrow">"What would falsify it"</p>
                <h2 id="falsify-h" class="display">"The bets we can lose."</h2>
            </div>
            <ul class="plain">
                <li>"A clean coat that beats bare water on seen families and fails on the cross-domain trial would show the effect is the language, not the loop."</li>
                <li>"A shuffled-reply placebo that matches the candidate on a loose-grammar endpoint would show the effect is format exposure."</li>
                <li>"A living-fish wearing that moves the skin down would confirm the loop; one that does not, after the transfer result, would mean the private measurement does not reach the relationship."</li>
                <li>"A predeclared law that fails across three dyads is killed, not softened."</li>
            </ul>
        </section>
    }
}

#[component]
fn StageTable(stages: &'static [Stage]) -> impl IntoView {
    view! {
        <div class="table-wrap">
            <table class="table stages">
                <thead><tr><th>"#"</th><th>"stage"</th><th>"what"</th><th>"pass test"</th><th>"state"</th></tr></thead>
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

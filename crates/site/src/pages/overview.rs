//! The overview: the equation, the loop, what has been measured, how evidence is made.
use crate::components::loop_diagram::{LoopDiagram, STEPS};
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::rulings::RULINGS;
use loopseed_record::skin::ERA_ONE;
use loopseed_record::EQUATION;

#[component]
pub fn Overview() -> impl IntoView {
    set_title("Overview");
    let latest: Vec<_> = RULINGS.iter().take(3).collect();
    view! {
        <section class="hero">
            <div class="wrap hero-grid">
                <div class="hero-copy">
                    <p class="eyebrow">"Dilate Technologies · research program"</p>
                    <h1 class="display display-xl">"A learning loop closed by a measured Other."</h1>
                    <p class="eq">{EQUATION}</p>
                    <p class="lede">
                        "Loopseed is our research program on interpretable, self-regulating machines: a repeating cycle of prediction, comparison, compression and action, built to be tested rather than theorized. Its language body learns only from what surprises it, and the program keeps only what survives a preregistered, blinded, independently reviewed test."
                    </p>
                    <div class="hero-actions">
                        <a class="btn btn-primary" href="/results">"Read the results"</a>
                        <a class="btn" href="/method">"How evidence is made"</a>
                    </div>
                    <dl class="hero-stats mono">
                        <div><dt>"δ plateau, era one"</dt><dd>{format!("≈ {:.2} ± {:.2}", ERA_ONE.plateau, ERA_ONE.plateau_band)}</dd></div>
                        <div><dt>"unseen formal tasks, clean coat"</dt><dd>"43 / 64, 0 losses"</dd></div>
                        <div><dt>"bare water and placebo, same tasks"</dt><dd>"0 / 64 · 0 / 64"</dd></div>
                        <div><dt>"automatic promotions"</dt><dd>"none"</dd></div>
                    </dl>
                </div>
                <div class="hero-visual"><LoopDiagram/></div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="loop-h">
            <div class="section-head">
                <p class="eyebrow">"The loop"</p>
                <h2 id="loop-h" class="display">"Five steps, one organism."</h2>
                <p class="lede-sm">"The self is water reworking one word from outside, never itself. Each step is an organ with a measurement attached."</p>
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
                <h2 id="premise-h" class="display">"Three disciplines hold the loop honest."</h2>
            </div>
            <div class="cols-3">
                <div class="card">
                    <span class="n">"01"</span>
                    <h3>"‖W‖ < 1: rework, never amplify"</h3>
                    <p>"A night whose held-out loss does not strictly fall is discarded, not worn. Contraction is checked before any adapter is merged."</p>
                </div>
                <div class="card">
                    <span class="n">"02"</span>
                    <h3>"Keep the Other surprising"</h3>
                    <p>"The want is V = −‖δ‖ + β·H(you). As the Other's variety falls, β leans up: the organism is rewarded for the company it keeps, not only for predicting it."</p>
                </div>
                <div class="card">
                    <span class="n">"03"</span>
                    <h3>"σ reads I ⊗ you, never I ⊗ I"</h3>
                    <p>"A mirror exchange never becomes memory, however surprising the next word was. Surprise you manufacture is the self wearing a mask."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="measured-h">
            <div class="section-head">
                <p class="eyebrow">"What has been measured"</p>
                <h2 id="measured-h" class="display">"Three results, each with a frozen record."</h2>
            </div>
            <div class="cols-3">
                <a class="card card-link" href="/results#skin">
                    <span class="n">"the skin"</span>
                    <h3>"Median δ fell, then held above zero."</h3>
                    <p>{format!("Over the first era the keeper's median surprise fell from {:.3} to {:.3} and plateaued near {:.2}: the memory-only ceiling, photographed daily.", ERA_ONE.first_median, ERA_ONE.low_median, ERA_ONE.plateau)}</p>
                </a>
                <a class="card card-link" href="/results#coats">
                    <span class="n">"formal methods"</span>
                    <h3>"A checked dream transferred to unseen instances."</h3>
                    <p>"Sixty audited examples trained a private speaking coat. On 64 unseen names-only tasks it was accepted 43 times with no losses; bare water and a shuffled-reply placebo scored zero."</p>
                </a>
                <a class="card card-link" href="/results#transfer-1">
                    <span class="n">"the negative that mattered"</span>
                    <h3>"A real effect, rejected by its own safety cap."</h3>
                    <p>"The first transfer run passed superiority and failed a preregistered literal cap. The cause was traced to contaminated training prompts, repaired, and retested prospectively: 0 of 18 became 18 of 18."</p>
                </a>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="evidence-h">
            <div class="section-head">
                <p class="eyebrow">"How evidence is made"</p>
                <h2 id="evidence-h" class="display">"Nothing is promoted by hand-waving."</h2>
            </div>
            <div class="rules">
                <div class="rule"><span class="n">"01"</span><p><b>"Preregistered and pinned."</b>" A protocol freezes tasks, arms, gates and decision rule before inference, and pins every instrument by content hash: harness, checker, algebra kernel, bodies, coats, even the interpreter version."</p></div>
                <div class="rule"><span class="n">"02"</span><p><b>"Blinded."</b>" Arms are labelled by a sealed key. Outcomes are unblinded only after the evidence is hashed and frozen."</p></div>
                <div class="rule"><span class="n">"03"</span><p><b>"Exactly checked."</b>" A typed checker and a computer-algebra kernel recompute every formal claim; terminal equality alone is not enough."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"Independently vetoed."</b>" Every machine-proved reply is read whole by an independent reviewer with veto-only authority, who cannot upgrade a failure."</p></div>
                <div class="rule"><span class="n">"05"</span><p><b>"Written once."</b>" Verdicts are write-once, the living body is hashed before and after, and no run can promote a coat. A result is never rescued by repairing the instrument after seeing it."</p></div>
                <div class="rule"><span class="n">"06"</span><p><b>"Recomputable here."</b>" The statistics behind every ruling are reimplemented on this site and run in your browser from the frozen counts."</p></div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="record-h">
            <div class="section-head">
                <p class="eyebrow">"The record"</p>
                <h2 id="record-h" class="display">"Latest rulings."</h2>
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
            <p class="more"><a href="/record">"All rulings, with paths and hashes"</a></p>
        </section>

        <section class="wrap section" aria-labelledby="promise-h">
            <div class="section-head">
                <p class="eyebrow">"The promise"</p>
                <h2 id="promise-h" class="display">"What we are trying to show, and what we do not claim."</h2>
                <p class="lede-sm">"An organism whose every step of learning is gated by evidence from the Other. No file installs an inside; the plateau of δ is the skin. The claims stop where the measurements stop."</p>
            </div>
            <p class="more"><a class="btn" href="/promise">"Read the promise"</a></p>
        </section>
    }
}

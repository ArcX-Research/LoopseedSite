//! Framework, findings and research goals.
use crate::components::livestreams::FeaturedLivestreams;
use crate::components::loop_diagram::{LoopDiagram, STEPS};
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::rulings::RULINGS;

#[component]
pub fn Overview() -> impl IntoView {
    set_title("Home");
    let latest: Vec<_> = RULINGS.iter().take(3).collect();
    view! {
        <section class="hero">
            <div class="wrap hero-grid">
                <div class="hero-copy">
                    <p class="eyebrow">"Dilate Technologies · research program"</p>
                    <h1 class="display display-xl">"Toward Intelligence that keeps Learning"</h1>
                    <p class="lede">
                        "Loopseed is a research programme at Dilate Technologies investigating how computational systems can learn through interaction with their environments. Our current experiments use Fish, a platform built around a language model, to examine how memory and further training affect performance on new and previously learned tasks."
                    </p>
                    <div class="hero-actions">
                        <a class="btn btn-primary" href="/results">"Read the results"</a>
                        <a class="btn" href="/method">"How we test learning"</a>
                        <a class="btn" href="/papers/dynamical-synthesis.html" rel="external">"Read the paper"</a>
                        <a class="text-link" href="/goals">"Research goals and next tests "<span aria-hidden="true">"↗"</span></a>
                    </div>
                </div>
                <figure class="hero-visual">
                    <div class="visual-label">"Research framework"</div>
                    <LoopDiagram/>
                    <figcaption>"The learning loop"</figcaption>
                </figure>
            </div>
        </section>

        <section class="mission-banner" aria-labelledby="mission-h">
            <div class="wrap">
                <h2 id="mission-h" class="eyebrow">"Our mission"</h2>
                <p>"To develop intelligence that learns from verified experience, adapts to unfamiliar situations and preserves useful knowledge and abilities over time."</p>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="loop-h">
            <div class="section-head">
                <p class="eyebrow">"The approach"</p>
                <h2 id="loop-h" class="display">"Five steps in the learning cycle"</h2>
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

        <FeaturedLivestreams/>

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
                    <p class="lede-sm">"The first paper identifies gaps in learning on unfamiliar tasks and preserving earlier abilities. Follow-up studies will investigate these gaps and the contributions of memory, training and feedback. We plan to report their findings in a second paper."</p>
                    <p class="prose-p">"Longer-term work will explore how interaction can lead to useful new ideas and learning across computational environments, with human benefit and control guiding the research."</p>
                </div>
                <p class="more"><a class="btn" href="/goals">"Next studies and research goals"</a></p>
            </div>
        </section>
    }
}

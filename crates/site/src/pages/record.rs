//! Research reports and source records.
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::rulings::RULINGS;
use loopseed_record::{ALBUM_PATH, CONTACT_EMAIL, CONTACT_URL, LAWS_PATH, REPOSITORY_URL};

#[component]
pub fn Record() -> impl IntoView {
    set_title("Record");
    view! {
        <section class="page-intro page-intro-record">
            <div class="wrap">
                <p class="eyebrow">"Record"</p>
                <h1 class="display display-xl">"Research reports, newest first"</h1>
                <p class="lede">"The archive contains study plans, transcripts, measurements and project decisions. These summaries distinguish completed results from preparation and observation. Formal-study reports identify source artifacts and hashes; early notes and figures have less extensive documentation. Original failed decisions remain part of the record."</p>
                <p class="prose-p">"Summaries were reviewed on 7 September 2026. The original reports sometimes use stronger terms such as ‘causal ruling’ or ‘method transfer’. Those labels name the project's decisions; the Results page states the narrower empirical scope, review process and statistical assumptions."</p>
                <p class="more"><a class="btn" href="#where-h">"Access the source records "<span aria-hidden="true">"↗"</span></a></p>
            </div>
        </section>

        <section class="wrap section">
            <div class="rulings rulings-full">
                {RULINGS.iter().map(|r| view! {
                    <article class="ruling">
                        <span class="mono meta">{r.date}</span>
                        <div class="ruling-body">
                            <h3>{r.title}</h3>
                            <p>{r.result}</p>
                            <details class="report-source">
                                <summary>"Source record"</summary>
                                <p class="mono small">{r.path}</p>
                                {r.hash.map(|h| view! { <p class="mono small muted">{format!("decision artifact sha256 {h}")}</p> })}
                            </details>
                        </div>
                    </article>
                }).collect_view()}
            </div>
        </section>

        <section class="wrap section" aria-labelledby="where-h">
            <div class="section-head">
                <p class="eyebrow">"Where the evidence lives"</p>
                <h2 id="where-h" class="display">"Find and verify the source records"</h2>
            </div>
            <div class="cols-2">
                <div>
                    <p class="prose-p">"File paths on this site start at the root of the Loopseed checkout associated with a run. Research reports are in "<code>{ALBUM_PATH}</code>"; recorded observations are in "<code>{LAWS_PATH}</code>". Formal-study directories hold protocols, coded condition keys, saved results, review materials and decisions. SHA-256 hashes let a reader verify that supplied bytes match the named artifact."</p>
                    {match REPOSITORY_URL {
                        Some(url) => view! { <p class="prose-p">"The repository is public: "<a href=url>{url}</a>"."</p> }.into_any(),
                        None => view! { <p class="prose-p">"The source repository and full run artifacts are currently private. Evidence is available on request. Until those materials can be independently inspected and the runs repeated, this site offers traceable summaries rather than a publicly reproducible evidence package."</p> }.into_any(),
                    }}
                </div>
                <div>
                    <p class="prose-p">"A matching hash verifies file identity. It does not establish that a measurement is correct, that a protocol was publicly registered, or that all possible information leakage was excluded. Software checks cover the conditions recorded by each instrument."</p>
                    <p class="prose-p">"For a technical review, request the run's frozen protocol, model and adapter identifiers, prompts, task-generation code, reply records, scorer, review materials and final decision. Name the run and the files when you "
                        {match CONTACT_EMAIL {
                            Some(mail) => view! { <a href=format!("mailto:{mail}")>{mail}</a> }.into_any(),
                            None => view! { <a href=CONTACT_URL>"contact Dilate Technologies"</a> }.into_any(),
                        }}
                    "."</p>
                </div>
            </div>
        </section>
    }
}

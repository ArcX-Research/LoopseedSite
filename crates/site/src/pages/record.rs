//! Research reports and source records.
use crate::components::page_nav::PageNav;
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::rulings::RULINGS;
use loopseed_record::{ALBUM_PATH, CONTACT_EMAIL, CONTACT_URL, LAWS_PATH, REPOSITORY_URL};

const SECTIONS: &[(&str, &str)] = &[
    ("#paper-h", "Paper and downloads"),
    ("#reports-h", "Study reports"),
    ("#where-h", "Source records"),
];

#[component]
pub fn Record() -> impl IntoView {
    set_title("Reports");
    view! {
        <section class="page-intro page-intro-record">
            <div class="wrap">
                <p class="eyebrow">"Reports"</p>
                <h1 class="display display-xl">"Study reports and supporting evidence"</h1>
                <p class="lede">"Read the working paper, follow the findings from individual studies and locate the records behind them. The report list includes completed comparisons, unsuccessful tests, study plans and observations."</p>
                <p class="prose-p">"Reports were reviewed on 11 September 2026 and cover experiments recorded through 7 September. Each entry identifies the type of record and its source. The "<a href="/results">"Results page"</a>" brings the measurements together with their methods and limitations."</p>
            </div>
        </section>

        <PageNav items=SECTIONS/>
        <section class="wrap section" aria-labelledby="paper-h">
            <div class="paper-callout">
                <p class="eyebrow">"Working paper · draft 0.4"</p>
                <h2 id="paper-h" class="display">"Dynamical Synthesis: Learning through Interaction"</h2>
                <p class="prose-p">"The paper sets out the computational framework and reviews the learning experiments, including improvements, failed tests and incomplete comparisons. Eight figures include the exchange plots, with measurement versions and observational limitations stated separately. The manuscript has not been peer reviewed."</p>
                <div class="paper-downloads">
                    <a class="btn btn-primary" href="/papers/dynamical-synthesis.pdf" rel="external">"Read the paper"</a>
                    <a class="btn" href="/papers/dynamical-synthesis-source.zip" download="dynamical-synthesis-source.zip">"Download LaTeX and analysis code"</a>
                    <a class="btn" href="/data/research-record.json" download="research-record.json">"Download summary data · JSON"</a>
                </div>
                <p class="prose-p">"The source bundle contains the editable manuscript, references, figure-generation code, summary measurements and an audit of the supporting evidence. Detailed experiment records are available on request."</p>
                <details class="study-details">
                    <summary>"What the archive contains"</summary>
                    <div class="study-details-content prose">
                        <p>"The archive inventory covers 23,099 files from available experiment directories, reports, laboratory code and seven separate experiment checkouts. Copies and repeated artifacts are included, so this is a file count rather than a count of observations."</p>
                        <p>"The catalogue includes failed, invalid and incomplete attempts. It records gaps and exclusions and identifies linked dependencies. It describes the files available when the archive was assembled; it cannot recover missing historical records."</p>
                        <p><a href="/data/data-inventory.json" download="data-inventory.json">"Download the archive inventory and its limits"</a>"."</p>
                    </div>
                </details>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="reports-h">
            <div class="section-head">
                <p class="eyebrow">"Study history"</p>
                <h2 id="reports-h" class="display">"Selected reports, newest first"</h2>
                <p class="lede-sm">"Comparison reports give measured outcomes. Plans describe intended tests, and observations document what happened without necessarily isolating a cause. An incomplete study has no final result for its planned comparison."</p>
            </div>
            <div class="rulings rulings-full">
                {RULINGS.iter().map(|r| view! {
                    <article class="ruling">
                        <time class="mono meta" datetime=r.date>{r.date}</time>
                        <div class="ruling-body">
                            <p class="report-kind">{r.kind}</p>
                            <h3>{r.title}</h3>
                            <p>{r.result}</p>
                            <details class="report-source">
                                <summary>"Source file and verification"</summary>
                                <p class="mono small">{r.path}</p>
                                {r.hash.map(|h| view! { <p class="mono small muted">{format!("Source file SHA-256: {h}")}</p> })}
                            </details>
                        </div>
                    </article>
                }).collect_view()}
            </div>
        </section>

        <section class="wrap section" aria-labelledby="where-h">
            <div class="section-head">
                <p class="eyebrow">"Evidence access"</p>
                <h2 id="where-h" class="display">"Inspect the records behind a result"</h2>
            </div>
            <div class="cols-2">
                <div>
                    <h3 class="track-h">"Available material"</h3>
                    {match REPOSITORY_URL {
                        Some(url) => view! { <p class="prose-p">"The paper, figures and summary data are linked above. The research repository is also "<a href=url>"available online"</a>"."</p> }.into_any(),
                        None => view! { <p class="prose-p">"The paper, figures, analysis code and summary data are downloadable above. The research repository and detailed experiment records remain private and can be requested for review."</p> }.into_any(),
                    }}
                    <p class="prose-p">"To examine a study, request its protocol, model and adapter versions, questions, replies, scoring code, review decisions and final result. Give the report date, title and source path when you "
                        {match CONTACT_EMAIL {
                            Some(mail) => view! { <a href=format!("mailto:{mail}")>"contact the research team"</a> }.into_any(),
                            None => view! { <a href=CONTACT_URL>"contact the research team"</a> }.into_any(),
                        }}
                    "."</p>
                </div>
                <div>
                    <h3 class="track-h">"Paths and file checks"</h3>
                    <p class="prose-p">"Source paths identify files in the associated research checkout. Reports are stored in "<code>{ALBUM_PATH}</code>" and observations are indexed in "<code>{LAWS_PATH}</code>". A path beginning with a separate experiment directory identifies that study's isolated checkout."</p>
                    <p class="prose-p">"Where a SHA-256 checksum is shown, it applies to the named source file. A matching checksum confirms that a supplied copy has the same bytes. Assessing the result also requires inspecting the methods and observations; recalculating summary statistics does not replace an independent repetition of the experiment."</p>
                </div>
            </div>
        </section>
    }
}

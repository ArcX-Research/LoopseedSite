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
        <div class="record-page research-page">
        <section class="page-intro page-intro-record">
            <div class="wrap">
                <p class="eyebrow">"Reports"</p>
                <h1 class="display display-xl">"Study reports and supporting evidence"</h1>
                <p class="lede">"The reports document individual experiments, from study preparation to measured outcomes. Each entry includes a summary and source record, with the paper and supporting data available below."</p>
                <p class="prose-p">"Reviewed on 11 September 2026, this collection covers experiments recorded through 7 September. The "<a href="/results">"Results page"</a>" compares their findings and explains the limits of each study."</p>
            </div>
        </section>

        <div class="research-layout">
        <PageNav items=SECTIONS show_label=false sidebar=true/>
        <div class="research-sections">
        <section class="wrap section" aria-labelledby="paper-h">
            <div class="section-head">
                <p class="eyebrow">"Research paper · 11 September 2026"</p>
                <h2 id="paper-h" class="display">"Dynamical Synthesis: Learning through Interaction"</h2>
                <p class="lede-sm">"The paper presents the computational framework and reviews the learning experiments. Its eight figures include the exchange plots, keeping different scoring versions separate. It has not been peer reviewed."</p>
            </div>
                <div class="paper-downloads">
                    <a class="btn btn-primary" href="/papers/dynamical-synthesis.html" rel="external">"Read the paper"</a>
                    <a class="btn" href="/papers/dynamical-synthesis-source.zip" download="dynamical-synthesis-source.zip">"Paper sources and figure code"</a>
                    <a class="btn" href="/data/research-record.json" download="research-record.json">"Download summary data · JSON"</a>
                    <a class="btn" href="/papers/dynamical-synthesis-mathematics-evidence.zip" download="dynamical-synthesis-mathematics-evidence.zip">"Download mathematics evidence"</a>
                </div>
                <p class="prose-p">"The source bundle includes the editable manuscript, references, figure code and summary measurements. The mathematics package contains 1,024 recorded replies, 155 AI review decisions, task specifications, training splits and source code, with a script to check the reported counts. The reviewer model version was not recorded."</p>
                <p class="prose-p"><a href="/papers/dynamical-synthesis.html#citation">"Citation and reference downloads"</a></p>
                <details class="study-details">
                    <summary>"What the archive contains"</summary>
                    <div class="study-details-content prose">
                        <p>"The inventory lists 23,099 files from experiment directories, reports, laboratory code and seven separate experiment checkouts. This includes repeated files and copies, so the total is not a sample size. Some listed records are available only on request."</p>
                        <p>"Failed, invalid and incomplete attempts are included, with gaps, exclusions and linked dependencies identified. The inventory describes the files available when it was assembled."</p>
                        <p><a href="/data/data-inventory.json" download="data-inventory.json">"Download the archive inventory and its limits"</a>"."</p>
                    </div>
                </details>
        </section>

        <section class="wrap section" aria-labelledby="reports-h">
            <div class="section-head">
                <p class="eyebrow">"Study history"</p>
                <h2 id="reports-h" class="display">"Study reports"</h2>
                <p class="lede-sm">"Entries are listed newest first and labelled by record type. Completed comparisons report measured outcomes; plans and observations provide context. Incomplete studies have no final result for their planned comparison."</p>
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
                <h2 id="where-h" class="display">"Source records and access"</h2>
            </div>
                    <h3 class="figure-h">"Additional study records"</h3>
                    {match REPOSITORY_URL {
                        Some(url) => view! { <p class="prose-p">"Additional source records are in the "<a href=url>"research repository"</a>"."</p> }.into_any(),
                        None => view! { <p class="prose-p">"The complete repository and original records for studies outside the mathematics package remain private and can be requested for review."</p> }.into_any(),
                    }}
                    <p class="prose-p">"A review may require the protocol, model and adapter versions, questions, replies, scoring code, review decisions and final result. Include the report date, title and source path when you "
                        {match CONTACT_EMAIL {
                            Some(mail) => view! { <a href=format!("mailto:{mail}")>"contact the research team"</a> }.into_any(),
                            None => view! { <a href=CONTACT_URL>"contact the research team"</a> }.into_any(),
                        }}
                    "."</p>
                    <h3 class="figure-h">"Source paths and file checks"</h3>
                    <p class="prose-p">"Reports are stored in "<code>{ALBUM_PATH}</code>" and observations are indexed in "<code>{LAWS_PATH}</code>". Paths beginning with a separate experiment directory refer to that study’s isolated copy of the code and records."</p>
                    <p class="prose-p">"A matching SHA-256 checksum confirms that a supplied file is identical to the named source. It does not verify the study’s conclusions. That requires assessment of the methods and observations, with independent replication beyond recalculation of saved counts."</p>
        </section>
        </div>
        </div>
        </div>
    }
}

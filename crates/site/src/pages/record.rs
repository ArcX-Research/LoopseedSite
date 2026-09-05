//! Research reports and source records.
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::rulings::RULINGS;
use loopseed_record::{ALBUM_PATH, CONTACT_EMAIL, CONTACT_URL, LAWS_PATH, REPOSITORY_URL};

#[component]
pub fn Record() -> impl IntoView {
    set_title("Record");
    view! {
        <section class="wrap page">
            <p class="eyebrow">"Record"</p>
            <h1 class="display display-xl">"Research reports, newest first"</h1>
            <p class="lede">"The project archive contains figures, transcripts and experimental decisions. Each report identifies its run, file hashes and authorised next steps. Failed results are preserved without being overwritten or rescored. The summaries below use plain language; the source files hold the original records."</p>
        </section>

        <section class="wrap section">
            <div class="rulings rulings-full">
                {RULINGS.iter().map(|r| view! {
                    <article class="ruling">
                        <span class="mono meta">{r.date}</span>
                        <h3>{r.title}</h3>
                        <p>{r.result}</p>
                        <p class="mono small">{r.path}</p>
                        {r.hash.map(|h| view! { <p class="mono small muted">{format!("verdict sha256 {h}")}</p> })}
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
                    <p class="prose-p">"File paths on this site start at the root of the loopseed repository. Research reports are in "<code>{ALBUM_PATH}</code>"; recorded observations are in "<code>{LAWS_PATH}</code>". Each experiment's directory holds its protocol, coded condition key, saved results, review materials and final decision. File hashes allow readers to check that these records have not changed."</p>
                    {match REPOSITORY_URL {
                        Some(url) => view! { <p class="prose-p">"The repository is public: "<a href=url>{url}</a>"."</p> }.into_any(),
                        None => view! { <p class="prose-p">"The repository is private while the study runs. Evidence files are available on request, and every hash on this site can be checked against them."</p> }.into_any(),
                    }}
                </div>
                <div>
                    <p class="prose-p">"To request evidence, name the run and the file. "
                        {match CONTACT_EMAIL {
                            Some(mail) => view! { <a href=format!("mailto:{mail}")>{mail}</a> }.into_any(),
                            None => view! { <a href=CONTACT_URL>"Contact Dilate Technologies"</a> }.into_any(),
                        }}
                    "."</p>
                </div>
            </div>
        </section>
    }
}

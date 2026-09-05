//! The record: dated rulings with their paths and hashes.
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::rulings::RULINGS;
use loopseed_record::{ALBUM_PATH, CONTACT_EMAIL, LAWS_PATH, ORGANISATION_URL, REPOSITORY_URL};

#[component]
pub fn Record() -> impl IntoView {
    set_title("Record");
    view! {
        <section class="wrap page">
            <p class="eyebrow">"Record"</p>
            <h1 class="display display-xl">"Rulings, newest first."</h1>
            <p class="lede">"The album is the family record: photographs, transcripts and rulings, versioned and out of reach of overwrites. A ruling names its run, its content hashes, what it authorized and what it did not. Failed records are neither overwritten nor rescored."</p>
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
                <h2 id="where-h" class="display">"Paths, not prose."</h2>
            </div>
            <div class="cols-2">
                <div>
                    <p class="prose-p">"Paths on this site are relative to the loopseed repository. The album is "<code>{ALBUM_PATH}</code>"; the laws file is "<code>{LAWS_PATH}</code>". Each experiment's run directory holds its protocol, sealed arm key, write-once checkpoints, audit bundle, imported audit and verdict, all content-hashed."</p>
                    {match REPOSITORY_URL {
                        Some(url) => view! { <p class="prose-p">"The repository is public: "<a href=url>{url}</a>"."</p> }.into_any(),
                        None => view! { <p class="prose-p">"The repository is private while the study runs. Evidence files are available on request, and every hash on this site can be checked against them."</p> }.into_any(),
                    }}
                </div>
                <div>
                    <p class="prose-p">"To request evidence, name the run and the file. "
                        {match CONTACT_EMAIL {
                            Some(mail) => view! { <a href=format!("mailto:{mail}")>{mail}</a> }.into_any(),
                            None => view! { <a href=ORGANISATION_URL>"Contact Dilate Technologies"</a> }.into_any(),
                        }}
                    "."</p>
                </div>
            </div>
        </section>
    }
}

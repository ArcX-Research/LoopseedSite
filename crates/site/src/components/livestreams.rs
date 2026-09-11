use leptos::prelude::*;
use loopseed_record::livestreams::{Livestream, FEATURED, PLAYLIST_URL};

#[component]
fn StreamCard(stream: &'static Livestream) -> impl IntoView {
    let (playing, set_playing) = signal(false);
    view! {
        <article class="stream-card">
            <div class="stream-player">
                <Show when=move || playing.get() fallback=move || view! {
                    <button class="stream-preview" type="button"
                        aria-label=format!("Play recording: {}", stream.title)
                        on:click=move |_| set_playing.set(true)>
                        <img src=format!("/images/livestreams/{}.jpg", stream.video_id)
                            alt="" width="1280" height="720" loading="lazy" decoding="async"/>
                        <span class="stream-play" aria-hidden="true">"▶"</span>
                        <span class="stream-duration">{stream.duration}</span>
                    </button>
                }>
                    <iframe src=format!("https://www.youtube-nocookie.com/embed/{}?autoplay=1&rel=0", stream.video_id)
                        title=stream.title
                        allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture; web-share"
                        allowfullscreen
                        referrerpolicy="strict-origin-when-cross-origin"/>
                </Show>
            </div>
            <div class="stream-copy">
                <p class="eyebrow">{stream.theme}</p>
                <h3>{stream.title}</h3>
                <p>{stream.description}</p>
                <div class="stream-meta">
                    <time datetime=stream.datetime>{stream.date}</time>
                    <a href=format!("https://www.youtube.com/watch?v={}", stream.video_id)>"YouTube ↗"</a>
                </div>
            </div>
        </article>
    }
}

#[component]
pub fn FeaturedLivestreams() -> impl IntoView {
    view! {
        <section id="live-science" class="section section-feature" aria-labelledby="live-science-h">
            <div class="wrap">
                <div class="section-head">
                    <p class="eyebrow">"Live research and shared data"</p>
                    <h2 id="live-science-h" class="display">"Research in public"</h2>
                    <p class="lede-sm">"We livestream research sessions so anyone can follow the questions, code and experiments as we work. Watch the recordings below and examine the programme’s published results, data and analysis materials."</p>
                </div>
                <div class="stream-grid">
                    {FEATURED.iter().map(|stream| view! { <StreamCard stream/> }).collect_view()}
                </div>
                <div class="stream-footer">
                    <p>"Read the "<a href="/results">"research results"</a>" and inspect the "<a href="/record#paper-h">"published data, methods and source files"</a>"."</p>
                    <div class="stream-actions">
                        <a class="btn" href="/data/research-record.json" download="research-record.json">"Download summary data · JSON"</a>
                        <a class="btn" href=PLAYLIST_URL>"View all livestreams "<span aria-hidden="true">"↗"</span></a>
                    </div>
                </div>
            </div>
        </section>
    }
}

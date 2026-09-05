//! Routes and shared page layout.
use crate::components::mark::Mark;
use crate::pages::{
    method::Method, not_found::NotFound, overview::Overview, promise::Promise, record::Record,
    results::Results,
};
use crate::util::scroll_to_top;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;
use loopseed_record::{ORGANISATION, ORGANISATION_URL, REPOSITORY_URL};

pub const NAV: &[(&str, &str)] = &[
    ("/", "Overview"),
    ("/results", "Results"),
    ("/method", "Method"),
    ("/record", "Reports"),
    ("/promise", "Research goals"),
];

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <a class="skip-link" href="#main">"Skip to content"</a>
            <Header/>
            <main id="main" class="main" tabindex="-1">
                <Routes fallback=|| view! { <NotFound/> }>
                    <Route path=path!("/") view=Overview/>
                    <Route path=path!("/results") view=Results/>
                    <Route path=path!("/method") view=Method/>
                    <Route path=path!("/record") view=Record/>
                    <Route path=path!("/promise") view=Promise/>
                </Routes>
            </main>
            <Footer/>
            <ScrollKeeper/>
        </Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    let pathname = use_location().pathname;
    view! {
        <header class="site-header">
            <div class="wrap header-row">
                <a class="brand" href="/" aria-label="Loopseed home">
                    <Mark/>
                    <span class="brand-name">"Loopseed"</span>
                </a>
                <nav class="nav" aria-label="Primary">
                    {NAV.iter().map(|(href, label)| {
                        let href = *href;
                        let label = *label;
                        let current = move || {
                            let path = pathname.get();
                            if href == "/" { path == "/" } else { path.starts_with(href) }
                        };
                        view! { <a href=href aria-current=move || current().then_some("page")>{label}</a> }
                    }).collect_view()}
                </nav>
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="wrap footer-grid">
                <div class="footer-brand">
                    <Mark/>
                    <div>
                        <div class="brand-name">"Loopseed"</div>
                        <p class="footer-tag">"Research on systems that learn from interaction, by " <a href=ORGANISATION_URL>{ORGANISATION}</a> "."</p>
                    </div>
                </div>
                <div class="footer-cols">
                    <div>
                        <div class="footer-h">"Read"</div>
                        {NAV.iter().map(|(href, label)| view! { <a href=*href>{*label}</a> }).collect_view()}
                    </div>
                    <div>
                        <div class="footer-h">"Evidence"</div>
                        {match REPOSITORY_URL {
                            Some(url) => view! { <a href=url>"Repository"</a> }.into_any(),
                            None => view! { <span class="footer-muted">"Repository private during the study; evidence on request"</span> }.into_any(),
                        }}
                        <a href="/record">"Reports and source records"</a>
                    </div>
                    <div>
                        <div class="footer-h">"This site"</div>
                        <div class="footer-meta mono">
                            <div>"Rust · Leptos · WebAssembly"</div>
                            <div>"statistics recomputed in your browser"</div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="wrap footer-line mono">
                <span>{format!("© {ORGANISATION} · Loopseed")}</span>
                <span>"No tracking · light theme only"</span>
            </div>
        </footer>
    }
}

/// Scroll to the fragment, or the top when none is present.
#[component]
fn ScrollKeeper() -> impl IntoView {
    let location = use_location();
    let pathname = location.pathname;
    let hash = location.hash;
    Effect::new(move |_| {
        pathname.track();
        let fragment = hash.get();
        let id = fragment.trim_start_matches('#').to_string();
        if id.is_empty() {
            scroll_to_top();
            return;
        }
        fn attempt(id: String, tries_left: u32) {
            match document().get_element_by_id(&id) {
                Some(element) => element.scroll_into_view(),
                None if tries_left > 0 => set_timeout(
                    move || attempt(id, tries_left - 1),
                    std::time::Duration::from_millis(80),
                ),
                None => {}
            }
        }
        attempt(id, 12);
    });
    view! { <></> }
}

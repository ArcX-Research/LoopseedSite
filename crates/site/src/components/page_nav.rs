//! Section links for long research pages.
use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn PageNav(items: &'static [(&'static str, &'static str)]) -> impl IntoView {
    let hash = use_location().hash;
    view! {
        <nav class="page-nav" aria-label="On this page">
            <div class="wrap page-nav-inner">
                <span class="page-nav-label">"On this page"</span>
                <div class="page-nav-links">
                    {items.iter().map(|(target, label)| {
                        let target = *target;
                        view! {
                            <a href=target aria-current=move || (hash.get() == target).then_some("location")>{*label}</a>
                        }
                    }).collect_view()}
                </div>
            </div>
        </nav>
    }
}

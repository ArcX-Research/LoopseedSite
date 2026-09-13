//! Section links for long research pages.
use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn PageNav(
    items: &'static [(&'static str, &'static str)],
    #[prop(default = true)] show_label: bool,
    #[prop(default = false)] sidebar: bool,
) -> impl IntoView {
    let hash = use_location().hash;
    view! {
        <nav class=if sidebar { "page-nav page-nav-sidebar" } else { "page-nav" } aria-label="On this page">
            <div class="wrap page-nav-inner">
                {show_label.then(|| view! { <span class="page-nav-label">"On this page"</span> })}
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

//! The mark: a ring with one point on it, the loop and the one word from outside.
use leptos::prelude::*;

pub const MARK: &str = r##"<svg width="26" height="26" viewBox="0 0 32 32" fill="none" aria-hidden="true">
<circle cx="16" cy="16" r="11" stroke="#000020" stroke-width="1.4"/>
<circle cx="16" cy="5" r="2.6" fill="#4946ff"/>
<circle cx="16" cy="16" r="2.2" fill="#000020"/>
</svg>"##;

#[component]
pub fn Mark() -> impl IntoView {
    view! { <span class="mark" inner_html=MARK></span> }
}

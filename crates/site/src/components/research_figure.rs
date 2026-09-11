use leptos::prelude::*;

/// The same vector figures used in the paper, with accessible text equivalents.
#[component]
pub fn ResearchFigure(
    name: &'static str,
    alt: &'static str,
    caption: &'static str,
) -> impl IntoView {
    view! {
        <figure class="figure research-figure">
            <p class="chart-scroll-hint">"Scroll within the figure to read the full chart on a small screen."</p>
            <div class="research-figure-body" role="region" aria-label=alt tabindex="0">
                <img src=format!("/figures/dynamical-synthesis/{name}.svg") alt=alt loading="lazy" decoding="async"/>
            </div>
            <figcaption class="caption">{caption}</figcaption>
        </figure>
    }
}

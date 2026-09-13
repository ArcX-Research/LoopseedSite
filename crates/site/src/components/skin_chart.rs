//! Exchange figures share the paper's generated vector assets and frozen data.
use crate::components::research_figure::ResearchFigure;
use leptos::prelude::*;

#[component]
pub fn SkinChart() -> impl IntoView {
    view! {
        <ResearchFigure
            name="interaction-participant"
            alt="Prediction error for one participant in two panels: 652 version-1 scores in 15 sessions, and 65 version-2 scores in seven sessions."
            caption="Figure 6. Dots show individual exchanges; outlined markers show session medians at session start. All sessions are included, with a new session after a gap of at least 30 minutes within each score version. Lines stop across missing observation dates. Shading marks initial setup. The dashed line marks the reference memory threshold, θₘ = 0.35; it does not track historical changes to that threshold."
        />
    }
}

#[component]
pub fn GuestChart() -> impl IntoView {
    view! {
        <h4 class="figure-h">"AI teachers and external text"</h4>
        <ResearchFigure
            name="interaction-sources"
            alt="Three panels separate AI-teacher scores by measurement version and external-text scores: 3,179 teacher scores under version 1, 11,506 under version 2, and 2,708 external-text scores under version 2."
            caption="Figure 7. Dots show session medians for each source; outlined markers show pooled daily medians at midday UTC. Bands cover the middle 50% of scores and are not confidence intervals. Daily summaries weight each score equally. Lines and bands connect consecutive recorded dates within each score version. The dashed line marks the reference threshold, θₘ = 0.35."
        />
        <h4 class="figure-h">"Nine automated teaching programmes"</h4>
        <ResearchFigure
            name="interaction-programmes"
            alt="Nine separate mathematics programme panels containing 1,953 version-2 scores in 79 sessions. Each panel states the task, score count and session count."
            caption="Figure 8. Dots show session medians; outlined markers show daily medians. Each programme has a separate panel using score version 2. Lines connect consecutive recorded dates, leaving the gap before 31 August unconnected. The dashed line marks the reference threshold, θₘ = 0.35."
        />
        <p class="caption">
            "The other-source figures include 19,346 scores; 14 entries under another guest identifier are outside these groups. All three exchange figures use the paper’s vector assets and the data export saved on 5 September 2026. "
            <a href="/data/exchange-scores.json" download>"Download the exchange data"</a>
            " · "<a href="/data/exchange-audit.json" download>"Read the data and plotting audit"</a>
            " · "<a href="/papers/dynamical-synthesis.html" rel="external">"Figures 6–8 in the paper"</a>
        </p>
    }
}

//! Exchange figures share the paper's generated vector assets and frozen data.
use crate::components::research_figure::ResearchFigure;
use leptos::prelude::*;

#[component]
pub fn SkinChart() -> impl IntoView {
    view! {
        <ResearchFigure
            name="interaction-participant"
            alt="Prediction error for one participant in two panels: 652 version-1 scores in 15 sessions, and 65 version-2 scores in seven sessions."
            caption="Figure 6 in the paper. Dots are individual scores; outlined markers are session medians at session start. A gap of at least 30 minutes starts a session within each score version. Lines stop across missing observation dates. Shading marks the initial setup period. The dashed line is the reference memory threshold, θₘ = 0.35, rather than its complete historical record."
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
            caption="Figure 7 in the paper. Dots are session medians for each source; outlined markers are pooled daily medians at midday UTC. Bands span the first and third quartiles, describing the middle 50% of scores, not confidence intervals. Daily summaries give each score equal weight. Lines and bands connect consecutive recorded dates only. The dashed line marks the reference threshold, θₘ = 0.35."
        />
        <h4 class="figure-h">"Nine automated teaching programmes"</h4>
        <ResearchFigure
            name="interaction-programmes"
            alt="Nine separate mathematics programme panels containing 1,953 version-2 scores in 79 sessions. Each panel states the task, score count and session count."
            caption="Figure 8 in the paper. Dots are session medians; outlined markers are daily medians. Each programme is shown separately, using version 2 throughout. Lines connect consecutive recorded dates only; no line is drawn through the gap before 31 August. The dashed line marks the reference threshold, θₘ = 0.35. Different tasks and settings prevent ranking the programmes’ learning effectiveness from these curves."
        />
        <p class="caption">
            "These figures contain 19,346 scores from other input sources. Fourteen entries from another guest identifier are outside these groups. The website and paper use the same generated figures and the same numeric export, saved on 5 September 2026. "
            <a href="/data/exchange-scores.json" download>"Download the exchange data"</a>
            " · "<a href="/data/exchange-audit.json" download>"Read the data and plotting audit"</a>
            " · "<a href="/papers/dynamical-synthesis.html" rel="external">"Figures 6–8 in the paper"</a>
        </p>
    }
}

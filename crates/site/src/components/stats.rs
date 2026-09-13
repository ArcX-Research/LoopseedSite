//! Recorded statistics, browser recalculations and an interactive comparison.
use crate::components::tables::ScrollTable;
use leptos::prelude::*;
use loopseed_record::experiments::Comparison;
use loopseed_record::stats::{paired, scientific, signed};

#[component]
pub fn PairedTable(comparisons: &'static [Comparison]) -> impl IntoView {
    view! {
        <ScrollTable label="Paired comparisons and recalculated statistics">
            <table class="table paired-table">
                <thead>
                    <tr>
                        <th>"Comparison"</th>
                        <th class="num">"Matched tasks"</th>
                        <th class="num">"Gains"</th>
                        <th class="num">"Losses"</th>
                        <th class="num">"One-sided exact p-value"</th>
                        <th class="num">"95% lower bound on success-rate difference"</th>
                    </tr>
                </thead>
                <tbody>
                    {comparisons.iter().map(|c| {
                        let r = paired(c.gains.into(), c.losses.into(), c.n.into());
                        view! {
                            <tr>
                                <td>{c.label}</td>
                                <td class="num">{c.n}</td>
                                <td class="num">{c.gains}</td>
                                <td class="num">{c.losses}</td>
                                <td class="num">{scientific(r.p_value)}</td>
                                <td class="num">{signed(r.lower_difference, 4)}</td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>
        </ScrollTable>
        <p class="caption">"Calculated from the recorded paired outcomes. "<a href="#explorer-h">"Definitions, methods and assumptions"</a>"."</p>
    }
}

fn parse_count(value: String) -> u64 {
    value.trim().parse().unwrap_or(0)
}

#[component]
pub fn Explorer() -> impl IntoView {
    let gains = RwSignal::new(21u64);
    let losses = RwSignal::new(0u64);
    let n = RwSignal::new(128u64);
    let result = move || paired(gains.get(), losses.get(), n.get());
    view! {
        <div class="explorer">
            <div class="explorer-inputs">
                <label>"Gains: only the update passed"<input type="number" min="0" prop:value=move || gains.get().to_string() on:input=move |ev| gains.set(parse_count(event_target_value(&ev)))/></label>
                <label>"Losses: only the control passed"<input type="number" min="0" prop:value=move || losses.get().to_string() on:input=move |ev| losses.set(parse_count(event_target_value(&ev)))/></label>
                <label>"Total matched tasks, including ties"<input type="number" min="1" prop:value=move || n.get().to_string() on:input=move |ev| n.set(parse_count(event_target_value(&ev)))/></label>
            </div>
            <dl class="explorer-out mono" aria-live="polite" aria-atomic="true">
                <div><dt>"One-sided exact McNemar p-value"</dt><dd>{move || scientific(result().p_value)}</dd></div>
                <div><dt>"95% lower bound on the success-rate difference"</dt><dd>{move || signed(result().lower_difference, 4)}</dd></div>
                <div><dt>"Observed success-rate difference"</dt><dd>{move || signed(result().observed_difference, 4)}</dd></div>
                <div><dt>"Statistical criterion only"</dt><dd><span class=move || if result().superior() { "status-badge status-pass" } else { "status-badge status-fail" }>{move || if result().superior() { "Met" } else { "Not met" }}</span></dd></div>
            </dl>
        </div>
    }
}

//! Recorded statistics, browser recalculations and an interactive comparison.
use leptos::prelude::*;
use loopseed_record::experiments::Comparison;
use loopseed_record::stats::{paired, scientific, signed};

#[component]
pub fn PairedTable(comparisons: &'static [Comparison]) -> impl IntoView {
    view! {
        <div class="table-wrap">
            <table class="table">
                <thead>
                    <tr>
                        <th>"Comparison"</th>
                        <th class="num">"matched tasks"</th>
                        <th class="num">"gains"</th>
                        <th class="num">"losses"</th>
                        <th class="num">"one-sided exact p"</th>
                        <th class="num">"95 % lower bound on difference"</th>
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
                                <td class="num">
                                    <span class="frozen">{scientific(c.frozen_p)}</span>
                                    <span class="recomputed">{format!("recalculated {}", scientific(r.p_value))}</span>
                                </td>
                                <td class="num">
                                    <span class="frozen">{signed(c.frozen_lower, 4)}</span>
                                    <span class="recomputed">{format!("recalculated {}", signed(r.lower_difference, 4))}</span>
                                </td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>
        </div>
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
                <label>"gains"<input type="number" min="0" prop:value=move || gains.get().to_string() on:input=move |ev| gains.set(parse_count(event_target_value(&ev)))/></label>
                <label>"losses"<input type="number" min="0" prop:value=move || losses.get().to_string() on:input=move |ev| losses.set(parse_count(event_target_value(&ev)))/></label>
                <label>"matched tasks"<input type="number" min="1" prop:value=move || n.get().to_string() on:input=move |ev| n.set(parse_count(event_target_value(&ev)))/></label>
            </div>
            <dl class="explorer-out mono">
                <div><dt>"one-sided exact McNemar p"</dt><dd>{move || scientific(result().p_value)}</dd></div>
                <div><dt>"95 % lower bound on the success-rate difference"</dt><dd>{move || signed(result().lower_difference, 4)}</dd></div>
                <div><dt>"observed success-rate difference"</dt><dd>{move || signed(result().observed_difference, 4)}</dd></div>
                <div><dt>"statistical criterion"</dt><dd>{move || if result().superior() { "passes" } else { "fails" }}</dd></div>
            </dl>
        </div>
    }
}

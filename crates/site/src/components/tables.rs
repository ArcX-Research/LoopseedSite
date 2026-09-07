//! Experiment conditions, pass criteria and file hashes.
use leptos::prelude::*;
use loopseed_record::experiments::{Arm, Gate, Hash};

#[component]
pub fn ScrollTable(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="table-wrap" tabindex="0" role="region" aria-label=label>
            {children()}
        </div>
    }
}

#[component]
pub fn ArmTable(arms: &'static [Arm]) -> impl IntoView {
    view! {
        <ScrollTable label="Accepted answers by experimental condition">
            <table class="table arm-table">
                <caption class="sr-only">"Accepted answers and task-specific checks for negative constants"</caption>
                <thead>
                    <tr>
                        <th scope="col">"Condition"</th>
                        <th scope="col" class="num">"Accepted / tasks"</th>
                        <th scope="col" class="num">"Negative-constant replies"</th>
                    </tr>
                </thead>
                <tbody>
                    {arms.iter().map(|a| view! {
                        <tr>
                            <th scope="row" class="arm-name">
                                <span>{a.name}</span>
                                {(!a.note.is_empty()).then(|| view! { <span class="arm-note">{a.note}</span> })}
                            </th>
                            <td class="num score-cell">
                                <div class="score-number"><b>{a.successes}</b><span>{format!(" / {}", a.tasks)}</span></div>
                                <div class="score-track" aria-hidden="true"><span style:width=format!("{}%", 100.0 * f64::from(a.successes) / f64::from(a.tasks.max(1)))></span></div>
                            </td>
                            <td class="num">{a.negative_literal_cells}</td>
                        </tr>
                    }).collect_view()}
                </tbody>
            </table>
        </ScrollTable>
    }
}

#[component]
pub fn GateList(gates: &'static [Gate]) -> impl IntoView {
    view! {
        <ul class="gates">
            {gates.iter().map(|g| view! {
                <li class=if g.passed { "gate gate-pass" } else { "gate gate-fail" }>
                    <span class="gate-mark mono">{if g.passed { "pass" } else { "fail" }}</span>
                    <span>{g.name}</span>
                </li>
            }).collect_view()}
        </ul>
    }
}

#[component]
pub fn HashList(hashes: &'static [Hash]) -> impl IntoView {
    view! {
        <details class="hashes">
            <summary>{format!("{} file hashes", hashes.len())}</summary>
            <dl>
                {hashes.iter().map(|h| view! {
                    <div><dt>{h.label}</dt><dd class="mono">{h.value}</dd></div>
                }).collect_view()}
            </dl>
        </details>
    }
}

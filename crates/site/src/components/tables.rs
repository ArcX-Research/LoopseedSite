//! Experiment conditions, pass criteria and file hashes.
use leptos::prelude::*;
use loopseed_record::experiments::{Arm, Gate, Hash};

#[component]
pub fn ArmTable(arms: &'static [Arm]) -> impl IntoView {
    let notes = arms.iter().any(|a| !a.note.is_empty());
    view! {
        <div class="table-wrap">
            <table class="table">
                <thead>
                    <tr>
                        <th>"Condition"</th>
                        <th class="num">"accepted answers"</th>
                        <th class="num">"tasks"</th>
                        <th class="num">"replies with negative constants"</th>
                        {notes.then(|| view! { <th>"Note"</th> })}
                    </tr>
                </thead>
                <tbody>
                    {arms.iter().map(|a| view! {
                        <tr>
                            <td>{a.name}</td>
                            <td class="num">{a.successes}</td>
                            <td class="num">{a.tasks}</td>
                            <td class="num">{a.negative_literal_cells}</td>
                            {notes.then(|| view! { <td class="muted">{a.note}</td> })}
                        </tr>
                    }).collect_view()}
                </tbody>
            </table>
        </div>
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

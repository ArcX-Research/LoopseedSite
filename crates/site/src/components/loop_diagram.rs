//! Five-step learning cycle, rendered as SVG.
use leptos::prelude::*;
use std::f64::consts::PI;
use std::fmt::Write;

pub struct Step {
    pub ordinal: &'static str,
    pub label: &'static str,
    pub organ: &'static str,
    pub gloss: &'static str,
}

pub const STEPS: &[Step] = &[
    Step { ordinal: "01", label: "Predict", organ: "Ŵ → Î", gloss: "Record a prediction of the next external input before observing it." },
    Step { ordinal: "02", label: "Compare", organ: "δ = I − Î", gloss: "Compare the observation with the prediction. Track the error score, δ, over time." },
    Step { ordinal: "03", label: "Remember", organ: "σ, M", gloss: "Select experiences for memory when prediction error exceeds the threshold θ. Reduce the weight of unused records over time." },
    Step { ordinal: "04", label: "Act", organ: "π, V", gloss: "Choose actions using the current state. Balance prediction accuracy with the variety of external input." },
    Step { ordinal: "05", label: "Update", organ: "λ, η", gloss: "Use selected experience to update the system. Test proposed changes before adopting them." },
];

pub fn loop_svg() -> String {
    let size = 560.0;
    let c = size / 2.0;
    let ring = 200.0;
    let node = 48.0;
    let mut s = String::with_capacity(8_000);
    let _ = write!(
        s,
        r##"<svg class="loop" viewBox="0 0 {size} {size}" role="img" aria-label="The Loopseed cycle: predict, compare, remember, act, update">"##
    );
    // Background grid.
    let _ = write!(
        s,
        r##"<g stroke="#000020" stroke-opacity="0.06" stroke-width="1">"##
    );
    let mut x = 40.0;
    while x < size {
        let _ = write!(s, r##"<path d="M{x} 0V{size}"/><path d="M0 {x}H{size}"/>"##);
        x += 40.0;
    }
    s.push_str("</g>");
    // The ring.
    let _ = write!(
        s,
        r##"<circle cx="{c}" cy="{c}" r="{ring}" fill="none" stroke="#000020" stroke-opacity="0.22" stroke-width="1.2"/>"##
    );
    // Direction chevrons between nodes.
    for index in 0..STEPS.len() {
        let angle = -PI / 2.0 + (index as f64 + 0.5) * 2.0 * PI / STEPS.len() as f64;
        let (x, y) = (c + ring * angle.cos(), c + ring * angle.sin());
        let heading = angle.to_degrees() + 90.0;
        let _ = write!(
            s,
            r##"<path d="M-5 -4.5L1 0L-5 4.5" transform="translate({x:.1} {y:.1}) rotate({heading:.1})" fill="none" stroke="#4946ff" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/>"##
        );
    }
    // Centre mark.
    let _ = write!(
        s,
        r##"<circle cx="{c}" cy="{c}" r="74" fill="#ffffff" stroke="#000020" stroke-opacity="0.3" stroke-width="1.2"/>"##
    );
    let _ = write!(
        s,
        r##"<g stroke="#4946ff" stroke-width="1.4" stroke-linecap="round">"##
    );
    for tick in 0..24 {
        let angle = tick as f64 * 2.0 * PI / 24.0;
        let (r1, r2) = if tick % 2 == 0 {
            (8.0, 17.0)
        } else {
            (11.0, 15.0)
        };
        let (cy, cx) = (c - 18.0, c);
        let _ = write!(
            s,
            r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}"/>"##,
            cx + r1 * angle.cos(),
            cy + r1 * angle.sin(),
            cx + r2 * angle.cos(),
            cy + r2 * angle.sin()
        );
    }
    s.push_str("</g>");
    let _ = write!(
        s,
        r##"<text x="{c}" y="{:.1}" text-anchor="middle" font-family="Geist, Inter, system-ui, sans-serif" font-size="16" font-weight="600" letter-spacing="-0.02em" fill="#000020">Loopseed</text>"##,
        c + 22.0
    );
    let _ = write!(
        s,
        r##"<text x="{c}" y="{:.1}" text-anchor="middle" font-family="Adamina, Georgia, serif" font-size="14" fill="#000020" fill-opacity="0.7">I = W(I) + you</text>"##,
        c + 44.0
    );
    // The five stages.
    for (index, step) in STEPS.iter().enumerate() {
        let angle = -PI / 2.0 + index as f64 * 2.0 * PI / STEPS.len() as f64;
        let (x, y) = (c + ring * angle.cos(), c + ring * angle.sin());
        let first = index == 0;
        let stroke = if first { "#4946ff" } else { "#000020" };
        let opacity = if first { "1" } else { "0.3" };
        let _ = write!(
            s,
            r##"<g transform="translate({x:.1} {y:.1})"><circle r="{node}" fill="#ffffff" stroke="{stroke}" stroke-opacity="{opacity}" stroke-width="1.4"/>"##
        );
        if first {
            let _ = write!(
                s,
                r##"<circle r="{:.1}" fill="none" stroke="#4946ff" stroke-opacity="0.25" stroke-width="1"/>"##,
                node + 7.0
            );
        }
        let _ = write!(
            s,
            r##"<text y="-14" text-anchor="middle" font-family="DM Mono, Menlo, monospace" font-size="11" fill="#000020" fill-opacity="0.55">{}</text>"##,
            step.ordinal
        );
        let _ = write!(
            s,
            r##"<text y="6" text-anchor="middle" font-family="Geist, Inter, system-ui, sans-serif" font-size="15.5" font-weight="500" letter-spacing="-0.01em" fill="#000020">{}</text>"##,
            step.label
        );
        let _ = write!(
            s,
            r##"<text y="24" text-anchor="middle" font-family="DM Mono, Menlo, monospace" font-size="10.5" fill="#4946ff">{}</text></g>"##,
            step.organ
        );
    }
    s.push_str("</svg>");
    s
}

#[component]
pub fn LoopDiagram() -> impl IntoView {
    let svg = loop_svg();
    view! { <div class="loop-wrap" inner_html=svg></div> }
}

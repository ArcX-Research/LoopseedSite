//! The skin: the keeper's δ per sitting, drawn from the exported record. Pure SVG generated in
//! the browser; native `<title>` tooltips, no scripting beyond the toggle.
use leptos::prelude::*;
use loopseed_record::skin::{self, Skin};
use std::fmt::Write;

const WIDTH: f64 = 960.0;
const HEIGHT: f64 = 380.0;
const LEFT: f64 = 52.0;
const RIGHT: f64 = 20.0;
const TOP: f64 = 18.0;
const BOTTOM: f64 = 44.0;
const Y_MAX: f64 = 0.9;
const DAY: i64 = 86_400;

const INK: &str = "#000020";
const V1: &str = "#4946ff";
const V2: &str = "#c98a00";

/// Civil date from unix seconds (UTC), by the days-from-civil inverse.
fn civil(unix: i64) -> (i64, u32, u32) {
    let days = unix.div_euclid(DAY);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

fn day_label(unix: i64) -> String {
    let (_, m, d) = civil(unix);
    format!("{} {}", MONTHS[(m - 1) as usize], d)
}

fn iso_date(unix: i64) -> String {
    let (y, m, d) = civil(unix);
    format!("{y:04}-{m:02}-{d:02}")
}

pub fn skin_svg(skin: &Skin, show_grades: bool) -> String {
    let t_min = skin.grades.iter().map(|g| g.t).min().unwrap_or(0);
    let t_max = skin.grades.iter().map(|g| g.t).max().unwrap_or(t_min + DAY);
    let x0 = t_min.div_euclid(DAY) * DAY;
    let x1 = (t_max.div_euclid(DAY) + 1) * DAY;
    let plot_w = WIDTH - LEFT - RIGHT;
    let plot_h = HEIGHT - TOP - BOTTOM;
    let x = |t: i64| LEFT + (t - x0) as f64 / (x1 - x0) as f64 * plot_w;
    let y = |d: f64| TOP + (1.0 - d.min(Y_MAX) / Y_MAX) * plot_h;

    let mut s = String::with_capacity(60_000);
    let _ = write!(
        s,
        r##"<svg class="skin" viewBox="0 0 {WIDTH} {HEIGHT}" role="img" aria-label="The keeper's delta per sitting, August 2026">"##
    );
    // Instrumentation evening, shaded.
    let inst_end = skin.instrumentation_end_unix;
    if inst_end > x0 {
        let _ = write!(
            s,
            r##"<rect x="{LEFT}" y="{TOP}" width="{:.1}" height="{plot_h}" fill="{INK}" fill-opacity="0.035"/>"##,
            x(inst_end) - LEFT
        );
        let _ = write!(
            s,
            r##"<text x="{:.1}" y="{:.1}" font-family="DM Mono, Menlo, monospace" font-size="10.5" fill="{INK}" fill-opacity="0.5">instrumentation</text>"##,
            LEFT + 6.0,
            TOP + 14.0
        );
    }
    // Horizontal grid and y labels.
    let mut level = 0.0;
    while level <= Y_MAX + 1e-9 {
        let yy = y(level);
        let _ = write!(
            s,
            r##"<line x1="{LEFT}" y1="{yy:.1}" x2="{:.1}" y2="{yy:.1}" stroke="{INK}" stroke-opacity="0.08"/>"##,
            WIDTH - RIGHT
        );
        let _ = write!(
            s,
            r##"<text x="{:.1}" y="{:.1}" text-anchor="end" font-family="DM Mono, Menlo, monospace" font-size="11" fill="{INK}" fill-opacity="0.55">{level:.1}</text>"##,
            LEFT - 8.0,
            yy + 4.0
        );
        level += 0.2;
    }
    // Day ticks.
    let mut day = x0;
    while day <= x1 {
        let xx = x(day);
        let _ = write!(
            s,
            r##"<line x1="{xx:.1}" y1="{:.1}" x2="{xx:.1}" y2="{:.1}" stroke="{INK}" stroke-opacity="0.18"/>"##,
            TOP + plot_h,
            TOP + plot_h + 5.0
        );
        if (day - x0) / DAY % 2 == 0 {
            let _ = write!(
                s,
                r##"<text x="{xx:.1}" y="{:.1}" text-anchor="middle" font-family="DM Mono, Menlo, monospace" font-size="11" fill="{INK}" fill-opacity="0.55">{}</text>"##,
                TOP + plot_h + 20.0,
                day_label(day)
            );
        }
        day += DAY;
    }
    // Axes.
    let _ = write!(
        s,
        r##"<path d="M{LEFT} {TOP}V{:.1}H{:.1}" fill="none" stroke="{INK}" stroke-opacity="0.3"/>"##,
        TOP + plot_h,
        WIDTH - RIGHT
    );
    // θ and the v2 epoch.
    let ty = y(skin.theta);
    let _ = write!(
        s,
        r##"<line x1="{LEFT}" y1="{ty:.1}" x2="{:.1}" y2="{ty:.1}" stroke="{INK}" stroke-opacity="0.45" stroke-dasharray="4 4"/>"##,
        WIDTH - RIGHT
    );
    let _ = write!(
        s,
        r##"<text x="{:.1}" y="{:.1}" text-anchor="end" font-family="DM Mono, Menlo, monospace" font-size="10.5" fill="{INK}" fill-opacity="0.6">θ = {:.2}</text>"##,
        WIDTH - RIGHT - 4.0,
        ty - 5.0,
        skin.theta
    );
    if let Some(epoch) = skin.epoch_unix {
        let ex = x(epoch);
        let _ = write!(
            s,
            r##"<line x1="{ex:.1}" y1="{TOP}" x2="{ex:.1}" y2="{:.1}" stroke="{V2}" stroke-opacity="0.7" stroke-dasharray="4 4"/>"##,
            TOP + plot_h
        );
        let _ = write!(
            s,
            r##"<text x="{:.1}" y="{:.1}" font-family="DM Mono, Menlo, monospace" font-size="10.5" fill="{V2}">δ v2 epoch</text>"##,
            ex + 5.0,
            TOP + 14.0
        );
    }
    // Every keeper grade.
    if show_grades {
        for grade in &skin.grades {
            let (fill, opacity) = if grade.v == 2 {
                (V2, 0.45)
            } else {
                (INK, 0.22)
            };
            let _ = write!(
                s,
                r##"<circle cx="{:.1}" cy="{:.1}" r="2.1" fill="{fill}" fill-opacity="{opacity}"/>"##,
                x(grade.t),
                y(grade.d)
            );
        }
    }
    // Sitting medians, joined per instrument.
    for (version, colour) in [(1u8, V1), (2u8, V2)] {
        let points: Vec<(f64, f64, &skin::Sitting)> = skin
            .sittings(version)
            .map(|w| (x(w.t), y(w.median), w))
            .collect();
        if points.is_empty() {
            continue;
        }
        let path: Vec<String> = points
            .iter()
            .enumerate()
            .map(|(i, (px, py, _))| format!("{}{px:.1} {py:.1}", if i == 0 { "M" } else { "L" }))
            .collect();
        let _ = write!(
            s,
            r##"<path d="{}" fill="none" stroke="{colour}" stroke-width="1.8" stroke-linejoin="round"/>"##,
            path.join("")
        );
        for (px, py, w) in points {
            let _ = write!(
                s,
                r##"<circle cx="{px:.1}" cy="{py:.1}" r="4.2" fill="#ffffff" stroke="{colour}" stroke-width="1.8"><title>{} · n = {} · median δ = {:.3}</title></circle>"##,
                iso_date(w.t),
                w.n,
                w.median
            );
        }
    }
    s.push_str("</svg>");
    s
}

#[component]
pub fn SkinChart() -> impl IntoView {
    let skin = skin::load();
    let show_grades = RwSignal::new(true);
    let counts = (skin.grade_count(1), skin.grade_count(2));
    let sittings = (skin.sittings(1).count(), skin.sittings(2).count());
    let exported = skin.exported_at.clone();
    let guests = skin.guest_grades_excluded;
    let svg = move || skin_svg(&skin, show_grades.get());
    view! {
        <figure class="figure">
            <div class="figure-body" inner_html=svg></div>
            <div class="legend">
                <span><i class="swatch" style="background:#4946ff"></i>"median per sitting, δ v1 (1 − cos)"</span>
                <span><i class="swatch" style="background:#c98a00"></i>"median per sitting, δ v2 (the blend)"</span>
                <span><i class="swatch" style="background:#000020;opacity:.35"></i>"each keeper grade"</span>
                <label class="toggle"><input type="checkbox" prop:checked=move || show_grades.get() on:change=move |_| show_grades.update(|v| *v = !*v)/>" show grades"</label>
            </div>
            <figcaption class="caption">
                {format!("The keeper's δ, one point per grade and one marker per sitting (grades separated by less than thirty minutes). {} v1 grades in {} sittings and {} v2 grades in {} sittings; {} guest grades are recorded in the body and never pooled here. θ is the σ write threshold. Exported read-only from the living sediment on {}.", counts.0, sittings.0, counts.1, sittings.1, crate::util::grouped(guests as u64), exported)}
            </figcaption>
        </figure>
    }
}

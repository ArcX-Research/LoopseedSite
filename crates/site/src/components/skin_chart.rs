//! Prediction-error charts from exported records, with SVG tooltips: the regular participant's
//! sessions, and guest exchanges by group.
use leptos::prelude::*;
use loopseed_record::skin::{self, speaker_title, GuestDay, Skin};
use std::fmt::Write;

const WIDTH: f64 = 960.0;
const HEIGHT: f64 = 380.0;
const LEFT: f64 = 52.0;
const RIGHT: f64 = 20.0;
const TOP: f64 = 18.0;
const BOTTOM: f64 = 44.0;
const DAY: i64 = 86_400;

const INK: &str = "#000020";
const V1: &str = "#4946ff";
const V2: &str = "#c98a00";
const TEACHERS: &str = "#0454ff";
const WEATHER: &str = "#5c5c70";
const MONO: &str = "DM Mono, Menlo, monospace";

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

/// The shared frame of both charts: time on x, δ on y, day ticks, a grid, θ and the v2 epoch.
struct Frame {
    x0: i64,
    x1: i64,
    y_max: f64,
    plot_w: f64,
    plot_h: f64,
}

impl Frame {
    fn new(t_min: i64, t_max: i64, y_max: f64) -> Frame {
        Frame {
            x0: t_min.div_euclid(DAY) * DAY,
            x1: (t_max.div_euclid(DAY) + 1) * DAY,
            y_max,
            plot_w: WIDTH - LEFT - RIGHT,
            plot_h: HEIGHT - TOP - BOTTOM,
        }
    }

    fn x(&self, t: i64) -> f64 {
        LEFT + (t - self.x0) as f64 / (self.x1 - self.x0) as f64 * self.plot_w
    }

    fn y(&self, d: f64) -> f64 {
        TOP + (1.0 - d.min(self.y_max) / self.y_max) * self.plot_h
    }

    fn bottom(&self) -> f64 {
        TOP + self.plot_h
    }

    fn open(&self, s: &mut String, class: &str, label: &str) {
        let _ = write!(
            s,
            r##"<svg class="{class}" viewBox="0 0 {WIDTH} {HEIGHT}" role="img" aria-label="{label}">"##
        );
    }

    /// A shaded span from the left edge to `until`, with a small label.
    fn shade(&self, s: &mut String, until: i64, label: &str) {
        if until <= self.x0 {
            return;
        }
        let _ = write!(
            s,
            r##"<rect x="{LEFT}" y="{TOP}" width="{:.1}" height="{}" fill="{INK}" fill-opacity="0.035"/><text x="{:.1}" y="{:.1}" font-family="{MONO}" font-size="10.5" fill="{INK}" fill-opacity="0.5">{label}</text>"##,
            self.x(until) - LEFT,
            self.plot_h,
            LEFT + 6.0,
            TOP + 14.0
        );
    }

    fn grid(&self, s: &mut String, label_every_days: i64) {
        let mut level = 0.0;
        while level <= self.y_max + 1e-9 {
            let yy = self.y(level);
            let _ = write!(
                s,
                r##"<line x1="{LEFT}" y1="{yy:.1}" x2="{:.1}" y2="{yy:.1}" stroke="{INK}" stroke-opacity="0.08"/><text x="{:.1}" y="{:.1}" text-anchor="end" font-family="{MONO}" font-size="11" fill="{INK}" fill-opacity="0.55">{level:.1}</text>"##,
                WIDTH - RIGHT,
                LEFT - 8.0,
                yy + 4.0
            );
            level += 0.2;
        }
        let mut day = self.x0;
        while day <= self.x1 {
            let xx = self.x(day);
            let _ = write!(
                s,
                r##"<line x1="{xx:.1}" y1="{:.1}" x2="{xx:.1}" y2="{:.1}" stroke="{INK}" stroke-opacity="0.18"/>"##,
                self.bottom(),
                self.bottom() + 5.0
            );
            if (day - self.x0) / DAY % label_every_days == 0 {
                let _ = write!(
                    s,
                    r##"<text x="{xx:.1}" y="{:.1}" text-anchor="middle" font-family="{MONO}" font-size="11" fill="{INK}" fill-opacity="0.55">{}</text>"##,
                    self.bottom() + 20.0,
                    day_label(day)
                );
            }
            day += DAY;
        }
        let _ = write!(
            s,
            r##"<path d="M{LEFT} {TOP}V{:.1}H{:.1}" fill="none" stroke="{INK}" stroke-opacity="0.3"/>"##,
            self.bottom(),
            WIDTH - RIGHT
        );
    }

    fn rules(&self, s: &mut String, theta: f64, epoch: Option<i64>) {
        let ty = self.y(theta);
        let _ = write!(
            s,
            r##"<line x1="{LEFT}" y1="{ty:.1}" x2="{:.1}" y2="{ty:.1}" stroke="{INK}" stroke-opacity="0.45" stroke-dasharray="4 4"/><text x="{:.1}" y="{:.1}" text-anchor="end" font-family="{MONO}" font-size="10.5" fill="{INK}" fill-opacity="0.6">θ = {theta:.2}</text>"##,
            WIDTH - RIGHT,
            WIDTH - RIGHT - 4.0,
            ty - 5.0
        );
        if let Some(epoch) = epoch {
            let ex = self.x(epoch);
            let _ = write!(
                s,
                r##"<line x1="{ex:.1}" y1="{TOP}" x2="{ex:.1}" y2="{:.1}" stroke="{V2}" stroke-opacity="0.7" stroke-dasharray="4 4"/><text x="{:.1}" y="{:.1}" font-family="{MONO}" font-size="10.5" fill="{V2}">δ v2 begins</text>"##,
                self.bottom(),
                ex + 5.0,
                TOP + 14.0
            );
        }
    }
}

pub fn skin_svg(skin: &Skin, show_grades: bool) -> String {
    let t_min = skin.grades.iter().map(|g| g.t).min().unwrap_or(0);
    let t_max = skin.grades.iter().map(|g| g.t).max().unwrap_or(t_min + DAY);
    let f = Frame::new(t_min, t_max, 0.9);
    let mut s = String::with_capacity(60_000);
    f.open(
        &mut s,
        "skin",
        "Prediction error per session for the regular human participant, August 2026",
    );
    f.shade(&mut s, skin.instrumentation_end_unix, "setup period");
    f.grid(&mut s, 2);
    f.rules(&mut s, skin.theta, skin.epoch_unix);
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
                f.x(grade.t),
                f.y(grade.d)
            );
        }
    }
    for (version, colour) in [(1u8, V1), (2u8, V2)] {
        let points: Vec<(f64, f64, &skin::Sitting)> = skin
            .sittings(version)
            .map(|w| (f.x(w.t), f.y(w.median), w))
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

/// Nine shades for the nine classrooms, darkest first; the classroom number picks the shade.
const CLASSROOM_PALETTE: [&str; 9] = [
    "#3b1f5e", "#4f2a7c", "#63359a", "#7b3fa3", "#8f4cb6", "#a35bc4", "#b46fd0", "#c284da",
    "#d09ae4",
];

/// The colour of one series: a pooled group's own colour, or a classroom's shade.
pub fn series_colour(group: &str, series: &str) -> &'static str {
    match group {
        "teachers" => TEACHERS,
        "weather" => WEATHER,
        _ => {
            let number: usize = series
                .trim_start_matches("class")
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap_or(1);
            CLASSROOM_PALETTE[(number.max(1) - 1) % CLASSROOM_PALETTE.len()]
        }
    }
}

fn display_group_label(group: &skin::GuestGroup) -> &str {
    match group.key.as_str() {
        "classrooms" => "nine automated teaching programs",
        "weather" => "external input",
        _ => &group.label,
    }
}

/// Consecutive-day runs of one group's day records, so a line never bridges a gap of days.
fn runs<'a>(days: &[&'a GuestDay]) -> Vec<Vec<&'a GuestDay>> {
    let mut out: Vec<Vec<&GuestDay>> = Vec::new();
    for day in days {
        match out.last_mut() {
            Some(run) if day.t - run.last().map_or(day.t, |d| d.t) == DAY => run.push(day),
            _ => out.push(vec![day]),
        }
    }
    out
}

pub fn guest_svg(skin: &Skin, show_sittings: bool) -> String {
    let guests = &skin.guests;
    let t_min = skin
        .grades
        .iter()
        .map(|g| g.t)
        .chain(guests.sittings.iter().map(|w| w.t))
        .min()
        .unwrap_or(0);
    let t_max = guests
        .sittings
        .iter()
        .map(|w| w.end)
        .max()
        .unwrap_or(t_min + DAY);
    let f = Frame::new(t_min, t_max, 1.0);
    let mut s = String::with_capacity(40_000);
    f.open(
        &mut s,
        "skin",
        "Prediction error for guest exchanges, August 2026. Each teaching program is shown separately. Outlined markers show daily medians; bands span the middle 50% of scores for the AI teacher and external input groups; dots show session medians when enabled. Guest results are separate from the regular participant's results",
    );
    f.grid(&mut s, 4);
    f.rules(&mut s, skin.theta, skin.epoch_unix);
    let series_of = |group: &skin::GuestGroup| -> Vec<(String, String)> {
        if group.pooled {
            vec![(group.key.clone(), display_group_label(group).to_string())]
        } else {
            group
                .speakers
                .iter()
                .map(|s| (s.speaker.clone(), speaker_title(&s.speaker).to_string()))
                .collect()
        }
    };
    for group in &guests.groups {
        for (series, title) in series_of(group) {
            let colour = series_colour(&group.key, &series);
            for version in [1u8, 2u8] {
                let days: Vec<&GuestDay> = guests
                    .days
                    .iter()
                    .filter(|d| d.g == group.key && d.s == series && d.v == version)
                    .collect();
                for run in runs(&days) {
                    if run.len() >= 2 && group.pooled {
                        let upper: Vec<String> = run
                            .iter()
                            .map(|d| format!("{:.1},{:.1}", f.x(d.t + DAY / 2), f.y(d.q3)))
                            .collect();
                        let lower: Vec<String> = run
                            .iter()
                            .rev()
                            .map(|d| format!("{:.1},{:.1}", f.x(d.t + DAY / 2), f.y(d.q1)))
                            .collect();
                        let _ = write!(
                            s,
                            r##"<polygon points="{} {}" fill="{colour}" fill-opacity="0.1"/>"##,
                            upper.join(" "),
                            lower.join(" ")
                        );
                    }
                    if run.len() >= 2 {
                        let line: Vec<String> = run
                            .iter()
                            .enumerate()
                            .map(|(i, d)| {
                                format!(
                                    "{}{:.1} {:.1}",
                                    if i == 0 { "M" } else { "L" },
                                    f.x(d.t + DAY / 2),
                                    f.y(d.median)
                                )
                            })
                            .collect();
                        let dash = if version == 1 {
                            r#" stroke-dasharray="5 4""#
                        } else {
                            ""
                        };
                        let _ = write!(
                            s,
                            r##"<path d="{}" fill="none" stroke="{colour}" stroke-width="1.8" stroke-linejoin="round"{dash}/>"##,
                            line.join("")
                        );
                    }
                    for d in &run {
                        let cx = f.x(d.t + DAY / 2);
                        if run.len() < 2 && group.pooled {
                            let _ = write!(
                                s,
                                r##"<line x1="{cx:.1}" y1="{:.1}" x2="{cx:.1}" y2="{:.1}" stroke="{colour}" stroke-opacity="0.35" stroke-width="5" stroke-linecap="round"/>"##,
                                f.y(d.q3),
                                f.y(d.q1)
                            );
                        }
                        let radius = if group.pooled { 3.6 } else { 3.0 };
                        let _ = write!(
                            s,
                            r##"<circle cx="{cx:.1}" cy="{:.1}" r="{radius}" fill="#ffffff" stroke="{colour}" stroke-width="1.8"><title>{} · {} · δ v{} · n = {} · median {:.3} · quartiles {:.3}–{:.3}</title></circle>"##,
                            f.y(d.median),
                            title,
                            iso_date(d.t),
                            d.v,
                            d.n,
                            d.median,
                            d.q1,
                            d.q3
                        );
                    }
                }
            }
        }
    }
    if show_sittings {
        for w in &guests.sittings {
            let _ = write!(
                s,
                r##"<circle cx="{:.1}" cy="{:.1}" r="2.4" fill="{}" fill-opacity="0.45"><title>{} · {} · δ v{} · n = {} · median {:.3}</title></circle>"##,
                f.x(w.t),
                f.y(w.median),
                series_colour(&w.g, &w.speaker),
                speaker_title(&w.speaker),
                iso_date(w.t),
                w.v,
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
                <span><i class="swatch" style="background:#4946ff"></i>"session median, δ v1 (cosine distance)"</span>
                <span><i class="swatch" style="background:#c98a00"></i>"session median, δ v2 (combined score)"</span>
                <span><i class="swatch" style="background:#000020;opacity:.35"></i>"individual exchange"</span>
                <label class="toggle"><input type="checkbox" prop:checked=move || show_grades.get() on:change=move |_| show_grades.update(|v| *v = !*v)/>" show individual scores"</label>
            </div>
            <figcaption class="caption">
                {format!("Prediction error for the regular participant: one dot per exchange and one outlined marker per session median. Sessions group scores less than thirty minutes apart. There are {} v1 scores in {} sessions and {} v2 scores in {} sessions; {} recorded guest scores are excluded and charted below. θ marks the memory threshold. Data exported without changing the live database on {}.", counts.0, sittings.0, counts.1, sittings.1, crate::util::grouped(guests as u64), exported)}
            </figcaption>
        </figure>
    }
}

#[component]
pub fn GuestChart() -> impl IntoView {
    let skin = skin::load();
    let show_sittings = RwSignal::new(true);
    let summary: Vec<String> = skin
        .guests
        .groups
        .iter()
        .map(|g| {
            let speakers: Vec<String> = g
                .speakers
                .iter()
                .map(|s| {
                    if g.pooled {
                        speaker_title(&s.speaker).to_string()
                    } else {
                        format!(
                            "{}: {} scores (median {:.3})",
                            speaker_title(&s.speaker),
                            s.grades,
                            s.median
                        )
                    }
                })
                .collect();
            if g.pooled {
                let label = display_group_label(g);
                let named_group = if speakers.len() == 1 && speakers[0].eq_ignore_ascii_case(label)
                {
                    label.to_string()
                } else {
                    format!("{} ({})", label, speakers.join(", "))
                };
                format!(
                    "{}: {} scores, median {}",
                    named_group,
                    crate::util::grouped(g.grades.into()),
                    g.median.map_or("—".to_string(), |m| format!("{m:.3}"))
                )
            } else {
                format!(
                    "{}, {} scores in total: {}",
                    display_group_label(g),
                    crate::util::grouped(g.grades.into()),
                    speakers.join(", ")
                )
            }
        })
        .collect();
    let omitted = skin.guests.omitted.clone();
    let day_records = skin.guests.days.len();
    let session_records = skin.guests.sittings.len();
    let legend: Vec<(String, &'static str)> = skin
        .guests
        .groups
        .iter()
        .flat_map(|g| {
            if g.pooled {
                vec![(
                    display_group_label(g).to_string(),
                    series_colour(&g.key, &g.key),
                )]
            } else {
                g.speakers
                    .iter()
                    .map(|s| {
                        (
                            speaker_title(&s.speaker).to_string(),
                            series_colour(&g.key, &s.speaker),
                        )
                    })
                    .collect()
            }
        })
        .collect();
    let svg = move || guest_svg(&skin, show_sittings.get());
    view! {
        <figure class="figure">
            <div class="figure-body" inner_html=svg></div>
            <div class="legend">
                {legend.into_iter().map(|(label, colour)| view! {
                    <span><i class="swatch" style=format!("background:{colour}")></i>{label}</span>
                }).collect_view()}
            </div>
            <div class="legend">
                <span>"outlined marker: daily median · band: middle 50% of scores for a pooled group · dashed line: δ v1 · solid line: δ v2 · dot: session median"</span>
                <label class="toggle"><input type="checkbox" prop:checked=move || show_sittings.get() on:change=move |_| show_sittings.update(|v| *v = !*v)/>" show session medians"</label>
            </div>
            <figcaption class="caption">
                {format!("Guest exchanges use the same prediction error measure and are reported separately, never pooled with the regular participant's results. {}. The chart includes {} daily records and {} sessions; {} scores labelled {} are omitted because there are too few to chart.", summary.join("; "), day_records, session_records, omitted.grades, omitted.speakers.join(", "))}
            </figcaption>
        </figure>
    }
}

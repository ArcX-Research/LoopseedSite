//! Read-only prediction-error export. Guest scores are excluded; v1 and v2 stay separate.

use serde::Deserialize;

pub const SKIN_JSON: &str = include_str!("../data/skin.json");

#[derive(Debug, Clone, Deserialize)]
pub struct Grade {
    /// Unix seconds, UTC.
    pub t: i64,
    /// Prediction error δ.
    pub d: f64,
    /// 1 = cosine distance; 2 = combined score.
    pub v: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sitting {
    pub v: u8,
    pub t: i64,
    pub end: i64,
    pub n: u32,
    pub median: f64,
}

/// One guest speaker's totals.
#[derive(Debug, Clone, Deserialize)]
pub struct GuestSpeaker {
    pub speaker: String,
    pub grades: u32,
    pub median: f64,
}

/// One group of guest speakers: AI teachers, the classrooms, or ocean weather. A pooled group
/// draws one series; an unpooled group draws one series per speaker.
#[derive(Debug, Clone, Deserialize)]
pub struct GuestGroup {
    pub key: String,
    pub label: String,
    pub pooled: bool,
    pub speakers: Vec<GuestSpeaker>,
    pub grades: u32,
    pub median: Option<f64>,
}

/// Quartiles of one series' scores on one UTC day, per instrument version. The series is the
/// group for a pooled group and the speaker otherwise.
#[derive(Debug, Clone, Deserialize)]
pub struct GuestDay {
    pub g: String,
    pub s: String,
    pub v: u8,
    /// Unix seconds of the day's start, UTC.
    pub t: i64,
    pub n: u32,
    pub q1: f64,
    pub median: f64,
    pub q3: f64,
}

/// One guest speaker's session: scores less than thirty minutes apart.
#[derive(Debug, Clone, Deserialize)]
pub struct GuestSitting {
    pub g: String,
    pub speaker: String,
    pub v: u8,
    pub t: i64,
    pub end: i64,
    pub n: u32,
    pub median: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Omitted {
    pub speakers: Vec<String>,
    pub grades: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Guests {
    pub groups: Vec<GuestGroup>,
    pub omitted: Omitted,
    pub days: Vec<GuestDay>,
    pub sittings: Vec<GuestSitting>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Skin {
    pub exported_at: String,
    pub source: String,
    pub theta: f64,
    pub epoch_unix: Option<i64>,
    pub instrumentation_end_unix: i64,
    pub keeper_grades: u32,
    pub guest_grades_excluded: u32,
    pub grades: Vec<Grade>,
    pub windows: Vec<Sitting>,
    pub guests: Guests,
}

pub fn load() -> Skin {
    serde_json::from_str(SKIN_JSON).expect("skin.json is valid")
}

/// Readable titles for the recorded speaker keys. The classroom titles are the techniques in
/// `fish/lab/classroom_curricula.py`; the teachers are named as they were at the port.
pub const SPEAKER_TITLES: &[(&str, &str)] = &[
    ("class01-factor", "Prime factorization"),
    ("class02-fraction", "Fraction reduction"),
    ("class03-division", "Quotient and remainder"),
    ("class04-euclid", "Euclidean recurrence"),
    ("class05-binomial", "Binomial recurrence"),
    ("class06-cancel", "Exact cancellation"),
    ("class07-paths", "Blocked lattice paths"),
    ("class08-choices", "Constrained combinations"),
    ("class09-rectangles", "Non-square rectangles"),
    ("claude", "Claude"),
    ("codex", "Codex"),
    ("fable", "Fable"),
    ("ocean", "External input"),
    ("stranger", "a stranger"),
];

pub fn speaker_title(key: &str) -> &str {
    SPEAKER_TITLES
        .iter()
        .find(|(k, _)| *k == key)
        .map_or(key, |(_, title)| title)
}

impl Skin {
    pub fn sittings(&self, version: u8) -> impl Iterator<Item = &Sitting> {
        self.windows.iter().filter(move |w| w.v == version)
    }

    pub fn grade_count(&self, version: u8) -> usize {
        self.grades.iter().filter(|g| g.v == version).count()
    }
}

/// First-period summary: `fish/album/2026-08-04-the-plateau-era-close.png`.
pub struct EraOne {
    pub first_median: f64,
    pub low_median: f64,
    pub plateau: f64,
    pub plateau_band: f64,
    pub close_date: &'static str,
    pub theta: f64,
}

pub const ERA_ONE: EraOne = EraOne {
    first_median: 0.479,
    low_median: 0.369,
    plateau: 0.40,
    plateau_band: 0.03,
    close_date: "2026-08-04",
    theta: 0.35,
};

/// Original prediction-error measure.
pub const DELTA_V1: &str = "δ = 1 − cos(E(Î), E(you))";
/// Combined measure, used from 2026-08-07.
pub const DELTA_V2: &str = "δ = ½ · ppl_norm(you) + ½ · (1 − cos(E(Î), E(you)))";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_export_is_consistent() {
        let skin = load();
        assert_eq!(skin.grades.len() as u32, skin.keeper_grades);
        assert!(skin
            .windows
            .iter()
            .all(|w| w.n >= 1 && (0.0..=1.0).contains(&w.median)));
        assert!(skin.grades.iter().all(|g| g.v == 1 || g.v == 2));
        assert_eq!(
            skin.windows.iter().map(|w| w.n).sum::<u32>() as usize,
            skin.grades.len()
        );
        assert_eq!(skin.theta, ERA_ONE.theta);
        assert!(skin.epoch_unix.is_some());
    }

    #[test]
    fn the_guest_export_is_consistent() {
        let skin = load();
        let guests = &skin.guests;
        let drawn: u32 = guests.groups.iter().map(|g| g.grades).sum();
        assert_eq!(drawn + guests.omitted.grades, skin.guest_grades_excluded);
        for group in &guests.groups {
            let by_day: u32 = guests
                .days
                .iter()
                .filter(|d| d.g == group.key)
                .map(|d| d.n)
                .sum();
            let by_sitting: u32 = guests
                .sittings
                .iter()
                .filter(|s| s.g == group.key)
                .map(|s| s.n)
                .sum();
            assert_eq!(by_day, group.grades, "{}", group.key);
            assert_eq!(by_sitting, group.grades, "{}", group.key);
            let by_speaker: u32 = group.speakers.iter().map(|s| s.grades).sum();
            assert_eq!(by_speaker, group.grades, "{}", group.key);
            let series: std::collections::BTreeSet<&str> = guests
                .days
                .iter()
                .filter(|d| d.g == group.key)
                .map(|d| d.s.as_str())
                .collect();
            if group.pooled {
                assert_eq!(series.len(), 1, "{}", group.key);
            } else {
                assert_eq!(series.len(), group.speakers.len(), "{}", group.key);
            }
        }
        assert!(guests
            .days
            .iter()
            .all(|d| 0.0 <= d.q1 && d.q1 <= d.median && d.median <= d.q3 && d.q3 <= 1.0));
        assert!(guests
            .sittings
            .windows(2)
            .all(|pair| pair[0].t <= pair[1].t));
    }

    #[test]
    fn every_recorded_speaker_has_a_title() {
        let skin = load();
        for group in &skin.guests.groups {
            for speaker in &group.speakers {
                assert_ne!(
                    speaker_title(&speaker.speaker),
                    speaker.speaker,
                    "{} has no title",
                    speaker.speaker
                );
            }
        }
        assert_eq!(speaker_title("class01-factor"), "Prime factorization");
        assert_eq!(speaker_title("unknown"), "unknown");
    }

    #[test]
    fn the_quoted_era_one_medians_are_in_the_data() {
        let skin = load();
        let medians: Vec<f64> = skin.sittings(1).map(|w| w.median).collect();
        let quoted = |value: f64| medians.iter().any(|m| (m - value).abs() < 1e-3);
        assert!(quoted(ERA_ONE.first_median));
        assert!(quoted(ERA_ONE.low_median));
    }
}

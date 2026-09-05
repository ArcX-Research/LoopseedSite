//! The skin: the keeper's δ grades and the median of each sitting, exported read-only from the
//! living sediment by `scripts/export_skin.py`. Guest grades are recorded in the body and never
//! enter the medians; the two instruments (v1 and v2) never share a median.

use serde::Deserialize;

pub const SKIN_JSON: &str = include_str!("../data/skin.json");

#[derive(Debug, Clone, Deserialize)]
pub struct Grade {
    /// Unix seconds, UTC.
    pub t: i64,
    /// δ for this grade.
    pub d: f64,
    /// 1 = embedding distance alone, 2 = the stage 4.5 blend.
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
}

pub fn load() -> Skin {
    serde_json::from_str(SKIN_JSON).expect("skin.json is valid")
}

impl Skin {
    pub fn sittings(&self, version: u8) -> impl Iterator<Item = &Sitting> {
        self.windows.iter().filter(move |w| w.v == version)
    }

    pub fn grade_count(&self, version: u8) -> usize {
        self.grades.iter().filter(|g| g.v == version).count()
    }
}

/// The era-one reading, as photographed at the era's close (fish/album/README.md,
/// `2026-08-04-the-plateau-era-close.png`).
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

/// δ v2, the constitution's blend, live from stage 4.5 (2026-08-07).
pub const DELTA_V1: &str = "δ = 1 − cos(E(Î), E(you))";
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
    fn the_quoted_era_one_medians_are_in_the_data() {
        let skin = load();
        let medians: Vec<f64> = skin.sittings(1).map(|w| w.median).collect();
        let quoted = |value: f64| medians.iter().any(|m| (m - value).abs() < 1e-3);
        assert!(quoted(ERA_ONE.first_median));
        assert!(quoted(ERA_ONE.low_median));
    }
}

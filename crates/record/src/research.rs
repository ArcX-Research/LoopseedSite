//! Dated aggregate data shared with the Dynamical Synthesis paper and figures.
use serde::Deserialize;
use std::{collections::BTreeMap, sync::OnceLock};

#[derive(Debug, Deserialize)]
pub struct ResearchRecord {
    pub reviewed_on: String,
    pub latest_experiment_date: String,
    pub decisive_pilots: Vec<Pilot>,
    pub worlds: Worlds,
    pub rule_use: RuleUse,
    pub acquisition: Acquisition,
}

#[derive(Debug, Deserialize)]
pub struct Pilot {
    pub version: u32,
    pub saved: u32,
    pub scheduled: u32,
    pub pilot_complete: bool,
    pub learning_effect_established: bool,
    pub status: String,
    pub cells: BTreeMap<String, PilotCounts>,
}

#[derive(Debug, Deserialize)]
pub struct PilotCounts {
    #[serde(rename = "SOUND")]
    pub sound: u32,
    #[serde(rename = "FAIL")]
    pub fail: u32,
    #[serde(rename = "INVALID")]
    pub invalid: u32,
}

impl Pilot {
    pub fn counts(&self) -> PilotCounts {
        self.cells.values().fold(
            PilotCounts {
                sound: 0,
                fail: 0,
                invalid: 0,
            },
            |mut total, cell| {
                total.sound += cell.sound;
                total.fail += cell.fail;
                total.invalid += cell.invalid;
                total
            },
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Worlds {
    pub calls: u32,
    pub worlds: u32,
    pub initial_rules_correct: u32,
    pub repaired_rules_correct: u32,
    pub own: u32,
    pub raw: u32,
    pub none: u32,
    pub queries_per_condition: u32,
    pub gate_passed: bool,
}

#[derive(Debug, Deserialize)]
pub struct RuleUse {
    pub calls: u32,
    pub conditions: BTreeMap<String, RuleCondition>,
    pub learning_established: bool,
}

#[derive(Debug, Deserialize)]
pub struct RuleCondition {
    pub exact: u32,
    pub n: u32,
    pub total_generated_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct Acquisition {
    pub calls: u32,
    pub rules_correct: u32,
    pub worlds: u32,
    pub conditions: BTreeMap<String, AnswerCount>,
}

#[derive(Debug, Deserialize)]
pub struct AnswerCount {
    pub exact: u32,
    pub n: u32,
    pub capped: u32,
}

pub fn research_record() -> &'static ResearchRecord {
    static RECORD: OnceLock<ResearchRecord> = OnceLock::new();
    RECORD.get_or_init(|| {
        serde_json::from_str(include_str!("../../../static/data/research-record.json"))
            .expect("the reviewed research record must parse")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pilot_collection_and_validity_remain_distinct() {
        let record = research_record();
        for pilot in &record.decisive_pilots {
            let counts = pilot.counts();
            assert_eq!(counts.sound + counts.fail + counts.invalid, pilot.saved);
            assert!(pilot.saved <= pilot.scheduled);
            assert_eq!(pilot.pilot_complete, pilot.saved == pilot.scheduled);
            assert!(!pilot.learning_effect_established);
        }
        let paused = record
            .decisive_pilots
            .iter()
            .find(|p| p.status == "Paused")
            .unwrap();
        assert!(paused.saved < paused.scheduled);
        assert!(record
            .decisive_pilots
            .iter()
            .any(|p| p.pilot_complete && p.status == "Allocation refused"));
    }

    #[test]
    fn world_study_denominators_include_acquisition_failures() {
        let record = research_record();
        assert!(record.acquisition.rules_correct < record.acquisition.worlds);
        for condition in record.acquisition.conditions.values() {
            assert_eq!(condition.n, record.acquisition.worlds);
            assert!(condition.exact + condition.capped <= condition.n);
        }
        let primary_calls: u32 = record.rule_use.conditions.values().map(|c| c.n).sum();
        assert!(primary_calls < record.rule_use.calls); // Recorded repeats are additional calls.
        assert!(!record.rule_use.learning_established);
    }
}

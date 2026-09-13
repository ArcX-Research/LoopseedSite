//! External-input, model and memory measurements with source references.

pub struct OceanReading {
    pub breaths: u32,
    pub paired: Option<f64>,
    pub shuffled: Option<f64>,
    pub mutual_information: f64,
    pub note: &'static str,
}

/// Stage 6: word-based estimates of external-input coupling.
pub const OCEAN: &[OceanReading] = &[
    OceanReading {
        breaths: 8,
        paired: Some(0.0315),
        shuffled: Some(0.0339),
        mutual_information: 0.0244,
        note: "baseline: paired and shuffled overlap were similar",
    },
    OceanReading {
        breaths: 17,
        paired: None,
        shuffled: None,
        mutual_information: 0.0347,
        note: "estimate increased",
    },
    OceanReading {
        breaths: 32,
        paired: None,
        shuffled: None,
        mutual_information: 0.0653,
        note: "estimate increased",
    },
    OceanReading {
        breaths: 63,
        paired: None,
        shuffled: None,
        mutual_information: 0.0762,
        note: "estimate began to level off; overlap was stable",
    },
    OceanReading {
        breaths: 128,
        paired: Some(0.0717),
        shuffled: Some(0.0371),
        mutual_information: 0.0712,
        note: "slightly below the preceding value; the scale’s maximum is 1.0",
    },
];
pub const OCEAN_SOURCE: &str = "docs/ROADMAP.md, stage 6 (approved 2026-08-12)";

pub struct BodyReading {
    pub body: &'static str,
    pub register_hold: f64,
    pub echo_rate: f64,
    pub stroke_accuracy: f64,
    pub dream_seconds: f64,
}

/// S6: baseline responses before training, plus training time; not learning gains.
pub const SWEEP: &[BodyReading] = &[
    BodyReading {
        body: "4B dense",
        register_hold: 1.00,
        echo_rate: 0.00,
        stroke_accuracy: 0.55,
        dream_seconds: 187.528,
    },
    BodyReading {
        body: "8B dense",
        register_hold: 1.00,
        echo_rate: 0.05,
        stroke_accuracy: 0.70,
        dream_seconds: 274.831,
    },
    BodyReading {
        body: "30B mixture of experts, 3B active",
        register_hold: 1.00,
        echo_rate: 0.05,
        stroke_accuracy: 0.45,
        dream_seconds: 195.266,
    },
];
pub const SWEEP_SOURCE: &str = "docs/ROADMAP.md, scaling stage S6 (2026-08-14)";

pub struct Figure {
    pub claim: &'static str,
    pub number: &'static str,
    pub evidence: &'static str,
}

/// The memory laws' numbers, from bag 2 of the laws file.
pub const MEMORY: &[Figure] = &[
    Figure {
        claim: "Recall and judgement with a supplied record",
        number: "With the record supplied, correct recall rose from 0 of 12 to 12 of 12; correct judgements fell from 12 of 12 to 4 of 12",
        evidence: "LAWS.md bag 2, rows 240–241",
    },
    Figure {
        claim: "Errors with an incorrect stored record",
        number: "A sixfold increase was reported; full counts are not given in this summary",
        evidence: "LAWS.md bag 2, rows 236–237",
    },
    Figure {
        claim: "Answers before and after a worked example",
        number: "0 of 32 before; 48 of 48 later, using different test questions",
        evidence: "LAWS.md bag 2, row 267",
    },
    Figure {
        claim: "Reuse of a lesson on a similar puzzle",
        number: "Success was reported in about one quarter of attempts; the total is not given here",
        evidence: "LAWS.md bag 2, rows 256–257",
    },
];

/// First full-system comparison: no measured gain or loss.
pub const BODY_RULING: &[Figure] = &[
    Figure { claim: "Test replies collected", number: "396 of 396 planned", evidence: "fish/album/2026-08-14T074428-EAT-body-evolution.md; the live database file was unchanged" },
    Figure { claim: "Changes detected by the comparison", number: "No measured gains, losses or interaction with memory", evidence: "the same report" },
    Figure { claim: "Records retrieved when memory was enabled", number: "0", evidence: "no records passed the retrieval threshold, including on tests of earlier abilities" },
];

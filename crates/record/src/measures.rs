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
        note: "remained level, about 14 times below the 1.0 maximum",
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
        body: "30B, 3B active (MoE)",
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
        claim: "One stored record improved recall but reduced judgement accuracy",
        number: "recall 0/12 → 12/12; judgement 12/12 → 4/12 with the same record",
        evidence: "LAWS.md bag 2, rows 240–241",
    },
    Figure {
        claim: "One incorrect memory increased the error rate",
        number: "6×; storing a correction alongside it did not remove the effect",
        evidence: "LAWS.md bag 2, rows 236–237",
    },
    Figure {
        claim: "Performance improved after one worked example",
        number: "0/32 before; 48/48 on the later test set",
        evidence: "LAWS.md bag 2, row 267",
    },
    Figure {
        claim: "A stored lesson helped on a puzzle with the same structure",
        number: "about a quarter of the time",
        evidence: "LAWS.md bag 2, rows 256–257",
    },
];

/// First full-system comparison: no measured gain or loss.
pub const BODY_RULING: &[Figure] = &[
    Figure { claim: "Isolated requests completed with the live database unchanged", number: "396 of 396", evidence: "fish/album/2026-08-14T074428-EAT-body-evolution.md" },
    Figure { claim: "Gains, losses and measured memory interaction", number: "0, 0, 0", evidence: "the same report" },
    Figure { claim: "Records retrieved with memory enabled, including retention controls", number: "0", evidence: "no records passed the L2 distance threshold of 0.70; later calibration kept this threshold unchanged" },
];

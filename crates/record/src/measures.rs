//! Measurements other than the coat experiments: the ocean's mutual information, the sweep of
//! bodies, the memory laws' numbers, and the first production-shaped body ruling. Each quotes
//! the loopseed roadmap or laws file named in `source`.

pub struct OceanReading {
    pub breaths: u32,
    pub paired: Option<f64>,
    pub shuffled: Option<f64>,
    pub mutual_information: f64,
    pub note: &'static str,
}

/// Stage 6, the ocean: a second port whose weather couples to the conversation. Pass test:
/// mutual information rises, then saturates below the total. Lexical-proxy measurements.
pub const OCEAN: &[OceanReading] = &[
    OceanReading {
        breaths: 8,
        paired: Some(0.0315),
        shuffled: Some(0.0339),
        mutual_information: 0.0244,
        note: "floor: paired equals shuffled, zero coupling",
    },
    OceanReading {
        breaths: 17,
        paired: None,
        shuffled: None,
        mutual_information: 0.0347,
        note: "rise",
    },
    OceanReading {
        breaths: 32,
        paired: None,
        shuffled: None,
        mutual_information: 0.0653,
        note: "rise",
    },
    OceanReading {
        breaths: 63,
        paired: None,
        shuffled: None,
        mutual_information: 0.0762,
        note: "saturation onset, overlap flat",
    },
    OceanReading {
        breaths: 128,
        paired: Some(0.0717),
        shuffled: Some(0.0371),
        mutual_information: 0.0712,
        note: "saturation confirmed, 14× under the 1.0 ceiling",
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

/// Scaling stage S6, the sweep: three bodies, identical thirty-turn teaching, twenty frozen
/// probes before the dream on fresh no-ledger chairs, two-hundred-iteration dreams. Bare
/// response and training cost side by side; neither a winner nor an evolution claim.
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
        claim: "M amplifies retrieval, and that is all it does",
        number:
            "recall 0/12 → 12/12 with the right kept row; judgement 12/12 → 4/12 with the same row",
        evidence: "LAWS.md bag 2, rows 240–241",
    },
    Figure {
        claim: "one wrong kept row multiplies the error rate",
        number: "6×; a correction beside the error does not cancel it",
        evidence: "LAWS.md bag 2, rows 236–237",
    },
    Figure {
        claim: "one worked example teaches the method, not the answer",
        number: "0/32 → 48/48",
        evidence: "LAWS.md bag 2, row 267",
    },
    Figure {
        claim: "a kept lesson from one puzzle answers a structurally identical puzzle's table",
        number: "about a quarter of the time",
        evidence: "LAWS.md bag 2, rows 256–257",
    },
];

/// The first production-shaped body ruling: valid, clean, and null.
pub const BODY_RULING: &[Figure] = &[
    Figure { claim: "isolated requests completed with unchanged living sediment", number: "396 of 396", evidence: "fish/album/2026-08-14T074428-EAT-body-evolution.md" },
    Figure { claim: "organism gains, losses, and memory interaction", number: "0, 0, 0", evidence: "the same ruling" },
    Figure { claim: "rows retrieved by memory-on requests, retention controls included", number: "0", evidence: "the L2 ≤ 0.70 starvation boundary; calibrated admission was built afterwards without lowering it" },
];

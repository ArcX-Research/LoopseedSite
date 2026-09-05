//! The dated record: rulings and photographs in `fish/album/` of the loopseed repository, newest
//! first. A ruling names its run, its hashes and what it did not authorize.

pub struct Ruling {
    pub date: &'static str,
    pub title: &'static str,
    pub result: &'static str,
    pub path: &'static str,
    pub hash: Option<&'static str>,
}

pub const RULINGS: &[Ruling] = &[
    Ruling {
        date: "2026-09-04",
        title: "Clean formal-claim transfer: positive causal ruling",
        result: "43 of 64 independently accepted on unseen names-only tasks against 0 for bare water and 0 for a shuffled-reply placebo; no losses, no negative literals. method_transfer = true; living promotion not authorized.",
        path: "fish/album/2026-09-04T0043-EAT-clean-formal-claim-transfer.md",
        hash: Some("0655eaba6047287f17792f14c4e3d09a58b902ad41fc617fb4a98e107f20f3ba"),
    },
    Ruling {
        date: "2026-09-03",
        title: "Clean formal-claim coat: blinded place-value safety result",
        result: "18 of 18 with zero stray literals for the clean coat against 0 of 18 for bare water and the contaminated coat. A full blinded transfer run authorized; wearing not.",
        path: "fish/album/2026-09-03T1945-EAT-clean-claim-safety.md",
        hash: Some("e4cbc7ee39576620e7482a0d51aecc5b81c19d542a8510476c2ee5df9b66fa8a"),
    },
    Ruling {
        date: "2026-09-03",
        title: "Formal claim transfer, stage 1: negative ruling",
        result: "25 of 64 names-only against 0 and 0, yet 3 of 39 failed replies carried a negative literal, above the 2 % cap. Nothing authorized; contamination of the dream prompts diagnosed.",
        path: "fish/album/2026-09-03T1908-EAT-formal-claim-transfer-stage1.md",
        hash: Some("9caa5ba0bb6d8e7e99f623208b17577e0f5a4c9474721c37ec4da3f5cd02f286"),
    },
    Ruling {
        date: "2026-09-03",
        title: "Formal claim-coat primary ruling",
        result: "94 against 73 and 52 of 128 under node-specified prompts, 21 gains and no losses against bare water; the negative-literal gate failed on one cell. No transfer stage, no wearing.",
        path: "fish/album/2026-09-03T0028-EAT-claim-coat-primary-ruling.md",
        hash: Some("a03699a11fefdf7ec8cb80caf145fd3d1d628483d08e78d4f96fda38f69c7dcc"),
    },
    Ruling {
        date: "2026-08-16",
        title: "Prediction-coat promotion plan and ruling",
        result: "The nightly prediction coat's promotion procedure, ruled on before the coat now worn was activated.",
        path: "fish/album/2026-08-16T1950-EAT-prediction-coat-promotion-ruling.md",
        hash: None,
    },
    Ruling {
        date: "2026-08-14",
        title: "First production-shaped body ruling",
        result: "396 of 396 isolated requests, unchanged living sediment, no leak: valid, clean and null. Every memory-on request retrieved zero rows, diagnosing the admission boundary rather than the sitting.",
        path: "fish/album/2026-08-14T074428-EAT-body-evolution.md",
        hash: None,
    },
    Ruling {
        date: "2026-08-14",
        title: "Scaling track: the hand, the eye, the sweep",
        result: "S5 passed 19 of 20 audited acts; S5a passed 4 of 4 bootstrap and 4 of 4 transfer; S6 photographed three bodies side by side. S2, S3, S4 and S7 remained open on their stated tests.",
        path: "fish/album/2026-08-14T0358-EAT-scaling-S3-S5-rerun.md",
        hash: None,
    },
    Ruling {
        date: "2026-08-12",
        title: "Stage 6, the ocean, approved",
        result: "Mutual information rose from 0.0244 at 8 breaths to 0.0762 at 63 and saturated at 0.0712 at 128, far below the ceiling. Lexical-proxy measurement; a new metric epoch began the same day.",
        path: "docs/ROADMAP.md, stage 6",
        hash: None,
    },
    Ruling {
        date: "2026-08-05",
        title: "The second visit",
        result: "About 295 exchanges with a guest at the port while the keeper watched: what the fish is, measured. A library, not a workbench.",
        path: "fish/album/2026-08-05-the-second-visit.md",
        hash: None,
    },
    Ruling {
        date: "2026-08-04",
        title: "Bare-water control",
        result: "Same weights, same server, same sampling; the only variable is everything the project built. Bare water beside the fish, verbatim from the sediment.",
        path: "fish/album/2026-08-04-bare-water-control.md",
        hash: None,
    },
    Ruling {
        date: "2026-08-04",
        title: "The plateau, era one closed",
        result: "Median δ fell 0.479 → 0.369, then held near 0.40 ± 0.03 as the keeper's range grew: the memory-only ceiling.",
        path: "fish/album/2026-08-04-the-plateau-era-close.png",
        hash: None,
    },
    Ruling {
        date: "2026-08-03",
        title: "The first fall",
        result: "The skin's first full day. Median δ fell 0.48 → 0.34 by afternoon, crossing θ as the mirror learned the keeper in real time.",
        path: "fish/album/2026-08-03-the-first-fall.png",
        hash: None,
    },
];

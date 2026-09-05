//! Plain-language report summaries, newest first, with original paths and hashes.

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
        title: "Clean training transferred to new formal tasks",
        result: "The candidate produced 43 accepted answers on 64 new tasks without supplied calculation steps. The base model and shuffled-reply placebo each scored zero. The candidate had no paired losses or negative constants. method_transfer = true; deployment was not authorised.",
        path: "fish/album/2026-09-04T0043-EAT-clean-formal-claim-transfer.md",
        hash: Some("0655eaba6047287f17792f14c4e3d09a58b902ad41fc617fb4a98e107f20f3ba"),
    },
    Ruling {
        date: "2026-09-03",
        title: "Cleaner prompts improved place-value answers",
        result: "The clean adapter scored 18 of 18 with no unrelated constants. The base model and earlier adapter each scored 0 of 18. A full blinded transfer test was authorised; deployment was not.",
        path: "fish/album/2026-09-03T1945-EAT-clean-claim-safety.md",
        hash: Some("e4cbc7ee39576620e7482a0d51aecc5b81c19d542a8510476c2ee5df9b66fa8a"),
    },
    Ruling {
        date: "2026-09-03",
        title: "First transfer test failed a preset error limit",
        result: "The candidate scored 25 of 64 without supplied steps; both controls scored zero. However, 3 of 39 failed replies contained a negative constant, exceeding the 2 % limit. The result did not authorise further work with that adapter; contaminated training prompts were identified as a likely cause.",
        path: "fish/album/2026-09-03T1908-EAT-formal-claim-transfer-stage1.md",
        hash: Some("9caa5ba0bb6d8e7e99f623208b17577e0f5a4c9474721c37ec4da3f5cd02f286"),
    },
    Ruling {
        date: "2026-09-03",
        title: "Structured-answer training improved scores but failed one criterion",
        result: "With calculation steps supplied, the candidate scored 94 of 128, compared with 73 for the base model and 52 for the prediction adapter. It gained 21 tasks and lost none against the base model. One reply failed the negative-constant criterion, so the result authorised neither transfer testing nor deployment.",
        path: "fish/album/2026-09-03T0028-EAT-claim-coat-primary-ruling.md",
        hash: Some("a03699a11fefdf7ec8cb80caf145fd3d1d628483d08e78d4f96fda38f69c7dcc"),
    },
    Ruling {
        date: "2026-08-16",
        title: "Prediction-adapter deployment procedure",
        result: "The procedure and decision were recorded before the prediction adapter was activated.",
        path: "fish/album/2026-08-16T1950-EAT-prediction-coat-promotion-ruling.md",
        hash: None,
    },
    Ruling {
        date: "2026-08-14",
        title: "Full-system comparison found no measured gain or loss",
        result: "All 396 isolated requests completed, with the live database unchanged and no information leak. No memories were retrieved, even when retrieval was enabled. The result exposed a retrieval-threshold problem.",
        path: "fish/album/2026-08-14T074428-EAT-body-evolution.md",
        hash: None,
    },
    Ruling {
        date: "2026-08-14",
        title: "Tool use, image display and model comparison",
        result: "S5 passed 19 of 20 reviewed tool requests. S5a passed 4 of 4 initial tests and 4 of 4 transfer tests. S6 recorded comparable metrics for three models. S2, S3, S4 and S7 remained unresolved.",
        path: "fish/album/2026-08-14T0358-EAT-scaling-S3-S5-rerun.md",
        hash: None,
    },
    Ruling {
        date: "2026-08-12",
        title: "External-input study met its criterion",
        result: "Estimated mutual information rose from 0.0244 at 8 input cycles to 0.0762 at 63, then levelled off at 0.0712 at 128. This word-based estimate stayed below its maximum. Changes that day began a separate measurement period.",
        path: "docs/ROADMAP.md, stage 6",
        hash: None,
    },
    Ruling {
        date: "2026-08-05",
        title: "Guest conversation and memory observations",
        result: "About 295 exchanges with a guest, observed by the human supervisor, examined Fish's behaviour and the limits of memory retrieval as a reasoning aid.",
        path: "fish/album/2026-08-05-the-second-visit.md",
        hash: None,
    },
    Ruling {
        date: "2026-08-04",
        title: "Base-model control",
        result: "The comparison kept model weights, server and sampling settings the same. It compared the base model with the full Fish system and recorded both sets of replies directly from the database.",
        path: "fish/album/2026-08-04-bare-water-control.md",
        hash: None,
    },
    Ruling {
        date: "2026-08-04",
        title: "Prediction error levelled off in the first observation period",
        result: "Median δ fell from 0.479 to 0.369, then stayed near 0.40 ± 0.03 as the regular participant's messages became more varied. This records the plateau reached in that setting.",
        path: "fish/album/2026-08-04-the-plateau-era-close.png",
        hash: None,
    },
    Ruling {
        date: "2026-08-03",
        title: "Prediction error fell during the first full day",
        result: "Median δ fell from 0.48 to 0.34 by afternoon, crossing the memory threshold θ. The predictor became more accurate for the regular participant during that observation period.",
        path: "fish/album/2026-08-03-the-first-fall.png",
        hash: None,
    },
];

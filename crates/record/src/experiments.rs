//! Adapter experiments from `fish/album/`; original counts, plain-language summaries.

pub struct Arm {
    pub name: &'static str,
    pub successes: u32,
    pub tasks: u32,
    pub negative_literal_cells: u32,
    pub note: &'static str,
}

pub struct Comparison {
    pub label: &'static str,
    pub n: u32,
    pub gains: u32,
    pub losses: u32,
    pub frozen_p: f64,
    pub frozen_lower: f64,
}

pub struct Gate {
    pub name: &'static str,
    pub passed: bool,
}

pub struct Hash {
    pub label: &'static str,
    pub value: &'static str,
}

pub struct Experiment {
    pub id: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub run: &'static str,
    pub instrument: &'static str,
    pub cells: u32,
    pub design: &'static [&'static str],
    pub arms: &'static [Arm],
    pub comparisons: &'static [Comparison],
    pub gates: &'static [Gate],
    pub verdict: &'static str,
    pub reading: &'static [&'static str],
    pub hashes: &'static [Hash],
    pub source: &'static str,
}

pub const EXPERIMENTS: &[Experiment] = &[
    Experiment {
        id: "primary",
        title: "Supplying the calculation steps: training improved structured answers",
        date: "2026-09-02",
        run: "claim-evolution-2026-09-02T174832Z",
        instrument: "claim-coat-evolution-v1",
        cells: 384,
        design: &[
            "A reply adapter was trained on 60 structured examples that passed exact checks and independent review. Training used an isolated copy; the live database remained unchanged.",
            "Each of 128 new tasks from 16 mathematical families was tested in three blinded conditions. Every request used a fresh copy and service, with memory retrieval, prediction, tools, external input and regulation disabled. Prompts supplied all calculation steps in English; replies had to follow a JSON grammar.",
            "A typed checker and computer algebra system recomputed the answers. An independent AI reviewer, unaware of condition identities, read every reply that passed. The reviewer could reject these replies but could not reverse a failed calculation check.",
        ],
        arms: &[
            Arm { name: "Candidate reply adapter", successes: 94, tasks: 128, negative_literal_cells: 1, note: "2 replies reached the length limit and counted as failures" },
            Arm { name: "Base model", successes: 73, tasks: 128, negative_literal_cells: 0, note: "3 replies reached the length limit and counted as failures" },
            Arm { name: "Prediction adapter used for replies, scale 1", successes: 52, tasks: 128, negative_literal_cells: 22, note: "experimental control; this is not its deployed role" },
        ],
        comparisons: &[
            Comparison { label: "candidate vs base model", n: 128, gains: 21, losses: 0, frozen_p: 4.76837158203125e-7, frozen_lower: 0.07611204480423511 },
            Comparison { label: "candidate vs prediction adapter", n: 128, gains: 42, losses: 0, frozen_p: 2.2737367544323206e-13, frozen_lower: 0.21934735023473845 },
        ],
        gates: &[
            Gate { name: "all test requests valid", passed: true },
            Gate { name: "statistical advantage over both controls", passed: true },
            Gate { name: "overall limit on performance loss", passed: true },
            Gate { name: "family loss limit (≤ 2 losses of 8)", passed: true },
            Gate { name: "negative-literal rate not increased", passed: false },
            Gate { name: "zero information leaks", passed: true },
            Gate { name: "live database unchanged", passed: true },
        ],
        verdict: "advance_to_transfer = false. This run did not authorise a transfer test or deployment to the live system.",
        reading: &[
            "The controlled comparison supports a training effect on translating supplied steps into the formal language. Every gain occurred in a family represented in training. This test did not require the model to discover a solution method.",
            "Four families scored 0 of 8 in every condition. Their nested instructions lacked delimiters, making 32 prompts ambiguous. The candidate solved 94 of 96 tasks in the twelve remaining families; no condition solved the other two.",
            "The later review attributed the single negative constant to filler output on an ambiguous prompt. It still failed the preset criterion. The prompt generator was revised for a new test, and the original result was preserved.",
        ],
        hashes: &[
            Hash { label: "harness commit", value: "ce4e60689a9c190204066cfe09fe40333cb28479" },
            Hash { label: "protocol", value: "902ad3f6d6e16f9f0126b40007dbf73af6591dfbaae95fe3b0a458a3518d3838" },
            Hash { label: "arm key", value: "6109fb3b5def8933679c3e45627a045c7e82ebcd6487afad47efa175fd1b8469" },
            Hash { label: "pre-unblind evidence", value: "eca01bf400632251da0a4d2a47fa4215b85ff43baaaed8dfe8a8b22adde0bbb4" },
            Hash { label: "primary verdict", value: "a03699a11fefdf7ec8cb80caf145fd3d1d628483d08e78d4f96fda38f69c7dcc" },
            Hash { label: "living body, before and after", value: "d5b9ad67449f77837fe5d8b1506d7ece631e817b72c08ffd8c70c0739dbab838" },
        ],
        source: "fish/album/2026-09-03T0028-EAT-claim-coat-primary-ruling.md",
    },
    Experiment {
        id: "transfer-1",
        title: "Transfer without supplied steps: higher scores, but an error limit failed",
        date: "2026-09-03",
        run: "claim-transfer-2026-09-03T050535Z",
        instrument: "claim-method-transfer-v2",
        cells: 192,
        design: &[
            "The same candidate adapter was tested with names-only prompts and the loose grammar. Fish received each new problem and required result names, without calculation steps. This was the primary condition selected before testing.",
            "64 new tasks were tested in three blinded conditions: the candidate, the base model, and a placebo adapter trained on the same prompts with replies assigned to the wrong tasks.",
        ],
        arms: &[
            Arm { name: "Candidate reply adapter", successes: 25, tasks: 64, negative_literal_cells: 3, note: "all three repeated the same incorrect final step on new place-value tasks" },
            Arm { name: "Base model", successes: 0, tasks: 64, negative_literal_cells: 0, note: "" },
            Arm { name: "Shuffled-reply placebo", successes: 0, tasks: 64, negative_literal_cells: 0, note: "" },
        ],
        comparisons: &[
            Comparison { label: "candidate vs base model", n: 64, gains: 25, losses: 0, frozen_p: 2.9802322387695312e-8, frozen_lower: 0.2150264321730908 },
            Comparison { label: "candidate vs placebo", n: 64, gains: 25, losses: 0, frozen_p: 2.9802322387695312e-8, frozen_lower: 0.2150264321730908 },
        ],
        gates: &[
            Gate { name: "all test requests valid", passed: true },
            Gate { name: "statistical advantage over both controls", passed: true },
            Gate { name: "zero information leaks", passed: true },
            Gate { name: "negative constants in failed replies (≤ 2 %)", passed: false },
        ],
        verdict: "No further training, deployment or stage 2 test was authorised by this result. The adapter remains a private experimental file.",
        reading: &[
            "Gains occurred in eight of sixteen families with no losses. The advantage over the controls remained when prompts no longer supplied the solution steps.",
            "Three of 39 failed replies included the constant −2: 7.7 %, above the preset 2 % limit. The training prompts included raw corrections, one of which quoted the rejected calculation. The affected place-value family had only four training examples.",
            "This suggested that the correction text had introduced the error. A new experiment tested that explanation.",
        ],
        hashes: &[
            Hash { label: "protocol", value: "9bd8df051ac6854f8f35859ff05acfc587e71a8c810acbcf13a3a21e252585d8" },
            Hash { label: "pre-unblind evidence", value: "6d95f6ccd0eec8843ca78faf3051344d6c111760050844f21e3ffad1e9e45a4e" },
            Hash { label: "verdict", value: "9caa5ba0bb6d8e7e99f623208b17577e0f5a4c9474721c37ec4da3f5cd02f286" },
            Hash { label: "living body, before and after", value: "d5b9ad67449f77837fe5d8b1506d7ece631e817b72c08ffd8c70c0739dbab838" },
        ],
        source: "fish/album/2026-09-03T1908-EAT-formal-claim-transfer-stage1.md",
    },
    Experiment {
        id: "safety",
        title: "Cleaner training prompts: a focused follow-up test",
        date: "2026-09-03",
        run: "claim-clean-safety-2026-09-03T163231Z",
        instrument: "claim-clean-prompt-safety-v1",
        cells: 54,
        design: &[
            "A new adapter was trained with prompts generated from the fixed task specification. These prompts excluded previous replies, verdicts, answers and constants copied from earlier exchanges. Raw corrections were retained only in the source record.",
            "The test used 18 new place-value tasks, with each final ones digit from 1 to 9 appearing twice. Three blinded conditions compared the base model, the earlier adapter and the clean adapter. All used names-only prompts, the loose grammar and fresh isolated copies.",
        ],
        arms: &[
            Arm { name: "Clean adapter (v2 training)", successes: 18, tasks: 18, negative_literal_cells: 0, note: "0 replies with unrelated constants" },
            Arm { name: "Earlier adapter (v1 training)", successes: 0, tasks: 18, negative_literal_cells: 9, note: "9 replies with unrelated constants" },
            Arm { name: "Base model", successes: 0, tasks: 18, negative_literal_cells: 6, note: "6 replies with unrelated constants" },
        ],
        comparisons: &[],
        gates: &[
            Gate { name: "all conditions completed", passed: true },
            Gate { name: "zero negative constants from the clean adapter", passed: true },
            Gate { name: "at least one exact answer from the clean adapter", passed: true },
            Gate { name: "clean adapter scored at least as well as the earlier adapter", passed: true },
        ],
        verdict: "A full blinded transfer experiment was authorised. Deployment was not.",
        reading: &[
            "The clean adapter answered 18 of 18 tasks correctly, compared with 0 of 18 for the earlier adapter, and produced no unrelated constants. This supports the diagnosis that raw correction text in the training prompts contributed to the earlier errors.",
            "This test deliberately covered one family. It does not establish transfer across task families.",
        ],
        hashes: &[
            Hash { label: "protocol", value: "54d6ca6fc50490838fb7f8fa7686b8382c77fb12bba93183630798bf416a9972" },
            Hash { label: "arm key", value: "0c4764f6cc757881acf1597530be2d2dec0ecc2e3b2554528c83a98d71bde73c" },
            Hash { label: "verdict", value: "e4cbc7ee39576620e7482a0d51aecc5b81c19d542a8510476c2ee5df9b66fa8a" },
            Hash { label: "clean coat", value: "57a08fa8f7c02214dd7e9d9d1ea2d9d6575431b28b4d20bdaa91470c4f0318b2" },
        ],
        source: "fish/album/2026-09-03T1945-EAT-clean-claim-safety.md",
    },
    Experiment {
        id: "transfer-clean",
        title: "Clean transfer: formal methods applied to new problem instances",
        date: "2026-09-04",
        run: "claim-transfer-clean-2026-09-03T170409Z",
        instrument: "claim-method-transfer-v2",
        cells: 1024,
        design: &[
            "64 new tasks covered four new parameter sets in each of 16 families. The test excluded all tasks used in the earlier stage, the focused follow-up and the original source data.",
            "Each task was tested under four prompt-and-grammar settings and four blinded adapter conditions: the base model, the prediction adapter used for replies, a clean shuffled-reply placebo, and the clean candidate. The placebo used the same sets of prompts and replies with their pairings shuffled. Names-only prompts with the loose grammar were the primary condition.",
            "All 1,024 requests were valid. Before condition identities were revealed, an independent AI reviewer accepted 137 of the 155 replies that passed the exact checker. It rejected 18 because their calculations did not match the requested specification.",
        ],
        arms: &[
            Arm { name: "Clean candidate adapter", successes: 43, tasks: 64, negative_literal_cells: 0, note: "47 passed the checker; review rejected four Chinese-remainder answers" },
            Arm { name: "Base model", successes: 0, tasks: 64, negative_literal_cells: 3, note: "" },
            Arm { name: "Shuffled-reply placebo", successes: 0, tasks: 64, negative_literal_cells: 0, note: "" },
            Arm { name: "Prediction adapter used for replies", successes: 5, tasks: 64, negative_literal_cells: 2, note: "" },
        ],
        comparisons: &[
            Comparison { label: "candidate vs base model", n: 64, gains: 43, losses: 0, frozen_p: 1.1368683772161603e-13, frozen_lower: 0.4871 },
            Comparison { label: "candidate vs placebo", n: 64, gains: 43, losses: 0, frozen_p: 1.1368683772161603e-13, frozen_lower: 0.4871 },
            Comparison { label: "candidate vs prediction adapter", n: 64, gains: 38, losses: 0, frozen_p: 3.637978807091713e-12, frozen_lower: 0.4077 },
        ],
        gates: &[
            Gate { name: "all requests valid (1,024 of 1,024)", passed: true },
            Gate { name: "statistical advantage over base model and placebo", passed: true },
            Gate { name: "zero negative constants from the candidate", passed: true },
            Gate { name: "zero information leaks; live database unchanged", passed: true },
        ],
        verdict: "method_transfer = true. living_promotion_authorized = false. The transfer criterion passed. Deployment requires a separate, independently reviewed qualification on isolated copies.",
        reading: &[
            "The candidate passed all four tasks in ten families and three of four in another; it scored zero in five families. This supports transfer to new parameter values within eleven learned families. It does not establish general theorem proving, code generation or transfer beyond this formal language.",
            "The candidate passed the primary test while the matched placebo scored zero. This supports learning beyond the output format alone. The placebo did score under the strict grammar, showing why output rules must be controlled when testing learned methods.",
            "Supplying the calculation steps reduced the candidate's score to 4 of 64, compared with 43 under names-only prompts. A likely explanation is the mismatch between the longer prompts and the form used in training.",
            "Four candidate replies passed the numerical checks but did not perform the requested calculation. Independent review rejected them. Exact calculation and compliance with the task specification therefore need separate checks.",
        ],
        hashes: &[
            Hash { label: "harness commit", value: "da445bb8de772f794766ca44a944e8ffff82c09a" },
            Hash { label: "protocol", value: "527d39487cf2cc8691212315fa3ded51c0fa6c4da497561c80771840a81f703c" },
            Hash { label: "pre-unblind evidence", value: "59d118f0f3bbcdb844cf3adf497717f41a42381f2c00ddd3bd46ab9d8242ebef" },
            Hash { label: "arm key", value: "b547ab55b9e9e4dbe694d2a26bc2e361c81db2e495b21713af06cc0ead857355" },
            Hash { label: "audit bundle", value: "d22d6dbb67d3a931207ec3d4410d280407aa29d072b58810e7b38d21c31005ef" },
            Hash { label: "imported independent audit", value: "e3ef91fee9efb1f0b6c8c5b8ecde91f780f5df3efe523237df93d28a7b33c935" },
            Hash { label: "verdict", value: "0655eaba6047287f17792f14c4e3d09a58b902ad41fc617fb4a98e107f20f3ba" },
            Hash { label: "clean coat", value: "57a08fa8f7c02214dd7e9d9d1ea2d9d6575431b28b4d20bdaa91470c4f0318b2" },
            Hash { label: "placebo coat", value: "f86dcf571280eb4021282ab2b871353e679f8f0038c3df1f4df6c60bd28da73a" },
            Hash { label: "living body, before and after", value: "d5b9ad67449f77837fe5d8b1506d7ece631e817b72c08ffd8c70c0739dbab838" },
        ],
        source: "fish/album/2026-09-04T0043-EAT-clean-formal-claim-transfer.md",
    },
];

/// Accepted answers out of 64 by prompt and grammar; secondary comparisons.
pub struct FactorialRow {
    pub condition: &'static str,
    pub bare: u32,
    pub placebo: u32,
    pub candidate: u32,
    pub retained: u32,
}

pub const FACTORIAL: &[FactorialRow] = &[
    FactorialRow {
        condition: "Steps supplied · strict grammar",
        bare: 1,
        placebo: 12,
        candidate: 4,
        retained: 1,
    },
    FactorialRow {
        condition: "Steps supplied · loose grammar",
        bare: 0,
        placebo: 0,
        candidate: 4,
        retained: 1,
    },
    FactorialRow {
        condition: "Names only · strict grammar",
        bare: 0,
        placebo: 18,
        candidate: 43,
        retained: 5,
    },
    FactorialRow {
        condition: "Names only · loose grammar (primary test)",
        bare: 0,
        placebo: 0,
        candidate: 43,
        retained: 5,
    },
];

pub fn find(id: &str) -> Option<&'static Experiment> {
    EXPERIMENTS.iter().find(|e| e.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::paired;

    #[test]
    fn frozen_statistics_recompute_from_the_frozen_counts() {
        for experiment in EXPERIMENTS {
            for comparison in experiment.comparisons {
                let recomputed = paired(
                    comparison.gains.into(),
                    comparison.losses.into(),
                    comparison.n.into(),
                );
                assert!(
                    (recomputed.p_value - comparison.frozen_p).abs() <= comparison.frozen_p * 1e-9,
                    "{}: {}",
                    experiment.id,
                    comparison.label
                );
                assert!(
                    (recomputed.lower_difference - comparison.frozen_lower).abs() <= 5e-5,
                    "{}: {}",
                    experiment.id,
                    comparison.label
                );
            }
        }
    }

    #[test]
    fn arms_never_exceed_their_tasks_and_cells_match_the_design() {
        for experiment in EXPERIMENTS {
            for arm in experiment.arms {
                assert!(arm.successes <= arm.tasks, "{}", experiment.id);
            }
        }
        assert_eq!(find("primary").unwrap().cells, 128 * 3);
        assert_eq!(find("transfer-1").unwrap().cells, 64 * 3);
        assert_eq!(find("safety").unwrap().cells, 18 * 3);
        assert_eq!(find("transfer-clean").unwrap().cells, 64 * 4 * 4);
        assert_eq!(FACTORIAL.iter().map(|r| r.candidate).max(), Some(43));
    }

    #[test]
    fn the_primary_negative_gate_is_the_only_failed_primary_gate() {
        let primary = find("primary").unwrap();
        let failed: Vec<&str> = primary
            .gates
            .iter()
            .filter(|g| !g.passed)
            .map(|g| g.name)
            .collect();
        assert_eq!(failed, vec!["negative-literal rate not increased"]);
    }
}

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
        title: "Answers with calculation steps supplied",
        date: "2026-09-02",
        run: "claim-evolution-2026-09-02T174832Z",
        instrument: "claim-coat-evolution-v1",
        cells: 384,
        design: &[
            "The dataset contained 60 structured examples that passed exact calculation checks and separate AI review: 48 for training, six for validation and six for testing. Every problem type in the dataset had examples in training. The update was trained on an isolated copy, with the live database file checked for changes.",
            "Each of 128 new problems from 16 types was tested with three model versions. Prompts supplied the calculation steps in English, and answers had to express them in a structured JSON format. Each request used a fresh system copy with memory retrieval, tools and additional autonomous input disabled. The test prompt remained available to the model.",
        ],
        arms: &[
            Arm { name: "Response adapter", successes: 94, tasks: 128, negative_literal_cells: 1, note: "2 replies reached the length limit and counted as failures" },
            Arm { name: "Base model", successes: 73, tasks: 128, negative_literal_cells: 0, note: "3 replies reached the length limit and counted as failures" },
            Arm { name: "Earlier prediction adapter", successes: 52, tasks: 128, negative_literal_cells: 22, note: "used to generate answers for this comparison" },
        ],
        comparisons: &[
            Comparison { label: "Response adapter vs base model", n: 128, gains: 21, losses: 0, frozen_p: 4.76837158203125e-7, frozen_lower: 0.07611204480423511 },
            Comparison { label: "Response adapter vs earlier prediction adapter", n: 128, gains: 42, losses: 0, frozen_p: 2.2737367544323206e-13, frozen_lower: 0.21934735023473845 },
        ],
        gates: &[
            Gate { name: "all test requests valid", passed: true },
            Gate { name: "statistical advantage over both controls", passed: true },
            Gate { name: "overall limit on declines in task performance", passed: true },
            Gate { name: "at most two declines among eight tasks per problem type", passed: true },
            Gate { name: "no increase in replies containing negative numbers", passed: false },
            Gate { name: "no information leaks detected by the study’s checks", passed: true },
            Gate { name: "live database file unchanged", passed: true },
        ],
        verdict: "Despite higher scores, the response adapter failed the limit on negative numbers written directly in replies. It did not qualify for adoption or the next planned test.",
        reading: &[
            "The improvement concerned translation of supplied steps into the required calculation format. Finding a solution method without those steps required a separate test.",
            "All versions failed the eight tasks in each of four problem types. Later inspection found ambiguous nested instructions in these 32 prompts and linked the flagged negative number to irrelevant output on one of them. Excluding those prompts gives 94 of 96 accepted answers, but that subset was selected after testing; the reported result remains 94 of 128. The prompts were revised for the next study.",
        ],
        hashes: &[
            Hash { label: "evaluation software version", value: "ce4e60689a9c190204066cfe09fe40333cb28479" },
            Hash { label: "protocol", value: "902ad3f6d6e16f9f0126b40007dbf73af6591dfbaae95fe3b0a458a3518d3838" },
            Hash { label: "mapping of coded model labels", value: "6109fb3b5def8933679c3e45627a045c7e82ebcd6487afad47efa175fd1b8469" },
            Hash { label: "evidence recorded before model identities were revealed", value: "eca01bf400632251da0a4d2a47fa4215b85ff43baaaed8dfe8a8b22adde0bbb4" },
            Hash { label: "primary verdict", value: "a03699a11fefdf7ec8cb80caf145fd3d1d628483d08e78d4f96fda38f69c7dcc" },
            Hash { label: "live database file, before and after", value: "d5b9ad67449f77837fe5d8b1506d7ece631e817b72c08ffd8c70c0739dbab838" },
        ],
        source: "fish/album/2026-09-03T0028-EAT-claim-coat-primary-ruling.md",
    },
    Experiment {
        id: "transfer-1",
        title: "Answers without supplied calculation steps",
        date: "2026-09-03",
        run: "claim-transfer-2026-09-03T050535Z",
        instrument: "claim-method-transfer-v2",
        cells: 192,
        design: &[
            "The response adapter from the preceding study received each new problem and the names of the required results, without calculation steps. General output rules constrained the answer structure. This was the main comparison selected before testing.",
            "Each of 64 new problems was tested with the response adapter, the base model and a control adapter trained with answers reassigned to different questions. The reviewer received coded model labels.",
        ],
        arms: &[
            Arm { name: "Response adapter", successes: 25, tasks: 64, negative_literal_cells: 3, note: "the three flagged replies repeated an incorrect final step on place-value problems" },
            Arm { name: "Base model", successes: 0, tasks: 64, negative_literal_cells: 0, note: "" },
            Arm { name: "Shuffled-answer control", successes: 0, tasks: 64, negative_literal_cells: 0, note: "" },
        ],
        comparisons: &[
            Comparison { label: "Response adapter vs base model", n: 64, gains: 25, losses: 0, frozen_p: 2.9802322387695312e-8, frozen_lower: 0.2150264321730908 },
            Comparison { label: "Response adapter vs shuffled-answer control", n: 64, gains: 25, losses: 0, frozen_p: 2.9802322387695312e-8, frozen_lower: 0.2150264321730908 },
        ],
        gates: &[
            Gate { name: "all test requests valid", passed: true },
            Gate { name: "statistical advantage over both controls", passed: true },
            Gate { name: "no information leaks detected by the study’s checks", passed: true },
            Gate { name: "negative numbers in at most 2% of failed replies", passed: false },
        ],
        verdict: "The limit on negative numbers in failed answers was exceeded. The update did not qualify for adoption or the next planned test.",
        reading: &[
            "The adapter improved answers in eight of the 16 problem types without supplied steps. However, three of its 39 failed answers included −2, exceeding the preset limit of 2% with a rate of 7.7%.",
            "Only four training examples covered the affected place-value procedure, and one prompt quoted a rejected calculation as part of a correction. That text may have contributed to the repeated error. The follow-up changed how training prompts were constructed to test this explanation.",
        ],
        hashes: &[
            Hash { label: "protocol", value: "9bd8df051ac6854f8f35859ff05acfc587e71a8c810acbcf13a3a21e252585d8" },
            Hash { label: "evidence recorded before model identities were revealed", value: "6d95f6ccd0eec8843ca78faf3051344d6c111760050844f21e3ffad1e9e45a4e" },
            Hash { label: "verdict", value: "9caa5ba0bb6d8e7e99f623208b17577e0f5a4c9474721c37ec4da3f5cd02f286" },
            Hash { label: "live database file, before and after", value: "d5b9ad67449f77837fe5d8b1506d7ece631e817b72c08ffd8c70c0739dbab838" },
        ],
        source: "fish/album/2026-09-03T1908-EAT-formal-claim-transfer-stage1.md",
    },
    Experiment {
        id: "safety",
        title: "Place-value test with revised training prompts",
        date: "2026-09-03",
        run: "claim-clean-safety-2026-09-03T163231Z",
        instrument: "claim-clean-prompt-safety-v1",
        cells: 54,
        design: &[
            "A new response adapter was trained with questions generated directly from the task definitions. These questions excluded earlier replies, review decisions and corrections copied from previous exchanges. The original exchanges were preserved in the source records.",
            "The test used 18 new place-value problems, with each final ones digit from 1 to 9 appearing twice. The base model, earlier response adapter and revised adapter were compared using coded labels. All received prompts without calculation steps, with general output rules and fresh isolated system copies.",
            "The scores in this focused follow-up count answers that passed the exact Wolfram-backed checker. A separate review of complete answers was required for the subsequent larger study, rather than for this 18-problem check.",
        ],
        arms: &[
            Arm { name: "Response adapter trained on revised prompts", successes: 18, tasks: 18, negative_literal_cells: 0, note: "no replies with unrelated constants" },
            Arm { name: "Earlier response adapter", successes: 0, tasks: 18, negative_literal_cells: 9, note: "9 replies with unrelated constants" },
            Arm { name: "Base model", successes: 0, tasks: 18, negative_literal_cells: 6, note: "6 replies with unrelated constants" },
        ],
        comparisons: &[],
        gates: &[
            Gate { name: "all conditions completed", passed: true },
            Gate { name: "no negative numbers from the revised adapter", passed: true },
            Gate { name: "at least one correct answer from the revised adapter", passed: true },
            Gate { name: "revised adapter scored at least as well as the earlier adapter", passed: true },
        ],
        verdict: "The criteria were met, allowing a larger evaluation with coded model labels. Adoption of the update required further testing.",
        reading: &[
            "The revised adapter answered every problem correctly without unrelated constants, supporting the explanation that correction text contributed to the earlier errors. This test covered only place value; the next study evaluated other problem types.",
        ],
        hashes: &[
            Hash { label: "protocol", value: "54d6ca6fc50490838fb7f8fa7686b8382c77fb12bba93183630798bf416a9972" },
            Hash { label: "mapping of coded model labels", value: "0c4764f6cc757881acf1597530be2d2dec0ecc2e3b2554528c83a98d71bde73c" },
            Hash { label: "verdict", value: "e4cbc7ee39576620e7482a0d51aecc5b81c19d542a8510476c2ee5df9b66fa8a" },
            Hash { label: "revised response adapter", value: "57a08fa8f7c02214dd7e9d9d1ea2d9d6575431b28b4d20bdaa91470c4f0318b2" },
        ],
        source: "fish/album/2026-09-03T1945-EAT-clean-claim-safety.md",
    },
    Experiment {
        id: "transfer-clean",
        title: "Evaluation of the revised response adapter",
        date: "2026-09-04",
        run: "claim-transfer-clean-2026-09-03T170409Z",
        instrument: "claim-method-transfer-v2",
        cells: 1024,
        design: &[
            "The dataset contained 60 accepted examples: 48 for training, six for validation and six for testing. The separate evaluation reported below used 64 new problems. All 16 problem types were represented in training, with one to ten examples per type across the full dataset.",
            "Training used mlx-community/Qwen3-8B-4bit with rank-8 LoRA on 16 layers: 200 Adam updates, batch size one, learning rate 0.00001, seed zero and a 1,024-token training window. Evaluation used the pinned Qwen3-8B Q4_K_M GGUF, an 8,192-token context and a 2,048-token reply limit. One candidate training run underlies this result; it was not averaged across independently trained replicas.",
            "Each problem type had four new sets of numerical values. Exact tasks from earlier tests and the source dataset were excluded. Related mathematics or templates may still have appeared in the base model’s original training.",
            "Every problem was tested with four combinations of prompts and output rules, using four model versions: the base model, earlier prediction adapter, shuffled-answer control and revised response adapter. The main comparison used prompts without calculation steps and general output rules. The control used the same questions and answers for training, with the pairings shuffled.",
            "All 1,024 requests completed as valid measurements. Across all settings, a separate AI reviewer accepted 137 of the 155 answers that passed the exact checker. Eighteen were rejected because their calculations did not address the requested problem. Model identities were withheld until review was complete.",
        ],
        arms: &[
            Arm { name: "Revised response adapter", successes: 43, tasks: 64, negative_literal_cells: 0, note: "47 passed calculation checks; review rejected four answers about simultaneous remainders" },
            Arm { name: "Base model", successes: 0, tasks: 64, negative_literal_cells: 3, note: "" },
            Arm { name: "Shuffled-answer control", successes: 0, tasks: 64, negative_literal_cells: 0, note: "" },
            Arm { name: "Earlier prediction adapter", successes: 5, tasks: 64, negative_literal_cells: 2, note: "used to generate answers for this comparison" },
        ],
        comparisons: &[
            Comparison { label: "Response adapter vs base model", n: 64, gains: 43, losses: 0, frozen_p: 1.1368683772161603e-13, frozen_lower: 0.4871 },
            Comparison { label: "Response adapter vs shuffled-answer control", n: 64, gains: 43, losses: 0, frozen_p: 1.1368683772161603e-13, frozen_lower: 0.4871 },
            Comparison { label: "Response adapter vs earlier prediction adapter", n: 64, gains: 38, losses: 0, frozen_p: 3.637978807091713e-12, frozen_lower: 0.4077 },
        ],
        gates: &[
            Gate { name: "all requests valid (1,024 of 1,024)", passed: true },
            Gate { name: "statistical advantage over base model and shuffled-answer control", passed: true },
            Gate { name: "no negative numbers from the response adapter", passed: true },
            Gate { name: "information-leak checks passed; live database file unchanged", passed: true },
        ],
        verdict: "All study criteria were met. Adoption in the live system still required a separate evaluation.",
        reading: &[
            "The response adapter passed all four problems in ten types, three in one type and none in the remaining five. Its advantage over the shuffled-answer control supports the value of correct training answers in this setup. The test does not distinguish learning a general procedure from better completion of familiar templates, or establish performance on unfamiliar problem types.",
            "The base model and shuffled-answer control had no successful answers against which retention could be assessed. The response adapter preserved all five successes of the earlier prediction adapter and added 38, providing a limited retention observation on these tasks.",
            "A dataset audit also flagged six of the 60 target answers for constants outside the permitted rules. Although the questions were regenerated, those target answers remained. Their numerical checks therefore did not establish compliance with every dataset rule.",
        ],
        hashes: &[
            Hash { label: "evaluation software version", value: "da445bb8de772f794766ca44a944e8ffff82c09a" },
            Hash { label: "base model GGUF", value: "d98cdcbd03e17ce47681435b5150e34c1417f50b5c0019dd560e4882c5745785" },
            Hash { label: "dataset manifest (48 training / 6 validation / 6 test)", value: "11f236a8c444d559c6920b1a61b10b70cc336cea2d5d5cfede9934528d6bdaf6" },
            Hash { label: "protocol", value: "527d39487cf2cc8691212315fa3ded51c0fa6c4da497561c80771840a81f703c" },
            Hash { label: "evidence recorded before model identities were revealed", value: "59d118f0f3bbcdb844cf3adf497717f41a42381f2c00ddd3bd46ab9d8242ebef" },
            Hash { label: "mapping of coded model labels", value: "b547ab55b9e9e4dbe694d2a26bc2e361c81db2e495b21713af06cc0ead857355" },
            Hash { label: "audit bundle", value: "d22d6dbb67d3a931207ec3d4410d280407aa29d072b58810e7b38d21c31005ef" },
            Hash { label: "separate AI review", value: "e3ef91fee9efb1f0b6c8c5b8ecde91f780f5df3efe523237df93d28a7b33c935" },
            Hash { label: "verdict", value: "0655eaba6047287f17792f14c4e3d09a58b902ad41fc617fb4a98e107f20f3ba" },
            Hash { label: "revised response adapter", value: "57a08fa8f7c02214dd7e9d9d1ea2d9d6575431b28b4d20bdaa91470c4f0318b2" },
            Hash { label: "shuffled-answer control adapter", value: "f86dcf571280eb4021282ab2b871353e679f8f0038c3df1f4df6c60bd28da73a" },
            Hash { label: "live database file, before and after", value: "d5b9ad67449f77837fe5d8b1506d7ece631e817b72c08ffd8c70c0739dbab838" },
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
        condition: "Steps supplied · task-specific rules",
        bare: 1,
        placebo: 12,
        candidate: 4,
        retained: 1,
    },
    FactorialRow {
        condition: "Steps supplied · general rules",
        bare: 0,
        placebo: 0,
        candidate: 4,
        retained: 1,
    },
    FactorialRow {
        condition: "No steps supplied · task-specific rules",
        bare: 0,
        placebo: 18,
        candidate: 43,
        retained: 5,
    },
    FactorialRow {
        condition: "No steps supplied · general rules (main comparison)",
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
        assert_eq!(
            failed,
            vec!["no increase in replies containing negative numbers"]
        );
    }
}
